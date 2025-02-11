mod opts;
mod process;

pub use opts::{Opts, SubCommand,CsvOpts};

pub use process::process_csv_to_json;