use std::sync::Arc;

use sqlx::{Pool, Sqlite};

pub struct Repository {
    pool: Pool<Sqlite>,
}

impl Repository {
    pub fn new(pool: Pool<Sqlite>) -> Arc<Self> {
        Arc::new(Self { pool })
    }
}
