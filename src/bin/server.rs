use warp_rest_api::config;
use warp_rest_api::{run, setup_store};

#[tokio::main]
async fn main() -> Result<(), handle_errors::Error> {
    dotenv::dotenv().ok();

    let config = config::Config::new().expect("Config can't be set");
    let store = setup_store(&config).await?;

    tracing::info!("warp-rest-api build ID {}", env!("WARP_REST_API_VERSION"));

    run(config, store).await;

    Ok(())
}
