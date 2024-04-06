pub mod mongo;
pub mod post;
pub mod user;

pub trait DBHandler: user::UserDb + user::UnconfirmedUserDb + post::PostDb {}
