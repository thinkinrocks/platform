use chrono::{DateTime, Utc};
use sqlx::{Pool, Sqlite, sqlite::SqlitePoolOptions};

pub struct Datastore {
    pool: Pool<Sqlite>
}

impl Datastore {
    pub async fn new_in_memory() -> Self {
        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect("./app.sqlite3")
            .await.unwrap();

        sqlx::migrate!("./migrations").run(&pool).await.unwrap();

        todo!()
    }

    pub async fn add_user(&self, display_name: impl ToString, github: impl ToString, initiated_at: DateTime<Utc>) {
        let res = sqlx::query!("INSERT INTO user (display_name, github, sire_id, initiated_at) VALUES (?, ?, NULL, ?)");
    } 
}
