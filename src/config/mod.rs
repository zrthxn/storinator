use eyre::WrapErr;
use color_eyre::Result;
use serde::{Deserialize};
use dotenv::dotenv;

use tracing::{info, instrument};
use tracing_subscriber::EnvFilter;

#[derive(Debug, Deserialize)]
pub struct Config {
  pub host: String,
  pub port: u16
}

impl Config {
  #[instrument]
  pub fn loadenv() -> Result<Config> {
    dotenv().ok();
    tracing_subscriber::fmt()
      .with_env_filter(EnvFilter::from_default_env())
      .init();

    info!("Loading configuration...");
    let mut conf = config::Config::new();
    conf.merge(config::Environment::default())?;

    conf.try_into().context("Reading .env variables")
  }
}