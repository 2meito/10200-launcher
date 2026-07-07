use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Serialize, Debug)]
pub struct PassportRequest<'a> {
    #[serde(rename = "productId")]
    product_id: &'a str,
}

#[derive(Deserialize, Debug)]
pub struct PassportResponse {
    #[serde(rename = "need_update_session")]
    pub need_update_session: bool,
    #[serde(rename = "passport")]
    pub passport: String,
}

pub async fn get_passport<'a>(
    client: &Client,
    product_id: i32,
) -> Result<PassportResponse, Box<dyn Error>> {
    let endpoint = "https://www.nexon.com/api/passport/v2/passport";
    let body = PassportRequest { product_id : &product_id.to_string() };
    let response = client.post(endpoint).json(&body).send().await?;
    Ok(response.json().await?)
}
