mod client;
mod config;
mod entry;

use crate::client::Client;
use crate::config::Config;
use crate::entry::Entry;
use clap::{App, Arg, SubCommand};
use std::fs::File;
use std::io::prelude::*;

fn new_app<'a, 'b>() -> App<'a, 'b> {
    App::new("hatena-blog")
        .version("0.1.0")
        .author("bouzuya <m@bouzuya.net>")
        .subcommand(
            SubCommand::with_name("create")
                .about("create an entry")
                .arg(
                    Arg::with_name("title")
                        .long("title")
                        .required(true)
                        .value_name("TITLE")
                        .help("set title")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("content")
                        .long("content")
                        .required(true)
                        .value_name("FILE")
                        .help("set content (markdown file only)")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("updated")
                        .long("updated")
                        .value_name("UPDATED")
                        .help("set updated")
                        .takes_value(true),
                )
                .arg(Arg::with_name("draft").long("draft").help("set draft")),
        )
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let app = new_app();
    let config = Config::new_from_env().expect("invalid env");
    let client = Client::new(&config);
    let matches = app.get_matches();
    match matches.subcommand() {
        ("create", Some(matches)) => {
            let title = matches.value_of("title").unwrap();
            // content
            let content_file = matches.value_of("content").unwrap();
            let mut file = File::open(content_file)?;
            let mut content = String::new();
            file.read_to_string(&mut content)?;
            let updated = matches
                .value_of("updated")
                .unwrap_or("2020-03-14T00:00:00Z"); // FIXME
            let draft = matches.is_present("draft");

            let entry = Entry::new(
                title,
                &config.hatena_id,
                &vec![], // TODO
                &content,
                &updated,
                draft,
            );
            client.create_entry(&entry).await?;
        }
        _ => {}
    }
    Ok(())
}

#[test]
fn app_create() {
    let app = new_app();
    let m = app.get_matches_from(vec![
        "hatena-blog",
        "create",
        "--title",
        "TITLE",
        "--content",
        "FILE",
        "--updated",
        "UPDATED",
        "--draft",
    ]);
    assert_eq!(m.subcommand().0, "create");
    let sm = m.subcommand().1.unwrap();
    assert_eq!(sm.value_of("title"), Some("TITLE"));
    assert_eq!(sm.value_of("content"), Some("FILE"));
    assert_eq!(sm.value_of("updated"), Some("UPDATED"));
    assert_eq!(sm.is_present("draft"), true);
}

#[test]
fn command_h() -> Result<(), Box<dyn std::error::Error>> {
    use assert_cmd::prelude::*;
    use predicates::prelude::*;
    use std::process::Command;
    let mut cmd = Command::cargo_bin("hatena-blog")?;
    cmd.arg("-h");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("USAGE:"))
        .stdout(predicate::str::contains("FLAGS:"));
    Ok(())
}

#[test]
fn command_help() -> Result<(), Box<dyn std::error::Error>> {
    use assert_cmd::prelude::*;
    use predicates::prelude::*;
    use std::process::Command;
    let mut cmd = Command::cargo_bin("hatena-blog")?;
    cmd.arg("--help");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("USAGE:"))
        .stdout(predicate::str::contains("FLAGS:"));
    Ok(())
}

#[test]
fn command_v() -> Result<(), Box<dyn std::error::Error>> {
    use assert_cmd::prelude::*;
    use predicates::prelude::*;
    use std::process::Command;
    let mut cmd = Command::cargo_bin("hatena-blog")?;
    cmd.arg("-V");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("hatena-blog 0.1.0"));
    Ok(())
}

#[test]
fn command_version() -> Result<(), Box<dyn std::error::Error>> {
    use assert_cmd::prelude::*;
    use predicates::prelude::*;
    use std::process::Command;
    let mut cmd = Command::cargo_bin("hatena-blog")?;
    cmd.arg("--version");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("hatena-blog 0.1.0"));
    Ok(())
}
