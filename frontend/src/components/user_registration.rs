use wasm_bindgen_futures::spawn_local;
use web_sys::{HtmlInputElement, SubmitEvent};
use yew::prelude::*;
use yew_router::prelude::*;

use crate::{
    components::{NotificationLevel, ServiceNotification},
    routes::AppRoute,
    services::auth::{AuthError, AuthService},
};
use common::utils::{is_valid_email, is_valid_password, is_valid_username};

#[function_component(UserRegistration)]
pub fn user_registration() -> Html {
    let username = use_state(String::new);
    let email = use_state(String::new);
    let password = use_state(String::new);
    let service_notification_text = use_state(String::new);
    let service_notification_level = use_state(NotificationLevel::default);
    let disable_submit = use_state(|| false);

    let valid_username = is_valid_username(&username);
    let valid_email = is_valid_email(&email);
    let valid_password = is_valid_password(&password);
    let enabled = valid_username && valid_email && valid_password && !*disable_submit;

    let onsubmit = {
        let service_notification_text = service_notification_text.clone();
        let service_notification_level = service_notification_level.clone();
        let disable_submit = disable_submit.clone();
        let username = username.clone();
        let email = email.clone();
        let password = password.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            service_notification_text.set(String::new());
            disable_submit.set(true);

            let username = username.clone();
            let email = email.clone();
            let password = password.clone();
            let service_notification_text = service_notification_text.clone();
            let service_notification_level = service_notification_level.clone();
            let disable_submit = disable_submit.clone();
            let success_text = "An e-mail was sent to you for confirmation".to_string();

            spawn_local(async move {
                match AuthService::register(username.as_str(), email.as_str(), password.as_str())
                    .await
                {
                    Ok(()) => {
                        service_notification_text.set(success_text.clone());
                        service_notification_level.set(NotificationLevel::Info);
                    }
                    Err(err) => {
                        disable_submit.set(false);
                        let error_text = match err {
                            AuthError::RegistrationConflict(field) => {
                                format!("There is already one account with the same {field}")
                            }
                            _ => "Please try again later".to_string(),
                        };
                        service_notification_text
                            .set(format!("Error while trying to register, {}", error_text));
                        service_notification_level.set(NotificationLevel::Error);
                    }
                }
            });
        })
    };

    let on_username_input = {
        let username = username.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                username.set(input.value());
            }
        })
    };

    let on_email_input = {
        let email = email.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                email.set(input.value());
            }
        })
    };

    let on_password_input = {
        let password = password.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                password.set(input.value());
            }
        })
    };

    html! {
        <div class="register">
            <h2>{"User registration"}</h2>
            if !(*service_notification_text).is_empty() {
                <ServiceNotification message={(*service_notification_text).clone()} level={*service_notification_level} />
            }

            <form onsubmit={onsubmit}>
                <div class="input-wrapper">
                    <i class="fas fa-user icon"></i>
                    <input
                        type="text"
                        placeholder="Username"
                        value={(*username).clone()}
                        oninput={on_username_input}
                    />
                </div>
                <div class="input-wrapper">
                    <i class="fas fa-envelope icon"></i>
                    <input
                        type="email"
                        placeholder="Email"
                        value={(*email).clone()}
                        oninput={on_email_input}
                    />
                </div>
                <div class="input-wrapper">
                    <i class="fas fa-lock icon"></i>
                    <input
                        type="password"
                        placeholder="Password"
                        value={(*password).clone()}
                        oninput={on_password_input}
                    />
                </div>
                <button disabled={!enabled} type="submit">{"Register"}</button>
            </form>

            <Link<AppRoute> to={AppRoute::Login} classes="bottom">{"Already have an account? Login"}</Link<AppRoute>>
        </div>
    }
}
