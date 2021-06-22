use std::env;

use anyhow::Context as _;
use hatena_blog::{Client, Config, EntryId, EntryParams};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let entry_id = env::args().nth(1).context("no args")?.parse::<EntryId>()?;
    let config = Config::new_from_env()?;
    let client = Client::new(&config);
    let response_body = client
        .update_entry(
            &entry_id,
            EntryParams::new(
                config.hatena_id,
                "title2".to_string(),
                "content2".to_string(),
                "2021-06-20T15:49:00+09:00".to_string(),
                vec![],
                true,
            ),
        )
        .await?;
    println!("{:?}", response_body);
    Ok(())
}
