use crate::error::CustomError;
use chrono::{DateTime, Utc};
use derust::StatusCode;
use derust::httpx::{HttpError, HttpTags};
use sqlx::FromRow;
use sqlx::types::BigDecimal;

#[derive(Clone, FromRow)]
pub struct Node {
    pub id: i64,
    pub public_key: String,
    pub alias: String,
    pub capacity: BigDecimal,
    pub first_seen: DateTime<Utc>,
}

impl Node {
    /// Creates a new `Node` instance with the provided parameters.
    ///
    /// This constructor converts the capacity from satoshis to Bitcoin and validates
    /// the timestamp for `first_seen`.
    ///
    /// # Arguments
    ///
    /// * `bitcoin_sats` - The number of satoshis per Bitcoin (typically 100,000,000.0)
    /// * `public_key` - The node's public key identifier
    /// * `alias` - The human-readable alias for the node
    /// * `sats` - The node's capacity in satoshis
    /// * `first_seen` - Unix timestamp (seconds since epoch) when the node was first seen
    /// * `tags` - HTTP tags for error context and tracing
    ///
    /// # Returns
    ///
    /// Returns `Ok(Node)` with a new node instance having `id` set to 0, or
    /// `Err(HttpError)` if the timestamp is invalid.
    pub fn new(
        bitcoin_sats: u64,
        public_key: String,
        alias: String,
        sats: u64,
        first_seen: i64,
        tags: &HttpTags,
    ) -> Result<Self, HttpError> {
        Ok(Self {
            id: 0,
            public_key,
            alias,
            capacity: BigDecimal::from(sats) / BigDecimal::from(bitcoin_sats),
            first_seen: DateTime::from_timestamp(first_seen, 0).ok_or(HttpError::business(
                StatusCode::BAD_REQUEST,
                &format!("Invalid first_seen={first_seen}"),
                tags,
            ))?,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::node::model::Node;
    use chrono::{Timelike, Utc};
    use derust::httpx::HttpTags;
    use sqlx::types::BigDecimal;
    use std::str::FromStr;

    #[test]
    fn should_create_node_with_valid_data() {
        let bitcoin_sats = 100_000_000;
        let public_key = "test_public_key";
        let alias = "test_alias";
        let datetime = Utc::now();

        let node = Node::new(
            bitcoin_sats,
            public_key.to_string(),
            alias.to_string(),
            550_000,
            datetime.timestamp(),
            &HttpTags::default(),
        )
        .unwrap();

        assert_eq!(node.public_key, public_key);
        assert_eq!(node.alias, alias);
        assert_eq!(node.capacity, BigDecimal::from_str("0.0055").unwrap());
        assert_eq!(node.first_seen, datetime.with_nanosecond(0).unwrap());
    }
}
