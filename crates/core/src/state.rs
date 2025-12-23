use crate::config::AppConfig;
use derust::envx::{Environment, load_app_config};
use derust::http_clientx::HttpClient;

#[derive(Clone)]
pub struct AppState {
    pub app_config: AppConfig,
    pub gateway: GatewayState,
}

#[derive(Clone)]
pub struct GatewayState {
    pub lightning_nodes: HttpClient,
}

impl AppState {
    pub async fn new(env: Environment) -> Result<Self, Box<dyn std::error::Error>> {
        let secrets_manager_ids = vec!["bipa_secrets"];
        let app_config = load_app_config::<AppConfig>(env, None, secrets_manager_ids).await?;
        let gateway = GatewayState {
            lightning_nodes: HttpClient::new(
                "lightning_nodes",
                &app_config.lightning_api.base_url,
                app_config.lightning_api.timeout.unwrap_or(3000),
                app_config.lightning_api.connection_timeout.unwrap_or(1000),
            ).await?,
        };

        Ok(Self { app_config, gateway })
    }
}
