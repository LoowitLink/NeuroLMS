use std::env;
use dotenv::dotenv;
use mongodb::{bson::Document, sync::{Client, Collection}};

pub struct MongoRepo {
    pub col_users: Collection<Document>,
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
}
