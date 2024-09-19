use anyhow::{anyhow, Result};
use rocket::figment::Figment;

mod config;
mod main_service;
mod web_routes;

#[rocket::main]
async fn main() -> Result<()> {
    let figment = config::load_config_figment();
    let state = main_service::AppState::new();
    let rocket = rocket::custom(figment)
        .mount("/", web_routes::routes())
        .manage(state);
    rocket
        .launch()
        .await
        .map_err(|err| anyhow!(err.to_string()))?;
    Ok(())
}
