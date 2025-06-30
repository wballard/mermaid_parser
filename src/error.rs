//! Error types for the Mermaid parser

use std::fmt;

/// Result type alias for parser operations
pub type Result<T> = std::result::Result<T, ParseError>;

/// Errors that can occur during parsing
#[derive(Debug, Clone, PartialEq)]
pub enum ParseError {
    /// The input is empty or contains only whitespace/comments
    EmptyInput,

    /// Unknown diagram type encountered
    UnknownDiagramType(String),

    /// Diagram type is recognized but not yet supported
    UnsupportedDiagramType(String),

    /// Lexical analysis error
    LexError {
        message: String,
        line: usize,
        column: usize,
    },

    /// Syntax parsing error
    SyntaxError {
        message: String,
        expected: Vec<String>,
        found: String,
        line: usize,
        column: usize,
    },

    /// Semantic error (valid syntax but invalid meaning)
    SemanticError { message: String, context: String },

    /// I/O error when reading input
    IoError(String),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::EmptyInput => {
                write!(f, "Input is empty or contains no valid diagram content")
            }
            ParseError::UnknownDiagramType(diagram_type) => {
                write!(f, "Unknown diagram type: '{}'", diagram_type)
            }
            ParseError::UnsupportedDiagramType(diagram_type) => {
                write!(f, "Diagram type '{}' is not yet supported", diagram_type)
            }
            ParseError::LexError {
                message,
                line,
                column,
            } => {
                write!(
                    f,
                    "Lexical error at line {}, column {}: {}",
                    line, column, message
                )
            }
            ParseError::SyntaxError {
                message,
                expected,
                found,
                line,
                column,
            } => {
                write!(
                    f,
                    "Syntax error at line {}, column {}: {}. Expected one of: [{}], but found: '{}'",
                    line,
                    column,
                    message,
                    expected.join(", "),
                    found
                )
            }
            ParseError::SemanticError { message, context } => {
                write!(f, "Semantic error in {}: {}", context, message)
            }
            ParseError::IoError(message) => {
                write!(f, "I/O error: {}", message)
            }
        }
    }
}

impl std::error::Error for ParseError {}

impl From<std::io::Error> for ParseError {
    fn from(error: std::io::Error) -> Self {
        ParseError::IoError(error.to_string())
    }
}

// Note: Chumsky error conversion will be implemented later
// when we have actual parsers using the library

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let error = ParseError::UnknownDiagramType("invalid".to_string());
        assert_eq!(error.to_string(), "Unknown diagram type: 'invalid'");

        let error = ParseError::EmptyInput;
        assert_eq!(
            error.to_string(),
            "Input is empty or contains no valid diagram content"
        );

        let error = ParseError::SyntaxError {
            message: "Unexpected token".to_string(),
            expected: vec!["identifier".to_string(), "number".to_string()],
            found: "symbol".to_string(),
            line: 5,
            column: 10,
        };
        assert!(error.to_string().contains("line 5, column 10"));
        assert!(error.to_string().contains("identifier, number"));
    }

    #[test]
    fn test_error_equality() {
        let error1 = ParseError::EmptyInput;
        let error2 = ParseError::EmptyInput;
        assert_eq!(error1, error2);

        let error3 = ParseError::UnknownDiagramType("test".to_string());
        assert_ne!(error1, error3);
    }
}

