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
        #[arg(long)]
        column_names: bool,
    },
    Filter {
        path: String,
        #[arg(short, long)]
        column: String,
        #[arg(short, long)]
        equals: String,
        #[arg(short, long)]
        separator: Option<String>,
    },
}
