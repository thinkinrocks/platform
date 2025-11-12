use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: i64,
    pub display_name: String,
    pub github: Option<String>,
    pub sire_id: Option<i64>,
    pub initiated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Token {
    pub id: i64,
    pub created_at: DateTime<Utc>,
    pub token: String,
    pub owner: i64,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Commitment {
    pub performed_at: DateTime<Utc>,
    pub token_id: i64,
    pub chlide_id: i64,
}
