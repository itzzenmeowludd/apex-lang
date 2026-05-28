use std::fmt;

#[derive(Debug, Clone)]
pub struct ApexError {
    pub message: String,
    pub line: usize,
    pub column: usize,
    pub file: Option<String>,
}

impl ApexError {
    pub fn new(message: impl Into<String>, line: usize, column: usize) -> Self {
        Self { message: message.into(), line, column, file: None }
    }

    pub fn with_file(mut self, file: impl Into<String>) -> Self {
        self.file = Some(file.into());
        self
    }
}

impl fmt::Display for ApexError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.file {
            Some(file) => write!(f, "{file}:{}:{}: {}", self.line, self.column, self.message),
            None => write!(f, "{}:{}: {}", self.line, self.column, self.message),
        }
    }
}

impl std::error::Error for ApexError {}

pub type Result<T> = std::result::Result<T, ApexError>;
