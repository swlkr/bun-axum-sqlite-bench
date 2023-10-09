use axum::{extract::State, response::Json, routing::get, Router};
use serde::Serialize;
use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};

#[derive(sqlx::FromRow, Serialize)]
struct Message {
    msg: String,
}

async fn root(State(db): State<SqlitePool>) -> Json<Vec<Message>> {
    let rows: Vec<Message> = sqlx::query_as("select msg from messages limit 100")
        .persistent(true)
        .fetch_all(&db)
        .await
        .unwrap();
    // Remove double encode
    // let rows = serde_json::to_string(&rows).unwrap();
    Json(rows)
}

#[tokio::main]
async fn main() {
    let db_options = sqlx::sqlite::SqliteConnectOptions::new().filename("db.sqlite3");
    let db = SqlitePoolOptions::new()
        .max_connections(1) // changed to 1 from 50
        .connect_with(db_options)
        .await
        .unwrap();

    let app = Router::new().route("/", get(root)).with_state(db);

    axum::Server::bind(&"0.0.0.0:9001".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// fn build_table_stuff() {
//     let _x = db
//         .execute("create table messages (msg text not null);")
//         .await
//         .unwrap();
//     for i in 0..1000 {
//         let _x = sqlx::query("insert into messages (msg) values (?)")
//             .bind(i)
//             .fetch_all(&db)
//             .await
//             .unwrap();
//     }
// }
