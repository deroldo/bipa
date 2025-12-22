use crate::node::NodeUseCase;
use crate::node::model::Node;
use crate::state::AppState;
use derust::httpx::{AppContext, HttpError, HttpTags};

impl NodeUseCase {
    /// Inserts or updates multiple nodes in the database.
    ///
    /// This method performs an upsert operation on the provided nodes. If a node with the same
    /// public key already exists, it will be updated; otherwise, a new node will be created.
    /// The operation is performed in batches to handle large numbers of nodes efficiently.
    ///
    /// # Arguments
    ///
    /// * `context` - Application context containing database connection and state
    /// * `nodes` - Slice of nodes to insert or update
    /// * `tags` - HTTP tags for error context and tracing
    ///
    /// # Returns
    ///
    /// Returns `Ok(Vec<Node>)` containing all inserted or updated nodes with their database IDs,
    /// or `Err(HttpError)` if the database operation fails.
    pub async fn insert_nodes(
        context: &AppContext<AppState>,
        nodes: &[Node],
        tags: &HttpTags,
    ) -> Result<Vec<Node>, HttpError> {
        Node::upsert(context, None, nodes, tags).await
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
    async fn should_create_or_update_nodes(ctx: &mut UseCaseContext) -> Result<(), Box<dyn Error>> {
        // given
        let mut nodes = Vec::new();

        // new nodes
        for _ in 0..Fixture::random_u64_in_range(1, 100) {
            nodes.push(Fixture::create_node());
        }

        // nodes to update
        for _ in 0..Fixture::random_u64_in_range(1, 100) {
            let mut saved_node = TestRepository::save(&ctx.context, &Fixture::create_node()).await;
            saved_node.alias = Fixture::random_string(6);
            nodes.push(saved_node);
        }

        // when
        let inserted_nodes = NodeUseCase::insert_nodes(&ctx.context, &nodes, &HttpTags::default()).await?;

        // then
        assert_eq!(inserted_nodes.len(), nodes.len());

        let stored_nodes = TestRepository::find_all_nodes(&ctx.context).await;
        for node in nodes {
            let stored_node = stored_nodes.iter().find(|n| n.public_key == node.public_key).unwrap();
            assert_eq!(stored_node.public_key, node.public_key);
            assert_eq!(stored_node.alias, node.alias);
            assert_eq!(stored_node.capacity, node.capacity);
            assert_eq!(stored_node.first_seen, node.first_seen);
        }

        Ok(())
    }
}
