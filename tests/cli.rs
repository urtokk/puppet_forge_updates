use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn runs_with_help() {
    let mut cmd = Command::cargo_bin("puppet_forge_updates").unwrap();
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("USAGE").or(predicate::str::contains("Usage")));
}

#[test]
fn fails_with_nonexistent_file() {
    let mut cmd = Command::cargo_bin("puppet_forge_updates").unwrap();
    cmd.arg("not_a_real_file.txt")
        .assert()
        .failure()
        .stderr(predicate::str::contains("No such file").or(predicate::str::contains("not found")));
}

#[test]
fn fails_with_no_arguments() {
    let mut cmd = Command::cargo_bin("puppet_forge_updates").unwrap();
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("USAGE").or(predicate::str::contains("Usage")));
}
