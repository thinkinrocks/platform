use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, Deserialize)]
pub struct User {
    pub telegram_username: String,
    pub sire: Option<String>,
}

#[derive(Debug, Clone, FromRow, Serialize)]
pub struct Entry {
    pub id: String,
    pub name: String,
    pub image: Option<String>,
    pub description: Option<String>,
    pub note: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub stored_in: Option<String>,
    pub responsible_person: Option<String>,
}
