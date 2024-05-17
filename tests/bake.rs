mod cli;
use cli::*;

#[test]
fn test_cli_bake_subcommand_fails_on_missing_name() {
    let mut cmd = create_command();
    cmd.arg("bake");
    cmd.assert().failure();
}

#[test]
fn test_cli_bake_subcommand_succeeds_with_os() {
    let random_name = format!("test-sample-{}", random_string());

    let mut cmd = create_command();
    cmd.arg("bake").arg(&random_name).arg("--os").arg("debian");
    cmd.assert().success();

    cleanup(&random_name);
}

#[test]
fn test_cli_bake_subcommand_fails_on_unknown_arch() {
    let random_name = format!("test-sample-{}", random_string());

    let mut cmd = create_command();
    cmd.arg("bake")
        .arg(&random_name)
        .arg("--arch")
        .arg("unknown");
    cmd.assert().failure();
}


#[test]
fn test_bake_extension_release_any_os() {
    let name = "test-any";
    let mut cmd = create_command();
    cmd.arg("bake").arg(&name).arg("--os").arg("_any");
    cmd.assert().success();

    let actual_path = format!(
        "{}/usr/lib/extension-release.d/extension-release.{}",
        &name, &name
    );
    let expected = std::fs::read_to_string("tests/samples/_any_os.txt").unwrap();
    let actual = std::fs::read_to_string(&actual_path).unwrap();

    cleanup(&name);
    assert_eq!(actual, expected);
}

#[test]
fn test_bake_extension_release_debian_os() {
    let name = "test-debian";
    let mut cmd = create_command();
    cmd.arg("bake").arg(&name).arg("--os").arg("debian");
    cmd.assert().success();

    let actual_path = format!(
        "{}/usr/lib/extension-release.d/extension-release.{}",
        &name, &name
    );
    let expected = std::fs::read_to_string("tests/samples/debian_os.txt").unwrap();
    let actual = std::fs::read_to_string(&actual_path).unwrap();

    cleanup(&name);
    assert_eq!(actual, expected);
}

#[test]
fn test_bake_extension_release_any_os_x86_64_arch() {
    let name = "test-any-x86_64";
    let mut cmd = create_command();
    cmd.arg("bake")
        .arg(&name)
        .arg("--os")
        .arg("_any")
        .arg("--arch")
        .arg("x86-64");
    cmd.assert().success();

    let actual_path = format!(
        "{}/usr/lib/extension-release.d/extension-release.{}",
        &name, &name
    );
    let expected = std::fs::read_to_string("tests/samples/_any_os_x86_64_arch.txt").unwrap();
    let actual = std::fs::read_to_string(&actual_path).unwrap();

    cleanup(&name);
    assert_eq!(actual, expected);
}

#[test]
fn test_bake_extension_release_any_os_aarch64_arch() {
    let name = "test-any-aarch64";
    let mut cmd = create_command();
    cmd.arg("bake")
        .arg(&name)
        .arg("--os")
        .arg("_any")
        .arg("--arch")
        .arg("aarch64");
    cmd.assert().success();

    let actual_path = format!(
        "{}/usr/lib/extension-release.d/extension-release.{}",
        &name, &name
    );
    let expected = std::fs::read_to_string("tests/samples/_any_os_aarch64_arch.txt").unwrap();
    let actual = std::fs::read_to_string(&actual_path).unwrap();
    cleanup(&name);
    assert_eq!(actual, expected);
}
