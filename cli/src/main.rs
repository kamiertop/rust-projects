use clap::Parser;
use cli::{process_csv_to_json, Opts, SubCommand};

fn main() {
	let opts = Opts::parse();
	match opts.cmd {
		SubCommand::Csv(opts) => {
			process_csv_to_json(&opts.input, &opts.output).unwrap();
		}
	}
}


