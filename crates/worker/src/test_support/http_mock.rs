use crate::test_support::worker_setup::WorkerContext;
use derust::StatusCode;
use serde_json::Value;
use wiremock::matchers::{method, path};
use wiremock::{Mock, ResponseTemplate};

pub struct HttpMock;

impl HttpMock {
    pub async fn mock_lightning_nodes(
        ctx: &mut WorkerContext,
        status_code: StatusCode,
        lightning_nodes: &[Value],
    ) {
        Mock::given(method("GET"))
            .and(path("/api/v1/lightning/nodes/rankings/connectivity"))
            .respond_with(ResponseTemplate::new(status_code.as_u16()).set_body_json(lightning_nodes))
            .mount(ctx.mock_server().await)
            .await;
    }
}
