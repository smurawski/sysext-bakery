use assert_cmd::Command;
use rand::{distributions::Alphanumeric, Rng};

pub fn random_string() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(7)
        .map(char::from)
        .collect()
}

pub fn cleanup(name: &str) {
    let path = format!("{}.raw", name);
    std::fs::remove_dir_all(name).unwrap();
    std::fs::remove_file(path).unwrap();
}

pub fn create_command() -> Command {
    Command::cargo_bin("bakery").unwrap()
}

#[test]
fn test_cli_runs() {
    let random_name = format!("test-sample-{}", random_string());

    let mut cmd = create_command();
    cmd.arg("bake");
    cmd.arg(&random_name);
    cmd.assert().success();

    cleanup(&random_name);
}

#[test]
fn test_cli_help() {
    let mut cmd = create_command();
    cmd.arg("--help");
    cmd.assert().success();
}
