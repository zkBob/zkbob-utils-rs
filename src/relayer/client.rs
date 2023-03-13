use reqwest::{Client, Response};
use serde::{de::DeserializeOwned, Serialize};

use super::{error::RelayerError, types::{InfoResponse, TransactionRequest, TransactionResponse, JobResponse}};

pub const LIB_VERSION: &'static str = "2.0.2";

pub struct RelayerClient {
    url: String,
}

impl RelayerClient {
    pub fn new(url: &str) -> Result<RelayerClient, RelayerError> {
        Ok(RelayerClient {
            url: url.to_string(),
        })
    }

    pub async fn info(&self) -> Result<InfoResponse, RelayerError> {
        self.get("info").await
    }

    pub async fn transactions(&self, offset: u64, limit: u64) -> Result<Vec<String>, RelayerError> {
        self.get(&format!("transactions/v2?limit={limit}&offset={offset}"))
            .await
    }

    pub async fn send_transactions(&self, request: Vec<TransactionRequest>) -> Result<TransactionResponse, RelayerError> {
        self.post("sendTransactions", request).await
    }

    pub async fn job(&self, id: &str) -> Result<JobResponse, RelayerError> {
        self.get(&format!("job/{}", id)).await
    }

    async fn get<T: DeserializeOwned>(&self, query: &str) -> Result<T, RelayerError> {
        let response = Client::new()
            .get(format!("{}/{}", self.url, query))
            .header("zkbob-support-id", "zkbob-utils-rs")
            .header("zkbob-libjs-version", LIB_VERSION)
            .send()
            .await?;

        self.handle_response(response).await
    }

    async fn post<Request: Serialize, Response: DeserializeOwned>(&self, query: &str, request: Request) -> Result<Response, RelayerError> {
        let response = Client::new()
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
