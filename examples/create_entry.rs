use hatena_blog::{Client, Config};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = Config::new_from_env()?;
    let client = Client::new(&config);
    let response_body = client
        .create_entry(
            config.hatena_id,
            "title1".to_string(),
            "content1".to_string(),
            "2021-06-20T15:49:00+09:00".to_string(),
            vec![],
            true,
        )
        .await?;
    println!("{:?}", response_body);
    Ok(())
}
