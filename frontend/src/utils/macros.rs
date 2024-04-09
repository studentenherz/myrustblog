#[macro_export]
macro_rules! api_url {
    ($path:expr) => {
        format!("/api{}", $path)
    };
}
