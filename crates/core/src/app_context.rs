use crate::state::AppState;
use derust::databasex::PostgresDatabase;
use derust::envx::Environment;
use derust::metricx::PrometheusConfig;
use regex::Regex;
use std::error::Error;

pub async fn create_app_context(
    env: Environment,
    app_state: &AppState,
    database: PostgresDatabase,
) -> Result<derust::httpx::AppContext<AppState>, Box<dyn Error>> {
    let application_name = "bipa";

    let prometheus_config = PrometheusConfig {
        denied_metric_tags: vec![],
        denied_metric_tags_by_regex: vec![Regex::new(".+_id$")?],
    };

    derust::httpx::AppContext::new(application_name, env, database.clone(), prometheus_config.clone(), app_state.clone())
}
