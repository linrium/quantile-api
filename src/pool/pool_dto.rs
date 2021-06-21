use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct InsertDataDto {
    pub pool_id: i32,
    pub pool_values: Vec<i32>,
}

#[derive(Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub struct QueryDataDto {
    pub pool_id: i32,
    pub percentile: f32,
}
