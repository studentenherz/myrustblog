use wasm_bindgen_futures::spawn_local;
use web_sys::{wasm_bindgen::JsCast, HtmlInputElement};
use yew::prelude::*;
use yew_router::prelude::*;

use crate::{routes::AppRoute, services::auth::AuthService, utils::window::get_current_host};

#[derive(Debug, Clone, Default)]
pub struct UserRegistration {
    username: String,
    email: String,
    password: String,
    host: String,
}

pub enum Msg {
    UpdateUsername(String),
    UpdateEmail(String),
    UpdatePassword(String),
    Submit,
    Ignore,
}

impl Component for UserRegistration {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self::default()
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="register">
                <h2> {"User registration"} </h2>
                <form onsubmit={ctx.link().callback(|e: yew::events::SubmitEvent| {
                    e.prevent_default(); // Prevent the default form submission
                    Msg::Submit
                })}>
                    <div class="input-wrapper">
                        <i class="fas fa-user icon"></i>
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
                    </div>
                    <div class="input-wrapper">
                        <i class="fas fa-envelope icon"></i>
                        <input
                            type="email"
                            placeholder="Email"
                            value={self.email.clone()}
                            onchange={ctx.link().callback(|e: Event| {
                                if let Some(input) = e.target().and_then(|t| t.dyn_into::<HtmlInputElement>().ok()) {
                                    Msg::UpdateEmail(input.value())
                                } else {
                                    Msg::Ignore
                                }
                            })}
                        />
                    </div>
                    <div class="input-wrapper">
                        <i class="fas fa-lock icon"></i>
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
                    </div>
                    <button>{"Register"}</button>
                </form>

                <Link<AppRoute> to={AppRoute::Login} classes="bottom"> {"Already have an account? Login"} </Link<AppRoute>>
            </div>
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::UpdateUsername(value) => self.username = value,
            Msg::UpdateEmail(value) => self.email = value,
            Msg::UpdatePassword(value) => self.password = value,
            Msg::Submit => {
                let form = self.clone();
                spawn_local(async move {
                    match AuthService::register(
                        form.username.as_str(),
                        form.email.as_str(),
                        form.password.as_str(),
                    )
                    .await
                    {
                        Ok(()) => {}
                        Err(_) => {}
                    };
                });
            }
            Msg::Ignore => {}
        }
        true
    }
}
