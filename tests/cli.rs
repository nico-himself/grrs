use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions
use std::process::Command; // Run programs

#[test]
/**
 * This test evaluates the following:
 * 1. Run `grrs` with the arguments `"test"` and the path to a file that does not exist
 * 2. Assert that the command fails
 * 3. Assert that the command's stderr contains the string "No such file or directory"
 */
fn file_dne() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("grrs")?;

    cmd.arg("foobar")
        .arg("test/file_dne.txt")
        .assert()
        .failure()
        .stderr(predicate::str::contains("No such file or directory"));

    Ok(())
}

use assert_fs::prelude::*;

#[test]
/**
 * This test evaluates the following:
 * 1. Create a temporary file with the contents "A test\nActual content\nMore content\nAnother test"
 * 2. Run `grrs` with the arguments `"test"` and the path to the file
 * 3. Assert that the command succeeds
 *
 * @see https://docs.rs/assert_fs/1.0.1/assert_fs/macro.assert.html
 */
fn find_content_in_file() -> Result<(), Box<dyn std::error::Error>> {
    let file = assert_fs::NamedTempFile::new("sample.txt")?;
    file.write_str("A test\nActual content\nMore content\nAnother test")?;

    let mut cmd = Command::cargo_bin("grrs")?;
    cmd.arg("test").arg(file.path());
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("A test\nAnother test"));

    Ok(())
}

#[test]
/**
 * This test evaluates the following:
 * 1. Create a temporary file with an empty string (i.e. no content)
 * 2. Run `grrs` with the arguments `"test"` and the path to the file
 * 3. Assert that the command succeeds
 */
fn an_empty_str() -> Result<(), Box<dyn std::error::Error>> {
    // Create a file inside of `./tests` named `empty.txt` with no content
    let file = assert_fs::NamedTempFile::new("empty.txt")?;
    file.write_str("")?; // Explicitly write an empty string to ensure the file is created

    // Run `grrs` with the arguments `"test"` and the path to the file
    let mut cmd = Command::cargo_bin("grrs")?;
    cmd.arg("test").arg(file.path());
    cmd.assert().success().stdout(predicate::str::is_empty());

    Ok(())
}
