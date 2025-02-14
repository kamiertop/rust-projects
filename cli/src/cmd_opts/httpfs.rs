use std::path::PathBuf;
use clap::Parser;
use crate::CmdExecutor;

#[derive(Debug, Parser)]
pub struct HttpServeOpts {
	#[arg(long, help = "目录")]
	pub dir: PathBuf,

	#[arg(long, help = "端口号")]
	pub port: Option<u16>,
}

impl CmdExecutor for HttpServeOpts {
	async fn execute(self) -> anyhow::Result<()> {
		crate::process::process_http_serve(&self.dir, self.port).await
	}
}