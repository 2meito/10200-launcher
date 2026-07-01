use async_compat::Compat;
use log::LevelFilter;
use reqwest::Client;
use std::error::Error;

slint::include_modules!();

mod api;
mod util;

#[tokio::main]
async fn main() -> Result<(), slint::PlatformError> {
    env_logger::builder()
        .filter_level(LevelFilter::Debug)
        .init();

    let client: Client = Client::builder().cookie_store(true).build().unwrap();
    let ui = AppWindow::new()?;
    let ui_handle = ui.as_weak();
    ui.on_login_requested(move |email_or_nexon_id, password| {
        let ui = ui_handle.unwrap();
        let client = client.clone();
        ui.set_login_state(LoginState::LoggingIn);
        slint::spawn_local(Compat::new(async move {
            let result = login(&client, &email_or_nexon_id, &password).await;
            ui.set_login_state(LoginState::LoggedOut);
            result
        }))
        .unwrap();
    });
    ui.run()
}

async fn login(
    client: &Client,
    email_or_nexon_id: &str,
    password: &str,
) -> Result<(), Box<dyn Error>> {
    api::get_access_token(
        &client,
        false,
        &email_or_nexon_id,
        &password,
        &util::get_device_id(),
        &util::generate_captcha_token(),
    )
    .await?;
    let product_id = "10200"; // Mabinogi
    api::check_playable(&client, &product_id).await?;
    api::get_passport(&client, &product_id).await?;

    Ok(())
}
