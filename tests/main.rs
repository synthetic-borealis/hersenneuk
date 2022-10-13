use std::io::Write;
use std::process::{Command, Stdio};
use std::{env, path};

#[test]
fn cli_hello_world_fixed_block() {
    let exe_name = env::var("CARGO_PKG_NAME").unwrap();

    #[cfg(target_os = "windows")]
    let exe_name = exe_name + ".exe";

    #[cfg(target_os = "macos")]
    let exe_name = exe_name + ".dmg";

    let target_path =
        env::var("CARGO_MANIFEST_DIR").unwrap() + "/target/debug/" + exe_name.as_str();
    let target_path = path::Path::new(&target_path);
    let output = Command::new(target_path)
        .args(["test_assets/hello_world.bf"])
        .output()
        .unwrap();
    let output = String::from_utf8(output.stdout).unwrap();

    assert_eq!(output.trim(), "Hello World!");
}

#[test]
fn cli_hello_world_dynamic_block() {
    let exe_name = env::var("CARGO_PKG_NAME").unwrap();

    #[cfg(target_os = "windows")]
    let exe_name = exe_name + ".exe";

    #[cfg(target_os = "macos")]
    let exe_name = exe_name + ".dmg";

    let target_path =
        env::var("CARGO_MANIFEST_DIR").unwrap() + "/target/debug/" + exe_name.as_str();
    let target_path = path::Path::new(&target_path);
    let output = Command::new(target_path)
        .args(["test_assets/hello_world.bf", "-D"])
        .output()
        .unwrap();
    let output = String::from_utf8(output.stdout).unwrap();

    assert_eq!(output.trim(), "Hello World!");
}

#[test]
fn cli_user_input_fixed_block() {
    let exe_name = env::var("CARGO_PKG_NAME").unwrap();

    #[cfg(target_os = "windows")]
    let exe_name = exe_name + ".exe";

    #[cfg(target_os = "macos")]
    let exe_name = exe_name + ".dmg";

    let target_path =
        env::var("CARGO_MANIFEST_DIR").unwrap() + "/target/debug/" + exe_name.as_str();
    let target_path = path::Path::new(&target_path);
    let mut child = Command::new(target_path)
        .args(["test_assets/user_input.bf"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    // ensure child_stdin gets dropped
    {
        let child_stdin = child.stdin.as_mut().unwrap();
        child_stdin.write_all(b"c\n").unwrap();
    }

    let stdout = String::from_utf8(child.wait_with_output().unwrap().stdout).unwrap();
    assert_eq!(stdout.trim(), "c");
}
