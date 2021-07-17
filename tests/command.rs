use anyhow::Result;
use assert_cmd::Command;
use tempfile::{NamedTempFile, TempDir};

const PLAIN_TEXT: &str = "hello world 你好";
const ENCODED: &str = "hello%20world%20%E4%BD%A0%E5%A5%BD";
const COMMAND: &str = "pct";

#[test]
fn success_encode_with_piped_input() -> Result<()> {
    let temp_file = NamedTempFile::new()?;
    std::fs::write(temp_file.path(), PLAIN_TEXT)?;
    let mut cmd = Command::cargo_bin(COMMAND)?;
    let assert = cmd.pipe_stdin(temp_file.path())?.assert();
    assert.success().stdout(ENCODED);
    Ok(())
}

#[test]
fn success_encode_with_stdin() -> Result<()> {
    let mut cmd = Command::cargo_bin(COMMAND)?;
    let assert = cmd.write_stdin(PLAIN_TEXT).assert();
    assert.success().stdout(ENCODED);
    Ok(())
}

#[test]
fn success_encode_with_input_file() -> Result<()> {
    let tmp_file = NamedTempFile::new()?;
    std::fs::write(tmp_file.path(), PLAIN_TEXT)?;
    let mut cmd = Command::cargo_bin(COMMAND)?;
    let assert = cmd.arg("--input").arg(tmp_file.path()).assert();
    assert.success().stdout(ENCODED);
    Ok(())
}

#[test]
fn failure_encode_with_file_does_not_exist() -> Result<()> {
    let tmp_dir = TempDir::new()?;
    let file_path = tmp_dir.into_path().join("not-exists.txt");
    let mut cmd = Command::cargo_bin(COMMAND)?;
    let assert = cmd.arg("--input").arg(file_path).assert();
    assert.failure();
    Ok(())
}

#[test]
fn success_encode_with_relative_input() -> Result<()> {
    let tmp_file = NamedTempFile::new()?;
    std::env::set_current_dir(tmp_file.path().parent().expect("tmp dir not exists"))?;
    std::fs::write(
        tmp_file.path().file_name().expect("unable to get filename"),
        PLAIN_TEXT,
    )?;
    let mut cmd = Command::cargo_bin(COMMAND)?;
    let assert = cmd.arg("--input").arg(tmp_file.path()).assert();
    assert.success().stdout(ENCODED);
    Ok(())
}

// accept output flag
// relative output path
// overwrite output path
