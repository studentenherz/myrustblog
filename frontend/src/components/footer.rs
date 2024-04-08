use yew::prelude::*;

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
        <footer>
            { "by " } <a href={"https://github.com/studentenherz"} target={"_blank"}> {"Michel Romero Rodríguez"} </a>
        </footer>
    }
}
