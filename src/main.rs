use client_api::{some_test_printing, LoLClient};
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    some_test_printing::test_all().await?;

    Ok(())
}

