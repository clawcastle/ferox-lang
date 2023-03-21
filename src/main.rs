#![feature(let_chains)]

mod error;
mod scanner;
mod token;

use std::{
    io::{stdin, stdout, Write},
    path::Path,
};

use error::FeroxError;
use scanner::Scanner;

fn main() -> Result<(), FeroxError> {
    let mut args = std::env::args().skip(1);

    let mut ferox = Ferox::default();

    match args.len() {
        2.. => {
            println!("Usage: ferox [script]");
            std::process::exit(64);
        }
        1 => {
            if let Some(script_file_path_string) = args.next() {
                println!("{script_file_path_string}");
                ferox.run_script_file(Path::new(&script_file_path_string))
            } else {
                unreachable!()
            }
        }
        _ => ferox.run_prompt(),
    }
}

#[derive(Default)]
struct Ferox {
    had_error: bool,
    scanner: Scanner,
}

impl Ferox {
    fn run_script_file(&mut self, script_file_path: &Path) -> Result<(), FeroxError> {
        if let Ok(script_file_content) = std::fs::read_to_string(script_file_path) {
            self.run(&script_file_content)
        } else {
            Err(FeroxError::InvalidFilePathError {
                file_path: script_file_path.to_str().unwrap().to_string(),
            })
        }
    }

    fn run_prompt(&mut self) -> Result<(), FeroxError> {
        let mut line = String::new();

        loop {
            print!("> ");
            stdout().flush().unwrap();

            if stdin().read_line(&mut line).is_err() || line.is_empty() {
                break;
            }

            self.run(&line)?;

            line.clear();
            self.had_error = false;
        }

        Ok(())
    }

    fn run(&mut self, script: &str) -> Result<(), FeroxError> {
        println!("{script}");

        let mut scanner = Scanner::new(script);
        let tokens = scanner.scan_tokens().unwrap();

        println!("{:?}", &tokens);

        Ok(())
    }
}
