use wasm_bindgen_futures::spawn_local;
use web_sys::{wasm_bindgen::JsCast, HtmlInputElement};
use yew::prelude::*;

use crate::services::auth::AuthService;

#[derive(Default, Clone)]
pub struct LoginForm {
    username: String,
    password: String,
}

pub enum Msg {
    UpdateUsername(String),
    UpdatePassword(String),
    Submit,
    Ignore,
}

impl Component for LoginForm {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self::default()
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <form onsubmit={ctx.link().callback(|e: SubmitEvent| {
                e.prevent_default();
                Msg::Submit
            })}>
                <input
                    type="text"
                    placeholder="Username"
                    value={self.username.clone()}
                    onchange={ctx.link().callback(|e: Event| {
                        if let Some(input) = e.target().and_then(|t| t.dyn_into::<HtmlInputElement>().ok()) {
                            Msg::UpdateUsername(input.value())
                        } else {
                            Msg::Ignore
                        }
                    })}
                />
                <input
                    type="password"
                    placeholder="Password"
                    value={self.password.clone()}
                    onchange={ctx.link().callback(|e: Event| {
                        if let Some(input) = e.target().and_then(|t| t.dyn_into::<HtmlInputElement>().ok()) {
                            Msg::UpdatePassword(input.value())
                        } else {
                            Msg::Ignore
                        }
                    })}
                />
                <button type="submit">{"Log In"}</button>
            </form>
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Submit => {
                let form = self.clone();

                spawn_local(async move {
                    match AuthService::login(form.username.as_str(), form.password.as_str()).await {
                        Ok(()) => {}
                        Err(_) => {}
                    };
                });
            }
            Msg::UpdatePassword(value) => self.password = value,
            Msg::UpdateUsername(value) => self.username = value,
            Msg::Ignore => {}
        }
        true
    }
}
