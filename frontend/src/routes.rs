use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum AppRoute {
    #[at("/login")]
    Login,
    #[at("/register")]
    Register,
    #[at("/confirm/:token")]
    Confirm { token: String },
    #[at("/create")]
    Create,
    #[at("/edit/:slug")]
    Edit { slug: String },
}
