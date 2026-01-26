use std::sync::Arc;

use sqlx::types::chrono::NaiveDateTime;
use sqlx::{Pool, Sqlite};
use thiserror::Error;

use crate::models::{Entry, User};

#[derive(Error, Debug)]
pub enum RepositoryError {
    #[error("{0}")]
    Sqlx(#[from] sqlx::Error),
}

pub struct Repository {
    pool: Pool<Sqlite>,
}

impl Repository {
    pub fn new(pool: Pool<Sqlite>) -> Arc<Self> {
        Arc::new(Self { pool })
    }

    pub async fn add_user(&self, user: User) -> Result<(), RepositoryError> {
        sqlx::query!(
            r#"
        INSERT INTO users (telegram_username, sire)
        VALUES (?, ?)
        "#,
            user.telegram_username,
            user.sire,
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn add_entry(&self, entry: Entry) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
        INSERT INTO entries (
            id,
            name,
            image,
            description,
            note,
            created_at,
            stored_in,
            responsible_person
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?)
        "#,
            entry.id,
            entry.name,
            entry.image,
            entry.description,
            entry.note,
            entry.created_at,
            entry.stored_in,
            entry.responsible_person
        )
        .execute(&self.pool)
        .await
        .map(drop)
    }

    pub async fn get_user(
        &self,
        telegram_username: impl AsRef<str>,
    ) -> Result<Option<User>, RepositoryError> {
        let telegram_username = telegram_username.as_ref();

        let row = sqlx::query!(
            r#"
        SELECT telegram_username, sire
        FROM users
        WHERE telegram_username = ?
        "#,
            telegram_username
        )
        .fetch_optional(&self.pool)
        .await?;

        if let Some(row) = row {
            Ok(Some(User {
                telegram_username: row.telegram_username,
                sire: row.sire,
            }))
        } else {
            Ok(None)
        }
    }

    pub async fn get_entry(&self, id: impl AsRef<str>) -> Result<Option<Entry>, RepositoryError> {
        let id = id.as_ref();

        let row = sqlx::query!(
            r#"
        SELECT
            id,
            name,
            image,
            description,
            note,
            created_at,
            stored_in,
            responsible_person
        FROM entries
        WHERE id = ?
        "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        if let Some(row) = row {
            let created_at = row
                .created_at
                .as_deref()
                .and_then(|s| NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S").ok());

            Ok(Some(Entry {
                id: row.id.unwrap(),
                name: row.name,
                image: row.image,
                description: row.description,
                note: row.note,
                created_at,
                stored_in: row.stored_in,
                responsible_person: row.responsible_person,
            }))
        } else {
            Ok(None)
        }
    }

    pub async fn search_entries(
        &self,
        search: impl AsRef<str>,
        limit: u32,
    ) -> Result<Vec<Entry>, RepositoryError> {
        let search = search.as_ref();
        let query = r#"
SELECT
id,
name,
image,
description,
note,
created_at,
stored_in,
responsible_person
FROM entries
WHERE
id LIKE '%' || ? || '%'
OR name LIKE '%' || ? || '%'
OR description LIKE '%' || ? || '%'
LIMIT ?"#
            .trim();

        let entries = sqlx::query_as::<_, Entry>(query)
            .bind(&search)
            .bind(&search)
            .bind(&search)
            .bind(limit)
            .fetch_all(&self.pool)
            .await?;

        Ok(entries)
    }
}
