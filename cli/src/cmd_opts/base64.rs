use std::fmt;
use std::str::FromStr;
use clap::Parser;
use crate::Base64Subcommand::{Decode, Encode};
use crate::CmdExecutor;
use super::csv::verify_input_file;
use crate::process;

#[derive(Debug, Parser)]
pub enum Base64Subcommand {
	#[command(name="encode", about="base64编码")]
	Encode(Base64EncodeOpts),
	#[command(name="decode", about="base64解码")]
	Decode(Base64DecodeOpts),
}

impl CmdExecutor for Base64Subcommand {
	async fn execute(self) -> anyhow::Result<()> {
		match self {
			Encode(opts) => {
				process::process_encode(&opts.input, opts.format)
			},
			Decode(opts) => {
				process::process_decode(&opts.input, opts.format)
			}
		}
	}
}

#[derive(Debug, Parser)]
pub struct Base64EncodeOpts {
	#[arg(long, default_value = "-", value_parser=verify_input_file)]
	pub input: String,
	#[arg(long, default_value = "standard", value_parser=parse_base64_format)]
	pub format: Base64Format,
}

#[derive(Debug, Parser)]
pub struct Base64DecodeOpts {
	#[arg(long, default_value = "-")]
	input: String,
	#[arg(long, default_value = "standard", value_parser=parse_base64_format)]
	format: Base64Format,
}

#[derive(Debug,Clone, Copy)]
pub enum Base64Format {
	Standard,
	UrlSafe,
}

fn parse_base64_format(format: &str) -> Result<Base64Format, anyhow::Error> {
	format.parse()
}

impl FromStr for Base64Format {
	type Err = anyhow::Error;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"standard" => Ok(Base64Format::Standard),
			"urlsafe" => Ok(Base64Format::UrlSafe),
			_ => Err(anyhow::anyhow!("Invalid format")),
		}
	}
}

impl From<Base64Format> for &'static str {
	fn from(format: Base64Format) -> Self {
		match format {
			Base64Format::Standard => "standard",
			Base64Format::UrlSafe => "urlsafe",
		}
	}
}

impl fmt::Display for Base64Format {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", Into::<&str>::into(*self))
	}
}