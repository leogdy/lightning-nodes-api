use serde::{Deserialize, Serialize};

/// Represents the structure of a node as received from the external API.
#[derive(Debug, Serialize, Deserialize)]
pub struct ExternalNode {
    #[serde(rename = "publicKey")]
    pub public_key: String,
    pub alias: String,
    pub channels: i32,
    pub capacity: i64,
    #[serde(rename = "firstSeen")]
    pub first_seen: i64,
    #[serde(rename = "updatedAt")]
    pub updated_at: i64,
    pub city: Option<std::collections::HashMap<String, String>>,
    pub country: Option<std::collections::HashMap<String, String>>,
}

/// Represents the simplified structure of a node when fetched from the database for API response generation.
#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct DbNodeForApiResponse {
    pub public_key: String,
    pub alias: Option<String>,
    pub capacity: i64,
    pub first_seen: i64,
}

/// Represents the final JSON structure for a node as served by the GET /nodes endpoint.
#[derive(Debug, Serialize)]
pub struct NodeResponse {
    pub public_key: String,
    pub alias: String,
    pub capacity: String,
    pub first_seen: String,
}