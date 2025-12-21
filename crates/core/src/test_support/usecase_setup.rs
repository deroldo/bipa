use crate::app_context::create_app_context;
use crate::state::AppState;
use derust::databasex::PostgresDatabase;
use derust::envx::Environment;
use derust::httpx::AppContext;
use test_context::AsyncTestContext;
use tokio::sync::OnceCell;

static GLOBAL_CONTEXT: OnceCell<AppContext<AppState>> = OnceCell::const_new();

pub struct UseCaseContext {
    pub context: AppContext<AppState>,
}

impl AsyncTestContext for UseCaseContext {
    async fn setup() -> Self {
        let context = start_server().await;
        UseCaseContext { context }
    }
}

async fn start_server() -> AppContext<AppState> {
    GLOBAL_CONTEXT
        .get_or_init(|| async {
            let env = Environment::Test;
            let app_state = AppState::new(env).await.unwrap();
            let database = PostgresDatabase::create_from_config(&app_state.app_config.database).await.unwrap();
            create_app_context(env, &app_state, database.clone()).await.unwrap()
        })
        .await
        .clone()
}
