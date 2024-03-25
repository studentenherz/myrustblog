#[macro_export]
macro_rules! api_url {
    ($path:expr) => {
        format!("{}{}", std::env!("APP_BACKEND_URL"), $path)
    };
}
