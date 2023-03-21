use std::fmt::Display;

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
