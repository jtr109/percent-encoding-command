use anyhow::Result;
use percent_encoding::{percent_decode_str, utf8_percent_encode, NON_ALPHANUMERIC};
use std::io::{Read, Write};

pub fn encode(reader: &mut dyn Read, writer: &mut dyn Write) -> Result<()> {
    let mut input = String::new();
    reader.read_to_string(&mut input)?;
    let encoded = utf8_percent_encode(&input, NON_ALPHANUMERIC).to_string();
    writer.write(encoded.as_bytes())?;
    Ok(())
}

pub fn decode(reader: &mut dyn Read, writer: &mut dyn Write) -> Result<()> {
    let mut input = String::new();
    reader.read_to_string(&mut input)?;
    let decoded = percent_decode_str(&input).decode_utf8()?.to_string();
    writer.write(decoded.as_bytes())?;
    Ok(())
}
