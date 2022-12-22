use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum TelemetryKind {
    Stdout,
    Jaeger,
}

impl From<String> for TelemetryKind {
    fn from(s: String) -> Self {
        match s.as_str() {
            "stdout" => Self::Stdout,
            "jaeger" => Self::Jaeger,
            _ => Self::Stdout,
        }
    }
}

#[derive(Debug, Deserialize, Clone, Serialize, Copy)]
pub enum LogLevel {
    TRACE,
    DEBUG,
    INFO,
    WARN,
    ERROR,
}

impl From<String> for LogLevel {
    fn from(s: String) -> Self {
        match s.as_str() {
            "TRACE" => Self::TRACE,
            "DEBUG" => Self::DEBUG,
            "INFO" => Self::INFO,
            "WARN" => Self::WARN,
            "ERROR" => Self::ERROR,
            _ => Self::INFO,
        }
    }
}

impl Into<String> for LogLevel {
    fn into(self) -> String {
        match self {
            LogLevel::TRACE => "TRACE",
            LogLevel::DEBUG => "DEBUG",
            LogLevel::INFO => "INFO",
            LogLevel::WARN => "WARN",
            LogLevel::ERROR => "ERROR",
        }
        .to_string()
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TelemetrySettings {
    pub kind: TelemetryKind,
    pub endpoint: Option<String>,
    pub log_level: LogLevel,
}
