use anyhow::anyhow;
use config::{Config, File};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct AppSettings {
    pub server: Server,
    pub token: Token,
    pub frontend_url: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Server {
    pub protocol: String,
    pub host: String,
    pub port: u16,
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

#[allow(unused)]
impl Environment {
    pub fn settings_file_name(self) -> &'static str {
        match self {
            Environment::Development => "dev-settings.yaml",
            Environment::Production => "prod-settings.yaml",
        }
    }
}

/// Loads the applications's `settings` based on the specified application's environment in
/// the root ***.env*** file.
pub fn load_app_settings() -> anyhow::Result<AppSettings> {
    let settings_dir = std::env::current_dir()?.join("settings");
    let app_env: Environment = std::env::var("APP_ENV")?
        .try_into()
        .unwrap_or(Environment::Development);

    Config::builder()
        .add_source(File::from(settings_dir.join("settings.yaml")))
        .add_source(File::with_name(app_env.settings_file_name()))
        .build()
        .map_err(anyhow::Error::msg)
        .and_then(|c| c.try_deserialize().map_err(anyhow::Error::msg))
}
