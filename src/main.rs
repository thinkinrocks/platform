mod endpoints;
mod models;
mod repository;
mod sessions;

use std::sync::Arc;

use axum::{
    Router,
    extract::{Path, State},
    http::StatusCode,
    response::Html,
    routing::get,
};
use sqlx::sqlite::SqlitePoolOptions;
use tera::{Context, Tera};
use tokio::net::TcpListener;

use crate::{models::Entry, repository::Repository, sessions::Sessions};

async fn index(State(app): State<Arc<AppState>>) -> Result<Html<String>, StatusCode> {
    let context = Context::new();

    app.tera
        .render("index.html", &context)
        .map(Html)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

async fn entry(
    Path(id): Path<String>,
    State(app): State<Arc<AppState>>,
) -> Result<Html<String>, StatusCode> {
    let entry = app
        .repo
        .get_entry(&id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    let mut context = Context::new();
    context.insert("entry", &entry);

    app.tera
        .render("entry.html", &context)
        .map(Html)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

async fn htmx() -> &'static str {
    include_str!("js/htmx.min.js")
}

async fn css() -> &'static str {
    include_str!("style.css")
}

struct AppState {
    tera: Arc<Tera>,
    repo: Arc<Repository>,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().unwrap();

    let database_url = std::env::var("DATABASE_URL").unwrap();

    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect(&database_url)
        .await
        .unwrap();

    sqlx::query(include_str!("sql/setup.sql"))
        .execute(&pool)
        .await
        .unwrap();

    let repo = Repository::new(pool);
    let sessions = Sessions::new();

    let _ = repo
        .add_entry(Entry {
            id: "USBFLASH-001-100".to_string(),
            name: "USB Flash Memory Stick".to_string(),
            image: None,
            description: Some("Just an ordinary memory stick. Nothing special. One of the first items to put into storage.".to_string()),
            note: None,
            created_at: None,
            stored_in: None,
            responsible_person: None,
        })
        .await;

    println!("{:#?}", repo.get_entries().await.unwrap());

    let tera = Tera::new("templates/**/*").unwrap();

    let app = AppState {
        tera: Arc::new(tera),
        repo,
    };

    let app = Router::new()
        .route("/", get(index))
        .route("/entry/{id}", get(entry))
        .route("/htmx.min.js", get(htmx))
        .route("/style.css", get(css))
        .with_state(Arc::new(app));

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
