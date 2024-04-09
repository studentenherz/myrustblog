use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum AppRoute {
    #[at("/login")]
    Login,
    #[at("/register")]
    Register,
    #[at("/confirm/:token")]
    Confirm { token: String },
    #[at("/blog")]
    Blog,
    #[at("/post/:slug")]
    Post { slug: String },
    #[at("/")]
    Home,
    #[at("/404")]
    NotFound,
    #[at("/create")]
    Create,
}
