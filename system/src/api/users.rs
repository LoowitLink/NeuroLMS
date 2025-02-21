use crate::{db::{models::user::User, connector::MongoRepo}};
use axum::{
    extract::{Extension, Json}, http::{HeaderMap, StatusCode}, response::{IntoResponse, Json as AxumJson}, routing::post, Router
};
use axum_extra::headers;
use jsonwebtoken::{EncodingKey, Header};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation, errors::Error as JwtError};
use mongodb::bson::{doc, oid::ObjectId};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Arc;
use std::env;
use sha2::{Sha256, Digest};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    _id: ObjectId,
    name: String,
    email: String,
    role: String,
    company: String,
    profile_image: Option<String>,
    active: bool,
    exp: usize,
}
pub struct ClientReq {
    authorization: Option<String>,
}
pub async fn my_data(
    Extension(db): Extension<Arc<MongoRepo>>,
    headers: HeaderMap
) -> impl IntoResponse {
    let authorization = headers.get("Authorization").unwrap();
    let client_req = ClientReq { authorization: Some(authorization.to_str().unwrap().to_string()) };
    let token = match client_req.authorization {
        Some(token) => token,
        None => return Err(StatusCode::UNAUTHORIZED),
    };

    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set in .env file");

    let token_data = match decode::<Claims>(
        &token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::new(Algorithm::HS256),
    ) {
        Ok(token_data) => token_data,
        Err(err) => {
            eprintln!("Error decoding token: {}", err); // Print the error
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        },
    };
    
            let version = env!("CARGO_PKG_VERSION");
            let mut response_json = json!({
                "version": format!("R-{}", version),
            });
            Ok(Json(response_json))

}