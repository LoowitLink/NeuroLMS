use std::env;
extern crate dotenv;
use crate::db::models::user::User;
use chrono::Utc;
use dotenv::dotenv;
use lettre::transport::smtp::response::Code;
use rand::{distributions::Alphanumeric, Rng, RngCore};
use sha2::{digest::{typenum::Or, Update}, Digest, Sha256};
use mongodb::{
    bson::{self, doc, oid::ObjectId, Bson, Document, Uuid}, error::Error, results::{InsertOneResult, UpdateResult}, sync::{Client, Collection}
};
use anyhow::{Result, Context};


pub struct MongoRepo {
    col_users: Collection<Document>,
}

impl MongoRepo {
    pub fn init() -> Self {
        dotenv().ok();
        let uri = env::var("MONGOURI").expect("MONGOURI must be set in .env file");
        let client = Client::with_uri_str(&uri).expect("Failed to initialize MongoDB client");
        let db = client.database("neurolms");
        let col_users: Collection<Document> = db.collection("users");
        MongoRepo { col_users }
    }

    pub fn count_user_documents(&self, filter: Document) -> Result<i64, Error> {
        let count_result = self.col_users.count_documents(filter, None)?;
        Ok(count_result as i64)
    }


}