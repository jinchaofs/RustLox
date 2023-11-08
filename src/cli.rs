use std::{
    io::{self, Write},
    path::PathBuf,
};

use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Optional file path
    file_path: Option<PathBuf>,
}

impl Cli {
    pub fn run() {
        println!("Welcome to the Rust Lox 0.1.0 interpreter.");
        let cli = Cli::parse();
        let lox = rustlox::RustLox::new();
        // Run with File
        if let Some(file_path) = cli.file_path.as_deref() {
            lox.run_with_file(file_path);
            return;
        }
        // Run with prompt
        Self::readline(&lox);
    }

    fn readline(lox: &rustlox::RustLox) {
        loop {
            print!(">>> ");
            io::stdout().flush().expect("Failed to flush");

            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            if input.is_empty() {
                continue;
            }

            lox.run_with_prompt(input.clone());
        }
    }
}
