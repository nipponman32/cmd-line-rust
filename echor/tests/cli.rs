use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;

type TypeResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn dies_no_args() -> TypeResult {
    let mut cmd = Command::cargo_bin("echor")?;
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("USAGE"));
    Ok(())
}

fn runs(args: &[&str], expected_file: &str) -> TypeResult {
    let expected = fs::read_to_string(expected_file)?;
    Command::cargo_bin("echor")?
        .args(args)
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}

#[test]
fn hello1() -> TypeResult {
    runs(&["Hello there"], "tests/expected/hello1.txt")
}

#[test]
fn hello2() -> TypeResult {
    runs(&["Hello", "there"], "tests/expected/hello2.txt")
}

#[test]
fn hello1_no_newline() -> TypeResult {
    runs(&["Hello", "there", "-n"], "tests/expected/hello1.n.txt")
}

#[test]
fn hello2_no_newline() -> TypeResult {
    runs(&["-n", "Hello", "there"], "tests/expected/hello2.n.txt")
}
