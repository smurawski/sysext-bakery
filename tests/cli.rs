use assert_cmd::Command;
use rand::{distributions::Alphanumeric, Rng};

fn random_string() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(7)
        .map(char::from)
        .collect()
}

fn cleanup(name: &str) {
    let path = format!("{}.raw", name);
    std::fs::remove_dir_all(name).unwrap();
    std::fs::remove_file(path).unwrap();
}

#[test]
fn cli_runs() {
    let random_name = format!("test-sample-{}", random_string());

    let mut cmd = Command::cargo_bin("bakery").unwrap();
    cmd.arg(&random_name);
    cmd.assert().success();
    
    cleanup(&random_name);
}

#[test]
fn cli_fails_on_missing_name() {
    let mut cmd = Command::cargo_bin("bakery").unwrap();
    cmd.assert().failure();
}

#[test]
fn cli_succeeds_with_os() {
    let random_name = format!("test-sample-{}", random_string());

    let mut cmd = Command::cargo_bin("bakery").unwrap();
    cmd.arg(&random_name).arg("--os").arg("debian");
    cmd.assert().success();

    cleanup(&random_name);
}

#[test]
fn cli_fails_on_unknown_arch() {
    let random_name = format!("test-sample-{}", random_string());

    let mut cmd = Command::cargo_bin("bakery").unwrap();
    cmd.arg(&random_name).arg("--arch").arg("unknown");
    cmd.assert().failure();
}

#[test]
fn cli_help() {
    let mut cmd = Command::cargo_bin("bakery").unwrap();
    cmd.arg("--help");
    cmd.assert().success();
}

#[test]
fn extension_release_any_os() {
    let name = "test-any";
    let mut cmd = Command::cargo_bin("bakery").unwrap();
    cmd.arg(&name).arg("--os").arg("_any");
    cmd.assert().success();

    let expected = std::fs::read_to_string("tests/samples/_any_os.txt").unwrap();
    let actual = std::fs::read_to_string("test-any/usr/lib/extension-release.d/extension-release.test-any").unwrap();
    
    cleanup(&name);
    assert_eq!(actual, expected);
}

#[test]
fn extension_release_debian_os() {
    let name = "test-debian";
    let mut cmd = Command::cargo_bin("bakery").unwrap();
    cmd.arg(&name).arg("--os").arg("debian");
    cmd.assert().success();

    let expected = std::fs::read_to_string("tests/samples/debian_os.txt").unwrap();
    let actual = std::fs::read_to_string("test-debian/usr/lib/extension-release.d/extension-release.test-debian").unwrap();
    cleanup(&name);
    assert_eq!(actual, expected);
}

#[test]
fn extension_release_any_os_x86_64_arch() {
    let name = "test-any-x86_64";
    let mut cmd = Command::cargo_bin("bakery").unwrap();
    cmd.arg(&name).arg("--os").arg("_any").arg("--arch").arg("x86-64");
    cmd.assert().success();

    let expected = std::fs::read_to_string("tests/samples/_any_os_x86_64_arch.txt").unwrap();
    let actual = std::fs::read_to_string("test-any-x86_64/usr/lib/extension-release.d/extension-release.test-any-x86_64").unwrap();
    cleanup(&name);
    assert_eq!(actual, expected);
}

#[test]
fn extension_release_any_os_aarch_arch() {
    let name = "test-any-aarch";
    let mut cmd = Command::cargo_bin("bakery").unwrap();
    cmd.arg(&name).arg("--os").arg("_any").arg("--arch").arg("aarch64");
    cmd.assert().success();

    let expected = std::fs::read_to_string("tests/samples/_any_os_aarch64_arch.txt").unwrap();
    let actual = std::fs::read_to_string("test-any-aarch64/usr/lib/extension-release.d/extension-release.test-any-aarch64").unwrap();
    cleanup(&name);
    assert_eq!(actual, expected);
}