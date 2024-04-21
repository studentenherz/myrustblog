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
        <header class="center-content">
            <Link<AppRoute> classes="logo hide-unless-hover" to={AppRoute::Home}> <span class="paren"> {"{"}</span> <span> { " st" } </span> <span class="hidden"> { "udentenherz" } </span> <span> { " " } </span> <span class="paren"> {"}"}</span> </Link<AppRoute>>
            <div class="separator"> </div>
            <nav>
                <Link<AppRoute> to={AppRoute::Blog}> { "Blog" } </Link<AppRoute>>
                <div> { "|" }</div>
                <div class="header-user">
                    if let Some(username) = loged_in_as() {
                        <Link<AppRoute> classes="clickable" to={AppRoute::Create}> <i class="fa-regular fa-pen-to-square icon"></i> { "Create" } </Link<AppRoute>>
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
