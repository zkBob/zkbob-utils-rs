use serde::{Deserialize, Serialize};
use serde_aux::field_attributes::deserialize_number_from_string;
use std::string::ToString;


use crate::telemetry::telemetry::TelemetrySettings;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ApplicationSettings {
    pub host: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub telemetry: TelemetrySettings,
}




pub fn get_config<S>() -> Result<S, config::ConfigError>
where
    for<'a> S: serde::Deserialize<'a>,
{
    let mut settings = config::Config::default();
    let base_path = std::env::current_dir().expect("failed to determine current dir");
    let configuration_directory = base_path.join("configuration");

    settings.merge(config::File::from(configuration_directory.join("base.yaml")).required(true))?;

    let environment: Environment = std::env::var("APP_ENVIRONMENT")
        .unwrap_or_else(|_| "local".to_string())
        .try_into()
        .expect("failed to parse environment");

    settings
        .merge(
            config::File::from(configuration_directory.join(environment.as_str())).required(true),
        )
        .expect("failed to apply env settings");

    settings.merge(config::Environment::with_prefix("app").separator("__"))?;
    settings.try_into()
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
