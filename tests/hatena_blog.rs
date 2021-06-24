use assert_cmd::Command;
use predicates::prelude::*;

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
