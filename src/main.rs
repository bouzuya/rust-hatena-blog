use hatena_blog::{Client, Config};
use std::fs;
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
        #[structopt(
            name = "FILE",
            long = "content",
            help = "set content (markdown file only)"
        )]
        content: PathBuf,
        #[structopt(long = "draft", help = "Creates as draft")]
        draft: bool,
        #[structopt(long = "title", name = "TITLE", help = "The title")]
        title: String,
        #[structopt(long = "updated", name = "UPDATED", help = "The date")]
        updated: String,
    },
    #[structopt(name = "get", about = "Gets the entry")]
    Get {
        #[structopt(name = "ENTRY_ID", help = "The entry id")]
        entry_id: String,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let opt = Opt::from_args();
    let config = Config::new_from_env().expect("invalid env");
    let client = Client::new(&config);
    match opt.subcommand {
        Subcommand::Create {
            content,
            draft,
            title,
            updated,
        } => {
            let content = fs::read_to_string(content.as_path())?;
            let entry = client
                .create_entry(
                    config.hatena_id,
                    title,
                    content,
                    updated,
                    vec![], // TODO
                    draft,
                )
                .await?;
            println!("{}", entry.to_json());
        }
        Subcommand::Get { entry_id } => {
            let entry = client.get_entry(entry_id.as_str()).await?;
            println!("{}", entry.to_json());
        }
    }
    Ok(())
}
