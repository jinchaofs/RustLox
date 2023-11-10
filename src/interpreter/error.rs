use core::fmt;
use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct LoxError {
    line: usize,
    at: Option<String>,
    message: String,
}

impl LoxError {
    pub fn new(line: usize, at: Option<String>, message: &str) -> Self {
        LoxError {
            line,
            at,
            message: message.to_string(),
        }
    }
}

impl Display for LoxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            gen_error(self.line, self.at.clone(), self.message.clone())
        )
    }
}

fn gen_error(line: usize, at: Option<String>, message: String) -> String {
    format!(
        "[line {}] Error {}: {}",
        line,
        if let Some(at) = at {
            format!("'{}'", at)
        } else {
            "".to_string()
        },
        message
    )
    .into()
}
