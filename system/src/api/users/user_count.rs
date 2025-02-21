use crate::{db::{models::user::User, connector::MongoRepo}};
use axum::{
    extract::Extension,
    http::StatusCode,
    response::{IntoResponse, Json as AxumJson},
};
use mongodb::bson::doc;
use serde_json::json;
use std::sync::Arc;

pub async fn return_total_users(
    Extension(db): Extension<Arc<MongoRepo>>,
) -> impl IntoResponse {
    let empty_filter = doc! {};

    match db.count_user_documents(empty_filter) {
        Ok(user_count) => {
            let version = env!("CARGO_PKG_VERSION");
            let response_json = json!({
                "version": format!("R-{}", version),
                "count": user_count
            });

            Ok(AxumJson(response_json))
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
