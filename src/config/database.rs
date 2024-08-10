use anyhow::Result;
use mongodb::{options::ClientOptions, Client, Database};

pub async fn get_database() -> Result<Database> {
    let client_options = ClientOptions::parse("mongodb://localhost:27017")
        .await
        .map_err(|e| anyhow::anyhow!("Failed to parse MongoDB connection string: {}", e))?;

    let client = Client::with_options(client_options)
        .map_err(|e| anyhow::anyhow!("Failed to create MongoDB client: {}", e))?;

    Ok(client.database("test_db"))
}
