use clap::Parser;
use cli::{CmdExecutor};
use cli::{Opts};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	tracing_subscriber::fmt::init();
	let opts = Opts::parse();

	opts.cmd.execute().await?;

	Ok(())
}
