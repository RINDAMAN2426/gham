use axum::{
    routing::{get, post},
    Router, Json
};
use serde_json::{json, Value};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/webhook", post(webhook))
        .route("/workflows", get(get_workflows));

    let host = "127.0.0.1";
    let port = 2426;
    let addr = format!("{host}:{port}");
    let listener = TcpListener::bind(&addr).await.unwrap();
    println!("Server is running on http://{addr}");

    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Hello, World!"
}

async fn webhook(Json(payload): Json<Value>) -> &'static str {
    if let Some(event) = payload.get("event") {
        println!("Github Event: {}", event);
    } else {
        println!("Invalid payload received");
    }
    "Webhook Received"
}

async fn get_workflows() -> Json<Vec<Value>> {
    let workflows = vec![
        json!({ "id": 1, "name": "CI Workflow", "status": "Success" }),
        json!({ "id": 2, "name": "Deploy Workflow", "status": "Success" }),
    ];
    Json(workflows)
}

