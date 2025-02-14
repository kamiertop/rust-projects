use clap::Parser;
use cli::process;
use cli::{Base64Subcommand, Opts, SubCommand::*};
use cli::process::process_http_serve;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	tracing_subscriber::fmt::init();
	let opts = Opts::parse();
	match opts.cmd {
		Csv(opts) => {
			let output = if let Some(output) = opts.output {
				output.clone()
			} else {
				format!("output.{}", opts.format)
			};
			process::process_csv(&opts.input, output, opts.format)?
		},
		GenPass(opts) => {
			let pwd = process::process_gen_password(opts.length, opts.uppercase, opts.lowercase, opts.numbers, opts.symbol, opts.show_strength)?;
			println!("{}", pwd)
		}
		Base64(sub_cmd) =>{
			match sub_cmd {
				Base64Subcommand::Encode(opts) => {
					process::process_encode(&opts.input, opts.format)?
				},
				Base64Subcommand::Decode(opts) => {
					process::process_decode(&opts.input, opts.format)?
				}
			}
		}

		Http(opts) => {
			process_http_serve(&opts.dir, opts.port).await?;
		}
	}

	Ok(())
}
