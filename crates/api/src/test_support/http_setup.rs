use crate::routes::Routes;
use bipa_core::app_context::create_app_context;
use bipa_core::state::AppState;
use derust::databasex::PostgresDatabase;
use derust::envx::Environment;
use derust::httpx::{AppContext, start_test};
use reqwest::Client;
use std::net::SocketAddr;
use test_context::AsyncTestContext;
use tokio::net::TcpListener;
use tokio::sync::OnceCell;
use tokio::task::JoinHandle;

static GLOBAL_CONTEXT: OnceCell<AppContext<AppState>> = OnceCell::const_new();

pub struct HttpContext {
    pub base_url: String,
    pub client: Client,
    pub context: AppContext<AppState>,
    shutdown: JoinHandle<()>,
}

impl AsyncTestContext for HttpContext {
    async fn setup() -> Self {
        let (context, addr, handler) = start_server().await;

        HttpContext {
            base_url: format!("http://{addr}"),
            client: Client::new(),
            context,
            shutdown: handler,
        }
    }
}

impl Drop for HttpContext {
    fn drop(&mut self) {
        self.shutdown.abort();
    }
}

async fn start_server() -> (AppContext<AppState>, SocketAddr, JoinHandle<()>) {
    let env = Environment::Test;

    let context = GLOBAL_CONTEXT
        .get_or_init(|| async {
            let app_state = AppState::new(env).await.unwrap();
            let database = PostgresDatabase::create_from_config(&app_state.app_config.database).await.unwrap();
            create_app_context(env, &app_state, database.clone()).await.unwrap()
        })
        .await;

    let app = Routes::routes(&env);
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let handle = tokio::spawn(async move { start_test(context.clone(), app, listener).await.unwrap() });

    (context.clone(), addr, handle)
}
