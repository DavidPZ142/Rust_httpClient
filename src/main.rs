use std::collections::HashMap;
use crate::infrastructure::http_client::HttpClient;
use crate::services::http_service::HttpService;
use anyhow::Result;
use log::{info, LevelFilter};
use crate::infrastructure::http_client_2::HttpClient2;

mod student;
mod domain;
mod errors;
mod infrastructure;
mod services;
#[tokio::main]
async fn main() -> Result<()> {
    env_logger::Builder::new()
        .filter_level(LevelFilter::Info)
        .init();
    let client = HttpClient2::new();
    let mut obj = HashMap::new();
    obj.insert("url ", "xd");
    obj.insert("title", "xd");
    obj.insert("description", "xd");

    let response = client.post("https://webhook.site/e148ef42-a63e-41b2-8bf1-ce32d75b280c", &obj);
    info!("{:?}", response);
    Ok(())
}