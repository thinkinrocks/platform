use std::sync::Arc;

use base64::Engine;
use base64::prelude::BASE64_STANDARD;
use sha2::{Digest, Sha512};
use sqlx::types::chrono::NaiveDateTime;
use sqlx::{Pool, Sqlite};
use thiserror::Error;

use crate::models::{Entry, User};

#[derive(Error, Debug)]
pub enum RepositoryError {
    #[error("{0}")]
    Sqlx(#[from] sqlx::Error),
}

pub struct EntryReserved {
    pub reserver: String,
    pub start: NaiveDateTime,
    pub end: NaiveDateTime,
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
            VALUES (?, ?)"#,
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
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?)"#,
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

    pub async fn is_entry_reserved(
        &self,
        entry_id: impl AsRef<str>,
        check_start: NaiveDateTime,
        check_end: NaiveDateTime,
    ) -> Result<Option<EntryReserved>, RepositoryError> {
        let entry_id = entry_id.as_ref();

        let check_start_str = check_start.format("%Y-%m-%d %H:%M:%S").to_string();
        let check_end_str = check_end.format("%Y-%m-%d %H:%M:%S").to_string();

        let row = sqlx::query!(
            r#"
        SELECT u.telegram_username AS reserver_name,
               r.start_ts,
               r.end_ts
        FROM reservations_entries re
        JOIN reservations r ON re.reservation_id = r.id
        JOIN users u ON r.made_by = u.telegram_username
        WHERE re.entry_id = ?
          AND r.start_ts < ?
          AND r.end_ts > ?
        ORDER BY r.start_ts ASC
        LIMIT 1
        "#,
            entry_id,
            check_end_str,
            check_start_str
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| {
            let start_ts = NaiveDateTime::parse_from_str(&r.start_ts, "%Y-%m-%d %H:%M:%S")
                .expect("Invalid start_ts format in DB");
            let end_ts = NaiveDateTime::parse_from_str(&r.end_ts, "%Y-%m-%d %H:%M:%S")
                .expect("Invalid end_ts format in DB");

            EntryReserved {
                reserver: r.reserver_name,
                start: start_ts,
                end: end_ts,
            }
        }))
    }

    pub async fn reserve_entries(
        &self,
        entry_ids: &[impl AsRef<str>],
        start_ts: NaiveDateTime,
        end_ts: NaiveDateTime,
        made_by: &str,
    ) -> Result<String, RepositoryError> {
        let start_str = start_ts.format("%Y-%m-%d %H:%M:%S").to_string();
        let end_str = end_ts.format("%Y-%m-%d %H:%M:%S").to_string();

        let mut hasher = Sha512::default();
        hasher.update(start_str.as_bytes());
        hasher.update(end_str.as_bytes());
        hasher.update(made_by.as_bytes());
        let hash = hasher.finalize();

        let reservation_id = BASE64_STANDARD.encode(hash);

        // TODO: use a transaction

        sqlx::query!(
            r#"
            INSERT INTO reservations (id, start_ts, end_ts)
            VALUES (?, ?, ?)
            "#,
            reservation_id,
            start_str,
            end_str
        )
        .execute(&self.pool)
        .await?;

        for entry_id in entry_ids {
            let entry_id = entry_id.as_ref();
            sqlx::query!(
                r#"
                INSERT INTO reservations_entries (reservation_id, entry_id)
                VALUES (?, ?)
                "#,
                reservation_id,
                entry_id
            )
            .execute(&self.pool)
            .await?;
        }

        Ok(reservation_id)
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

    pub async fn add_to_cart(&self, user_id: &str, entry_id: &str) -> Result<(), RepositoryError> {
        // TODO: check if entry exists and is not still added

        sqlx::query!(
            r#"
        INSERT OR IGNORE INTO cart (id, entry_id)
        VALUES (?, ?)
        "#,
            user_id,
            entry_id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn remove_from_cart(
        &self,
        user_id: &str,
        entry_id: &str,
    ) -> Result<bool, RepositoryError> {
        let result = sqlx::query!(
            r#"
        DELETE FROM cart
        WHERE id = ? AND entry_id = ?
        "#,
            user_id,
            entry_id
        )
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    pub async fn get_cart(&self, user_id: &str) -> Result<Vec<Entry>, RepositoryError> {
        let entries = sqlx::query_as::<_, Entry>(
            r#"
        SELECT
            e.id,
            e.name,
            e.image,
            e.description,
            e.note,
            e.created_at,
            e.stored_in,
            e.responsible_person
        FROM cart c
        JOIN entries e ON e.id = c.entry_id
        WHERE c.id = ?
        ORDER BY e.name
        "#,
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(entries)
    }
}
