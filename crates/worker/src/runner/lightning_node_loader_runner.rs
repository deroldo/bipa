use crate::dto::lightning_node_dto::LightningNodeResponseDto;
use bipa_core::node::NodeUseCase;
use bipa_core::node::model::Node;
use bipa_core::state::AppState;
use derust::httpx::{AppContext, HttpTags};
use std::error::Error;
use tracing::info;
use uuid::Uuid;

pub struct LightningNodeLoaderRunner;

impl LightningNodeLoaderRunner {
    pub async fn run(context: &AppContext<AppState>) -> Result<(), Box<dyn Error>> {
        info!("Bipa-worker running");
        let tags = HttpTags::from([("worker", "true"), ("traceId", &Uuid::now_v7().to_string())]);

        let lightning_nodes: Vec<LightningNodeResponseDto> = context
            .state()
            .gateway
            .lightning_nodes
            .get(context, "/api/v1/lightning/nodes/rankings/connectivity", None, None, &tags)
            .await?
            .body
            .unwrap_or(vec![]);

        info!("Bipa-worker lightning nodes: {}", lightning_nodes.len());

        let mut nodes = Vec::new();
        for ln in lightning_nodes {
            nodes.push(Node::new(
                context.state().app_config.bitcoin_sats,
                ln.public_key.to_string(),
                ln.alias.to_string(),
                ln.capacity,
                ln.first_seen,
                &tags,
            )?)
        }

        info!("Bipa-worker nodes: {}", nodes.len());

        let stored_nodes = NodeUseCase::insert_nodes(context, &nodes, &tags).await?;

        info!("Bipa-worker stored nodes: {}", stored_nodes.len());

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::runner::lightning_node_loader_runner::LightningNodeLoaderRunner;
    use crate::test_support::http_mock::HttpMock;
    use crate::test_support::worker_setup::WorkerContext;
    use bipa_core::node::NodeUseCase;
    use derust::StatusCode;
    use derust::httpx::HttpTags;
    use serde_json::json;
    use std::error::Error;
    use test_context::test_context;

    #[test_context(WorkerContext)]
    #[tokio::test]
    async fn should_run_worker_and_insert_nodes(ctx: &mut WorkerContext) -> Result<(), Box<dyn Error>> {
        // given
        let ln_node_1 = json!({
            "publicKey": "pubkey1",
            "alias": "alias1",
            "capacity": 1000000,
            "firstSeen": 1600000000
        });

        let ln_node_2 = json!({
            "publicKey": "pubkey2",
            "alias": "alias2",
            "capacity": 2000000,
            "firstSeen": 1610000000
        });

        HttpMock::mock_lightning_nodes(ctx, StatusCode::OK, &[ln_node_1, ln_node_2]).await;

        // when
        LightningNodeLoaderRunner::run(&ctx.context).await?;

        // then
        let nodes = NodeUseCase::find_nodes(&ctx.context, &HttpTags::default()).await?;

        let node1 = nodes.iter().find(|n| n.public_key == "pubkey1").unwrap();
        assert_eq!(node1.alias, "alias1");
        assert_eq!(node1.capacity.to_string(), "0.0100");

        let node2 = nodes.iter().find(|n| n.public_key == "pubkey2").unwrap();
        assert_eq!(node2.alias, "alias2");
        assert_eq!(node2.capacity.to_string(), "0.0200");

        Ok(())
    }

    #[test_context(WorkerContext)]
    #[tokio::test]
    async fn should_fail_when_lightning_api_is_not_reachable(ctx: &mut WorkerContext) -> Result<(), Box<dyn Error>> {
        // given
        HttpMock::mock_lightning_nodes(ctx, StatusCode::INTERNAL_SERVER_ERROR, &[]).await;

        // when
        let result = LightningNodeLoaderRunner::run(&ctx.context).await;

        // then
        assert!(result.is_err());

        Ok(())
    }
}
