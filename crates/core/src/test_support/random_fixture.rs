use crate::test_support::Fixture;
use rand::Rng;
use rand::distr::Alphabetic;

impl Fixture {
    pub fn random_string(len: usize) -> String {
        rand::rng().sample_iter(&Alphabetic).take(len).map(char::from).collect()
    }

    pub fn random_u64_in_range(
        min: u64,
        max: u64,
    ) -> u64 {
        rand::rng().random_range(min..=max)
    }
}
