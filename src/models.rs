use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: i64,
    pub display_name: String,
    pub access_code_hash: String,
    pub sign_up_code_id: Option<i64>,
    pub initiated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct SignUpCode {
    pub id: i64,
    pub granted_by: i64,
    pub code: String,
    pub at: NaiveDateTime,
}
