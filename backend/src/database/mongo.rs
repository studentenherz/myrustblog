use std::error::Error;

use futures_util::TryStreamExt;
use mongodb::{
    bson::doc,
    options::{ClientOptions, IndexOptions},
    Client, IndexModel,
};

use super::{
    post::PostDb,
    user::{UnconfirmedUserDb, UserDb},
    DBHandler,
};
use crate::models::{PostModel, PostsQueryParams, UnconfirmedUser, User};
use common::Post;

#[derive(Clone)]
pub struct MongoDBHandler {
    user_collection: mongodb::Collection<User>,
    unconfirmed_user_collection: mongodb::Collection<UnconfirmedUser>,
    post_collection: mongodb::Collection<PostModel>,
}

impl MongoDBHandler {
    pub async fn new(database_url: &str, database: &str) -> Result<Self, Box<dyn Error>> {
        let client_options = ClientOptions::parse(database_url)
            .await
            .expect("Failed to parse client options");

        let db_client = Client::with_options(client_options)
            .expect("Failed to initialize database client")
            .database(database);

        let user_collection = db_client.collection::<User>("users");
        let unconfirmed_user_collection =
            db_client.collection::<UnconfirmedUser>("unconfirmed_users");
        let post_collection = db_client.collection::<PostModel>("posts");

        let options = IndexOptions::builder()
            .expire_after(std::time::Duration::from_secs(24 * 60 * 60))
            .build();

        unconfirmed_user_collection
            .create_index(
                IndexModel::builder()
                    .keys(doc! {"created_at": 1})
                    .options(options)
                    .build(),
            )
            .await?;

        Ok(Self {
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
            .find_one(doc! {"username": username})
            .await
            .or(Err(()))
    }

    async fn find_user_by_email(&self, email: &str) -> Result<Option<User>, ()> {
        self.user_collection
            .find_one(doc! {"email": email})
            .await
            .or(Err(()))
    }

    async fn insert_user(&self, user: &User) -> Result<(), ()> {
        match self.user_collection.insert_one(user).await {
            Ok(_) => Ok(()),
            Err(_) => Err(()),
        }
    }
}

impl UnconfirmedUserDb for MongoDBHandler {
    async fn insert_unconfirmed_user(&self, user: &UnconfirmedUser) -> Result<(), ()> {
        match self.unconfirmed_user_collection.insert_one(user).await {
            Ok(_) => Ok(()),
            Err(_) => Err(()),
        }
    }

    async fn confirm_user(&self, confirmation_token: &str) -> Result<Option<UnconfirmedUser>, ()> {
        if let Ok(user_option) = self
            .unconfirmed_user_collection
            .find_one(doc! {"confirmation_token": confirmation_token})
            .await
        {
            match user_option {
                Some(user) if !user.confirmed => {
                    if self
                        .unconfirmed_user_collection
                        .update_one(
                            doc! {"confirmation_token": confirmation_token},
                            doc! {"$set": doc! {"confirmed": true}},
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
            .find_one(doc! {"username": username})
            .await
            .or(Err(()))
    }

    async fn find_unconfirmed_user_user_by_email(
        &self,
        email: &str,
    ) -> Result<Option<UnconfirmedUser>, ()> {
        self.unconfirmed_user_collection
            .find_one(doc! {"email": email})
            .await
            .or(Err(()))
    }
}

impl PostDb for MongoDBHandler {
    async fn create_post(&self, post: &Post) -> Result<(), ()> {
        match self
            .post_collection
            .insert_one(PostModel::from(post.clone()))
            .await
        {
            Ok(_) => Ok(()),
            Err(_) => Err(()),
        }
    }

    async fn update_post(
        &self,
        slug: &str,
        updated_content: &str,
        updated_title: &str,
        updated_summary: Option<&str>,
    ) -> Result<u64, ()> {
        match self
            .post_collection
            .update_one(
                doc! {"slug": slug},
                doc! {"$set": doc! {"content": updated_content, "title": updated_title, "summary": updated_summary}},
            )
            .await
        {
            Ok(result) => Ok(result.modified_count),
            Err(_) => Err(()),
        }
    }

    async fn delete_post(&self, slug: &str) -> Result<u64, ()> {
        match self.post_collection.delete_one(doc! {"slug": slug}).await {
            Ok(result) => Ok(result.deleted_count),
            Err(_) => Err(()),
        }
    }

    async fn get_post(&self, slug: &str) -> Result<Option<Post>, ()> {
        match self.post_collection.find_one(doc! {"slug": slug}).await {
            Ok(Some(post)) => Ok(Some(post.into())),
            Ok(None) => Ok(None),
            Err(_) => Err(()),
        }
    }

    async fn get_posts(&self, query: &PostsQueryParams) -> Result<Vec<Post>, ()> {
        let page = query.page.unwrap_or(1);
        let per_page = query.per_page.unwrap_or(10);
        let limit = per_page as i64;
        let offset = (page - 1) * per_page;

        let sort_option = if let Some(ref sort_by) = query.sort_by {
            let sort_order = if query.sort_order.as_deref() == Some("desc") {
                -1
            } else {
                1
            };
            doc! {sort_by: sort_order}
        } else {
            doc! {}
        };

        if let Ok(cursor) = self
            .post_collection
            .find(doc! {})
            .limit(limit)
            .skip(offset)
            .sort(sort_option)
            .await
        {
            match cursor.try_collect::<Vec<PostModel>>().await {
                Ok(v) => {
                    return Ok(v.into_iter().map(|post| post.into()).collect());
                }
                Err(e) => {
                    println!("Error: {}", e);
                }
            }
        }

        Err(())
    }

    async fn calculate_total_pages(&self, per_page: u64) -> Result<u64, ()> {
        if let Ok(total_posts) = self.post_collection.count_documents(doc! {}).await {
            let total_pages = (total_posts as f64 / per_page as f64).ceil() as u64;
            return Ok(total_pages);
        }
        Err(())
    }
}
