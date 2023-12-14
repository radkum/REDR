mod scan_file;

use std::{
    env,
    ffi::OsString,
};

use clap::Parser;
use log::LevelFilter;

#[derive(Parser)]
#[clap(author, about)]
pub struct Commands {
    /// Increase log message verbosity
    #[clap(short, long, parse(from_occurrences))]
    pub log_level: u32,
    #[clap(value_name = "PATH")]
    pub file_path: String,
    #[clap(short, long)]
    /// Print version information
    pub version: bool,
}

fn get_command_line_args() -> impl Iterator<Item = OsString> {
    let mut args = env::args_os().collect::<Vec<_>>();
    // Clap returns exit code 2 (parsing error), if no arguments provided.
    // Invoking with --help provides same output as with no arguments, but the exit code is 0
    // (success).
    if args.len() == 1 {
        args.push(OsString::from("--help"));
    }
    args.into_iter()
}

pub fn main() -> anyhow::Result<()> {
    let args = Commands::parse_from(get_command_line_args());
    //print build info

    let level_filter = match args.log_level {
        0 => LevelFilter::Off,
        1 => LevelFilter::Error,
        2 => LevelFilter::Warn,
        3 => LevelFilter::Info,
        4 => LevelFilter::Debug,
        _ => LevelFilter::Trace,
    };

    env_logger::Builder::new().filter_level(level_filter).init();

    scan_file::scan_file(args.file_path.as_str())?;

    Ok(())
}
