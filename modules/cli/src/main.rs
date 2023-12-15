mod scan_file;

use std::{
    env,
    ffi::OsString,
};

use clap::{
    Parser,
    Subcommand,
};
use log::LevelFilter;

#[derive(clap::Args)]
pub struct Signature {
    #[clap(short, long)]
    pub malware_dir: String,
    #[clap(short, long)]
    pub out_path: String,
}

// #[derive(clap::Args)]
// pub struct Evaluate {
//     #[clap(short, long)]
//     pub sig_path: String,
//     #[clap(value_name = "PATH")]
//     pub file_path: String,
// }

#[derive(Subcommand)]
enum Commands {
    /// Build malware signature set
    Signature {
        #[clap(short, long)]
        malware_dir: String,
        #[clap(short, long)]
        out_path: String,
    },
    /// Evaluate a suspected file
    Evaluate {
        #[clap(short, long)]
        sig_path: String,
        #[clap(value_name = "PATH")]
        file_path: String,
    },
}

#[derive(Parser)]
#[clap(author, about)]
pub struct Cli {
    /// Increase log message verbosity
    #[clap(short, long, parse(from_occurrences))]
    log_level: u32,
    #[clap(short, long)]
    /// Print version information
    version: bool,
    #[clap(subcommand)]
    commands: Commands,
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
    let args = Cli::parse_from(get_command_line_args());
    //print build info

    let level_filter = match args.log_level {
        0 => LevelFilter::Off,
        1 => LevelFilter::Error,
        2 => LevelFilter::Warn,
        3 => LevelFilter::Info,
        4 => LevelFilter::Debug,
        _ => LevelFilter::Trace,
    };

    //env_logger::Builder::new().filter_level(level_filter).init();
    env_logger::Builder::new().filter_level(LevelFilter::Trace).init();

    match args.commands {
        Commands::Signature { malware_dir, out_path } => {
            signatures::Signatures::create_msig_file(malware_dir.as_str(), out_path.as_str())?
        },
        Commands::Evaluate { file_path, sig_path } => {
            scan_file::scan_file(file_path.as_str(), sig_path.as_str())?
        },
    }

    Ok(())
}
