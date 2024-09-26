use anyhow::{anyhow, Context, Result};
use clap::Parser;
use tracing::info;

mod config;
mod main_service;
mod web_routes;

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    /// Path to the configuration file
    #[arg(short, long)]
    config: Option<String>,
}

#[rocket::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let args = Args::parse();

    info!("Starting {{ app_name }}");
    info!("Supported methods:");
    for method in main_service::rpc_methods() {
        info!("  /prpc/{method}");
    }

    let figment = config::load_config_figment(args.config.as_deref());
    let config = figment
        .extract::<config::AppConfig>()
        .context("Failed to load config")?;
    let state = main_service::AppState::new(config);
    let rocket = rocket::custom(figment)
        .mount("/", web_routes::routes())
        .manage(state);
    rocket
        .launch()
        .await
        .map_err(|err| anyhow!(err.to_string()))?;
    Ok(())
}
