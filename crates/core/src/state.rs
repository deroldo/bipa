use crate::config::AppConfig;
use derust::envx::{Environment, load_app_config};

#[derive(Clone)]
pub struct AppState {
    pub app_config: AppConfig,
}

impl AppState {
    pub async fn new(env: Environment) -> Result<Self, Box<dyn std::error::Error>> {
        let secrets_manager_ids = vec!["bipa_secrets", "bipa"];
        let app_config = load_app_config::<AppConfig>(env, None, secrets_manager_ids).await?;

        Ok(Self { app_config })
    }
}
