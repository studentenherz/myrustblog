use wasm_bindgen_futures::spawn_local;
use web_sys::{wasm_bindgen::JsCast, HtmlInputElement};
use yew::prelude::*;
use yew_router::prelude::*;

use crate::{
    components::service_notifications::{NotificationLevel, ServiceNotification},
    routes::AppRoute,
    services::auth::{AuthError, AuthService},
};

use common::utils::{is_valid_email, is_valid_password, is_valid_username};

#[derive(Debug, Clone, Default)]
pub struct UserRegistration {
    username: String,
    email: String,
    password: String,
    service_notification_text: String,
    service_notification_level: NotificationLevel,
    disable_submit: bool,
}

pub enum Msg {
    UpdateUsername(String),
    UpdateEmail(String),
    UpdatePassword(String),
    Submit,
    Ignore,
    Success(String),
    Error(String),
}

impl Component for UserRegistration {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self::default()
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let valid_username = is_valid_username(&self.username);
        let valid_email = is_valid_email(&self.email);
        let valid_password = is_valid_password(&self.password);
        let enabled = valid_username && valid_email && valid_password && !self.disable_submit;

        html! {
            <div class="register">
                <h2> {"User registration"} </h2>
                if !self.service_notification_text.is_empty() {
                    < ServiceNotification message={self.service_notification_text.clone()} level={self.service_notification_level} />
                }

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
                        <i class="fas fa-envelope icon"></i>
                        <input
                            type="email"
                            placeholder="Email"
                            value={self.email.clone()}
                            oninput={ctx.link().callback(|e: InputEvent| {
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
                            oninput={ctx.link().callback(|e: InputEvent| {
                                if let Some(input) = e.target().and_then(|t| t.dyn_into::<HtmlInputElement>().ok()) {
                                    Msg::UpdatePassword(input.value())
                                } else {
                                    Msg::Ignore
                                }
                            })}
                        />
                    </div>
                    <button disabled={!enabled}>{"Register"}</button>
                </form>

                <Link<AppRoute> to={AppRoute::Login} classes="bottom"> {"Already have an account? Login"} </Link<AppRoute>>
            </div>
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::UpdateUsername(value) => self.username = value,
            Msg::UpdateEmail(value) => self.email = value,
            Msg::UpdatePassword(value) => self.password = value,
            Msg::Submit => {
                self.service_notification_text.clear();
                self.disable_submit = true;
                let form = self.clone();
                let success_callback = ctx.link().callback(|_| {
                    Msg::Success("An e-mail was sent to you for confirmation".to_string())
                });
                let error_callback = ctx.link().callback(|err: AuthError| {
                    Msg::Error(format!(
                        "Error while trying to register, {}",
                        match err {
                            AuthError::RegistrationConflict(field) => {
                                format!("there is already one account with the same {field}")
                            }
                            _ => "please try again later".to_string(),
                        },
                    ))
                });

                spawn_local(async move {
                    match AuthService::register(
                        form.username.as_str(),
                        form.email.as_str(),
                        form.password.as_str(),
                    )
                    .await
                    {
                        Ok(()) => {
                            success_callback.emit(());
                        }
                        Err(err) => {
                            error_callback.emit(err);
                        }
                    };
                });
            }
            Msg::Success(text) => {
                self.service_notification_text = text;
                self.service_notification_level = NotificationLevel::Info;
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
