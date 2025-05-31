use stark_squeeze::cli;
use clap::{Parser, Subcommand};

const APP_NAME: &str = "StarkSqueeze CLI";
const APP_ABOUT: &str = "Interact with StarkSqueeze";
/// CLI arguments for StarkSqueeze
#[derive(Parser, Debug)]
#[command(name = APP_NAME, about = APP_ABOUT)]
struct CliArgs {
    #[command(subcommand)]
    command: Option<Commands>,
}

/// Commands for the StarkSqueeze CLI
#[derive(Subcommand, Debug)]
enum Commands {
    /// Upload data to StarkNet
    Upload {
        #[arg(long)]
        disable_file_size_limit: bool,
    },

    /// Retrieve data from StarkNet
    Retrieve,
    /// List all uploaded data
    List,
}

#[tokio::main]
async fn main() {
    let args = CliArgs::parse();

    match args.command {
        Some(Commands::Upload { disable_file_size_limit }) => {
            cli::upload_data_cli(disable_file_size_limit).await;
        }
        // Some(Commands::Upload) => cli::upload_data_cli().await,
        Some(Commands::Retrieve) => cli::retrieve_data_cli().await,
        Some(Commands::List) => cli::list_all_uploads().await,
        None => cli::main_menu().await,
    }
}
