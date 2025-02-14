use std::io::{Read};
use base64::Engine;
use base64::engine::general_purpose::{STANDARD, URL_SAFE_NO_PAD};
use crate::Base64Format;
use super::utils::get_reader;
pub fn process_encode(input: &str, format: Base64Format) -> anyhow::Result<()> {
	let mut reader = get_reader(input)?;
	let mut buf:Vec<u8> = Vec::new();

 	reader.read_to_end(&mut buf)?;

	match format {
		Base64Format::Standard => {
			println!("{}",STANDARD.encode(buf));
		}
		Base64Format::UrlSafe => {
			println!("{}",URL_SAFE_NO_PAD.encode(buf));
		}
	}

	Ok(())
}

pub fn process_decode(input: &str,  format: Base64Format) -> anyhow::Result<()> {
	let mut reader = get_reader(input)?;
	let mut buf = String::new();

 	reader.read_to_string(&mut buf)?;

	let buf = buf.trim();

	// TODO: decode结果可能不是字符串
	let result = match format {
		Base64Format::Standard => {
			STANDARD.decode(buf)?
		},
		Base64Format::UrlSafe => {
			URL_SAFE_NO_PAD.decode(buf)?
		}
	};

	println!("{}", String::from_utf8(result)?.trim());

	Ok(())
}

