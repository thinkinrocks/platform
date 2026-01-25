use chrono::NaiveDateTime;
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct User {
    pub login: String,
    pub username: String,
    pub salt: String,
    pub password_hash: String,
    pub sire: Option<String>,
}

#[derive(Debug, Clone, FromRow)]
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
