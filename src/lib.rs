use std::path::Path;

#[macro_use]
extern crate log;
extern crate env_logger;

mod interpreter;
use interpreter::Interpreter;
pub struct RustLox {
    interpreter: Interpreter,
}

impl RustLox {
    pub fn new() -> Self {
        RustLox {
            interpreter: Interpreter::new(),
        }
    }
    pub fn run_with_prompt(&self, prompt: String) {
        info!("Run with prompt: {prompt}");
        self.interpreter.run(prompt);
    }

    pub fn run_with_file(&self, file_path: &Path) {
        info!("Run with file_path: {:?}", file_path);
    }
}
