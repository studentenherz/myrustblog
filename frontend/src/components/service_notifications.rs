use yew::prelude::*;

#[derive(PartialEq, Default, Clone, Debug, Copy)]
pub enum NotificationLevel {
    #[default]
    Success,
    Info,
    #[allow(dead_code)]
    Warning,
    Error,
}

#[derive(Properties, PartialEq, Default, Clone, Debug)]
pub struct ServiceNotificationProps {
    pub message: String,
    pub level: NotificationLevel,
}

#[function_component(ServiceNotification)]
pub fn service_notification(props: &ServiceNotificationProps) -> Html {
    let class = String::from("service-notification ")
        + match props.level {
            NotificationLevel::Success => "success",
            NotificationLevel::Info => "info",
            NotificationLevel::Warning => "warning",
            NotificationLevel::Error => "error",
        };

    html! {
        <div class={class} >
            {props.message.clone()}
        </div>
    }
}
