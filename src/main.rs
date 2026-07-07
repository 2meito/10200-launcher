use async_compat::Compat;
use std::path::PathBuf;
use crate::auth::Launcher;
use crate::product::Product;

slint::include_modules!();

mod api;
mod product;
mod auth;
mod utils;

#[tokio::main]
async fn main() -> Result<(), slint::PlatformError> {
    env_logger::init();
    let game_path = PathBuf::from(std::env::var("STEAM_COMPAT_INSTALL_PATH").unwrap())
        .join("Client.exe")
        .into_os_string()
        .into_string().unwrap();
    let launcher = Launcher::new(Product::Mabinogi, game_path).unwrap();

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
