use clap::{Args, Parser, Subcommand};
use color_eyre::eyre::eyre;
use color_eyre::{Result, Section};
use confy::ConfyError;
use finch_server::config::Config;
use std::env::current_dir;
use std::ffi::OsString;
use std::path::PathBuf;
use tracing::trace;

#[derive(Parser)]
#[clap(author, version, about, long_about = None, propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
    #[clap(short, long)]
    verbose: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Run the server.
    Run(Run),
}

#[derive(Args)]
struct Run {
    /// The path to start the server in.
    path: Option<PathBuf>,
    /// Override with a custom config file.
    config_file: Option<PathBuf>,
}

#[tokio::main]
pub async fn main() -> Result<()> {
    let cli = Cli::parse();

    color_eyre::install()?;

    if cli.verbose {
        tracing_subscriber::fmt()
            .event_format(tracing_subscriber::fmt::format().pretty())
            .init();
    }

    trace!("Succesfully registered eyre and tracing.");

    match &cli.command {
        Commands::Run(runnable) => {
            let (cfg, file_name): (Result<Config, ConfyError>, OsString) =
                if let Some(path) = &runnable.config_file {
                    (confy::load_path(path), path.as_os_str().to_owned())
                } else {
                    let cfg = confy::load("finch", None);
                    let file_name = confy::get_configuration_file_path("finch", None)?
                        .as_os_str()
                        .to_owned();
                    (cfg, file_name)
                };

            let file_name = file_name.into_string().unwrap_or("".to_string());
            let cfg = cfg
                .map_err(|e| {
                    eyre!("Failed to load configuration")
                        .with_error(|| e)
                        .with_note(|| {
                            "Looking for the configuration file: ".to_owned() + &file_name.clone()
                        })
                })
                .with_suggestion(|| {
                    "Run with --help to see the options for specifying a configuration file."
                })
                .with_suggestion(|| "Ensure the file exists.")
                .with_suggestion(|| "Ensure the TOML values are valid.");

            trace!("Loaded configuration {}", &file_name);

            finch_server::start(current_dir()?, cfg?).await?;
        }
    }

    Ok(())
}
