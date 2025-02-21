mod api;
mod db;
use axum::extract::Extension;
use axum::http::{self, Method};
use axum::{
    http::StatusCode,
    routing::{get,post,delete},
    Json, Router
};
use db::connector::MongoRepo;

use log::info;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Arc;
use tokio;
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::{self, FmtSubscriber};

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[tokio::main]
async fn main() {
tracing_subscriber::fmt::init();
let db: Arc<MongoRepo> = Arc::new(MongoRepo::init());

let subscriber = FmtSubscriber::new();
tracing::subscriber::set_global_default(FmtSubscriber::default()); 
info!("Starting server");
let app = Router::new()
.route("/", get(root))
.route("/ver", get(ver))
.route("/api/users/count", get(api::users::user_count::return_total_users))
.route("/api/auth/register", post(api::users::register_user::user_register_service))
.route("/api/user/account", get(api::users::my_account_data::user_fetch_data_service))
.layer(Extension(db))
.layer(
    CorsLayer::new()
       .allow_methods(vec![Method::GET, Method::POST, Method::DELETE])
       .allow_origin([
    "http://localhost:3000".parse::<http::HeaderValue>().unwrap(),
    "https://loowitlink.com".parse::<http::HeaderValue>().unwrap()
       ])
       .allow_headers(Any)
);
let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
axum::serve(listener, app).await.expect("Failed to start server");//+
}

async fn root() -> &'static str {
    "Welcome to the LoowitLink, LLC API!"
}

async fn ver() -> Json<serde_json::Value>{
    let message = json!({
        "serverMessage": "We're online and rolling!",
        "version": format!("R-{}", VERSION)
    });
    Json(message)
}