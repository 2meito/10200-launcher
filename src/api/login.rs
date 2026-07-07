use std::error::Error;
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug)]
pub struct LoginRequest<'a> {
    #[serde(rename = "autoLogin")]
    auto_login: bool,
    #[serde(rename = "captchaToken")]
    captcha_token: &'a str,
    #[serde(rename = "captchaVersion")]
    captcha_version: &'a str,
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

#[derive(Serialize, Debug)]
pub struct AutoLoginRequest<'a> {
    #[serde(rename = "deviceId")]
    device_id: &'a str,
}

#[derive(Deserialize, Debug)]
pub struct LoginResponse {
    #[serde(rename = "countryCode")]
    pub country_code: String,
    #[serde(rename = "globalUserNo")]
    pub global_user_no: i64,
    #[serde(rename = "hashedGlobalUserNo")]
    pub hashed_global_user_no: Option<String>,
    #[serde(rename = "isVerified")]
    pub is_verified: bool,
    #[serde(rename = "loginSessionExpiresIn")]
    pub login_session_expires_in: i64,
    #[serde(rename = "userNo")]
    pub user_no: i64,
}


pub async fn login(
    client: &Client,
    auto_login: bool,
    email_or_nexon_id: &str,
    password: &str,
    device_id: &str,
    captcha_token: &str,
) -> Result<LoginResponse, Box<dyn Error>> {
    let endpoint = "https://www.nexon.com/api/regional-auth/v1.0/no-auth/launcher/email/login";
    let body = LoginRequest {
        auto_login,
        captcha_token,
        captcha_version: "v3",
        device_id,
        email_or_nexon_id,
        password,
        local_time: chrono::Utc::now().timestamp_millis(),
        time_offset: chrono::Local::now().offset().utc_minus_local() / 60,
    };
    let response = client.post(endpoint).json(&body).send().await?;
    Ok(response.json().await?)
}
