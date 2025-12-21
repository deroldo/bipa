use derust::databasex::DatabaseConfig;
use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct AppConfig {
    pub port: Option<u16>,
    pub database: DatabaseConfig,
    pub bitcoin_sats: f64,
}
