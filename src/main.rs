use clap::Parser;
use log::LevelFilter;
use reqwest::Client;
use reqwest::cookie::Jar;
use std::error::Error;
use std::sync::Arc;
use zeroize::Zeroize;

mod api;
mod util;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    email_or_nexon_id: String,
    #[arg(short, long)]
    password: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    env_logger::builder()
        .filter_level(LevelFilter::Debug)
        .init();

    let cookie_store = Arc::new(Jar::default());
    let client: Client = Client::builder()
        .cookie_provider(Arc::clone(&cookie_store))
        .build()?;

    let auto_login: bool = false;
    let email_or_nexon_id = args.email_or_nexon_id;
    let mut password = args.password;
    let device_id: String = util::get_device_id();
    let captcha_token: String = util::generate_captcha_token();
    api::get_access_token(
        &client,
        auto_login,
        &email_or_nexon_id,
        &password,
        &device_id,
        &captcha_token,
    )
    .await?;
    password.zeroize();

    let product_id = "10200"; // Mabinogi
    api::check_playable(&client, &product_id).await?;
    api::get_passport(&client, &product_id).await?;

    Ok(())
}
