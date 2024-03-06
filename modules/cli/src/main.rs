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
use signatures::MsetSerializer;

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
#[derive(clap::Args)]
struct Compile {
    #[clap(short, long)]
    raw: bool,
    #[clap(short, long)]
    dir: String,
    #[clap(short, long)]
    out_path: String,
}

#[derive(clap::Args)]
struct Unpack {
    #[clap(short, long)]
    mset: String,
    #[clap(short, long)]
    out_dir: String,
}

#[derive(Subcommand)]
pub enum SignatureCommand {
    Compile(Compile),
    Unpack(Unpack),
    //List(List),
}

#[derive(Subcommand)]
enum Commands {
    /// Build malware signature set
    #[command(subcommand)]
    Signature(SignatureCommand),
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
    #[arg(short, long, action = clap::ArgAction::Count)]
    log_level: u8,
    #[arg(short = 'V', long)]
    /// Print version information
    version: bool,
    #[command(subcommand)]
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

    let _level_filter = match args.log_level {
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
        Commands::Signature ( signature_command ) => {
            match signature_command {
                SignatureCommand::Compile(args) => {
                    if args.raw {
                        let malset = signatures::MalwareSet::from_dir(args.dir.as_str())?;
                        let ser = MsetSerializer::new(&malset);
                        ser.serialize(&args.out_path)?;
                    } else {
                        //let malset = signatures::MalwareSet::from_signatures(args.dir.as_str())?;
                        //let ser = MsetSerializer::new(&malset);
                        //ser.serialize(&args.out_path)?;
                    }
                }
                SignatureCommand::Unpack(args) => {
                    let malset = signatures::get_malware_set(args.mset.as_str())?;

                    if std::path::Path::new(&args.out_dir).exists() {
                        let md = std::fs::metadata(&args.out_dir)?;
                        if md.is_file() {
                            //error
                            todo!()
                        }

                        let _ = std::fs::remove_dir(&args.out_dir);
                    }

                    std::fs::create_dir(&args.out_dir).unwrap();
                    malset.unpack_to_dir(args.out_dir);
                }
            }

        },
        Commands::Evaluate { file_path, sig_path } => {
            scan_file::scan_file(file_path.as_str(), sig_path.as_str())?
        },
    }

    Ok(())
}
