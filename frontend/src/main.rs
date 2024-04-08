use yew::prelude::*;
use yew_router::prelude::*;

mod components;
mod pages;
mod routes;
mod services;
mod utils;

use components::{Home, LoginForm, UserConfirmation, UserRegistration};
use pages::*;
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
        AppRoute::Home => html! { <Layout> <Home /> </Layout>},
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
