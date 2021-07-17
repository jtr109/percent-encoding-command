use anyhow::Result;
use assert_cmd::Command;
use tempfile::NamedTempFile;

const PLAIN_TEXT: &str = "hello world 你好";
const EXPECTED_ENCODED: &str = "hello%20world%20%E4%BD%A0%E5%A5%BD";
const COMMAND: &str = "pct";

#[test]
fn encode_works_with_piped_input() -> Result<()> {
    let temp_file = NamedTempFile::new()?;
    std::fs::write(temp_file.path(), PLAIN_TEXT)?;
    let mut cmd = Command::cargo_bin(COMMAND)?;
    let assert = cmd.pipe_stdin(temp_file.path())?.assert();
    assert.success().stdout(EXPECTED_ENCODED);
    Ok(())
}

#[test]
fn encode_works_with_stdin() -> Result<()> {
    let mut cmd = Command::cargo_bin(COMMAND)?;
    let assert = cmd.write_stdin(PLAIN_TEXT).assert();
    assert.success().stdout(EXPECTED_ENCODED);
    Ok(())
}

#[test]
fn encode_works_with_input_file() -> Result<()> {
    let tmp_file = NamedTempFile::new()?;
    std::fs::write(tmp_file.path(), PLAIN_TEXT)?;
    let mut cmd = Command::cargo_bin(COMMAND)?;
    let assert = cmd.arg("--input").arg(tmp_file.path()).assert();
    assert.success().stdout(EXPECTED_ENCODED);
    Ok(())
}

// accept input flag
// accept output flag
// relative input path
// relative output path
// overwrite output path
