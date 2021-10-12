use assert_cmd::prelude::*;
use predicates;
use std::process::Command;

#[test]
fn run_with_defaults() {
    Command::cargo_bin("shelly")
        .expect("binary exists")
        .assert()
        .success()
        .stdout(predicates::str::contains("Hi, I'm Shelly!"))
        .stdout(predicates::str::contains("| o  |"));
}

#[test]
fn run_with_custom_message() {
    Command::cargo_bin("shelly")
        .expect("binary exists")
        .args(&["lmao", "im", "a", "turtle"])
        .assert()
        .success()
        .stdout(predicates::str::contains("lmao im a turtle"))
        .stdout(predicates::str::contains("| o  |"));
}

#[test]
fn run_with_dead() {
    Command::cargo_bin("shelly")
        .expect("binary exists")
        .args(&["-d"])
        .assert()
        .success()
        .stdout(predicates::str::contains("Hi, I'm Shelly!"))
        .stdout(predicates::str::contains("| x  |"));
}

#[test]
fn run_with_smile() {
    Command::cargo_bin("shelly")
        .expect("binary exists")
        .args(&["-s"])
        .assert()
        .success()
        .stdout(predicates::str::contains("Hi, I'm Shelly!"))
        .stdout(predicates::str::contains("|_/__"));
}

#[test]
fn run_with_custom_template() {
    Command::cargo_bin("shelly")
        .expect("binary exists")
        .args(&["-f", "templates/bigshelly.txt"])
        .assert()
        .success()
        .stdout(predicates::str::contains("Hi, I'm Shelly!"))
        .stdout(predicates::str::contains(r"/^\__/^\"))
        .stdout(predicates::str::contains(
            r"~-----||====/~     |==================|       |/~~~~~",
        ));
}

#[test]
fn run_with_big() {
    Command::cargo_bin("shelly")
        .expect("binary exists")
        .args(&["-b"])
        .assert()
        .success()
        .stdout(predicates::str::contains("Hi, I'm Shelly!"))
        .stdout(predicates::str::contains(r"/^\__/^\"))
        .stdout(predicates::str::contains(
            r"~-----||====/~     |==================|       |/~~~~~",
        ));
}
