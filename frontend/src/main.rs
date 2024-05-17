use yew::prelude::*;
use yew_router::prelude::*;

mod components;
mod pages;
mod routes;
mod services;
mod utils;

use components::{CreatePost, EditPost, LoginForm, UserConfirmation, UserRegistration};
use routes::AppRoute;

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<AppRoute> render={switch} />
        </BrowserRouter>
    }
}

fn switch(routes: AppRoute) -> Html {
    match routes {
        AppRoute::Login => html! { <LoginForm /> },
        AppRoute::Register => html! { <UserRegistration /> },
        AppRoute::Confirm { token } => html! { <UserConfirmation token={token} /> },
        AppRoute::Create => html! { <CreatePost /> },
        AppRoute::Edit { slug } => html! { <EditPost slug={ slug } /> },
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
