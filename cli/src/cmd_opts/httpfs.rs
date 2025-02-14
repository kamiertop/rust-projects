use std::path::PathBuf;
use clap::Parser;

#[derive(Debug, Parser)]
pub struct HttpServeOpts {
	#[arg(long, help = "目录")]
	pub dir: PathBuf,

	#[arg(long, help = "端口号")]
	pub port: Option<u16>,
}