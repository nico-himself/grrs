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
    let mut cmd = Command::cargo_bin("rcli")?;

    cmd.arg("grep")
        .arg("foobar")
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
    cmd.arg("grep").arg("test").arg(file.path());
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
    cmd.arg("grep").arg("test").arg(file.path());
    cmd.assert().success().stdout(predicate::str::is_empty());

    Ok(())
}

#[test]
// -i flag for case insensitive search
fn case_insensitive_search() -> Result<(), Box<dyn std::error::Error>> {
    let file = assert_fs::NamedTempFile::new("sample.txt")?;
    file.write_str("Some Text\nsome text\nAnother Text")?;

    let mut cmd = Command::cargo_bin("grrs")?;
    cmd.arg("grep").arg("-i").arg("some").arg(file.path());
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Some Text\nsome text"));

    Ok(())
}

#[test]
// -v flag for invert match
fn print_line_numbers() -> Result<(), Box<dyn std::error::Error>> {
    let file = assert_fs::NamedTempFile::new("sample.txt")?;
    file.write_str("first line\nrelevant line\nlast line")?;

    let mut cmd = Command::cargo_bin("grrs")?;
    cmd.arg("grep").arg("-n").arg("line").arg(file.path());
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("2:relevant line"));

    Ok(())
}

#[test]
// -o flag for only matching part of the line
fn only_matching_part_of_line() -> Result<(), Box<dyn std::error::Error>> {
    let file = assert_fs::NamedTempFile::new("sample.txt")?;
    file.write_str("line with the secret code 12345")?;

    let mut cmd = Command::cargo_bin("grrs")?;
    cmd.arg("grep")
        .arg("-o")
        .arg("secret code")
        .arg(file.path());
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("secret code"));

    Ok(())
}

#[test]
// -v flag for invert match
fn invert_match() -> Result<(), Box<dyn std::error::Error>> {
    let file = assert_fs::NamedTempFile::new("sample.txt")?;
    file.write_str("line one\nline two\nsomething else")?;

    let mut cmd = Command::cargo_bin("grrs")?;
    cmd.arg("grep").arg("-v").arg("line").arg(file.path());
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("something else"));

    Ok(())
}
