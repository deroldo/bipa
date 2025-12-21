use crate::node::model::Node;
use crate::test_support::usecase_setup::UseCaseContext;
use sqlx::query_as;

pub struct TestRepository;

impl TestRepository {
    pub async fn save(ctx: &mut UseCaseContext, node: &Node) -> Node {
        query_as("insert into node (public_key, alias, capacity, first_seen) values ($1, $2, $3, $4) returning *")
            .bind(node.public_key.clone())
            .bind(node.alias.clone())
            .bind(node.capacity.clone())
            .bind(node.first_seen.clone())
            .fetch_one(&ctx.context.database().read_write).await.unwrap()
    }
    
    pub async fn find_all_nodes(ctx: &mut UseCaseContext) -> Vec<Node> {
        query_as("select * from node").fetch_all(&ctx.context.database().read_write).await.unwrap()
    }
}
