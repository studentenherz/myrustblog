use yew::prelude::*;

use crate::utils::*;

#[derive(PartialEq, Properties)]
pub struct HeaderProps {
    pub user: Option<User>,
}

#[function_component(Header)]
pub fn header(HeaderProps { user }: &HeaderProps) -> Html {
    html! {
        <header class="center-content">
            <a
                class="logo hide-unless-hover"
                href={ "/" }> <span class="paren">
                    {"{"}</span> <span> { " st" } </span> <span class="hidden"> { "udentenherz" } </span>
                    <span> { " " } </span> <span class="paren"> {"}"}</span>
            </a>
            <div class="separator"> </div>
            <nav>
                <a href={ "/" }> { "Home" } </a>
                <a href={ "/blog" }> { "Blog" } </a>
                <div> { "|" }</div>
                <div class="header-user">
                    if let Some(User {username, role }) = user {
                        if role == "Admin" || role == "Editor" {
                            <a classes="clickable" href={ "/create" }>
                                <i class="icon-edit icon"></i> { "Create" }
                            </a>
                        }
                        <div class="username">{ username }</div>
                        <a class="button" href="/logout"> { "Logout" } </a>
                    }
                    else {
                        <a class="button" href="/login"> { "Login" } </a>
                    }
                </div>
            </nav>
        </header>
    }
}
