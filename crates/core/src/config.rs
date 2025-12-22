use derust::databasex::DatabaseConfig;
use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct AppConfig {
    pub port: Option<u16>,
    pub database: DatabaseConfig,
    pub bitcoin_sats: u64,
    pub lightning_api: LightningApiConfig,
}

#[derive(Clone, Deserialize)]
pub struct LightningApiConfig {
    pub base_url: String,
    pub timeout: Option<u64>,
    pub connection_timeout: Option<u64>,
}
