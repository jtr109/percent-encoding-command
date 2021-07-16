use anyhow::Result;
use assert_cmd::Command;

#[test]
fn encode_works_with_piped_input() -> Result<()> {
    let mut cmd = Command::cargo_bin("pct")?;
    let assert = cmd
        .pipe_stdin(std::path::Path::new("./tests/fixtures/hello.txt"))?
        .assert();
    assert
        .success()
        .stdout("hello%20world%20%E4%BD%A0%E5%A5%BD");
    Ok(())
}
