use anyhow::Result;
use assert_cmd::Command;

const PLAIN_TEXT: &str = "hello world 你好";
const EXPECTED_ENCODED: &str = "hello%20world%20%E4%BD%A0%E5%A5%BD";

#[test]
fn encode_works_with_piped_input() -> Result<()> {
    let mut cmd = Command::cargo_bin("pct")?;
    let assert = cmd
        .pipe_stdin(std::path::Path::new("./tests/fixtures/hello.txt"))?
        .assert();
    assert.success().stdout(EXPECTED_ENCODED);
    Ok(())
}

#[test]
fn encode_works_with_stdin() -> Result<()> {
    let mut cmd = Command::cargo_bin("pct")?;
    let assert = cmd.write_stdin(PLAIN_TEXT).assert();
    assert.success().stdout(EXPECTED_ENCODED);
    Ok(())
}
