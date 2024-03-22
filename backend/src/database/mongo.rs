use mongodb::{bson::doc, options::ClientOptions, Client, Database};

use super::{
    user::{UnconfirmedUserDb, UserDb},
    DBHandler,
};
use crate::models::{UnconfirmedUser, User};

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

    async fn find_user_by_email(&self, email: &str) -> Result<Option<User>, ()> {
        let collection = self.db_client.collection::<User>("users");

        collection
            .find_one(doc! {"email": email}, None)
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

impl UnconfirmedUserDb for MongoDBHandler {
    async fn insert_unconfirmed_user(&self, user: &UnconfirmedUser) -> Result<(), ()> {
        let collection = self
            .db_client
            .collection::<UnconfirmedUser>("unconfirmed_users");

        match collection.insert_one(user, None).await {
            Ok(_) => Ok(()),
            Err(_) => Err(()),
        }
    }

    async fn confirm_user(&self, confirmation_token: &str) -> Result<Option<UnconfirmedUser>, ()> {
        let collection = self
            .db_client
            .collection::<UnconfirmedUser>("unconfirmed_users");

        if let Ok(user_option) = collection
            .find_one(doc! {"confirmation_token": confirmation_token}, None)
            .await
        {
            match user_option {
                Some(user) => {
                    if collection
                        .update_one(
                            doc! {"confirmation_token": confirmation_token},
                            doc! {"$set": doc! {"confirmed": true}},
                            None,
                        )
                        .await
                        .is_ok()
                    {
                        return Ok(Some(user));
                    }
                }
                None => return Ok(None),
            }
        }

        Err(())
    }

    async fn find_unconfirmed_user(&self, username: &str) -> Result<Option<UnconfirmedUser>, ()> {
        let collection = self
            .db_client
            .collection::<UnconfirmedUser>("unconfirmed_users");

        collection
            .find_one(doc! {"username": username}, None)
            .await
            .or(Err(()))
    }

    async fn find_unconfirmed_user_user_by_email(
        &self,
        email: &str,
    ) -> Result<Option<UnconfirmedUser>, ()> {
        let collection = self
            .db_client
            .collection::<UnconfirmedUser>("unconfirmed_users");

        collection
            .find_one(doc! {"email": email}, None)
            .await
            .or(Err(()))
    }
}
