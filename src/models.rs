use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Row, sqlite::SqliteRow};

#[derive(Debug, Clone, FromRow, Deserialize)]
pub struct User {
    pub telegram_username: String,
    pub sire: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
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

impl<'r> FromRow<'r, SqliteRow> for Entry {
    fn from_row(row: &'r SqliteRow) -> Result<Self, sqlx::Error> {
        let created_at_str: Option<String> = row.try_get("created_at")?;

        let created_at = created_at_str
            .and_then(|s| chrono::NaiveDateTime::parse_from_str(&s, "%Y-%m-%d %H:%M:%S").ok());

        Ok(Self {
            id: row.try_get("id")?,
            name: row.try_get("name")?,
            image: row.try_get("image")?,
            description: row.try_get("description")?,
            note: row.try_get("note")?,
            created_at,
            stored_in: row.try_get("stored_in")?,
            responsible_person: row.try_get("responsible_person")?,
        })
    }
}
