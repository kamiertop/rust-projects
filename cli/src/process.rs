use csv::Reader;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
struct Player {
	name: String,
	position:String,
	#[serde(rename = "DOB")]
	dob: String,
	nationality:String,
	#[serde(rename="Kit Number")]
	kit_number: u8
}

pub fn process_csv_to_json(input: &str, output: &str) -> anyhow::Result<()> {
	let mut res = Vec::with_capacity(128);
	let mut reader = Reader::from_path(input)?;
	for result in reader.deserialize() {
		let record: Player = result?;
		res.push(record);
	}
	let json = serde_json::to_string_pretty(&res)?;
	fs::write(output, json)?;
	Ok(())
}