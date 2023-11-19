use crate::scanner::Scanner;
use std::error::Error;
use std::fs;
use std::io::Write;
use std::sync::OnceLock;

mod scanner;
mod token;
mod utils;

static HAD_ERROR: OnceLock<bool> = OnceLock::new();

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();
    match args.len() {
        2 => Lox::default().run_file(&args[1])?,
        3.. => {
            println!("Usage lox-rs [script]");
            std::process::exit(64);
        }
        _ => Lox::default().run_prompt()?
    }
    Ok(())
}

pub fn error(line: usize, message: &str) -> Result<(), Box<dyn Error>> {
    report(line, "", message)?;
    Ok(())
}

fn report(line: usize, location: &str, message: &str) -> Result<(), Box<dyn Error>> {
    println!("[line {line}] Error{location}: {message}");

    HAD_ERROR.set(false).unwrap();
    Ok(())
}

#[derive(Default)]
struct Lox {}

impl Lox {
    fn run_prompt(&mut self) -> Result<(), Box<dyn Error>> {
        loop {
            print!("> ");
            std::io::stdout().flush()?;

            let mut input = String::new();
            match std::io::stdin().read_line(&mut input) {
                Ok(_) => {
                    if input.trim().is_empty() {
                        break;
                    }
                    self.run(&input)?;

                    HAD_ERROR.set(false).unwrap();
                }
                Err(error) => println!("{error}"),
            }
        }
        Ok(())
    }

    fn run_file(&self, file_path: &str) -> Result<(), Box<dyn Error>> {
        let source = fs::read_to_string(file_path)?;
        self.run(&source)?;

        if HAD_ERROR.get().is_some_and(|e| *e) {
            std::process::exit(65);
        }
        Ok(())
    }

    fn run(&self, source: &str) -> Result<(), Box<dyn Error>> {
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens();

        for token in tokens {
            println!("{token}");
        }
        Ok(())
    }
}
