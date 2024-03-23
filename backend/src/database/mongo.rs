use std::error::Error;

use mongodb::{
    bson::doc,
    options::{ClientOptions, IndexOptions},
    Client, Database, IndexModel,
};

use super::{
    user::{UnconfirmedUserDb, UserDb},
    DBHandler,
};
use crate::models::{UnconfirmedUser, User};

#[derive(Clone)]
pub struct MongoDBHandler {
    pub db_client: Database,
    user_collection: mongodb::Collection<User>,
    unconfirmed_user_collection: mongodb::Collection<UnconfirmedUser>,
}

impl MongoDBHandler {
    pub async fn new(database_url: &str, database: &str) -> Result<Self, Box<dyn Error>> {
        let client_options = ClientOptions::parse(&database_url)
            .await
            .expect("Failed to parse client options");

        let db_client = Client::with_options(client_options)
            .expect("Failed to initialize database client")
            .database(database);

        let user_collection = db_client.collection::<User>("users");
        let unconfirmed_user_collection =
            db_client.collection::<UnconfirmedUser>("unconfirmed_users");

        let options = IndexOptions::builder()
            .expire_after(std::time::Duration::from_secs(24 * 60 * 60))
            .build();

        unconfirmed_user_collection
            .create_index(
                IndexModel::builder()
                    .keys(doc! {"created_at": 1})
                    .options(options)
                    .build(),
                None,
            )
            .await?;

        Ok(Self {
            db_client,
            user_collection,
            unconfirmed_user_collection,
        })
    }
}

impl DBHandler for MongoDBHandler {}

impl UserDb for MongoDBHandler {
    async fn find_user(&self, username: &str) -> Result<Option<User>, ()> {
        self.user_collection
            .find_one(doc! {"username": username}, None)
            .await
            .or(Err(()))
    }

    async fn find_user_by_email(&self, email: &str) -> Result<Option<User>, ()> {
        self.user_collection
            .find_one(doc! {"email": email}, None)
            .await
            .or(Err(()))
    }

    async fn insert_user(&self, user: &User) -> Result<(), ()> {
        match self.user_collection.insert_one(user, None).await {
            Ok(_) => Ok(()),
            Err(_) => Err(()),
        }
    }
}

impl UnconfirmedUserDb for MongoDBHandler {
    async fn insert_unconfirmed_user(&self, user: &UnconfirmedUser) -> Result<(), ()> {
        match self
            .unconfirmed_user_collection
            .insert_one(user, None)
            .await
        {
            Ok(_) => Ok(()),
            Err(_) => Err(()),
        }
    }

    async fn confirm_user(&self, confirmation_token: &str) -> Result<Option<UnconfirmedUser>, ()> {
        if let Ok(user_option) = self
            .unconfirmed_user_collection
            .find_one(doc! {"confirmation_token": confirmation_token}, None)
            .await
        {
            match user_option {
                Some(user) if !user.confirmed => {
                    if self
                        .unconfirmed_user_collection
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
                _ => return Ok(None),
            }
        }

        Err(())
    }

    async fn find_unconfirmed_user(&self, username: &str) -> Result<Option<UnconfirmedUser>, ()> {
        self.unconfirmed_user_collection
            .find_one(doc! {"username": username}, None)
            .await
            .or(Err(()))
    }

    async fn find_unconfirmed_user_user_by_email(
        &self,
        email: &str,
    ) -> Result<Option<UnconfirmedUser>, ()> {
        self.unconfirmed_user_collection
            .find_one(doc! {"email": email}, None)
            .await
            .or(Err(()))
    }
}
