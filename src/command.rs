use hatena_blog_api::{Client, Config, Entry, EntryId, EntryParams, PartialList};
use serde::Serialize;
use serde_json::json;
use std::{convert::TryInto, fs::File, io, path::PathBuf};

#[derive(Debug, Serialize)]
struct EntryJson {
    author_name: String,
    categories: Vec<String>,
    content: String,
    draft: bool,
    edit_url: String,
    edited: String,
    id: String,
    published: String,
    title: String,
    updated: String,
    url: String,
}

impl std::fmt::Display for EntryJson {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl From<Entry> for EntryJson {
    fn from(entry: Entry) -> Self {
        EntryJson {
            author_name: entry.author_name,
            categories: entry.categories,
            content: entry.content,
            draft: entry.draft,
            edit_url: entry.edit_url,
            edited: entry.edited.to_string(),
            id: entry.id.to_string(),
            published: entry.published.to_string(),
            title: entry.title,
            updated: entry.updated.to_string(),
            url: entry.url,
        }
    }
}

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
    let entry: Entry = client
        .create_entry(EntryParams::new(
            config.hatena_id,
            title,
            content,
            updated,
            categories,
            draft,
        ))
        .await?
        .try_into()?;
    println!("{}", EntryJson::from(entry));
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
    let entry: Entry = client.get_entry(&entry_id).await?.try_into()?;
    println!("{}", EntryJson::from(entry));
    Ok(())
}

pub async fn list(page: Option<String>) -> anyhow::Result<()> {
    let config = Config::new_from_env()?;
    let client = Client::new(&config);
    let (next_page, entry_ids): PartialList = client
        .list_entries_in_page(page.as_deref())
        .await?
        .try_into()?;
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
    let categories: Vec<String> = client.list_categories().await?.try_into()?;
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
    let entry: Entry = client
        .update_entry(
            &entry_id,
            EntryParams::new(config.hatena_id, title, content, updated, categories, draft),
        )
        .await?
        .try_into()?;
    println!("{}", EntryJson::from(entry));
    Ok(())
}
