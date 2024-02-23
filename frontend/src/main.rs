use components::{user_login::LoginForm, user_registration::UserRegistration};
use services::api::Api;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

mod components;
mod services;
mod utils;

#[function_component(App)]
fn app() -> Html {
    let onclick = Callback::from(|_| {
        // Create a local closure that makes the `get_home` call
        let get_home = async {
            match Api::get_home().await {
                Ok(()) => {
                    log::info!("Successfully fetched home!");
                    // Perform actions on success, e.g., update state or UI
                }
                Err(()) => {
                    log::error!("Failed to fetch home!");
                    // Perform actions on failure, e.g., show an error message
                }
            }
        };

        // Spawn the local async task
        spawn_local(get_home);
    });

    html! {
        <div>
            <h1>{ "Hello, Yew!" }</h1>
            <UserRegistration />
            <LoginForm />
            <button {onclick}>{ "Test Get Home" }</button>
        </div>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
