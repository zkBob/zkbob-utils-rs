use opentelemetry::{global, runtime::Tokio};
use opentelemetry_jaeger::Propagator;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::{fmt, layer::SubscriberExt, prelude::*, EnvFilter, Registry};

use super::config::{TelemetryKind, TelemetrySettings};

pub fn init_stdout(name: String, env_filter: String) {
    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(env_filter));

    let formatting_layer = BunyanFormattingLayer::new(name, || std::io::stdout());
    Registry::default()
        .with(env_filter)
        .with(formatting_layer)
        .with(JsonStorageLayer)
        .init();
}

pub fn init_sink(name: String, env_filter: String) {
    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(env_filter));

    let formatting_layer = BunyanFormattingLayer::new(name, || std::io::sink());
    Registry::default()
        .with(env_filter)
        .with(formatting_layer)
        .with(JsonStorageLayer)
        .init();
}

pub fn init_jaeger(name: String, log_level: String, endpoint: &Option<String>) {
    global::set_text_map_propagator(Propagator::new());

    let mut agent_pipeline = opentelemetry_jaeger::new_agent_pipeline().with_service_name(name);

    if let Some(agent_endpoint) = endpoint {
        agent_pipeline = agent_pipeline.with_endpoint(agent_endpoint);
    }

    let tracer = agent_pipeline.install_batch(Tokio).unwrap();

    Registry::default()
        .with(EnvFilter::new(log_level))
        .with(fmt::layer().with_target(false))
        .with(tracing_opentelemetry::layer().with_tracer(tracer))
        .init();
}

pub fn setup_telemetry(config: &TelemetrySettings) {
    let name = "zkbob-relayer".to_string();
    let log_level = config.log_level.into();

    match config.kind {
        TelemetryKind::Stdout => init_stdout(name, log_level),
        TelemetryKind::Jaeger => init_jaeger(name, log_level, &config.endpoint),
    }
}
