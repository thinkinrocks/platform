use std::sync::Arc;

use axum::{Router, extract::State, http::StatusCode, routing::get};
use csv::Writer;
use tokio::net::{TcpListener, ToSocketAddrs};

use crate::repository::Repository;

pub async fn serve_web(addr: impl ToSocketAddrs, repo: Arc<Repository>) {
    async fn csv(State(repo): State<Arc<Repository>>) -> Result<String, StatusCode> {
        let entries = repo
            .get_entries()
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        let mut wtr = Writer::from_writer(vec![]);

        for entry in entries {
            wtr.serialize(entry)
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        }

        wtr.flush().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        let data = String::from_utf8(
            wtr.into_inner()
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
        )
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        Ok(data)
    }

    let app = Router::new()
        .route("/entries.csv", get(csv))
        .with_state(repo);

    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
