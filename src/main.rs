use hatena_blog::{Client, Config, Entry};
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
    #[structopt(name = "create", about = "create an entry")]
    Create {
        #[structopt(
            name = "FILE",
            long = "content",
            help = "set content (markdown file only)"
        )]
        content: PathBuf,
        #[structopt(long = "draft", help = "set draft")]
        draft: bool,
        #[structopt(name = "TITLE", long = "title", help = "set title")]
        title: String,
        #[structopt(name = "UPDATED", long = "updated", help = "set updated")]
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
            let entry = Entry::new(
                title,
                config.hatena_id,
                vec![], // TODO
                content,
                updated,
                draft,
            );
            client.create_entry(&entry).await?;
        }
        Subcommand::Get { entry_id } => {
            let entry = client.get_entry(entry_id.as_str()).await?;
            println!("{}", entry);
        }
    }
    Ok(())
}
