use crate::launcher::Launcher;
use async_compat::Compat;
use log::LevelFilter;
use reqwest::Client;
use std::env;
use std::path::PathBuf;

slint::include_modules!();

mod api;
mod launcher;
mod util;

#[tokio::main]
async fn main() -> Result<(), slint::PlatformError> {
    env_logger::builder()
        .filter_level(LevelFilter::Debug)
        .init();

    let client: Client = Client::builder().cookie_store(true).build().unwrap();
    let path = PathBuf::from(env::var("STEAM_COMPAT_INSTALL_PATH").unwrap())
        .join("Client.exe")
        .into_os_string()
        .into_string()
        .unwrap();
    let launcher: Launcher = Launcher::new(client.clone(), path);

    let ui = AppWindow::new()?;
    let ui_handle = ui.as_weak();
    ui.on_login_requested(move |email_or_nexon_id, password| {
        let ui = ui_handle.unwrap();
        let launcher = launcher.clone();
        ui.set_login_state(LoginState::LoggingIn);
        slint::spawn_local(Compat::new(async move {
            let child_process = launcher
                .launch(&email_or_nexon_id.clone(), &password.clone())
                .await;
            match child_process {
                Ok(mut value) => {
                    ui.hide().unwrap();
                    value.wait().await.unwrap();
                    slint::quit_event_loop().unwrap()
                },
                Err(_) => {}
            }
        }))
        .unwrap();
    });
    ui.run()
}
