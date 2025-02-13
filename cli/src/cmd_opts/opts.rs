use clap::Parser;
use crate::cmd_opts::base64::Base64Subcommand;
use super::csv::CsvOpts;
use crate::{GenPassOpts, HttpServeOpts};

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
	GenPass(GenPassOpts),

	#[command(name="base64",subcommand)]
	Base64(Base64Subcommand),

	#[command(name="http" ,about="使用http服务一个目录")]
	Http(HttpServeOpts)
}

#[allow(async_fn_in_trait)]
pub trait CmdExecutor {
	async fn execute(self) -> anyhow::Result<()>;
}

impl CmdExecutor for SubCommand {
	async fn execute(self) -> anyhow::Result<()> {
		match self {
			SubCommand::Csv(opts) => opts.execute().await,
			SubCommand::GenPass(opts) => opts.execute().await,
			SubCommand::Base64(opts) => opts.execute().await,
			SubCommand::Http(opts) => opts.execute().await,
		}
	}
}