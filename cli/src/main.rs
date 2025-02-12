use clap::Parser;
use cli::{Opts, SubCommand};
use cli::process;

fn main() -> anyhow::Result<()> {
	let opts = Opts::parse();
	match opts.cmd {
		SubCommand::Csv(opts) => {
			let output = if let Some(output) = opts.output {
				output.clone()
			} else {
				format!("output.{}", opts.format)
			};
			process::process_csv(&opts.input, output, opts.format)?
		},
		SubCommand::GenPass(opts) => {
			process::process_gen_password(opts.length, opts.uppercase, opts.lowercase, opts.numbers, opts.symbol)?
		}
	}

	Ok(())
}


