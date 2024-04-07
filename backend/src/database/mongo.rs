use std::error::Error;

use futures_util::TryStreamExt;
use mongodb::{
    bson::doc,
    options::{ClientOptions, FindOptions, IndexOptions},
    Client, Database, IndexModel,
};

use super::{
    post::PostDb,
    user::{UnconfirmedUserDb, UserDb},
    DBHandler,
};
use crate::models::{Post, PostsQueryParams, UnconfirmedUser, User};

#[derive(Clone)]
pub struct MongoDBHandler {
    pub db_client: Database,
    user_collection: mongodb::Collection<User>,
    unconfirmed_user_collection: mongodb::Collection<UnconfirmedUser>,
    post_collection: mongodb::Collection<Post>,
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
        let post_collection = db_client.collection::<Post>("posts");

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
            post_collection,
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

impl PostDb for MongoDBHandler {
    async fn create_post(&self, post: &Post) -> Result<(), ()> {
        match self.post_collection.insert_one(post, None).await {
            Ok(_) => Ok(()),
            Err(_) => Err(()),
        }
    }

    async fn update_post(&self, slug: &str, updated_content: &str) -> Result<u64, ()> {
        match self
            .post_collection
            .update_one(
                doc! {"slug": slug},
                doc! {"$set": doc! {"content": updated_content}},
                None,
            )
            .await
        {
            Ok(result) => Ok(result.modified_count),
            Err(_) => Err(()),
        }
    }

    async fn delete_post(&self, slug: &str) -> Result<u64, ()> {
        match self
            .post_collection
            .delete_one(doc! {"slug": slug}, None)
            .await
        {
            Ok(result) => Ok(result.deleted_count),
            Err(_) => Err(()),
        }
    }

    async fn get_post(&self, slug: &str) -> Result<Option<Post>, ()> {
        self.post_collection
            .find_one(doc! {"slug": slug}, None)
            .await
            .or(Err(()))
    }

    async fn get_posts(&self, query: &PostsQueryParams) -> Result<Vec<Post>, ()> {
        let mut options = FindOptions::default();

        let page = query.page.unwrap_or(1);
        let per_page = query.per_page.unwrap_or(10);
        options.limit = Some(per_page as i64);
        options.skip = Some((page - 1) * per_page);

        if let Some(ref sort_by) = query.sort_by {
            let sort_order = if query.sort_order.as_deref() == Some("desc") {
                -1
            } else {
                1
            };
            options.sort = Some(doc! {sort_by: sort_order});
        }

        if let Ok(cursor) = self.post_collection.find(None, options).await {
            if let Ok(v) = cursor.try_collect().await {
                return Ok(v);
            }
        }

        Err(())
    }
}
