use std::{fs::File, io::Read};

use anyhow::bail;
use cli::Command;

mod cli;
mod commands;

pub use crate::cli::Cli;

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

            let res = commands::view(file, rows, column_names);
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

            let res = commands::filter(file, column, equals, separator)?;
            println!("{res}");
        }
    }

    Ok(())
}

fn read_file(path: &str) -> std::io::Result<String> {
    let mut buf = String::new();
    let mut file = File::open(path)?;

    file.read_to_string(&mut buf)
        .expect("Cannot read from the file");
    Ok(buf)
}
