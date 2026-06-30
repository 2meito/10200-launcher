use crate::api::headers::default_headers;
use crate::api::response::handle_response;
use anyhow::Result;
use log::debug;
use reqwest::Client;
use reqwest::header::{ACCEPT, CONTENT_TYPE, HeaderMap, HeaderValue, USER_AGENT};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug)]
pub struct NexonPassportRequest <'a> {
    #[serde(rename = "productId")]
    product_id: &'a str,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NexonPassportResponse {
    #[serde(rename = "need_update_session")]
    need_update_session: bool,
    #[serde(rename = "passport")]
    passport: String,
}

pub async fn get_passport(client: &Client, product_id: &str) -> Result<NexonPassportResponse> {
    let endpoint = "https://www.nexon.com/api/passport/v2/passport";
    let headers = default_headers();
    let body = NexonPassportRequest { product_id };
    let request = client.post(endpoint).headers(headers).json(&body).build()?;
    debug!("\n{:#?}\n{:#?}", request, body);

    let response = client.execute(request).await?;
    handle_response(response).await
}
