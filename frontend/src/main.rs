use yew::prelude::*;

use components::{user_login::LoginForm, user_registration::UserRegistration};

mod components;
mod services;
mod utils;

#[function_component]
fn App() -> Html {
    html! {
        <div>
            <h1>{ "Hello, Yew!" }</h1>
            <UserRegistration />
            <LoginForm />
        </div>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
