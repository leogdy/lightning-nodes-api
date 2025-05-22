use actix_web::{web, HttpResponse, get, post, App, HttpServer, middleware::Logger};
use dotenv::dotenv;
use log::{info, warn, error};
use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite};
use anyhow::{Result, Context};
use chrono::{Utc, TimeZone};
use tokio::time::{self, Duration};

pub mod models;
use models::{ExternalNode, DbNodeForApiResponse, NodeResponse};

/// Application state, shared across handlers.
pub struct AppState {
    pub db_pool: Pool<Sqlite>,
    pub api_url: String,
}

/// The main entry point for the application's logic.
pub async fn run_app() -> Result<()> {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite://./data/lightning_nodes.db".into());
    let api_url = std::env::var("API_URL").unwrap_or_else(|_| "https://mempool.space/api/v1/lightning/nodes/rankings/connectivity".into());
    let server_addr = std::env::var("SERVER_ADDR").unwrap_or_else(|_| "0.0.0.0:8080".into());
    let import_interval_secs: u64 = std::env::var("IMPORT_INTERVAL_SECS").unwrap_or_else(|_| "600".into()).parse().unwrap_or(600);

    info!("Connecting to DB: {}", database_url);
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .context("Failed to connect to the database")?;

    sqlx::query(r#"
        CREATE TABLE IF NOT EXISTS nodes (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            public_key TEXT NOT NULL UNIQUE,
            alias TEXT,
            channels INTEGER,
            capacity INTEGER,
            first_seen INTEGER,
            updated_at INTEGER,
            city TEXT,
            country TEXT,
            imported_at TEXT DEFAULT CURRENT_TIMESTAMP
        )
    "#)
    .execute(&pool)
    .await
    .context("Failed to create 'nodes' table")?;

    let pool_clone = pool.clone();
    let api_clone = api_url.clone();
    tokio::spawn(async move {
        info!("Periodic import every {} seconds", import_interval_secs);
        let mut interval = time::interval(Duration::from_secs(import_interval_secs));
        loop {
            interval.tick().await;
            info!("Triggering periodic data import...");
            if let Err(e) = import_nodes_from_api(&api_clone, &pool_clone).await {
                error!("Periodic import failed: {:?}", e);
            }
        }
    });

    info!("Performing initial data import...");
    if let Err(e) = import_nodes_from_api(&api_url, &pool).await {
        warn!("Initial import failed: {:?}. Starting with stale/no data.", e);
    } else {
        info!("Initial data import successful.");
    }

    let app_state = web::Data::new(AppState { db_pool: pool, api_url });
    info!("Starting server at http://{}", server_addr);

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(app_state.clone())
            .service(health_check)
            .service(get_nodes)
            .service(trigger_import_admin)
    })
    .bind(&server_addr)?
    .run()
    .await
    .context("Server run failure")?;

    Ok(())
}

/// Health check endpoint.
#[get("/health")]
pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().body("OK")
}

/// GET /nodes: Fetches and returns formatted Lightning nodes from the database.
#[get("/nodes")]
pub async fn get_nodes(state: web::Data<AppState>) -> HttpResponse {
    match get_all_nodes_for_api(&state.db_pool).await {
        Ok(nodes) => HttpResponse::Ok().json(nodes),
        Err(e) => {
            error!("Error fetching nodes: {:?}", e);
            HttpResponse::InternalServerError().body("Internal server error while fetching nodes.")
        }
    }
}

/// POST /admin/import: Triggers a manual import of nodes from the external API.
#[post("/admin/import")]
pub async fn trigger_import_admin(state: web::Data<AppState>) -> HttpResponse {
    info!("Manual import triggered.");
    match import_nodes_from_api(&state.api_url, &state.db_pool).await {
        Ok(count) => HttpResponse::Ok().body(format!("Imported {} nodes", count)),
        Err(e) => {
            error!("Manual import failed: {:?}", e);
            HttpResponse::InternalServerError().body(format!("Import failed: {}", e))
        }
    }
}

/// Fetches nodes from the external API and upserts them into the database.
async fn import_nodes_from_api(api_url: &str, pool: &Pool<Sqlite>) -> Result<usize> {
    info!("Fetching from API: {}", api_url);
    let res = reqwest::get(api_url).await.context("API request failed")?;
    if !res.status().is_success() {
        let status = res.status();
        let body = res.text().await.unwrap_or_default();
        error!("API non-success {}: {}", status, body);
        return Err(anyhow::anyhow!("API error: Status {}, Body: {}", status, body));
    }
    let nodes: Vec<ExternalNode> = res.json().await.context("JSON parse failed")?;
    let count = nodes.len();
    let mut tx = pool.begin().await.context("Failed to begin database transaction")?;
    for node in nodes {
        let city = node.city.and_then(|c| serde_json::to_string(&c).ok());
        let country = node.country.and_then(|c| serde_json::to_string(&c).ok());
        sqlx::query(
            r#"INSERT INTO nodes
               (public_key, alias, channels, capacity, first_seen, updated_at, city, country, imported_at)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, CURRENT_TIMESTAMP)
               ON CONFLICT(public_key) DO UPDATE SET
                 alias=excluded.alias,
                 channels=excluded.channels,
                 capacity=excluded.capacity,
                 updated_at=excluded.updated_at,
                 city=excluded.city,
                 country=excluded.country,
                 imported_at=CURRENT_TIMESTAMP
            "#,
        )
        .bind(&node.public_key)
        .bind(&node.alias)
        .bind(node.channels)
        .bind(node.capacity)
        .bind(node.first_seen)
        .bind(node.updated_at)
        .bind(city)
        .bind(country)
        .execute(&mut *tx)
        .await
        .context("Failed to upsert node into database")?;
    }
    tx.commit().await.context("Failed to commit database transaction")?;
    info!("Imported {} nodes.", count);
    Ok(count)
}

/// Reads nodes from the database, converts capacity to BTC, and formats first_seen timestamp.
async fn get_all_nodes_for_api(pool: &Pool<Sqlite>) -> Result<Vec<NodeResponse>> {
    let rows: Vec<DbNodeForApiResponse> = sqlx::query_as(
        r#"SELECT public_key, alias, capacity, first_seen
           FROM nodes
           ORDER BY capacity DESC"#,
    )
    .fetch_all(pool)
    .await
    .context("Failed to fetch nodes from database")?;

    let mut out = Vec::with_capacity(rows.len());
    for r in rows {
        let btc = (r.capacity as f64) / 100_000_000.0;
        let ts = Utc.timestamp_opt(r.first_seen, 0).single()
            .unwrap_or(Utc::now());
        out.push(NodeResponse {
            public_key: r.public_key,
            alias: r.alias.unwrap_or_default(),
            capacity: format!("{:.8}", btc),
            first_seen: ts.to_rfc3339(),
        });
    }
    Ok(out)
}