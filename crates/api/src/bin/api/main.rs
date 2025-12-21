use bipa_api::routes::Routes;
use bipa_core::app_context::create_app_context;
use bipa_core::state::AppState;
use derust::databasex::PostgresDatabase;
use derust::envx::Environment;
use derust::httpx::start;
use derust::tracex;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _guard = tracex::init();

    let env = Environment::detect().ok().unwrap_or(Environment::Local);
    let app_state = AppState::new(env).await?;

    let database = PostgresDatabase::create_from_config(&app_state.app_config.database).await?;
    let context = create_app_context(env, &app_state, database.clone()).await?;
    let router = Routes::routes(&env);

    let port = app_state.app_config.port.unwrap_or(9095);

    let web_socket_enabled = false;

    /*
    http server start using my own crate
    this helps me standardize observability with logs, metrics, and traces over http call received,
    http call requested, and database calls.
    also this crate provide some features like outbox table pattern and feature flags.
    here for more: https://github.com/deroldo/derust
     */
    start(port, context, router, web_socket_enabled).await
}
