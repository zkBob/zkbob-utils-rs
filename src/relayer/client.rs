use reqwest::{Client, Response};
use serde::{de::DeserializeOwned, Serialize};

use super::{
    error::RelayerError,
    types::{FeeResponse, InfoResponse, JobResponse, TransactionRequest, TransactionResponse},
};

pub const LIB_VERSION: &str = "2.0.2";

pub struct RelayerClient {
    url: String,
    client: Client,
}

impl RelayerClient {
    pub fn new(url: &str) -> Result<RelayerClient, RelayerError> {
        Ok(RelayerClient {
            url: url.to_string(),
            client: Client::new()
        })
    }

    pub async fn info(&self) -> Result<InfoResponse, RelayerError> {
        self.get("info").await
    }

    pub async fn transactions(&self, offset: u64, limit: u64) -> Result<Vec<String>, RelayerError> {
        self.get(&format!("transactions/v2?limit={limit}&offset={offset}"))
            .await
    }

    pub async fn send_transactions(
        &self,
        request: Vec<TransactionRequest>,
    ) -> Result<TransactionResponse, RelayerError> {
        self.post("sendTransactions", request).await
    }

    pub async fn job(&self, id: &str) -> Result<JobResponse, RelayerError> {
        self.get(&format!("job/{}", id)).await
    }

    pub async fn fee(&self) -> Result<u64, RelayerError> {
        let fee: FeeResponse = self.get("fee").await?;
        fee.fee
            .parse::<u64>()
            .map_err(|err| RelayerError::UnknownError(format!("failed to parse fee: {}", err)))
    }

    async fn get<T: DeserializeOwned>(&self, query: &str) -> Result<T, RelayerError> {
        let response = self.client
            .get(format!("{}/{}", self.url, query))
            .header("zkbob-support-id", "zkbob-utils-rs")
            .header("zkbob-libjs-version", LIB_VERSION)
            .send()
            .await?;

        self.handle_response(response).await
    }

    async fn post<Request: Serialize, Response: DeserializeOwned>(
        &self,
        query: &str,
        request: Request,
    ) -> Result<Response, RelayerError> {
        let response = self.client
            .post(format!("{}/{}", self.url, query))
            .json(&request)
            .header("zkbob-support-id", "zkbob-utils-rs")
            .header("zkbob-libjs-version", LIB_VERSION)
            .send()
            .await?;

        self.handle_response(response).await
    }

    async fn handle_response<T: DeserializeOwned>(
        &self,
        response: Response,
    ) -> Result<T, RelayerError> {
        match response.status() {
            reqwest::StatusCode::OK => Ok(response.json::<T>().await?),
            code => match response.text().await {
                Ok(response) => Err(RelayerError::service_error(code, &response)),
                Err(err) => Err(RelayerError::UnknownError(err.to_string())),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::RelayerClient;

    #[tokio::test]
    #[ignore = "the test requires working relayer"]
    async fn info_request() {
        let client = RelayerClient::new("https://relayer.thgkjlr.website").unwrap();
        println!("{:?}", client.info().await);
    }

    #[tokio::test]
    #[ignore = "the test requires working relayer"]
    async fn transactions_request() {
        let client = RelayerClient::new("https://relayer.thgkjlr.website").unwrap();
        println!("{:?}", client.transactions(0, 10).await);
    }
}
