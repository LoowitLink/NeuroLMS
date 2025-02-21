use crate::{db::{models::user::User, connector::MongoRepo, models::user::{UserPermission,UserStatus, UserRole}}};
use axum::{
    extract::Extension,
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Json as AxumJson},
};
use bson::oid::ObjectId;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use mongodb::bson::doc;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sha2::{Sha256, Digest};
use std::{env, sync::Arc};

#[derive(Serialize, Deserialize)]
struct Token {
    id: Option<ObjectId>,
    name: String,
    email: String,
    avatar: Option<String>,
    permissions: Vec<UserPermission>,
    role: UserRole,
    enrolled_courses: Vec<String>,
    status: UserStatus,
    last_login: Option<i64>,
    created_at: Option<i64>,
    updated_at: Option<i64>,
    exp: usize,
}

pub async fn user_fetch_data_service(
    Extension(db): Extension<Arc<MongoRepo>>,
    headers: HeaderMap
) -> impl IntoResponse {
    let authorization = headers.get("Authorization");
    let token_data = match authorization.and_then(|h| h.to_str().ok()) {
        Some(token_data) => token_data,
        None => return Err(StatusCode::UNAUTHORIZED),
    };
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set in .env file");
    let token_data = match decode::<Token>(
        &token_data,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::new(Algorithm::HS256),
    ) {
        Ok(data) => data,
        Err(err) => {
            eprintln!("Error decoding token: {}", err);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        },
    };
    let user_id = match token_data.claims.id {
        Some(id) => id,
        None => return Err(StatusCode::BAD_REQUEST),
    };
    let filter = doc! { "_id": user_id };
    match db.get_user_information(filter) {
        Ok((user)) => {

            let response = json!({
                "version": format!("R-{}", env!("CARGO_PKG_VERSION")),
                "user": user,
            });
            // Return Ok with status and JSON
            Ok((StatusCode::OK, AxumJson(response)))
        },
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

