use yew::prelude::*;
use yew_router::prelude::*;

mod components;
mod routes;
mod services;
mod utils;

use components::{user_login::LoginForm, user_registration::UserRegistration};
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
        AppRoute::Home => html! { <h1>{ "Welcome to the Home Page" }</h1> },
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
