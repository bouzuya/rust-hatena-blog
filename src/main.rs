use hatena_blog::command;
use hatena_blog_api::EntryId;
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

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let opt = Opt::from_args();
    match opt.subcommand {
        Subcommand::Create {
            categories,
            content,
            draft,
            title,
            updated,
        } => command::create(categories, content, draft, title, updated).await,
        Subcommand::Delete { entry_id } => command::delete(entry_id).await,
        Subcommand::Get { entry_id } => command::get(entry_id).await,
        Subcommand::List { page } => command::list(page).await,
        Subcommand::ListCategories => command::list_categories().await,
        Subcommand::Update {
            categories,
            entry_id,
            content,
            draft,
            title,
            updated,
        } => command::update(categories, entry_id, content, draft, title, updated).await,
    }
}
