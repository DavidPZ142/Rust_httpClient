use crate::errors::http_error::HttpError;
use crate::infrastructure::http_client::HttpClient;
use anyhow::Result;
use serde::Serialize;
use serde_json::Value;

#[derive(Clone)]
pub struct HttpService {
    client: HttpClient,
}
impl HttpService {
    pub fn new(client: HttpClient) -> Self {
        Self { client }
    }

    pub async fn get(&self, endpoint: &str) -> Result<Value, HttpError> {
        self.client.get(endpoint).await
    }

    pub async fn post<T: Serialize>(&self, endpoint: &str, data: &T) -> Result<Value, HttpError> {
        self.client.post(endpoint, data).await
    }
}