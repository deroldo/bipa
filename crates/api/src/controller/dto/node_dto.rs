use bipa_core::node::model::Node;
use chrono::{DateTime, Utc};
use rust_decimal::prelude::ToPrimitive;
use serde::Serialize;

#[derive(Serialize)]
pub struct NodeResponseDto {
    pub public_key: String,
    pub alias: String,
    pub capacity: f64,
    pub first_seen: DateTime<Utc>,
}

impl From<&Node> for NodeResponseDto {
    fn from(node: &Node) -> Self {
        Self {
            public_key: node.public_key.clone(),
            alias: node.alias.clone(),
            capacity: node.capacity.to_f64().unwrap_or(0.0),
            first_seen: node.first_seen,
        }
    }
}
