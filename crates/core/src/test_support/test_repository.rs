use crate::node::model::Node;
use crate::state::AppState;
use derust::httpx::AppContext;
use sqlx::query_as;

pub struct TestRepository;

impl TestRepository {
    pub async fn save(
        context: &AppContext<AppState>,
        node: &Node,
    ) -> Node {
        query_as("insert into node (public_key, alias, capacity, first_seen) values ($1, $2, $3, $4) returning *")
            .bind(node.public_key.clone())
            .bind(node.alias.clone())
            .bind(node.capacity.clone())
            .bind(node.first_seen.clone())
            .fetch_one(&context.database().read_write)
            .await
            .unwrap()
    }

    pub async fn find_all_nodes(context: &AppContext<AppState>) -> Vec<Node> {
        query_as("select * from node").fetch_all(&context.database().read_write).await.unwrap()
    }
}
