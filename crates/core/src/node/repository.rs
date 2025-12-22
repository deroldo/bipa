use crate::node::model::Node;
use crate::state::AppState;
use derust::databasex::{PostgresTransaction, Repository};
use derust::httpx::{AppContext, HttpError, HttpTags};
use sqlx::{QueryBuilder, query_as};

impl Node {
    /// Upserts multiple nodes into the database in batches.
    ///
    /// This method inserts new nodes or updates existing ones based on their `public_key`.
    /// To handle large datasets efficiently, nodes are processed in chunks of 1000.
    ///
    /// # Arguments
    ///
    /// * `context` - Application context containing database connection and configuration
    /// * `transaction` - Optional database transaction for atomic operations
    /// * `nodes` - Slice of nodes to be upserted
    /// * `tags` - HTTP tags for error context and tracing
    ///
    /// # Returns
    ///
    /// Returns `Ok(Vec<Node>)` containing all upserted nodes with their database-assigned IDs,
    /// or `Err(HttpError)` if the database operation fails.
    pub(super) async fn upsert(
        context: &AppContext<AppState>,
        mut transaction: Option<&mut PostgresTransaction<'_, AppState>>,
        nodes: &[Node],
        tags: &HttpTags,
    ) -> Result<Vec<Node>, HttpError> {
        if nodes.is_empty() {
            return Ok(vec![]);
        }

        let mut results = Vec::new();

        for chunk in nodes.chunks(1000) {
            let inserted_nodes = upsert_chunks(context, transaction.as_deref_mut(), chunk, tags).await?;
            results.extend(inserted_nodes);
        }

        Ok(results)
    }

    /// Retrieves all nodes from the database.
    ///
    /// This method fetches all node records without any filtering or pagination.
    /// It can be executed within an existing transaction or as a standalone query.
    ///
    /// # Arguments
    ///
    /// * `context` - Application context containing database connection and configuration
    /// * `transaction` - Optional database transaction for atomic operations
    /// * `tags` - HTTP tags for error context and tracing
    ///
    /// # Returns
    ///
    /// Returns `Ok(Vec<Node>)` containing all nodes in the database,
    /// or `Err(HttpError)` if the database operation fails.
    pub(super) async fn find_all(
        context: &AppContext<AppState>,
        mut transaction: Option<&mut PostgresTransaction<'_, AppState>>,
        tags: &HttpTags,
    ) -> Result<Vec<Node>, HttpError> {
        let query = query_as("select * from node");
        transaction.fetch_all(context, "Node.find_all", query, tags).await
    }
}

async fn upsert_chunks(
    context: &AppContext<AppState>,
    mut transaction: Option<&mut PostgresTransaction<'_, AppState>>,
    nodes: &[Node],
    tags: &HttpTags,
) -> Result<Vec<Node>, HttpError> {
    let mut builder = QueryBuilder::new("INSERT INTO node (public_key, alias, capacity, first_seen) ");

    builder.push_values(nodes, |mut b, node| {
        b.push_bind(&node.public_key)
            .push_bind(&node.alias)
            .push_bind(&node.capacity)
            .push_bind(node.first_seen);
    });

    let query = builder
        .push(
            " ON CONFLICT (public_key) DO UPDATE SET
            alias = EXCLUDED.alias,
            capacity = EXCLUDED.capacity,
            first_seen = EXCLUDED.first_seen
            RETURNING *",
        )
        .build_query_as();

    transaction.fetch_all(context, "Node.upsert", query, tags).await
}
