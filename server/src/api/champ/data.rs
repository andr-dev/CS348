use rocket::serde::Serialize;
use sqlx::{FromRow, Pool, Sqlite};

#[derive(FromRow, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Champ {
    pub name: String,
    pub id: i64,
}

impl Champ {
    pub async fn get_by_name(pool: &Pool<Sqlite>, name: String) -> Option<Champ> {
        sqlx::query_as::<_, Champ>("SELECT * FROM champs WHERE name = ?")
            .bind(name)
            .fetch_one(pool)
            .await
            .ok()
    }

    pub async fn get_by_id(pool: &Pool<Sqlite>, id: i64) -> Option<Champ> {
        sqlx::query_as::<_, Champ>("SELECT * FROM champs WHERE id = ?")
            .bind(id)
            .fetch_one(pool)
            .await
            .ok()
    }
}
