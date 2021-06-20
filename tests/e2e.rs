use assert_cmd::Command;
use std::path;

#[test]
fn list_files() {
    let output = Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .arg("ls")
        .unwrap();

    let stdout = std::str::from_utf8(&output.stdout).unwrap();

    assert!(stdout.contains("Rust"), "should have a file for Rust");

    assert!(
        !stdout.contains("asdads"),
        "should not have a file for \"asdads\""
    );
}

#[test]
fn create_file() {
    let test_file_name = "test.gitignore";

    let output = Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .arg("new")
        .arg("ts")
        .arg("-n")
        .arg(test_file_name)
        .unwrap();

    let stdout = std::str::from_utf8(&output.stdout).unwrap();

    assert!(
        stdout.contains("Node"),
        "should have created a gitignore for Node"
    );

    let test_file_path = path::Path::new(test_file_name);

    assert!(test_file_path.is_file(), "should have created a gitignore");

    // Cleanup
    std::fs::remove_file(test_file_path).unwrap();
}
