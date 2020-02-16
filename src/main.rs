mod client;
mod config;
mod entry;

use crate::client::Client;
use crate::config::Config;
use crate::entry::Entry;
use clap::App;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let _ = App::new("hatena-blog")
        .version("0.1.0")
        .author("bouzuya <m@bouzuya.net>")
        .get_matches();
    let config = Config::new_from_env().expect("invalid env");
    let client = Client::new(&config);
    let entry = Entry::new_dummy();
    client.create_entry(&entry).await?;
    Ok(())
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
