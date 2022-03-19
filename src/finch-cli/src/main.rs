use std::env::current_dir;
use std::ffi::OsString;
use std::path::PathBuf;
use clap::{Parser, Subcommand, Args};
use tracing::trace;
use color_eyre::Result;

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
    /// The config variation to use.
    config_variant: Option<String>,
    /// Override with a custom config file.
    config_file: Option<PathBuf>,
}

#[tokio::main]
pub async fn main() -> Result<()> {
    let cli = Cli::parse();

    color_eyre::install()?;

    if cli.verbose {
        tracing_subscriber::fmt()
            .event_format(tracing_subscriber::fmt::format()
                .pretty())
            .init();
    }

    trace!("Succesfully registered eyre and tracing.");

    match &cli.command {
        Commands::Run(runnable) => {
            let (cfg, file_name): (finch_server::config::Config, OsString) = if let Some(path) = &runnable.config_file {
                (confy::load_path(path)?, path.as_os_str().to_owned())
            } else {
                let variant = runnable.config_variant.as_deref();
                let cfg = confy::load("finch", variant)?;
                let file_name = confy::get_configuration_file_path("finch", variant)?.as_os_str().to_owned();
                (cfg, file_name)
            };

            trace!("Loaded configuration {}", file_name.into_string().unwrap_or("".to_string()));

            finch_server::start(current_dir()?, cfg).await?;
        }
    }

    Ok(())
}
