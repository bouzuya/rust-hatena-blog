use std::{fs::File, io, path::PathBuf};

use serde_json::json;

use crate::{Client, Config, EntryId, EntryParams};

fn read_content(content: PathBuf) -> anyhow::Result<String> {
    let (mut stdin_read, mut file_read);
    let readable: &mut dyn io::Read = if content == PathBuf::from("-") {
        stdin_read = io::stdin();
        &mut stdin_read
    } else {
        file_read = File::open(content.as_path())?;
        &mut file_read
    };
    let mut content = String::new();
    readable.read_to_string(&mut content)?;
    Ok(content)
}

pub async fn create(
    categories: Vec<String>,
    content: PathBuf,
    draft: bool,
    title: String,
    updated: String,
) -> anyhow::Result<()> {
    let config = Config::new_from_env()?;
    let client = Client::new(&config);
    let content = read_content(content)?;
    let entry = client
        .create_entry(EntryParams::new(
            config.hatena_id,
            title,
            content,
            updated,
            categories,
            draft,
        ))
        .await?;
    println!("{}", entry.to_json());
    Ok(())
}

pub async fn delete(entry_id: EntryId) -> anyhow::Result<()> {
    let config = Config::new_from_env()?;
    let client = Client::new(&config);
    client.delete_entry(&entry_id).await?;
    Ok(())
}

pub async fn get(entry_id: EntryId) -> anyhow::Result<()> {
    let config = Config::new_from_env()?;
    let client = Client::new(&config);
    let entry = client.get_entry(&entry_id).await?;
    println!("{}", entry.to_json());
    Ok(())
}

pub async fn list(page: Option<String>) -> anyhow::Result<()> {
    let config = Config::new_from_env()?;
    let client = Client::new(&config);
    let (next_page, entry_ids) = client.list_entries_in_page(page.as_deref()).await?;
    println!(
        "{}",
        serde_json::Value::Object({
            let mut map = serde_json::Map::new();
            if let Some(next_page) = next_page {
                map.insert(
                    "next_page".to_string(),
                    serde_json::Value::String(next_page),
                );
            }
            map.insert(
                "entry_ids".to_string(),
                serde_json::Value::Array(
                    entry_ids
                        .into_iter()
                        .map(|entry_id| serde_json::Value::String(entry_id.to_string()))
                        .collect(),
                ),
            );
            map
        })
    );
    Ok(())
}

pub async fn list_categories() -> anyhow::Result<()> {
    let config = Config::new_from_env()?;
    let client = Client::new(&config);
    let categories = client.list_categories().await?;
    println!("{}", json!(categories));
    Ok(())
}

pub async fn update(
    categories: Vec<String>,
    entry_id: EntryId,
    content: PathBuf,
    draft: bool,
    title: String,
    updated: String,
) -> anyhow::Result<()> {
    let config = Config::new_from_env()?;
    let client = Client::new(&config);
    let content = read_content(content)?;
    let entry = client
        .update_entry(
            &entry_id,
            EntryParams::new(config.hatena_id, title, content, updated, categories, draft),
        )
        .await?;
    println!("{}", entry.to_json());
    Ok(())
}
