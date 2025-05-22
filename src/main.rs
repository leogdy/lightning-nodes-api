use lightning_nodes_api::run_app;
use anyhow::Result;

#[actix_web::main]
async fn main() -> Result<()> {
    run_app().await
}