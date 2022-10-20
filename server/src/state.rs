use rito::apis::configuration::ApiKey;
use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite};

use crate::{api::Champion, rito::client::RitoClient};

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

        AppState { pool, rito_client }.setup_db().await
    }

    pub async fn setup_db(self) -> Result<Self, Box<dyn std::error::Error>> {
        if Champion::is_table_empty(&self.pool)
            .await
            .map_err(|e| "failed to get is_table_empty")?
        {
            println!("FETCHING CHAMPIONS FROM RITO");

            let champions = self.rito_client.get_champions().await?;

            for (_, champ) in champions.data {
                println!("INSERTING CHAMP {:?}", champ.name);

                let champ = Champion::from_ddragon_champion(champ);

                Champion::insert(&self.pool, champ)
                    .await
                    .map_err(|e| "failed to insert champ")?;

                println!("INSERT CHAMP COPMLETE");
            }
        }

        Ok(self)
    }
}
