use yew::prelude::*;

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
        <footer class="center-content">
            <div class="separator" ></div>
            <div class="socials">
                <a href="https://github.com/studentenherz" target={"_blank"}><i class="icon fa-brands fa-github"></i></a>
                <a href="https://www.linkedin.com/in/studentenherz/" target={"_blank"}><i class="icon fa-brands fa-linkedin"></i></a>
                <a href="https://t.me/michelromero" target={"_blank"}><i class="icon fa-brands fa-telegram"></i></a>
            </div>
            <center>
                { "Made using " } <a href={"https://yew.rs/"} target={"_blank"}> {"Yew"} </a>
                <i>{ " by Michel Romero"}</i>
            </center>
        </footer>
    }
}
