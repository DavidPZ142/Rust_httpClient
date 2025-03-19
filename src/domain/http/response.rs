use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Response<T> {
    pub status: u16,
    pub headers: HashMap<String, String>,
    pub body: T,
}

impl Response<serde_json::Value> {
    pub fn new (status : u16, headers: HashMap<String,String>, body: serde_json::Value) -> Self{
        Self{status,headers,body}
    }
}