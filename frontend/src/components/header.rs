use log::info;
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::use_store;

use crate::{routes::AppRoute, services::auth::AuthService, utils::*};

#[function_component(Header)]
pub fn header() -> Html {
    let (state, _dispatch) = use_store::<AppState>();

    let logout = |_| match AuthService::logout() {
        Ok(_) => {
            info!("Loged out");
        }
        Err(_) => {
            log::error!("Error loging out")
        }
    };

    html! {
        <header class="center-content">
            <Link<AppRoute>
                classes="logo hide-unless-hover"
                to={AppRoute::Home}> <span class="paren">
                    {"{"}</span> <span> { " st" } </span> <span class="hidden"> { "udentenherz" } </span>
                    <span> { " " } </span> <span class="paren"> {"}"}</span>
            </Link<AppRoute>>
            <div class="separator"> </div>
            <nav>
                <Link<AppRoute> to={AppRoute::Home}> { "Home" } </Link<AppRoute>>
                <Link<AppRoute> to={AppRoute::Blog}> { "Blog" } </Link<AppRoute>>
                <div> { "|" }</div>
                <div class="header-user">
                    if let Some(User {username, role }) = &state.user {
                        if role == "Admin" || role == "Editor" {
                            <Link<AppRoute> classes="clickable" to={AppRoute::Create}>
                                <i class="fa-regular fa-pen-to-square icon"></i> { "Create" }
                            </Link<AppRoute>>
                        }
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
