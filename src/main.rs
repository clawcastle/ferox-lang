#![feature(let_chains)]

mod token;

use std::{
    fmt::Display,
    io::{stdin, stdout, Write},
    path::Path,
};

use token::{Token, TokenType};

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

    fn run(&self, script: &str) -> Result<(), FeroxError> {
        println!("{script}");

        Ok(())
    }
}

struct Scanner<'a> {
    source: &'a str,
    tokens: Vec<Token>,
    line_number: usize,
    current: Option<String>,
}

impl<'a> Scanner<'a> {
    fn new(source: &'a str) -> Self {
        Self {
            source,
            tokens: vec![],
            line_number: 0,
            current: None,
        }
    }

    fn scan_tokens(&mut self) -> Result<(), FeroxError> {
        while let Some(c) = self.source.chars().next() {
            // Handle single character tokens
            if self.current.is_none() && let Ok(token_type) = TokenType::try_from(c) {
                self.tokens.push(Token::new(token_type, c.to_string(), self.line_number))
            } else {

            }
        }

        self.tokens
            .push(Token::new(TokenType::Eof, String::new(), self.line_number));

        Ok(())
    }
}

#[derive(Debug)]
pub enum FeroxError {
    SyntaxError {
        error_description: String,
        line_number: usize,
    },
    InvalidFilePathError {
        file_path: String,
    },
}

impl Display for FeroxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FeroxError::SyntaxError {
                error_description,
                line_number,
            } => f.write_fmt(format_args!(
                "At line {}: {}",
                line_number, error_description
            )),
            FeroxError::InvalidFilePathError { file_path } => f.write_fmt(format_args!(
                "The path '{}' does not point to a valid Ferox script file.",
                file_path
            )),
        }
    }
}
