use anyhow::{Result, anyhow};
use log::debug;
use reqwest::Response;
use serde::Serialize;
use serde::de::DeserializeOwned;
use std::fmt::Debug;

pub async fn handle_response<T: DeserializeOwned + Debug + Serialize>(
    response: Response,
) -> Result<T> {
    let status = response.status();
    let headers = response.headers().clone();
    if status.is_success() {
        let json: T = response.json().await?;
        debug!("\nstatus: {:#?}\nheaders: {:#?}\nbody: {:#?}", status, headers, json);
        Ok(json)
    } else {
        Err(anyhow!("\nstatus: {:#?}\nheaders: {:#?}\nbody: {:#?}", status, headers, response))
    }
}
