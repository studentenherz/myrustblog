use crate::models::User;

pub trait UserDb {
    async fn find_user(&self, username: &str) -> Result<Option<User>, ()>;
    async fn find_user_by_email(&self, email: &str) -> Result<Option<User>, ()>;
    async fn insert_user(&self, user: &User) -> Result<(), ()>;
}
