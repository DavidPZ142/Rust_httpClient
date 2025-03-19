use crate::infrastructure::http_client::HttpClient;
use crate::services::http_service::HttpService;
use anyhow::Result;
use log::LevelFilter;

mod student;
mod domain;
mod errors;
mod infrastructure;
mod services;
#[tokio::main]
async fn main() -> Result<()> {
    env_logger::Builder::new()
        .filter_level(LevelFilter::Info)
        .init();    let client = HttpClient::new("https://catfact.ninja");
    let service = HttpService::new(client);
    let response = service.get("fact").await?;
    println!("{}", response);
    Ok(())
}