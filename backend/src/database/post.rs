use crate::models::PostsQueryParams;
use common::Post;

pub trait PostDb {
    async fn create_post(&self, post: &Post) -> Result<(), ()>;
    async fn update_post(
        &self,
        slug: &str,
        updated_content: &str,
        updated_title: &str,
        updated_summary: Option<&str>,
    ) -> Result<u64, ()>;
    async fn delete_post(&self, slug: &str) -> Result<u64, ()>;
    async fn get_post(&self, slug: &str) -> Result<Option<Post>, ()>;
    async fn get_posts(&self, query: &PostsQueryParams) -> Result<Vec<Post>, ()>;
    async fn calculate_total_pages(&self, per_page: u64) -> Result<u64, ()>;
}
