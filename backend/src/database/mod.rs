pub mod mongo;
pub mod user;

pub trait DBHandler: user::UserDb + user::UnconfirmedUserDb {}
