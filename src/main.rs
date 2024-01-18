use std::fmt::Write;
use std::{fs::File, io::Read};

use anyhow::bail;
use clap::{Parser, Subcommand};

const DEFAULT_NUMBER_OF_ROWS: usize = 5;
const SEPARATOR: &str = ",";

#[derive(Subcommand)]
enum Commands {
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
    },
}

#[derive(Parser)]
#[command(about = "Query CSV files", name = "csvq")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

fn read_file(path: &str) -> std::io::Result<String> {
    let mut buf = String::new();
    let mut file = File::open(path)?;

    file.read_to_string(&mut buf)?;
    Ok(buf)
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::View { path, rows } => {
            let file = match read_file(&path) {
                Ok(f) => f,
                Err(_) => bail!("No such file or directory"),
            };

            let res = file
                .lines()
                .skip(1)
                .take(rows.unwrap_or(DEFAULT_NUMBER_OF_ROWS))
                .fold(String::new(), |mut acc, l| {
                    writeln!(acc, "{}", l).unwrap();
                    acc
                });

            println!("{res}");
        }
        Commands::Filter {
            path,
            column,
            equals,
        } => {
            let file = match read_file(&path) {
                Ok(f) => f,
                Err(_) => bail!("No such file or directory"),
            };

            // TODO: Look for solutions that don't involve calling `lines()`
            let col_idx = match file.lines().next() {
                Some(columns) => {
                    match columns
                        .split(SEPARATOR)
                        .enumerate()
                        .find(|(_, col)| *col == column)
                    {
                        Some((idx, _)) => idx,
                        None => bail!("Column does not exist"),
                    }
                }
                None => bail!("The file is empty"),
            };

            let res = file
                .lines()
                .filter(|row| row.split(SEPARATOR).nth(col_idx).unwrap() == equals)
                .fold(String::new(), |mut acc, l| {
                    writeln!(acc, "{}", l).unwrap();
                    acc
                });

            println!("{res}");
        }
    }

    Ok(())
}
