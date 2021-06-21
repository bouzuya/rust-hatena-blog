use std::env;

use hatena_blog::{Client, Config};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let id = env::args().nth(1).unwrap();
    let config = Config::new_from_env()?;
    let client = Client::new(&config);
    client.delete_entry(id.as_str()).await?;
    Ok(())
}
