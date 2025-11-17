use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, Utc};
use snowflaked::Snowflake;
use sqlx::{Pool, Sqlite, sqlite::SqlitePoolOptions};

use crate::{models::User, snowflake::generate_id};

pub struct Datastore {
    pool: Pool<Sqlite>,
}

impl Datastore {
    pub async fn new() -> Self {
        #[cfg(test)]
        let url = ":memory:";
        #[cfg(not(test))]
        let url = "./app.sqlite3";

        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect(url)
            .await
            .unwrap();

        sqlx::migrate!("./migrations").run(&pool).await.unwrap();

        Self { pool }
    }

    pub async fn add_user(
        &self,
        display_name: impl ToString,
        access_code_hash: impl ToString,
        sign_up_code_id: Option<i64>,
        initiated_at: DateTime<Utc>,
    ) -> i64 {
        let display_name = display_name.to_string();
        let access_code_hash = access_code_hash.to_string();
        let initiated_at = initiated_at.to_rfc3339();
        let id = generate_id();

        let row = sqlx::query!(
            r#"
            INSERT INTO users (id, display_name, access_code_hash, sign_up_code_id, initiated_at)
            VALUES (?, ?, ?, ?, ?)
            "#,
            id,
            display_name,
            access_code_hash,
            sign_up_code_id,
            initiated_at,
        )
        .execute(&self.pool)
        .await
        .unwrap();

        row.last_insert_rowid()
    }

    pub async fn get_user_by_access_code(
        &self,
        access_code_hash: impl ToString,
    ) -> Result<Option<User>, sqlx::Error> {
        let access_code_hash = access_code_hash.to_string();

        let row = sqlx::query!(
            r#"
        SELECT id, display_name, access_code_hash, sign_up_code_id, initiated_at
        FROM users
        WHERE access_code_hash = ?
        "#,
            access_code_hash
        )
        .fetch_optional(&self.pool)
        .await?;

        let user = row.map(|r| User {
            id: r.id,
            display_name: r.display_name,
            access_code_hash: r.access_code_hash,
            sign_up_code_id: r.sign_up_code_id,
            initiated_at: NaiveDateTime::parse_from_str(&r.initiated_at, "%Y-%m-%d %H:%M:%S")
                .unwrap_or_else(|_| NaiveDateTime::new(NaiveDate::default(), NaiveTime::default())),
        });

        Ok(user)
    }
}

#[cfg(test)]
mod tests {
    use crate::hash::hash_digest;

    use super::*;

    #[tokio::test]
    async fn test_add_user() {
        let database = Datastore::new().await;

        let display_name = "Test";
        let access_code_hash = hash_digest(b"ABC123");

        let id = database
            .add_user(
                display_name,
                access_code_hash.clone(),
                None,
                DateTime::<Utc>::from_timestamp_nanos(0),
            )
            .await;

        let user = database
            .get_user_by_access_code(access_code_hash)
            .await
            .unwrap()
            .unwrap();
        
        assert_eq!(user.id, id);
        assert_eq!(user.display_name, display_name);
    }
}
