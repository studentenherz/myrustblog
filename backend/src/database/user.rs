use crate::models::{UnconfirmedUser, User};

pub trait UserDb {
    async fn find_user(&self, username: &str) -> Result<Option<User>, ()>;
    async fn find_user_by_email(&self, email: &str) -> Result<Option<User>, ()>;
    async fn insert_user(&self, user: &User) -> Result<(), ()>;
}

pub trait UnconfirmedUserDb {
    async fn find_unconfirmed_user(&self, username: &str) -> Result<Option<UnconfirmedUser>, ()>;
    async fn find_unconfirmed_user_user_by_email(
        &self,
        email: &str,
    ) -> Result<Option<UnconfirmedUser>, ()>;
    async fn insert_unconfirmed_user(&self, user: &UnconfirmedUser) -> Result<(), ()>;
    async fn confirm_user(&self, confirmation_token: &str) -> Result<Option<UnconfirmedUser>, ()>;
}
