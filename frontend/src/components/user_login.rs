use wasm_bindgen_futures::spawn_local;
use web_sys::{wasm_bindgen::JsCast, HtmlInputElement};
use yew::prelude::*;
use yew_router::{history::History, prelude::*};

use crate::{
    components::service_notifications::{NotificationLevel, ServiceNotification},
    routes::AppRoute,
    services::auth::{AuthError, AuthService},
    utils::is_loged_in,
};
use common::utils::{is_valid_password, is_valid_username};

#[derive(Default, Clone)]
pub struct LoginForm {
    username: String,
    password: String,
    service_notification_text: String,
    service_notification_level: NotificationLevel,
    disable_submit: bool,
}

pub enum Msg {
    UpdateUsername(String),
    UpdatePassword(String),
    Submit,
    Ignore,
    Success(String),
    Error(String),
}

impl Component for LoginForm {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self::default()
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        if is_loged_in() {
            return html! {
                <Redirect<AppRoute> to={AppRoute::Home}/>
            };
        }

        let valid_username = is_valid_username(&self.username);
        let valid_password = is_valid_password(&self.password);
        let enabled = valid_username && valid_password && !self.disable_submit;

        html! {
            <div class="login">
                <h2> {"User login"} </h2>
                if !self.service_notification_text.is_empty() {
                    < ServiceNotification message={self.service_notification_text.clone()} level={self.service_notification_level} />
                }

                <form onsubmit={ctx.link().callback(|e: SubmitEvent| {
                    e.prevent_default();
                    Msg::Submit
                })}>
                    <div class="input-wrapper">
                        <i class="fas fa-user icon"></i>
                        <input
                            type="text"
                            placeholder="Username"
                            value={self.username.clone()}
                            oninput={ctx.link().callback(|e: InputEvent| {
                                if let Some(input) = e.target().and_then(|t| t.dyn_into::<HtmlInputElement>().ok()) {
                                    Msg::UpdateUsername(input.value())
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
                            oninput={ctx.link().callback(|e: InputEvent| {
                                if let Some(input) = e.target().and_then(|t| t.dyn_into::<HtmlInputElement>().ok()) {
                                    Msg::UpdatePassword(input.value())
                                } else {
                                    Msg::Ignore
                                }
                            })}
                        />
                    </div>
                    <button disabled={!enabled} type="submit">{"Log In"}</button>
                </form>

                <Link<AppRoute> to={AppRoute::Login} classes="link"> {"Forgot username or password?"} </Link<AppRoute>>

                <Link<AppRoute> to={AppRoute::Register} classes="bottom"> {"Don't have an account? Register"} </Link<AppRoute>>
            </div>
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Submit => {
                self.service_notification_text.clear();
                self.disable_submit = true;

                let form = self.clone();
                let success_text = format!("Welcome back, {}!", self.username);
                let success_callback = ctx
                    .link()
                    .callback(move |_| Msg::Success(success_text.clone()));
                let error_callback = ctx.link().callback(|err: AuthError| {
                    Msg::Error(format!(
                        "Error login in, {}",
                        match err {
                            AuthError::LoginError(err_str) => err_str,
                            AuthError::NetworkError => "can't reach server".to_string(),
                            _ => "something went wrong".to_string(),
                        }
                    ))
                });

                spawn_local(async move {
                    match AuthService::login(form.username.as_str(), form.password.as_str()).await {
                        Ok(()) => {
                            success_callback.emit(());
                        }
                        Err(err) => {
                            error_callback.emit(err);
                        }
                    };
                });
            }
            Msg::UpdatePassword(value) => self.password = value,
            Msg::UpdateUsername(value) => self.username = value,
            Msg::Success(text) => {
                self.service_notification_text = text;
                self.service_notification_level = NotificationLevel::Success;

                let history = yew_router::history::BrowserHistory::new();
                history.replace(AppRoute::Home.to_path());
            }
            Msg::Error(text) => {
                self.disable_submit = false;
                self.service_notification_text = text;
                self.service_notification_level = NotificationLevel::Error;
            }
            Msg::Ignore => {}
        }
        true
    }
}
