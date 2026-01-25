use std::sync::Arc;

use sqlx::types::chrono::NaiveDateTime;
use sqlx::{Pool, Row, Sqlite};
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
        INSERT INTO users (login, username, salt, password_hash, sire)
        VALUES (?, ?, ?, ?, ?)
        "#,
            user.login,
            user.username,
            user.salt,
            user.password_hash,
            user.sire
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

    pub async fn get_user(&self, login: impl AsRef<str>) -> Result<Option<User>, RepositoryError> {
        let login = login.as_ref();

        let row = sqlx::query!(
            r#"
        SELECT login, username, salt, password_hash, sire
        FROM users
        WHERE login = ?
        "#,
            login
        )
        .fetch_optional(&self.pool)
        .await?;

        if let Some(row) = row {
            Ok(Some(User {
                login: row.login,
                username: row.username,
                salt: row.salt,
                password_hash: row.password_hash,
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

    pub async fn get_entries(&self) -> Result<Vec<Entry>, RepositoryError> {
        let rows = sqlx::query(
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
        "#,
        )
        .fetch_all(&self.pool)
        .await?;

        let entries: Vec<Entry> = rows
            .into_iter()
            .map(|row| {
                let created_at_str: Option<String> = row.try_get("created_at").ok();
                let created_at = created_at_str
                    .as_deref()
                    .and_then(|s| NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S").ok());

                Entry {
                    id: row.get("id"),
                    name: row.get("name"),
                    image: row.get("image"),
                    description: row.get("description"),
                    note: row.get("note"),
                    created_at,
                    stored_in: row.get("stored_in"),
                    responsible_person: row.get("responsible_person"),
                }
            })
            .collect();

        Ok(entries)
    }
}
