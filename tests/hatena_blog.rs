use std::env;

use assert_cmd::Command;
use mockito::mock;
use predicates::prelude::*;

// TODO: create
// TODO: delete
// TODO: get
// TODO: list
// TODO: update

#[test]
fn list_categories() -> anyhow::Result<()> {
    let _m = mock("GET", "/hatena_id1/blog_id1/atom/category")
        .with_status(200)
        .with_body(
            r#"<?xml version="1.0" encoding="utf-8"?>
    <app:categories
        xmlns:app="http://www.w3.org/2007/app"
        xmlns:atom="http://www.w3.org/2005/Atom"
        fixed="no">
      <atom:category term="Perl" />
      <atom:category term="Scala" />
    </app:categories>"#,
        )
        .create();
    env::set_var("HATENA_API_KEY", "api_key1");
    env::set_var("HATENA_BLOG_BASE_URL", mockito::server_url());
    env::set_var("HATENA_BLOG_ID", "blog_id1");
    env::set_var("HATENA_ID", "hatena_id1");
    Command::cargo_bin("hatena-blog")?
        .arg("list-categories")
        .assert()
        .success()
        .stdout(concat!(r#"["Perl","Scala"]"#, "\n"));
    Ok(())
}

#[test]
fn command_h() -> anyhow::Result<()> {
    Command::cargo_bin("hatena-blog")?
        .arg("-h")
        .assert()
        .success()
        .stdout(predicate::str::contains("USAGE:"))
        .stdout(predicate::str::contains("FLAGS:"));
    Ok(())
}

#[test]
fn command_help() -> anyhow::Result<()> {
    Command::cargo_bin("hatena-blog")?
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("USAGE:"))
        .stdout(predicate::str::contains("FLAGS:"));
    Ok(())
}

#[test]
fn command_v() -> anyhow::Result<()> {
    Command::cargo_bin("hatena-blog")?
        .arg("-V")
        .assert()
        .success()
        .stdout(predicate::str::contains("hatena-blog"));
    Ok(())
}

#[test]
fn command_version() -> anyhow::Result<()> {
    Command::cargo_bin("hatena-blog")?
        .arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("hatena-blog"));
    Ok(())
}
