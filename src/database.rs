use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::{Acquire, Postgres};

// the solution for passing in an object which could connect to and
// execute queries in the database came from:
// https://github.com/launchbadge/sqlx/issues/1635#issuecomment-1027791249

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

pub async fn list_games<'a>(conn: impl Acquire<'a, Database = Postgres>) -> Vec<Game> {
    let mut conn = conn
        .acquire()
        .await
        .expect("Failed to acquire a database connection.");

    let games = sqlx::query_as!(Game, r#"SELECT game_id, name, version, released FROM game"#)
        .fetch_all(&mut *conn)
        .await
        .expect("Failed to do query.");

    games
}

pub async fn list_scores<'a>(
    conn: impl Acquire<'a, Database = Postgres>,
    game_name: Option<String>,
) -> Vec<Score> {
    let mut conn = conn
        .acquire()
        .await
        .expect("Failed to acquire a database connection.");

    // TODO: figure out a reasonable way to do optional filters
    let scores = match &game_name {
        Some(game_name) => sqlx::query_as!(
            Score,
            r#"
                SELECT points, player, achieved, game_id FROM score
                JOIN game USING (game_id) WHERE name = $1
                "#,
            game_name
        )
        .fetch_all(&mut *conn)
        .await
        .expect("Failed to do query."),
        None => sqlx::query_as!(
            Score,
            r#"SELECT points, player, achieved, game_id FROM score"#
        )
        .fetch_all(&mut *conn)
        .await
        .expect("Failed to do query."),
    };

    scores
}
