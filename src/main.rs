use std::str::FromStr;
use std::time::Duration;

use axum::{extract::State, response::Json, routing::get, Router};
use serde::{Deserialize, Serialize};
use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};

#[derive(sqlx::FromRow, Serialize, Deserialize)]
struct Message {
    msg: String,
}

async fn root(State(db): State<SqlitePool>) -> Json<String> {
    let rows: Vec<Message> = sqlx::query_as("select msg from messages limit 100")
        .persistent(true)
        .fetch_all(&db)
        .await
        .unwrap();
    let rows = serde_json::to_string(&rows).unwrap();
    Json(rows)
}

#[tokio::main]
async fn main() {
    let db_options = sqlx::sqlite::SqliteConnectOptions::new()
        .filename("db.sqlite3")
        .create_if_missing(true)
        .journal_mode(sqlx::sqlite::SqliteJournalMode::Wal)
        .pragma("temp_store", "memory")
        .pragma("mmap_size", "30000000000")
        .synchronous(sqlx::sqlite::SqliteSynchronous::Off)
        .foreign_keys(false)
        .row_buffer_size(10000)
        .busy_timeout(Duration::from_secs(30))
        .page_size(32768);
    let db = SqlitePoolOptions::new()
        .max_connections(50)
        .idle_timeout(Duration::from_secs(30))
        .connect_with(db_options)
        .await
        .unwrap();
    // let _x = db
    //     .execute("create table messages (msg text not null);")
    //     .await
    //     .unwrap();

    // for i in 0..1000 {
    //     let _x = sqlx::query("insert into messages (msg) values (?)")
    //         .bind(i)
    //         .fetch_all(&db)
    //         .await
    //         .unwrap();
    // }

    let app = Router::new().route("/", get(root)).with_state(db);

    axum::Server::bind(&"0.0.0.0:9001".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
