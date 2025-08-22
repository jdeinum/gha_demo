use crate::error::Result;
use anyhow::Context;
use tracing::Subscriber;
use tracing::subscriber::set_global_default;
use tracing_log::LogTracer;
use tracing_subscriber::{EnvFilter, Registry, layer::SubscriberExt};

pub fn get_subscriber() -> Result<impl Subscriber + Send + Sync> {
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    let registry = Registry::default()
        .with(env_filter)
        .with(tracing_subscriber::fmt::Layer::new());

    Ok(registry)
}

pub fn init_subscriber(subscriber: impl Subscriber + Send + Sync) -> Result<()> {
    LogTracer::init().context("init log tracer")?;
    set_global_default(subscriber).expect("Failed to set subscriber");
    Ok(())
}
