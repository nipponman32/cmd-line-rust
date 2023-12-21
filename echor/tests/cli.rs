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

#[test]
fn hello1() -> TypeResult {
    let outfile = "tests/expected/hello1.txt";
    let expected = fs::read_to_string(outfile).unwrap();

    let mut cmd = Command::cargo_bin("echor")?;
    cmd.args(vec!["Hello".to_string(), "there".to_string()])
        .assert()
        .success()
        .stdout(expected);
    //cmd.arg("Hello").assert().success().stdout(expected);
    Ok(())
}

#[test]
fn hello2() -> TypeResult {
    let expected = fs::read_to_string("tests/expected/hello2.txt")?;
    let mut cmd = Command::cargo_bin("echor")?;
    cmd.args(vec!["Hello", "there"])
        .assert()
        .success()
        .stdout(expected);
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
fn hello_no_newline() -> TypeResult {
    runs(&["Hello", "there", "-n"], "tests/expected/hello1.n.txt")
}
