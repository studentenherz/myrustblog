use mongodb::{bson::doc, options::ClientOptions, Client, Database};

use super::{user::UserDb, DBHandler};
use crate::models::User;

#[derive(Clone)]
pub struct MongoDBHandler {
    pub db_client: Database,
}

impl MongoDBHandler {
    pub async fn new(database_url: &str, database: &str) -> Self {
        let client_options = ClientOptions::parse(&database_url)
            .await
            .expect("Failed to parse client options");

        let db_client = Client::with_options(client_options)
            .expect("Failed to initialize database client")
            .database(database);

        Self { db_client }
    }
}

impl DBHandler for MongoDBHandler {}

impl UserDb for MongoDBHandler {
    async fn find_user(&self, username: &str) -> Result<Option<User>, ()> {
        let collection = self.db_client.collection::<User>("users");

        collection
            .find_one(doc! {"username": username}, None)
            .await
            .or(Err(()))
    }
    async fn insert_user(&self, user: &User) -> Result<(), ()> {
        let collection = self.db_client.collection::<User>("users");

        match collection.insert_one(user, None).await {
            Ok(_) => Ok(()),
            Err(_) => Err(()),
        }
    }
}
