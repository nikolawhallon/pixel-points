use axum::{
    extract::Query, http::StatusCode, response::IntoResponse, routing::get, Extension, Json, Router,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::sync::Arc;

mod database;

#[derive(Serialize)]
pub struct Game {
    game_id: uuid::Uuid,
    name: String,
    version: String,
    released: DateTime<Utc>,
}

#[derive(Serialize)]
pub struct Score {
    points: i32,
    player: String,
    achieved: DateTime<Utc>,
    game_id: uuid::Uuid,
}

#[derive(Deserialize)]
pub struct ListScoresQuery {
    pub game_name: Option<String>,
}

pub async fn list_games(Extension(state): Extension<Arc<State>>) -> impl IntoResponse {
    let games = database::list_games(&state.db_pool).await;

    (StatusCode::OK, Json(games))
}

pub async fn list_scores(
    Extension(state): Extension<Arc<State>>,
    params: Query<ListScoresQuery>,
) -> impl IntoResponse {
    let scores = database::list_scores(&state.db_pool, params.game_name.clone()).await;

    (StatusCode::OK, Json(scores))
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
        .route("/v1/scores", get(list_scores))
        .layer(Extension(state));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
