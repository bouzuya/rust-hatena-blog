use std::convert::TryInto;

use hatena_blog::{Client, Config};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = Config::new_from_env()?;
    let client = Client::new(&config);
    let response = client.list_categories().await?;
    let categories: Vec<String> = response.try_into()?;
    println!("{:?}", categories);
    Ok(())
}
