use yew::prelude::*;
use yew_router::prelude::*;

use crate::{routes::AppRoute, utils::loged_in_as};

#[function_component(Header)]
pub fn header() -> Html {
    html! {
        <header>
            <div class="logo"> { "My Rust Blog" } </div>
            <div class="separator"> </div>
            <div class="header-user">
                if let Some(username) = loged_in_as() {
                    { username }
                }
                else {
                    <Link<AppRoute> classes="button" to={AppRoute::Login}> { "Login" } </Link<AppRoute>>
                }
            </div>
        </header>
    }
}
