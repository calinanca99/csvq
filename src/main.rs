use std::fmt::Write;
use std::{fs::File, io::Read};

use anyhow::bail;
use clap::{Parser, Subcommand};

#[derive(Subcommand)]
enum Commands {
    View {
        path: String,
        #[arg(short, long)]
        rows: Option<usize>,
    },
}

#[derive(Parser)]
#[command(about = "Query CSV files", name = "csvq")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::View { path, rows } => {
            let mut buf = String::new();
            let mut file = match File::open(path) {
                Ok(f) => f,
                Err(_) => {
                    bail!("No such file or directory")
                }
            };

            file.read_to_string(&mut buf)?;

            let rows = rows.unwrap_or(5);

            let res = buf
                .lines()
                .skip(1)
                .take(rows)
                .fold(String::new(), |mut acc, l| {
                    writeln!(acc, "{}", l).unwrap();
                    acc
                });

            println!("{res}");
        }
    }

    Ok(())
}
