use bipa_core::app_context::create_app_context;
use bipa_core::state::AppState;
use bipa_worker::runner::lightning_node_loader_runner::LightningNodeLoaderRunner;
use derust::databasex::PostgresDatabase;
use derust::envx::Environment;
use derust::tracex;
use std::error::Error;
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let _guard = tracex::init();
    info!("Bipa-worker starting");

    let env = Environment::detect().ok().unwrap_or(Environment::Local);
    let app_state = AppState::new(env).await?;
    let database = PostgresDatabase::create_from_config(&app_state.app_config.database).await?;
    let context = create_app_context(env, &app_state, database.clone()).await?;

    info!("Bipa-worker context loaded");

    LightningNodeLoaderRunner::run(&context).await
}
