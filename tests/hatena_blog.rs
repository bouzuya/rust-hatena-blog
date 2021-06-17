use assert_cmd::Command;
use predicates::prelude::*;

// TODO:
// #[test]
// fn app_create() {
//     let app = new_app();
//     let m = app.get_matches_from(vec![
//         "hatena-blog",
//         "create",
//         "--title",
//         "TITLE",
//         "--content",
//         "FILE",
//         "--updated",
//         "UPDATED",
//         "--draft",
//     ]);
//     assert_eq!(m.subcommand().0, "create");
//     let sm = m.subcommand().1.unwrap();
//     assert_eq!(sm.value_of("title"), Some("TITLE"));
//     assert_eq!(sm.value_of("content"), Some("FILE"));
//     assert_eq!(sm.value_of("updated"), Some("UPDATED"));
//     assert_eq!(sm.is_present("draft"), true);
// }

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
