use axum::{http::StatusCode, response::IntoResponse, routing::get, Extension, Json, Router};
use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::sync::Arc;

#[derive(Serialize)]
pub struct Game {
    game_id: uuid::Uuid,
    name: String,
    version: String,
    released: DateTime<Utc>,
}

pub async fn list_games(Extension(state): Extension<Arc<State>>) -> impl IntoResponse {
    let games = sqlx::query_as!(Game, r#"SELECT game_id, name, version, released FROM game"#)
        .fetch_all(&state.db_pool)
        .await
        .expect("Failed to do query.");

    (StatusCode::OK, Json(games))
}

pub struct State {
    pub db_pool: Pool<Postgres>,
}

#[tokio::main]
async fn main() {
    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgresql://pixel:pixel@localhost:5432/pixel")
        .await
        .expect("Could not connect to the database!");

    let state = Arc::new(State { db_pool });

    let app = Router::new()
        .route("/v1/games", get(list_games))
        .layer(Extension(state));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
