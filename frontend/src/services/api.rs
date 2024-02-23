use crate::services::auth::AuthService;

pub struct Api;

impl Api {
    pub async fn get_home() -> Result<(), ()> {
        if let Ok(builder) = AuthService::protected_get("http://localhost:8081/") {
            if let Ok(response) = builder.send().await {
                if response.status().is_success() {
                    return Ok(());
                }
            }
        }

        Err(())
    }
}
