use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LightningNodeResponseDto {
    pub public_key: String,
    pub alias: String,
    pub capacity: u64,
    pub first_seen: i64,
}