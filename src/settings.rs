use crate::error::Result;
use anyhow::Context;
use config::Config;
use secrecy::{ExposeSecret, SecretString};
use serde::Deserialize;
use serde_aux::field_attributes::deserialize_number_from_string;
use sqlx::postgres::{PgConnectOptions, PgSslMode};
use tracing::info;

#[derive(Deserialize, Debug, Clone)]
pub struct Settings {
    pub application: ApplicationSettings,
    pub db: DbSettings,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ApplicationSettings {
    pub host: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
}

impl ApplicationSettings {
    pub fn connection_string(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct DbSettings {
    pub username: String,
    pub password: SecretString,
    pub host: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub database: String,
    pub ssl: bool,
}

impl DbSettings {
    pub fn get_db_settings(&self) -> PgConnectOptions {
        let ssl_mode = if self.ssl {
            PgSslMode::Require
        } else {
            PgSslMode::Disable
        };

        PgConnectOptions::new()
            .host(&self.host)
            .port(self.port)
            .username(&self.username)
            .password(self.password.expose_secret())
            .ssl_mode(ssl_mode)
            .database(&self.database)
    }
}

pub fn get_settings() -> Result<Settings> {
    let environment = std::env::var("APP_ENV").unwrap_or("local".into());
    info!("using the {environment} env");

    let filename = format!("configuration/{}.yaml", environment);
    let settings = Config::builder()
        .add_source(config::File::with_name("configuration/base.yaml"))
        .add_source(config::File::with_name(&filename))
        .add_source(
            config::Environment::with_prefix("APP")
                .separator("__")
                .prefix_separator("_"),
        )
        .build()
        .context("build config")?;

    let s = settings
        .try_deserialize::<Settings>()
        .context("deserialize into settings")?;

    Ok(s)
}
