//! Shared parser utilities to eliminate code duplication across diagram parsers

use chumsky::prelude::*;

/// Common token types shared across parsers
#[derive(Debug, Clone, PartialEq)]
pub enum CommonToken {
    NewLine,
    Comment(String),
}

/// Parse comments that start with %% or //
pub fn parse_comment<'src>(
) -> impl Parser<'src, &'src str, CommonToken, extra::Err<Simple<'src, char>>> + Clone {
    choice((
        just("%%").then(none_of('\n').repeated()),
        just("//").then(none_of('\n').repeated()),
    ))
    .map(|_| CommonToken::Comment("".to_string()))
}

/// Parse whitespace (spaces and tabs)
pub fn parse_whitespace<'src>(
) -> impl Parser<'src, &'src str, (), extra::Err<Simple<'src, char>>> + Clone {
    one_of(" \t").repeated().ignored()
}

/// Parse one or more whitespace characters
pub fn parse_whitespace_required<'src>(
) -> impl Parser<'src, &'src str, (), extra::Err<Simple<'src, char>>> + Clone {
    one_of(" \t").repeated().at_least(1).ignored()
}

/// Parse newlines (both \n and \r\n)
pub fn parse_newline<'src>(
) -> impl Parser<'src, &'src str, CommonToken, extra::Err<Simple<'src, char>>> + Clone {
    choice((just("\n"), just("\r\n"))).map(|_| CommonToken::NewLine)
}

/// Macro to create standardized parse functions with consistent error handling
#[macro_export]
macro_rules! create_parser_fn {
    ($vis:vis fn $name:ident($input:ident: &str) -> Result<$output:ty> {
        lexer: $lexer:expr,
        parser: $parser:expr,
        diagram_type: $diagram_type:literal
    }) => {
        $vis fn $name($input: &str) -> $crate::error::Result<$output> {
            let tokens = $lexer()
                .parse($input)
                .into_result()
                .map_err(|e| $crate::error::ParseError::SyntaxError {
                    message: format!("Failed to tokenize {} diagram", $diagram_type),
                    expected: vec![],
                    found: format!("{:?}", e),
                    line: 0,
                    column: 0,
                })?;

            let result = $parser()
                .parse(&tokens[..])
                .into_result()
                .map_err(|e| $crate::error::ParseError::SyntaxError {
                    message: format!("Failed to parse {} diagram", $diagram_type),
                    expected: vec![],
                    found: format!("{:?}", e),
                    line: 0,
                    column: 0,
                })?;

            Ok(result)
        }
    };
}

/// Macro to create standardized parse functions for line-based parsers
#[macro_export]
macro_rules! create_line_parser_fn {
    ($vis:vis fn $name:ident($input:ident: &str) -> Result<$output:ty> {
        parser_fn: $parser_fn:expr,
        diagram_type: $diagram_type:literal
    }) => {
        $vis fn $name($input: &str) -> $crate::error::Result<$output> {
            let tokens = $parser_fn()
                .parse($input)
                .into_result()
                .map_err(|e| $crate::error::ParseError::SyntaxError {
                    message: format!("Failed to tokenize {} diagram", $diagram_type),
                    expected: vec![],
                    found: format!("{:?}", e),
                    line: 0,
                    column: 0,
                })?;

            $parser_fn(&tokens)
        }
    };
}
