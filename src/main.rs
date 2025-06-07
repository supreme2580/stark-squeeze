use clap::Parser;
use stark_squeeze::cli::main_menu;

const APP_NAME: &str = "StarkSqueeze CLI";
const APP_ABOUT: &str = "Interact with StarkSqueeze";

/// CLI arguments for StarkSqueeze
#[derive(Parser)]
#[command(name = APP_NAME, about = APP_ABOUT)]
#[command(arg_required_else_help = true)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

/// Commands for the StarkSqueeze CLI
#[derive(clap::Subcommand)]
enum Commands {
    /// Upload data to StarkNet
    Upload {
        /// Path to the file to upload
        #[arg(short, long)]
        file: Option<String>,
    },
    /// Retrieve data from StarkNet
    Retrieve {
        /// The upload ID or hash to retrieve
        #[arg(short, long)]
        id: Option<String>,
    },
    /// List all uploaded data
    List,
}

#[tokio::main]
async fn main() {
    // Run the interactive menu
    main_menu().await;
}
