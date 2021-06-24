use hatena_blog::{Client, Config, EntryId, EntryParams};
use serde_json::json;
use std::fs;
use std::io;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(subcommand)]
    subcommand: Subcommand,
}

#[derive(Debug, StructOpt)]
pub enum Subcommand {
    #[structopt(name = "create", about = "Creates a new entry")]
    Create {
        #[structopt(long = "category", name = "CATEGORY", help = "The category")]
        categories: Vec<String>,
        #[structopt(name = "FILE", help = "The file")]
        content: PathBuf,
        #[structopt(long = "draft", help = "Creates as draft")]
        draft: bool,
        #[structopt(long = "title", name = "TITLE", help = "The title")]
        title: String,
        #[structopt(long = "updated", name = "UPDATED", help = "The date")]
        updated: String,
    },
    #[structopt(name = "delete", about = "Deletes the entry")]
    Delete {
        #[structopt(name = "ENTRY_ID", help = "The entry id")]
        entry_id: EntryId,
    },
    #[structopt(name = "get", about = "Gets the entry")]
    Get {
        #[structopt(name = "ENTRY_ID", help = "The entry id")]
        entry_id: EntryId,
    },
    #[structopt(name = "list", about = "Lists the entries")]
    List {
        #[structopt(long = "page", name = "PAGE", help = "The page")]
        page: Option<String>,
    },
    #[structopt(name = "list-categories", about = "Lists all categories")]
    ListCategories,
    #[structopt(name = "update", about = "Updates the entry")]
    Update {
        #[structopt(long = "category", name = "CATEGORY", help = "The category")]
        categories: Vec<String>,
        #[structopt(name = "ENTRY_ID", help = "The entry id")]
        entry_id: EntryId,
        #[structopt(name = "FILE", help = "The file")]
        content: PathBuf,
        #[structopt(long = "draft", help = "Creates as draft")]
        draft: bool,
        #[structopt(long = "title", name = "TITLE", help = "The title")]
        title: String,
        #[structopt(long = "updated", name = "UPDATED", help = "The date")]
        updated: String,
    },
}

fn read_content(content: PathBuf) -> anyhow::Result<String> {
    let (mut stdin_read, mut file_read);
    let readable: &mut dyn io::Read = if content == PathBuf::from("-") {
        stdin_read = io::stdin();
        &mut stdin_read
    } else {
        file_read = fs::File::open(content.as_path())?;
        &mut file_read
    };
    let mut content = String::new();
    readable.read_to_string(&mut content)?;
    Ok(content)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let opt = Opt::from_args();
    let config = Config::new_from_env().expect("invalid env");
    let client = Client::new(&config);
    match opt.subcommand {
        Subcommand::Create {
            categories,
            content,
            draft,
            title,
            updated,
        } => {
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
        }
        Subcommand::Delete { entry_id } => {
            client.delete_entry(&entry_id).await?;
        }
        Subcommand::Get { entry_id } => {
            let entry = client.get_entry(&entry_id).await?;
            println!("{}", entry.to_json());
        }
        Subcommand::List { page } => {
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
        }
        Subcommand::ListCategories => {
            let categories = client.list_categories().await?;
            println!("{}", json!(categories));
        }
        Subcommand::Update {
            categories,
            entry_id,
            content,
            draft,
            title,
            updated,
        } => {
            let content = read_content(content)?;
            let entry = client
                .update_entry(
                    &entry_id,
                    EntryParams::new(config.hatena_id, title, content, updated, categories, draft),
                )
                .await?;
            println!("{}", entry.to_json());
        }
    }
    Ok(())
}
