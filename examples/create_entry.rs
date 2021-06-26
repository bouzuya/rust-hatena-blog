use std::convert::TryInto;

use hatena_blog::{Client, Config, Entry, EntryParams};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = Config::new_from_env()?;
    let client = Client::new(&config);
    let response = client
        .create_entry(EntryParams::new(
            config.hatena_id,
            "title1".to_string(),
            "content1".to_string(),
            "2021-06-20T15:49:00+09:00".to_string(),
            vec![],
            true,
        ))
        .await?;
    let entry: Entry = response.try_into()?;
    println!("{:?}", entry);
    Ok(())
}
