mod client;
mod config;
mod entry;

use crate::client::Client;
use crate::config::Config;
use crate::entry::Entry;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let config = Config::new_from_env().expect("invalid env");
    let client = Client::new(&config);
    let entry = Entry::new();
    client.create_entry(&entry).await?;
    Ok(())
}
