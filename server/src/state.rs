use rito::apis::configuration::ApiKey;
use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite};

use crate::rito::client::RitoClient;

pub struct AppState {
    pub pool: Pool<Sqlite>,
    pub rito_client: RitoClient,
}

impl AppState {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let pool = SqlitePoolOptions::new()
            .max_connections(4)
            .connect(&std::env::var("DATABASE_URL")?)
            .await?;

        let rg_api_key = std::env::var("RG_API_KEY")?;

        let rito_client = RitoClient::new(ApiKey {
            prefix: None,
            key: rg_api_key,
        });

        Ok(AppState { pool, rito_client })
    }
}
