mod test_handlers;
mod db;

use axum::{
    response::Html,
    routing::get,
    routing::post,
    Router,
};
use test_handlers::{name_handler, test_handler};
use tokio::net::TcpListener;
use axum::Json;
use db::connect_to_db;
#[tokio::main]
async fn main() {

    let db_result = connect_to_db().await;
    if let Err(err) = db_result {
        panic!("Failed to connect to database: {}", err);
    }
    let db= db_result.unwrap();
    println!("***Connected to database***");


    let app = Router::new()
        .route("/health_check", get(|| async {
            Json(serde_json::json!({ "status": "ok" }))
        }))
        .route("/", get(|| async { "Welcome to the Axum server!" }))
        .route(
            "/hello",
            get(|| async { Html("Hello, <strong>World</strong>!") }))
        .route("/test", get(test_handler))
        .route("/name", post(name_handler));

    // run the app with hyper on port 3000
    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("Listening on http://127.0.0.1:3000/hello");
    axum::serve(listener, app).await.unwrap();
}

