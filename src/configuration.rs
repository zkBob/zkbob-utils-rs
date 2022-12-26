use config::Config;
use serde::{Deserialize, Serialize};
use serde_aux::field_attributes::deserialize_number_from_string;
use std::string::ToString;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ApplicationSettings {
    pub host: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub telemetry: TelemetrySettings,
}
#[derive(Debug, Serialize, Deserialize, strum::EnumString, Clone)]
pub enum TelemetryKind {
    #[strum(serialize = "stdout")]
    Stdout,
    #[strum(serialize = "jaeger")]
    Jaeger,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TelemetrySettings {
    pub kind: TelemetryKind,
    pub endpoint: Option<String>,
    pub log_level: LogLevel,
    pub service_name: String
}

#[derive(Debug, Deserialize, Clone, Serialize, Copy, strum::EnumString, strum_macros::Display)]
pub enum LogLevel {
    TRACE,
    DEBUG,
    INFO,
    WARN,
    ERROR,
}

pub fn get_config<S>() -> Result<S, config::ConfigError>
where
    for<'a> S: serde::Deserialize<'a>,
{
    let environment: Environment = std::env::var("APP_ENVIRONMENT")
        .unwrap_or_else(|_| "local".to_string())
        .try_into()
        .expect("failed to parse environment");

    Config::builder()
        .add_source(config::File::with_name("configuration/base.yaml"))
        .add_source(config::File::with_name(&format!(
            "configuration/{}.yaml",
            environment.as_str()
        )))
        .add_source(
            config::Environment::with_prefix("APP")
                .prefix_separator("_")
                .separator("__"), // .try_parsing(true) //if passing a list via env is really necessary
                                  // .with_list_parse_key("foo.bar.buzz")
                                  // .list_separator(","),
        )
        .build()?
        .try_deserialize::<S>()
}

pub struct Environment {
    id: String,
}

impl Environment {
    pub fn new(id: String) -> Self {
        Self { id }
    }

    pub fn as_str<'a>(&'a self) -> &'a str {
        &self.id
    }
}
impl TryFrom<String> for Environment {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        Ok(Environment { id: s })
    }
}
