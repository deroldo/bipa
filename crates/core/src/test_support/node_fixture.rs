use crate::node::model::Node;
use crate::test_support::Fixture;
use derust::httpx::HttpTags;
use sqlx::types::chrono::Utc;
use std::time::Duration;

impl Fixture {
    pub fn create_node() -> Node {
        let date_time = Utc::now() - Duration::from_secs(Fixture::random_u64_in_range(0, 432000)); // max 5 days in seconds

        Node::new(
            100_000_000,
            Fixture::random_string(32),
            Fixture::random_string(6),
            Fixture::random_u64_in_range(1000, 110_000_000),
            date_time.timestamp(),
            &HttpTags::default(),
        )
        .unwrap()
    }
}
