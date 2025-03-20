use std::time::Duration;
use log::{error, info, warn};
use serde::Serialize;
use serde_json::{json, Value};
use ureq::{Agent, Body};
use ureq::config::Config;
use ureq::http::Response;
use crate::errors::http_error::HttpError;

#[derive(Clone)]
pub struct HttpClient2 {
    agent: Agent
}

impl HttpClient2 {
    pub fn new() -> Self {
        let mut config = Agent::config_builder()
            .timeout_global(Some(Duration::from_secs(5)))
            .build();

        let agent: Agent = config.into();
        Self { agent }
    }

    pub fn  get(&self, url : &str) -> Result<Value, HttpError>  {
        let mut response = self.agent.get(url).call().map_err(|e|{
            error!("error :{}", e);
            HttpError::RequestFailed(e.to_string());
        }).unwrap();
        let body = response.body_mut().read_to_string().unwrap();
        Self::deserialize_response(body)
    }

    pub fn post<T: Serialize>(&self, url : &str, data : &T) -> Result<Value, HttpError> {
        let mut response = self.agent.post(url)
            .header("Content-Type", "application/json")
            .send_json(data)
            .map_err(|e|{
                error!("error :{}", e);
                HttpError::RequestFailed(e.to_string());
            }).unwrap();
        let body = response.body_mut().read_to_string().unwrap();
        Self::deserialize_response(body)
    }

    pub fn deserialize_response(text : String) -> Result<Value, HttpError> {
        match serde_json::from_str::<Value>(&text) {
            Ok(r) => Ok(r),
            Err(e) => {
                warn!("Error deserializing to Json {}. ", e);
                Ok(json!({
                    "raw_request" : text
                }))
            }
        }
    }

}