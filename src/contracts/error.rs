use strum::Display;

#[derive(Debug, Display)]
pub enum PoolError {
    BadAbi(std::io::Error),
    GeneralError(String),
    ContractException(web3::contract::Error),
    RpcNodeUnavailable,
    Web3Error(web3::Error),
    RequestTimeout(tokio::time::error::Elapsed),
    RpcNodeInconsistency(String),
}

impl From<std::io::Error> for PoolError {
    fn from(e: std::io::Error) -> Self {
        PoolError::BadAbi(e)
    }
}

impl From<web3::contract::Error> for PoolError {
    fn from(e: web3::contract::Error) -> Self {
        PoolError::ContractException(e)
    }
}

impl From<web3::Error> for PoolError {
    fn from(e: web3::Error) -> Self {
        PoolError::Web3Error(e)
    }
}

impl From<tokio::time::error::Elapsed> for PoolError {
    fn from(e: tokio::time::error::Elapsed) -> Self {
        PoolError::RequestTimeout(e)
    }
}
