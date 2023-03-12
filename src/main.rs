#![feature(let_chains)]

use std::{
    io::{stdin, stdout, Write},
    path::Path,
};

fn main() {
    let mut args = std::env::args().skip(1);

    match args.len() {
        2.. => {
            println!("Usage: ferox [script]");
            std::process::exit(64);
        }
        1 => {
            if let Some(script_file_path_string) = args.next() {
                println!("{script_file_path_string}");
                run_script_file(Path::new(&script_file_path_string));
            }
        }
        _ => run_prompt(),
    };
}

fn run_script_file(script_file_path: &Path) {
    if let Ok(script_file_content) = std::fs::read_to_string(script_file_path) {
        run(&script_file_content);
    }
}

fn run_prompt() {
    let mut line = String::new();

    loop {
        print!("> ");
        stdout().flush().unwrap();

        if stdin().read_line(&mut line).is_err() || line.is_empty() {
            break;
        }

        run(&line);

        line.clear();
    }
}

fn run(script: &str) {
    println!("{script}");
}
