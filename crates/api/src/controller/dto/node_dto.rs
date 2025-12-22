use bipa_core::node::model::Node;
use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Serialize)]
pub struct NodeResponseDto {
    pub public_key: String,
    pub alias: String,
    pub capacity: String,
    pub first_seen: DateTime<Utc>,
}

impl From<&Node> for NodeResponseDto {
    fn from(node: &Node) -> Self {
        Self {
            public_key: node.public_key.clone(),
            alias: node.alias.clone(),
            capacity: format!("{:.8}", node.capacity),
            first_seen: node.first_seen,
        }
    }
}
