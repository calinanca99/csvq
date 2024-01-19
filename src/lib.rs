use std::{fmt::Write, fs::File, io::Read};

use anyhow::bail;
use cli::Command;

mod cli;

pub use crate::cli::Cli;

const DEFAULT_NUMBER_OF_ROWS: usize = 5;
const SEPARATOR: &str = ",";

pub fn run(cli: Cli) -> anyhow::Result<()> {
    match cli.command {
        Command::View {
            path,
            rows,
            column_names,
        } => {
            let file = match read_file(&path) {
                Ok(f) => f,
                Err(_) => bail!("No such file or directory"),
            };

            let rows_to_skip = if column_names { 0 } else { 1 };

            let res = file
                .lines()
                .skip(rows_to_skip)
                .take(rows.unwrap_or(DEFAULT_NUMBER_OF_ROWS))
                .fold(String::new(), |mut acc, l| {
                    writeln!(acc, "{}", l).unwrap();
                    acc
                });

            println!("{res}");
        }
        Command::Filter {
            path,
            column,
            equals,
            separator,
        } => {
            let file = match read_file(&path) {
                Ok(f) => f,
                Err(_) => bail!("No such file or directory"),
            };
            let separator = separator.unwrap_or(SEPARATOR.to_string());

            // TODO: Look for solutions that don't involve calling `lines()`
            let col_idx = match file.lines().next() {
                Some(columns) => {
                    match columns
                        .split(&separator)
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
                .filter(|row| row.split(&separator).nth(col_idx).unwrap() == equals)
                .fold(String::new(), |mut acc, l| {
                    writeln!(acc, "{}", l).unwrap();
                    acc
                });

            println!("{res}");
        }
    }

    Ok(())
}

fn read_file(path: &str) -> std::io::Result<String> {
    let mut buf = String::new();
    let mut file = File::open(path)?;

    file.read_to_string(&mut buf)?;
    Ok(buf)
}
