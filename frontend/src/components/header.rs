use log::info;
use wasm_bindgen::JsCast;
use web_sys::{window, Window};
use yew::prelude::*;
use yew_router::prelude::*;

use crate::{routes::AppRoute, services::auth::AuthService, utils::loged_in_as};

#[function_component(Header)]
pub fn header() -> Html {
    let logout = |_| match AuthService::logout() {
        Ok(_) => {
            info!("Loged out");
            if let Some(window) = window() {
                if let Ok(window) = window.dyn_into::<Window>() {
                    let _ = window.location().reload();
                }
            }
        }
        Err(_) => {
            log::error!("Error loging out")
        }
    };

    html! {
        <header>
            <Link<AppRoute> classes="logo" to={AppRoute::Home}> { "My Rust Blog" } </Link<AppRoute>>
            <div class="separator"> </div>
            <nav>
                <Link<AppRoute> classes="logo" to={AppRoute::Blog}> { "Blog" } </Link<AppRoute>>
                <div> { "|" }</div>
                <div class="header-user">
                    if let Some(username) = loged_in_as() {
                        <div>{ username }</div>
                        <button onclick={ logout }> { "Logout" } </button>
                    }
                    else {
                        <Link<AppRoute> classes="button" to={AppRoute::Login}> { "Login" } </Link<AppRoute>>
                    }
                </div>
            </nav>
        </header>
    }
}
