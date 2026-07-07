use reqwest::Client;
use reqwest::header::{ACCEPT, CONTENT_TYPE, HeaderMap, HeaderValue, USER_AGENT};
use std::path::PathBuf;
use tokio::process::{Child, Command};
use std::error::Error;
use std::sync::Arc;
use crate::api::login::login;
use crate::api::passport::{get_passport, PassportResponse};
use crate::api::playable::check_playable;
use crate::product::Product;
use crate::utils::{generate_session_id, get_device_id};

#[derive(Clone)]
pub struct Launcher {
    client: Client,
    config: Arc<Config>
}

pub struct Config {
    product: Product,
    game_path: String,
    device_id: String,
    captcha_token: String,
}

impl Launcher {
    pub fn new(product: Product, game_path: String) -> Result<Launcher, Box<dyn Error>> {
        let mut headers = HeaderMap::new();
        headers.insert(USER_AGENT, HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; WOW64) AppleWebKit/537.36 (KHTML, like Gecko) NexonLauncher/4.7.8 Chrome/108.0.5359.215 Electron/22.3.27 Safari/537.36"));
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert(ACCEPT, HeaderValue::from_static("application/json, text/plain, */*"));
        let client = Client::builder().connection_verbose(true).default_headers(headers).cookie_store(true).build()?;
        Ok(Launcher {
            client,
            config: Arc::new(Config { product, game_path, device_id: get_device_id()?, captcha_token: generate_session_id() }),
        })
    }

    pub(crate) async fn launch(&self, email_or_nexon_id: &str, password: &str) -> std::io::Result<Child> {
        let response = self.get_passport(email_or_nexon_id, password).await.unwrap();
        let args = match self.config.product {
            Product::Mabinogi => vec![
                "code:1622".to_owned(),
                "verstr:248".to_owned(),
                "ver:248".to_owned(),
                "locale:USA".to_owned(),
                "env:Regular".to_owned(),
                "setting:file://data/features.xml".to_owned(),
                "logip:35.162.171.43".to_owned(),
                "logport:11000".to_owned(),
                "chatip:54.214.176.167".to_owned(),
                "chatport:8002".to_owned(),
                format!("/P:{}", response.passport),
                "-bgloader".to_owned(),
            ],
            _ => vec![],
        };
        match std::env::var("SteamClientLaunch") {
            Ok(_) => self.launch_steam(args),
            Err(_) => self.launch_standalone(args),
        }
    }

    fn launch_steam(&self, args: Vec<String>) -> std::io::Result<Child> {
        let steam_compat_tool_paths = std::env::var("STEAM_COMPAT_TOOL_PATHS").unwrap();
        let (proton_path, _steam_linux_runtime_path) =
            steam_compat_tool_paths.split_once(':').unwrap_or(("", ""));

        let args: Vec<String> = ["waitforexitandrun".to_owned(), self.config.game_path.clone()]
            .into_iter()
            .chain(args)
            .collect();

        Command::new(PathBuf::from(proton_path).join("proton"))
            .args(args)
            .spawn()
    }

    fn launch_standalone(&self, args: Vec<String>) -> std::io::Result<Child> {
        Command::new(&self.config.game_path).args(&args).spawn()
    }

    async fn get_passport(&self, email_or_nexon_id: &str, password: &str) -> Result<PassportResponse, Box<dyn Error>> {
        login(&self.client, false, email_or_nexon_id, password, &self.config.device_id, &self.config.captcha_token).await?;
        check_playable(&self.client, self.config.product.id()).await?;
        let response = get_passport(&self.client, self.config.product.id()).await?;
        Ok(response)
    }
}

