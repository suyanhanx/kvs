#[cfg(test)]
mod command_test {
    use assert_cmd::prelude::*;
    use kvs::KvStore;
    use predicates::str::contains;
    use std::process::Command;

    #[test]
    fn cli_no_args() {
        Command::cargo_bin("kvs").unwrap().assert().failure();
    }

    #[test]
    fn cli_version() {
        Command::cargo_bin("kvs")
            .unwrap()
            .args(&["-V"])
            .assert()
            .stdout(contains(env!("CARGO_PKG_VERSION")));
    }

    #[test]
    fn cli_get() {
        Command::cargo_bin("kvs")
            .unwrap()
            .args(&["get", "key1"])
            .assert()
            .failure()
            .stderr(contains("unimplemented"));
    }

    #[test]
    fn cli_set() {
        Command::cargo_bin("kvs")
            .unwrap()
            .args(&["set", "key1", "value1"])
            .assert()
            .failure()
            .stderr(contains("unimplemented"));
    }

    #[test]
    fn cli_rm() {
        Command::cargo_bin("kvs")
            .unwrap()
            .args(&["rm", "key1"])
            .assert()
            .failure()
            .stderr(contains("unimplemented"));
    }

    #[test]
    fn cli_invalid_get() {
        Command::cargo_bin("kvs")
            .unwrap()
            .args(&["get"])
            .assert()
            .failure();

        Command::cargo_bin("kvs")
            .unwrap()
            .args(&["get", "extra", "field"])
            .assert()
            .failure();
    }

    #[test]
    fn cli_invalid_set() {
        Command::cargo_bin("kvs")
            .unwrap()
            .args(&["set"])
            .assert()
            .failure();

        Command::cargo_bin("kvs")
            .unwrap()
            .args(&["set", "missing_field"])
            .assert()
            .failure();

        Command::cargo_bin("kvs")
            .unwrap()
            .args(&["set", "extra", "extra", "field"])
            .assert()
            .failure();
    }

    #[test]
    fn cli_invalid_rm() {
        Command::cargo_bin("kvs")
            .unwrap()
            .args(&["rm"])
            .assert()
            .failure();

        Command::cargo_bin("kvs")
            .unwrap()
            .args(&["rm", "extra", "field"])
            .assert()
            .failure();
    }

    #[test]
    fn cli_invalid_command() {
        Command::cargo_bin("kvs")
            .unwrap()
            .args(&["unknown", "command"])
            .assert()
            .failure();
    }
}
