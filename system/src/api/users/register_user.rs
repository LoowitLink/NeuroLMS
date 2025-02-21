use crate::{db::{models::user::User, connector::MongoRepo, models::user::{UserPermission,UserStatus, UserRole}}};
use axum::{
    extract::Extension,
    http::StatusCode,
    response::{IntoResponse, Json as AxumJson},
};
use bson::oid::ObjectId;
use jsonwebtoken::{encode, EncodingKey, Header};
use mongodb::bson::doc;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sha2::{Sha256, Digest};
use std::{env, sync::Arc};

#[derive(Debug, Serialize, Deserialize)]
pub struct ReqBody {
    name: String,
    email: String,
    password: Option<String>,
}

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

pub async fn user_register_service(
    Extension(db): Extension<Arc<MongoRepo>>,
    AxumJson(body): AxumJson<ReqBody>,
) -> impl IntoResponse {
    let empty_filter = doc! {};

    match db.count_user_documents(empty_filter) {
        Ok(user_count) => {
            if user_count == 0 {
                let mut hasher = Sha256::new();
                hasher.update(body.password.clone().unwrap_or("".to_string()).as_bytes());
                let result = hasher.finalize();
                let result_str = format!("{:x}", result);
                let new_user = User {
                    id: None,
                    name: body.name.clone(),
                    pref_name: body.name.clone(),
                    email: body.email.clone(),
                    password: result_str,
                    avatar: None,
                    permissions: vec![UserPermission::AdministratorAccess],
                    role: UserRole::Admin,
                    enrolled_courses: vec![],
                    status: UserStatus::Active,
                    last_login: Some(0),
                    created_at: Some(0),
                    updated_at: Some(0),
                };
                let user_res = db.create_user(new_user).unwrap();
                let exp_time = chrono::Utc::now().checked_add_signed(chrono::Duration::seconds(604800))
                    .expect("valid timestamp")
                    .timestamp() as usize;
                let token_data = Token {
                    id: user_res.id,
                    name: user_res.name.clone(),
                    email: user_res.email.clone(),
                    avatar: None,
                    permissions: user_res.permissions.clone(),
                    role: user_res.role.clone(),
                    enrolled_courses: vec![],
                    status: user_res.status.clone(),
                    last_login: Some(0),
                    created_at: Some(0),
                    updated_at: Some(0),
                    exp: exp_time,
                };

                let secret = env::var("JWT_SECRET").expect("JWT SECRET must be set in the .env file before we can sign tokens.");
                let token = encode(
                    &Header::default(),
                    &token_data,
                    &EncodingKey::from_secret(secret.as_ref()),
                ).unwrap();

                let response = json!({
                    "version": format!("R-{}", env!("CARGO_PKG_VERSION")),
                    "token": token,
                    "user": user_res,
                });
                // Return Ok with status and JSON
                Ok((StatusCode::OK, AxumJson(response)))
            } else {
                let version = env!("CARGO_PKG_VERSION");

                let response_json = json!({
                    "version": format!("R-{}", version),
                    "count": user_count,
                });

                // Return Ok with status and JSON
                Ok((StatusCode::ALREADY_REPORTED, AxumJson(response_json)))
            }
        }
        Err(_) => {
            // Return an error with StatusCode
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
