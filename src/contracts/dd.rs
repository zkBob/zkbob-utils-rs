use std::time::Duration;

use ethabi::ethereum_types::{H160};
use tokio::time::timeout;
use web3::{transports::Http, contract::{Contract, Options}, Web3};

use super::error::PoolError;

pub struct DdContract {
    pub contract: Contract<Http>,
    timeout: Duration,
}

impl DdContract {
    pub fn new(address: H160, web3: Web3<Http>, timeout: Duration) -> Result<Self, PoolError> {
        let contract = Contract::from_json(
            web3.eth(),
            address,
            include_bytes!("dd-abi.json"),
        )
        .expect("failed to read contract");

        Ok(Self {
            contract,
            timeout,
        })
    }

    pub async fn fee(&self) -> Result<u64, PoolError> {
        let result = self.contract.query("directDepositFee", (), None, Options::default(), None);
        Ok(timeout(self.timeout, result).await??)
    }
}