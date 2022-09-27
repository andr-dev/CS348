use rocket::serde::Serialize;
use sqlx::FromRow;

#[derive(FromRow, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ResourceNotFoundError {}
