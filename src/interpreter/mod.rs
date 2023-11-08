mod scanner;
mod token;
mod token_type;

use scanner::Scanner;

pub struct Interpreter {}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {}
    }
    pub fn run(&self, source: String) {
        let scanner = Scanner::new(source);
        let tokens = scanner.scan();
        println!("Tokens: {:?}", tokens);
    }
}
