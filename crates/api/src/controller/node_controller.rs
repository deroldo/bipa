use crate::controller::dto::node_dto::NodeResponseDto;
use axum::Router;
use axum::extract::State;
use axum::routing::get;
use bipa_core::node::NodeUseCase;
use bipa_core::state::AppState;
use derust::StatusCode;
use derust::httpx::{AppContext, HttpError, HttpTags, JsonResponse};

pub struct NodeController;

impl NodeController {
    pub fn routes() -> Router<AppContext<AppState>> {
        Router::new().route("/", get(find_all))
    }
}

pub async fn find_all(State(context): State<AppContext<AppState>>) -> Result<JsonResponse<Vec<NodeResponseDto>>, HttpError> {
    let tags = HttpTags::default();

    let nodes = NodeUseCase::find_nodes(&context, &tags)
        .await?
        .iter()
        .map(|node| NodeResponseDto::from(node))
        .collect::<Vec<_>>();

    Ok(JsonResponse::new(StatusCode::OK, nodes, tags))
}

#[cfg(test)]
mod tests {
    use crate::test_support::Fixture;
    use crate::test_support::assertions::Assertions;
    use crate::test_support::http_setup::HttpContext;
    use bipa_core::node::NodeUseCase;
    use derust::httpx::HttpTags;
    use std::error::Error;
    use test_context::test_context;

    #[test_context(HttpContext)]
    #[tokio::test]
    async fn should_find_nodes(ctx: &mut HttpContext) -> Result<(), Box<dyn Error>> {
        // given
        let node_1 = Fixture::create_node();
        let node_2 = Fixture::create_node();

        NodeUseCase::insert_nodes(&ctx.context, &vec![node_1.clone(), node_2.clone()], &HttpTags::default()).await?;

        // when
        let response = ctx.client.get(&format!("{}/nodes", ctx.base_url)).send().await?;

        // then
        assert_eq!(
            response.status().as_u16(),
            200,
            "Failed to find nodes: {}",
            response.text().await.unwrap_or("<Payload not found>".to_string())
        );

        let json = response.text().await.expect("Failed to read response body");

        Assertions::json_path_any_assert_eq(&json, "$[*].public_key", &node_1.public_key);
        Assertions::json_path_any_assert_eq(&json, "$[*].alias", &node_1.alias);
        Assertions::json_path_any_assert_eq(&json, "$[*].capacity", &node_1.capacity.to_string());
        Assertions::json_path_any_assert_eq_date(&json, "$[*].first_seen", &node_1.first_seen);

        Assertions::json_path_any_assert_eq(&json, "$[*].public_key", &node_2.public_key);
        Assertions::json_path_any_assert_eq(&json, "$[*].alias", &node_2.alias);
        Assertions::json_path_any_assert_eq(&json, "$[*].capacity", &node_2.capacity.to_string());
        Assertions::json_path_any_assert_eq_date(&json, "$[*].first_seen", &node_2.first_seen);

        Ok(())
    }
}
