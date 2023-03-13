use libzeropool::fawkes_crypto::{ff_uint::Num, backend::bellman_groth16::prover};
use serde::{Serialize, Deserialize};

use crate::{Engine, Fr};


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

#[derive(Serialize, Deserialize)]
pub struct Proof {
    pub inputs: Vec<Num<Fr>>,
    pub proof: prover::Proof<Engine>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionRequest {
    pub uuid: Option<String>,
    pub proof: Proof,
    pub memo: String,
    pub tx_type: String,
    pub deposit_signature: Option<String>,
}

#[derive(Serialize,Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TransactionResponse {
    pub job_id: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JobResponse {
    pub state: String,
    pub tx_hash: Option<String>,
    pub failed_reason: Option<String>,
    pub created_on: u128,
    pub finished_on: Option<u128>,
}