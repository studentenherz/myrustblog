use yew::prelude::*;

use crate::utils::cookies::get_cookie;

#[derive(Debug, Default)]
pub struct Home {
    username: Option<String>,
}

pub enum Msg {}

impl Component for Home {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Home {
            username: get_cookie("_username"),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            if self.username.is_some() {
                <h1>{ format!("Welcome, {}", self.username.clone().unwrap()) }</h1>
            }
            else{
                <h1>{ "Welcome to the Home Page" }</h1>
            }
        }
    }
}
