use crate::api::headers::default_headers;
use crate::api::response::handle_response;
use anyhow::Result;
use log::debug;
use reqwest::Client;
use reqwest::header::{ACCEPT, CONTENT_TYPE, HeaderMap, HeaderValue, USER_AGENT};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug)]
pub struct NexonLoginRequest<'a> {
    #[serde(rename = "autoLogin")]
    auto_login: bool,
    #[serde(rename = "captchaToken")]
    captcha_token: &'a str,
    #[serde(rename = "captchaVersion")]
    captcha_version: &'static str,
    #[serde(rename = "deviceId")]
    device_id: &'a str,
    #[serde(rename = "id")]
    email_or_nexon_id: &'a str,
    password: &'a str,
    #[serde(rename = "localTime")]
    local_time: i64,
    #[serde(rename = "timeOffset")]
    time_offset: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NexonLoginResponse {
    #[serde(rename = "countryCode")]
    country_code: String,
    #[serde(rename = "globalUserNo")]
    global_user_no: i64,
    #[serde(rename = "hashedGlobalUserNo")]
    hashed_global_user_no: Option<String>,
    #[serde(rename = "isVerified")]
    is_verified: bool,
    #[serde(rename = "loginSessionExpiresIn")]
    login_session_expires_in: i64,
    #[serde(rename = "userNo")]
    user_no: i64,
}

pub async fn get_access_token(
    client: &Client,
    auto_login: bool,
    email_or_nexon_id: &str,
    password: &str,
    device_id: &str,
    captcha_token: &str,
) -> Result<NexonLoginResponse> {
    let endpoint = "https://www.nexon.com/api/regional-auth/v1.0/no-auth/launcher/email/login";
    let headers = default_headers();
    let body = NexonLoginRequest {
        auto_login,
        captcha_token,
        captcha_version: "v3",
        device_id,
        email_or_nexon_id,
        password,
        local_time: chrono::Utc::now().timestamp_millis(),
        time_offset: chrono::Local::now().offset().utc_minus_local() / 60,
    };
    let request = client.post(endpoint).headers(headers).json(&body).build()?;
    debug!("\n{:#?}\n{:#?}", request, body);

    let response = client.execute(request).await?;
    handle_response(response).await
}
