use clap::Parser;
use std::fmt;
use std::fmt::Formatter;
use std::str::FromStr;

#[derive(Debug, Parser)]
#[command(name="cli", version, author, about, long_about=None)]
pub struct Opts {
	#[command(subcommand)]
	pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
pub enum SubCommand {
	#[command(name="csv", about="展示csv或者转换csv到其他格式")]
	Csv(CsvOpts),

	#[command(name="genpass", about="生成随机密码")]
	GenPass(GenPassOpts)
}

#[derive(Debug, Parser)]
pub struct GenPassOpts {
	#[arg(short, long, default_value_t = 16)]
	pub length: u8,
	#[arg(long, default_value_t = true)]
	pub uppercase: bool,
	#[arg(long, default_value_t = true)]
	pub lowercase: bool,
	#[arg(long, default_value_t = true)]
	pub numbers: bool,
	#[arg(long, default_value_t = true)]
	pub symbol: bool

}
#[derive(Debug, Clone, Copy)]
pub enum OutputFormat {
	Json,
	Yaml,
	// Toml,
}

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

fn verify_input_file(filename: &str) -> Result<String, String> {
	if std::path::Path::new(filename).exists() {
		Ok(filename.to_string())
	} else {
		Err(format!("File {} does not exist", filename))
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

impl fmt::Display for OutputFormat {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		write!(f, "{}", Into::<&str>::into(*self))
	}
}