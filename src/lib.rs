use anyhow::Result;
use percent_encoding::{percent_decode_str, utf8_percent_encode, NON_ALPHANUMERIC};
use std::io::{Read, Write};

pub fn encode(reader: &mut dyn Read, writer: &mut dyn Write) -> Result<()> {
    let mut input = String::new();
    reader.read_to_string(&mut input)?;
    let encoded = utf8_percent_encode(&input, NON_ALPHANUMERIC).to_string();
    writer.write_all(encoded.as_bytes())?;
    Ok(())
}

pub fn decode(reader: &mut dyn Read, writer: &mut dyn Write) -> Result<()> {
    let mut input = String::new();
    reader.read_to_string(&mut input)?;
    let decoded = percent_decode_str(&input).decode_utf8()?.to_string();
    writer.write_all(decoded.as_bytes())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_works() {
        let mut reader = "hello world 你好".as_bytes();
        let mut writer = vec![];
        encode(&mut reader, &mut writer).expect("unexpected encode error");
        let result = std::str::from_utf8(&writer).unwrap();
        let expected = "hello%20world%20%E4%BD%A0%E5%A5%BD";
        assert_eq!(result, expected);
    }

    #[test]
    fn decode_works() {
        let mut reader = "hello%20world%20%E4%BD%A0%E5%A5%BD".as_bytes();
        let mut writer = vec![];
        decode(&mut reader, &mut writer).expect("unexpected decode error");
        let result = std::str::from_utf8(&writer).unwrap();
        let expected = "hello world 你好";
        assert_eq!(result, expected);
    }
}
