use crate::models::Post;

pub trait PostDb {
    async fn create_post(&self, post: &Post) -> Result<(), ()>;
    async fn update_post(&self, id: &str, updated_content: &str) -> Result<u64, ()>;
    async fn delete_post(&self, id: &str) -> Result<u64, ()>;
    async fn get_post(&self, id: &str) -> Result<Option<Post>, ()>;
}
