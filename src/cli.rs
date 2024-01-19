use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(about = "Query CSV files", name = "csvq")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    View {
        path: String,
        #[arg(short, long)]
        rows: Option<usize>,
    },
    Filter {
        path: String,
        #[arg(short, long)]
        column: String,
        #[arg(long)]
        equals: String,
        #[arg(long)]
        separator: Option<String>,
    },
}
