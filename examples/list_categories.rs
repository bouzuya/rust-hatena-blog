use hatena_blog::{Client, Config};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = Config::new_from_env()?;
    let client = Client::new(&config);
    let response_body = client.list_categories().await?;
    println!("{:?}", response_body);
    Ok(())
}
