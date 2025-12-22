use bipa_core::app_context::create_app_context;
use bipa_core::state::AppState;
use derust::databasex::PostgresDatabase;
use derust::envx::Environment;
use derust::http_clientx::HttpClient;
use derust::httpx::AppContext;
use test_context::AsyncTestContext;
use tokio::sync::OnceCell;
use wiremock::MockServer;

static GLOBAL_CONTEXT: OnceCell<AppContext<AppState>> = OnceCell::const_new();
static MOCK_SERVER: OnceCell<MockServer> = OnceCell::const_new();

pub struct WorkerContext {
    pub context: AppContext<AppState>,
}

impl WorkerContext {
    pub async fn mock_server(&self) -> &MockServer {
        MOCK_SERVER.get().unwrap()
    }
}

impl AsyncTestContext for WorkerContext {
    async fn setup() -> Self {
        let context = start_context().await;
        WorkerContext { context }
    }
}

async fn start_context() -> AppContext<AppState> {
    let mock_server = MOCK_SERVER.get_or_init(|| async { MockServer::start().await }).await;

    GLOBAL_CONTEXT
        .get_or_init(|| async {
            let env = Environment::Test;
            let mut app_state = AppState::new(env).await.unwrap();

            app_state.gateway.lightning_nodes = HttpClient::new("lightning_nodes", &mock_server.uri(), 3000, 3000).await.unwrap();

            let database = PostgresDatabase::create_from_config(&app_state.app_config.database).await.unwrap();
            create_app_context(env, &app_state, database.clone()).await.unwrap()
        })
        .await
        .clone()
}
