use crate::api::NexonPassportResponse;
use crate::{api, util};
use anyhow::Result;
use log::debug;
use reqwest::Client;
use std::env;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::process::{Child, Command};

#[derive(Copy, Clone)]
pub enum Product {
    Mabinogi = 10200,
}

#[derive(Clone)]
pub struct Launcher {
    config: Arc<Config>,
}

struct Config {
    pub client: Client,
    pub path: String,
    pub auto_login: bool,
    device_id: String,
    captcha_token: String,
    pub product: Product,
}

impl Config {
    pub fn new(client: Client, path: String) -> Self {
        Self {
            client,
            path,
            auto_login: false,
            device_id: util::get_device_id(),
            captcha_token: util::generate_captcha_token(),
            product: Product::Mabinogi,
        }
    }
}

impl Launcher {
    pub(crate) fn new(client: Client, path: String) -> Self {
        Self {
            config: Arc::new(Config::new(client, path)),
        }
    }

    pub async fn launch(&self, email_or_nexon_id: &str, password: &str) -> std::io::Result<Child> {
        debug!("Getting credentials... {}", self.config.path);
        let passport = self
            .get_nexon_passport(email_or_nexon_id, password)
            .await
            .expect("Failed to get passport");
        debug!("Launching \"{:?}\"...", &self.config.path);
        match self.config.product {
            Product::Mabinogi => self.launch_mabinogi(passport.passport),
        }
    }

    fn launch_mabinogi(&self, passport: String) -> std::io::Result<Child> {
        let args = [
            "code:1622",
            "verstr:248",
            "ver:248",
            "locale:USA",
            "env:Regular",
            "setting:file://data/features.xml",
            "logip:35.162.171.43",
            "logport:11000",
            "chatip:54.214.176.167",
            "chatport:8002",
            &format!("/P:{}", passport),
            "-bgloader",
        ]
        .map(String::from)
        .to_vec();

        match env::var("SteamClientLaunch") {
            Ok(_) => self.launch_steam(args),
            Err(_) => self.launch_standalone(args),
        }
    }

    fn launch_standalone(&self, args: Vec<String>) -> std::io::Result<Child> {
        Command::new(&self.config.path).args(&args).spawn()
    }

    fn launch_steam(&self, args: Vec<String>) -> std::io::Result<Child> {
        let steam_compat_tool_paths = env::var("STEAM_COMPAT_TOOL_PATHS").unwrap_or_default();
        let (proton_path, steam_linux_runtime_path) =
            steam_compat_tool_paths.split_once(':').unwrap_or(("", ""));

        let args: Vec<String> = ["waitforexitandrun".to_owned(), self.config.path.clone()]
            .into_iter()
            .chain(args)
            .collect();

        Command::new(PathBuf::from(proton_path).join("proton"))
            .args(args)
            .spawn()
    }

    async fn get_nexon_passport(
        &self,
        email_or_nexon_id: &str,
        password: &str,
    ) -> Result<NexonPassportResponse> {
        api::get_access_token(
            &self.config.client,
            self.config.auto_login,
            &email_or_nexon_id,
            &password,
            &self.config.device_id,
            &self.config.captcha_token,
        )
        .await?;

        let product_id = (self.config.product as i32).to_string();
        api::check_playable(&self.config.client, &product_id).await?;
        api::get_passport(&self.config.client, &product_id).await
    }
}
