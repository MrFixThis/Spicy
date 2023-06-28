use std::sync::OnceLock;

use anyhow::anyhow;
use config::{Config, File};
use serde::Deserialize;

static APP_SETTINGS: OnceLock<AppSettings> = OnceLock::new();

#[derive(Debug, Clone, Deserialize)]
pub struct AppSettings {
    pub frontend_url: String,
    pub server: Server,
    pub datasource: Datasource,
    pub token: [Token; 2],
}

impl AppSettings {
    pub fn get() -> &'static Self {
        APP_SETTINGS.get().unwrap()
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Server {
    pub protocol: String,
    pub host: String,
    pub port: u16,
}

impl Server {
    pub fn socket_address(&self) -> (String, u16) {
        (self.host.clone(), self.port)
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Datasource {
    pub name: String,
    pub host: String,
    pub username: String,
    pub password: String,
}

impl Datasource {
    const DB_BACKEND: &str = "mysql";

    pub fn into_raw_format(self) -> (String, String) {
        (
            format!(
                "{}://{}:{}@{}",
                Self::DB_BACKEND,
                self.username,
                self.password,
                self.host
            ),
            self.name,
        )
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Token {
    pub secret_key: String,
    pub implicit_assert: String,
    pub exp_time: i64,
}

#[derive(Debug)]
pub enum Environment {
    Development,
    Production,
}

impl ToString for Environment {
    fn to_string(&self) -> String {
        match self {
            Environment::Development => "development".to_owned(),
            Environment::Production => "production".to_owned(),
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "dev" | "development" => Ok(Self::Development),
            "prod" | "production" => Ok(Self::Production),
            other => Err(anyhow!(format!(r#"unknown environment "{other}""#))),
        }
    }
}

impl Environment {
    pub fn settings_file_name(self) -> &'static str {
        match self {
            Environment::Development => "dev-settings.toml",
            Environment::Production => "prod-settings.toml",
        }
    }
}

/// The `base` settings file's name.
///
/// This file is meant to hold the `common` settings shared by the _development_ and _production_
/// settings files.
const BASE_SETTINGS_FILE: &str = "settings.toml";

/// Parses the applications's `settings` based on the specified application's environment in
/// the root ***.env*** file.
pub fn parse_app_settings() -> anyhow::Result<()> {
    let settings_dir = std::env::current_dir()?.join("settings");
    let app_env: Environment = std::env::var("APP_ENV")
        .or(Ok::<_, anyhow::Error>(Environment::Development.to_string()))?
        .try_into()?;

    _ = APP_SETTINGS.set(
        Config::builder()
            .add_source(File::from(settings_dir.join(BASE_SETTINGS_FILE)))
            .add_source(File::from(settings_dir.join(app_env.settings_file_name())))
            .build()
            .map_err(anyhow::Error::msg)
            .and_then(|c| c.try_deserialize().map_err(anyhow::Error::msg))?,
    );

    Ok(())
}
