use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct InfoResponse {
    pub root: String,
    #[serde(rename = "optimisticRoot")]
    pub optimistic_root: String,
    #[serde(rename = "deltaIndex")]
    pub delta_index: u64,
    #[serde(rename = "optimisticDeltaIndex")]
    pub optimistic_delta_index: u64,
}