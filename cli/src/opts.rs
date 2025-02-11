use clap::Parser;

#[derive(Debug, Parser)]
#[command(name="cli", version, author, about, long_about=None)]
pub struct Opts {
	#[command(subcommand)]
	pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
pub enum SubCommand {
	#[command(name="csv", about="展示csv或者转换csv到其他格式")]
	Csv(CsvOpts)
}

#[derive(Debug, Parser)]
pub struct CsvOpts {
	#[arg(short, long, value_parser=verify_input_file)]
	pub input: String,

	#[arg(short, long,default_value="output.json")]
	pub output: String,

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

