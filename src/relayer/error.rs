use reqwest::StatusCode;


#[derive(Debug)]
pub enum RelayerError {
    NetworkError(String),
    ServiceError(StatusCode, String),
    UnknownError(String),
}

impl RelayerError {
    pub fn service_error(code: StatusCode, response: &str) -> RelayerError {
        RelayerError::ServiceError(code, response.to_string())
    }
}

impl<T: std::error::Error> From<T> for RelayerError {
    fn from(e: T) -> Self {
        RelayerError::NetworkError(e.to_string())
    }
}