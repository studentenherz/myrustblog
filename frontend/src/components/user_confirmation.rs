use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::services::auth::AuthService;

#[derive(Properties, PartialEq)]
pub struct ConfirmProps {
    pub token: String,
}

#[function_component(UserConfirmation)]
pub fn confirm_email(props: &ConfirmProps) -> Html {
    let confirmation_status = use_state(|| "Confirming...".to_string());
    let confirmed = use_state(|| false);

    {
        let confirmation_status = confirmation_status.clone();
        let confirmed = confirmed.clone();
        let token = props.token.clone();

        use_effect_with((), |_| {
            spawn_local(async move {
                match AuthService::confirm(&token).await {
                    Ok(_) => {
                        confirmation_status.set("Confirmed!!!".to_string());
                        confirmed.set(true);
                    }
                    Err(_) => confirmation_status.set("Error while confirming".to_string()),
                };
            })
        });
    }

    html! {
        <>
            { (*confirmation_status).clone() }
            if *confirmed {
                <a href="/"> { "Go to the homepage " } </a>
            }
        </>
    }
}
