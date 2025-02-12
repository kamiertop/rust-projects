use std::{fmt, path};
use std::fmt::Formatter;
use std::str::FromStr;
use clap::Parser;

#[derive(Debug, Parser)]
pub struct CsvOpts {
	#[arg(short, long, value_parser=verify_input_file)]
	pub input: String,

	#[arg(short, long)]
	pub output: Option<String>,

	#[arg(long, default_value = "json", value_parser=parse_format)]
	pub format: OutputFormat,

	#[arg(short, long,  default_value_t = ',')]
	pub delimiter: char,

	#[arg(long, default_value_t = true)]
	pub header: bool
}

pub fn verify_input_file(filename: &str) -> Result<String, &'static str> {
	if filename == "-" || path::Path::new(filename).exists() {
		Ok(filename.to_string())
	} else {

		Err("File does not exist")
		// Err(format!("File {} does not exist", filename))
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn test_verify_input_file() {
		assert_eq!(verify_input_file("-"), Ok("-".into()));
		assert_eq!(verify_input_file("*"), Err("File does not exist"));
		assert_eq!(verify_input_file("Cargo.toml"),Ok("Cargo.toml".into()));
		assert_eq!(verify_input_file("not-exist"),Err("File does not exist"));
	}
}

fn parse_format(format: &str) -> Result<OutputFormat, anyhow::Error> {
	format.parse::<OutputFormat>()
}

impl From<OutputFormat> for &'static str {
	fn from(format: OutputFormat) -> Self {
		match format {
			OutputFormat::Json => "json",
			OutputFormat::Yaml => "yaml",
			// OutputFormat::Toml => "toml",
		}
	}
}

impl FromStr for OutputFormat {
	type Err = anyhow::Error;
	fn from_str(value: &str) -> Result<Self, Self::Err> {
		match value.to_lowercase().as_str() {
			"json" => Ok(OutputFormat::Json),
			"yaml" => Ok(OutputFormat::Yaml),
			// "toml" => Ok(OutputFormat::Toml),
			v => anyhow::bail!("不支持的格式:{}",v),
		}
	}
}

#[derive(Debug, Clone, Copy)]
pub enum OutputFormat {
	Json,
	Yaml,
}
impl fmt::Display for OutputFormat {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		write!(f, "{}", Into::<&str>::into(*self))
	}
}
