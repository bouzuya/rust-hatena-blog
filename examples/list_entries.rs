use std::convert::TryInto;

use hatena_blog::{Client, Config, EntryId};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = Config::new_from_env()?;
    let client = Client::new(&config);
    let response = client.list_entries_in_page(None).await?;
    let partial_list: (Option<String>, Vec<EntryId>) = response.try_into()?;
    println!("{:?}", partial_list);
    Ok(())
}
