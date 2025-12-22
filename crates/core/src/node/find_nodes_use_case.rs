use crate::node::NodeUseCase;
use crate::node::model::Node;
use crate::state::AppState;
use derust::httpx::{AppContext, HttpError, HttpTags};

impl NodeUseCase {
    /// Retrieves all nodes from the system.
    ///
    /// This method queries the database to fetch all available nodes without any filtering.
    ///
    /// # Arguments
    ///
    /// * `context` - The application context containing the database connection pool and configuration
    /// * `tags` - HTTP tags for error context and distributed tracing
    ///
    /// # Returns
    ///
    /// Returns `Ok(Vec<Node>)` containing all nodes in the system, or `Err(HttpError)` if
    /// the database query fails or another error occurs during retrieval.
    pub async fn find_nodes(
        context: &AppContext<AppState>,
        tags: &HttpTags,
    ) -> Result<Vec<Node>, HttpError> {
        Node::find_all(context, None, tags).await
    }
}

#[cfg(test)]
mod tests {
    use crate::node::NodeUseCase;
    use crate::test_support::Fixture;
    use crate::test_support::test_repository::TestRepository;
    use crate::test_support::usecase_setup::UseCaseContext;
    use derust::httpx::HttpTags;
    use std::error::Error;
    use test_context::test_context;

    #[test_context(UseCaseContext)]
    #[tokio::test]
    async fn should_find_nodes(ctx: &mut UseCaseContext) -> Result<(), Box<dyn Error>> {
        // given
        let saved_node = TestRepository::save(&ctx.context, &Fixture::create_node()).await;

        // when
        let found_nodes = NodeUseCase::find_nodes(&ctx.context, &HttpTags::default()).await?;

        // then
        let found_node = found_nodes.iter().find(|n| n.public_key == saved_node.public_key).unwrap();

        assert_eq!(found_node.public_key, saved_node.public_key);
        assert_eq!(found_node.alias, saved_node.alias);
        assert_eq!(found_node.capacity, saved_node.capacity);
        assert_eq!(found_node.first_seen, saved_node.first_seen);

        Ok(())
    }
}
