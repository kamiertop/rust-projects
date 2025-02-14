use clap::Parser;
use crate::CmdExecutor;
use crate::process::process_gen_password;

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
	pub symbol: bool,
	#[arg(long, help = "是否展示密码强度, 默认为不展示" ,default_value_t = false)]
	pub show_strength: bool,
}

impl CmdExecutor for GenPassOpts {
	async fn execute(self) -> anyhow::Result<()> {
		let pwd = process_gen_password(self.length, self.uppercase, self.lowercase, self.numbers, self.symbol, self.show_strength)?;
		println!("{}", pwd);

		Ok(())
	}
}