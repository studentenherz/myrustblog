use yew::prelude::*;

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
        <footer class="center-content">
            <div class="separator" ></div>
            <section class="socials">
                <a href="https://github.com/studentenherz" target={"_blank"}><i class="icon fa-brands fa-github"></i></a>
                <a href="https://www.linkedin.com/in/studentenherz/" target={"_blank"}><i class="icon fa-brands fa-linkedin"></i></a>
                <a href="https://t.me/michelromero" target={"_blank"}><i class="icon fa-brands fa-telegram"></i></a>
                <a href="/rss" target={"_blank"}><i class="icon fa-solid fa-square-rss"></i></a>
            </section>
            <center>
                { "Made using " } <a href={"https://yew.rs/"} target={"_blank"}> {"Yew"} </a>
                {" and "} <a href={"https://actix.rs/"} target={"_blank"}> {"Actix"} </a>
                <i>{ " by Michel Romero "}</i>
                <a href={env!("APP_COMMIT_URL")} target={"_blank"}> {env!("APP_COMMIT_HASH")} </a>
            </center>
        </footer>
    }
}
