use std::env;

use anyhow::Context as _;
use hatena_blog::{Client, Config, EntryId};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let entry_id = env::args().nth(1).context("no args")?.parse::<EntryId>()?;
    let config = Config::new_from_env()?;
    let client = Client::new(&config);
    let response_body = client.get_entry(&entry_id).await?;
    println!("{:?}", response_body);
    Ok(())
}
