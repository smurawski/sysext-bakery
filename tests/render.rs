mod cli;
use cli::*;

pub fn cleanup_template(name: &str) {
    std::fs::remove_file(name).unwrap();
}

#[test]
fn test_render_from_cli_args() {
    let name = "./test-render-cli-args.txt";
    let mut cmd = create_command();
    cmd.arg("render")
        .arg("-t")
        .arg("./tests/templates/sample_template.txt")
        .arg("-o")
        .arg(&name)
        .arg("documentation_url=https://somewhere.over.the.rainbow.dev")
        .arg("executable_path=/usr/bin/some-executable")
        .arg("wants=network-online.target");
    cmd.assert().success();

    let expected = std::fs::read_to_string("./tests/samples/test-render-basic.txt").unwrap();
    let actual = std::fs::read_to_string(&name).unwrap();

    cleanup_template(&name);
    assert_eq!(actual, expected);
}

#[test]
fn test_render_from_config() {
    let name = "./test-render-config.txt";
    let mut cmd = create_command();
    cmd.arg("render")
        .arg("-c")
        .arg("./tests/config.yml");
    cmd.assert().success();

    let expected = std::fs::read_to_string("./tests/samples/test-render-basic.txt").unwrap();
    let actual = std::fs::read_to_string(&name).unwrap();

    cleanup_template(&name);
    assert_eq!(actual, expected);
}