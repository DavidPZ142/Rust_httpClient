use crate::errors::http_error::HttpError;
use log::*;
use reqwest::{Client, header};
use serde::Serialize;
use serde_json::Value;
use std::time::Duration;

#[derive(Clone)]
pub struct HttpClient {
    client: Client,
    base_url: String,
}

impl HttpClient {
    pub fn new(base_url: &str) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .expect("HTTP client error");

        Self {
            client,base_url: base_url.to_string(),
        }
    }

    pub async fn get(&self, endpoint : &str) -> Result<Value, HttpError> {
        let url = format!("{}/{}", self.base_url, endpoint);
        info!("GET to {}", &url);
        let response = self.client.get(&url).send().await.map_err(|e|{
            error!("Error when getting response: {}", e);
            HttpError::RequestFailed(e.to_string())
        })?;
        let status = response.status();
        if !status.is_success() {
            error!("Error from status getting response: {}", status);
            return Err(HttpError::ResponseError(status.as_u16()));
        }
        let json = response.json::<Value>().await.map_err(|e|{
        error!("Error deserializing response: {}", e);
        HttpError::DeserializationError(e.to_string())})?;
        Ok(json)
    }

    pub async fn post<T: Serialize>(&self, endpoint: &str, data: &T) -> Result<Value, HttpError> {
        let url = format!("{}/{}", self.base_url, endpoint);
        info!("POST to {}", &url);
        let response = self.client.post(&url).header(header::CONTENT_TYPE, "application/json")
            .json(data).send().await.map_err(|e|{
            error!("Error when posting to {}: {}", &url, e);
            HttpError::RequestFailed(e.to_string())
        })?;
        let status = response.status();
        if !status.is_success() {
            error!("Error when posting to {}: {}", &url, status);
            return Err(HttpError::ResponseError(status.as_u16()));
        }
        let json = response.json::<Value>().await.map_err(|e|{
            error!("Error deserializing response: {}", e);
            HttpError::DeserializationError(e.to_string())
        })?;
        Ok(json)
    }

}
