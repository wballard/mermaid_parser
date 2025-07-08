//! Shared parser utilities to eliminate code duplication across diagram parsers

use crate::common::ast::AccessibilityInfo;
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

/// Parses common diagram directives (title, accTitle, accDescr)
/// Returns true if the line was handled, false if not recognized
pub fn parse_common_directives(
    line: &str,
    title: &mut Option<String>,
    accessibility: &mut AccessibilityInfo,
) -> bool {
    let trimmed = line.trim();

    // Handle lines that start with \t by stripping it first
    let effective_trimmed = if trimmed.starts_with("\\t") {
        trimmed.strip_prefix("\\t").unwrap_or(trimmed).trim()
    } else {
        trimmed
    };

    // Parse title directive
    if let Some(title_text) = effective_trimmed.strip_prefix("title ") {
        *title = Some(title_text.trim().to_string());
        return true;
    }

    // Parse accessibility title (handle both "accTitle:" and "accTitle ")
    if let Some(acc_title) = effective_trimmed.strip_prefix("accTitle:") {
        accessibility.title = Some(acc_title.trim().to_string());
        return true;
    }
    if let Some(acc_title) = effective_trimmed.strip_prefix("accTitle ") {
        accessibility.title = Some(acc_title.trim().to_string());
        return true;
    }

    // Parse accessibility description (handle both "accDescr:" and "accDescr ")
    if let Some(acc_desc) = effective_trimmed.strip_prefix("accDescr:") {
        accessibility.description = Some(acc_desc.trim().to_string());
        return true;
    }
    if let Some(acc_desc) = effective_trimmed.strip_prefix("accDescr ") {
        accessibility.description = Some(acc_desc.trim().to_string());
        return true;
    }

    false
}

/// Enhanced directive parser that handles multi-line accessibility descriptions
pub struct CommonDirectiveParser {
    in_multiline_desc: bool,
    multiline_content: Vec<String>,
}

impl CommonDirectiveParser {
    pub fn new() -> Self {
        Self {
            in_multiline_desc: false,
            multiline_content: Vec::new(),
        }
    }

    /// Parse common directives with multi-line support
    /// Returns true if line was handled
    pub fn parse_line(
        &mut self,
        line: &str,
        title: &mut Option<String>,
        accessibility: &mut AccessibilityInfo,
    ) -> bool {
        let trimmed = line.trim();

        // Handle lines that start with \t by stripping it first
        let effective_trimmed = if trimmed.starts_with("\\t") {
            trimmed.strip_prefix("\\t").unwrap_or(trimmed).trim()
        } else {
            trimmed
        };

        // Handle multi-line accessibility description end
        if self.in_multiline_desc && effective_trimmed == "}" {
            self.in_multiline_desc = false;
            if !self.multiline_content.is_empty() {
                accessibility.description = Some(self.multiline_content.join(" "));
                self.multiline_content.clear();
            }
            return true;
        }

        // Handle multi-line accessibility description content
        if self.in_multiline_desc {
            if !effective_trimmed.is_empty()
                && !effective_trimmed.starts_with("//")
                && !effective_trimmed.starts_with("%%")
            {
                self.multiline_content.push(effective_trimmed.to_string());
            }
            return true;
        }

        // Handle multi-line accessibility description start
        if effective_trimmed.starts_with("accDescr {") {
            self.in_multiline_desc = true;
            return true;
        }

        // Handle single-line directives
        parse_common_directives(line, title, accessibility)
    }
}

impl Default for CommonDirectiveParser {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod directive_tests {
    use super::*;

    #[test]
    fn test_parse_title_directive() {
        let mut title = None;
        let mut acc = AccessibilityInfo::default();

        assert!(parse_common_directives(
            "title My Diagram",
            &mut title,
            &mut acc
        ));
        assert_eq!(title, Some("My Diagram".to_string()));
    }

    #[test]
    fn test_parse_title_with_whitespace() {
        let mut title = None;
        let mut acc = AccessibilityInfo::default();

        assert!(parse_common_directives(
            "title   My Diagram   ",
            &mut title,
            &mut acc
        ));
        assert_eq!(title, Some("My Diagram".to_string()));
    }

    #[test]
    fn test_parse_accessibility_title_colon() {
        let mut title = None;
        let mut acc = AccessibilityInfo::default();

        assert!(parse_common_directives(
            "accTitle: Accessible Title",
            &mut title,
            &mut acc
        ));
        assert_eq!(acc.title, Some("Accessible Title".to_string()));
    }

    #[test]
    fn test_parse_accessibility_title_space() {
        let mut title = None;
        let mut acc = AccessibilityInfo::default();

        assert!(parse_common_directives(
            "accTitle Accessible Title",
            &mut title,
            &mut acc
        ));
        assert_eq!(acc.title, Some("Accessible Title".to_string()));
    }

    #[test]
    fn test_parse_accessibility_description_colon() {
        let mut title = None;
        let mut acc = AccessibilityInfo::default();

        assert!(parse_common_directives(
            "accDescr: Description text",
            &mut title,
            &mut acc
        ));
        assert_eq!(acc.description, Some("Description text".to_string()));
    }

    #[test]
    fn test_parse_accessibility_description_space() {
        let mut title = None;
        let mut acc = AccessibilityInfo::default();

        assert!(parse_common_directives(
            "accDescr Description text",
            &mut title,
            &mut acc
        ));
        assert_eq!(acc.description, Some("Description text".to_string()));
    }

    #[test]
    fn test_multiline_accessibility_description() {
        let mut parser = CommonDirectiveParser::new();
        let mut title = None;
        let mut acc = AccessibilityInfo::default();

        // Start multi-line
        assert!(parser.parse_line("accDescr {", &mut title, &mut acc));

        // Content lines
        assert!(parser.parse_line("This is a long", &mut title, &mut acc));
        assert!(parser.parse_line("accessibility description", &mut title, &mut acc));

        // End multi-line
        assert!(parser.parse_line("}", &mut title, &mut acc));

        assert_eq!(
            acc.description,
            Some("This is a long accessibility description".to_string())
        );
    }

    #[test]
    fn test_multiline_description_ignores_comments() {
        let mut parser = CommonDirectiveParser::new();
        let mut title = None;
        let mut acc = AccessibilityInfo::default();

        // Start multi-line
        assert!(parser.parse_line("accDescr {", &mut title, &mut acc));

        // Content with comments and empty lines
        assert!(parser.parse_line("This is content", &mut title, &mut acc));
        assert!(parser.parse_line("// This is a comment", &mut title, &mut acc));
        assert!(parser.parse_line("%% Another comment", &mut title, &mut acc));
        assert!(parser.parse_line("", &mut title, &mut acc));
        assert!(parser.parse_line("More content", &mut title, &mut acc));

        // End multi-line
        assert!(parser.parse_line("}", &mut title, &mut acc));

        assert_eq!(
            acc.description,
            Some("This is content More content".to_string())
        );
    }

    #[test]
    fn test_unrecognized_directive() {
        let mut title = None;
        let mut acc = AccessibilityInfo::default();

        assert!(!parse_common_directives(
            "unknown directive",
            &mut title,
            &mut acc
        ));
        assert_eq!(title, None);
        assert_eq!(acc.title, None);
        assert_eq!(acc.description, None);
    }

    #[test]
    fn test_parser_handles_single_line_directives() {
        let mut parser = CommonDirectiveParser::new();
        let mut title = None;
        let mut acc = AccessibilityInfo::default();

        assert!(parser.parse_line("title Test Title", &mut title, &mut acc));
        assert_eq!(title, Some("Test Title".to_string()));

        assert!(parser.parse_line("accTitle: Test Acc Title", &mut title, &mut acc));
        assert_eq!(acc.title, Some("Test Acc Title".to_string()));

        assert!(parser.parse_line("accDescr: Test Description", &mut title, &mut acc));
        assert_eq!(acc.description, Some("Test Description".to_string()));
    }

    #[test]
    fn test_parser_not_in_multiline_mode_initially() {
        let parser = CommonDirectiveParser::new();
        assert!(!parser.in_multiline_desc);
        assert!(parser.multiline_content.is_empty());
    }
}

/// Validates diagram header and handles first line processing
/// Returns Ok(true) if line was handled (skip to next line)
/// Returns Ok(false) if line should be processed by parser
/// Returns Err() if invalid header found
pub fn validate_diagram_header(
    line: &str,
    line_num: usize,
    expected_headers: &[&str],
    first_line_processed: &mut bool,
) -> crate::error::Result<bool> {
    use crate::error::ParseError;
    
    if *first_line_processed {
        return Ok(false);
    }
    
    let trimmed = line.trim();
    if should_skip_line(trimmed) {
        return Ok(true); // Skip this line
    }
    
    if !expected_headers.iter().any(|h| trimmed.starts_with(h)) {
        return Err(ParseError::SyntaxError {
            message: format!("Expected {} header", expected_headers[0]),
            expected: expected_headers.iter().map(|s| s.to_string()).collect(),
            found: trimmed.to_string(),
            line: line_num + 1,
            column: 1,
        });
    }
    
    *first_line_processed = true;
    Ok(true)
}

/// Checks if a line should be skipped (empty, comments)
pub fn should_skip_line(line: &str) -> bool {
    let trimmed = line.trim();
    trimmed.is_empty() || trimmed.starts_with("//") || trimmed.starts_with("%%")
}

#[cfg(test)]
mod header_validation_tests {
    use super::*;
    use crate::error::ParseError;

    #[test]
    fn test_validate_diagram_header_success() {
        let mut first_line = false;
        let result = validate_diagram_header(
            "architecture", 
            0, 
            &["architecture", "architecture-beta"], 
            &mut first_line
        );
        assert!(result.is_ok());
        assert!(result.unwrap());
        assert!(first_line);
    }

    #[test]
    fn test_validate_diagram_header_success_beta() {
        let mut first_line = false;
        let result = validate_diagram_header(
            "architecture-beta", 
            0, 
            &["architecture", "architecture-beta"], 
            &mut first_line
        );
        assert!(result.is_ok());
        assert!(result.unwrap());
        assert!(first_line);
    }

    #[test]
    fn test_validate_diagram_header_invalid() {
        let mut first_line = false;
        let result = validate_diagram_header(
            "invalid", 
            0, 
            &["architecture"], 
            &mut first_line
        );
        assert!(result.is_err());
        
        if let Err(ParseError::SyntaxError { message, expected, found, line, column }) = result {
            assert_eq!(message, "Expected architecture header");
            assert_eq!(expected, vec!["architecture".to_string()]);
            assert_eq!(found, "invalid");
            assert_eq!(line, 1);
            assert_eq!(column, 1);
        } else {
            panic!("Expected SyntaxError");
        }
    }

    #[test]
    fn test_validate_diagram_header_skips_processed() {
        let mut first_line = true;
        let result = validate_diagram_header(
            "anything", 
            0, 
            &["architecture"], 
            &mut first_line
        );
        assert!(result.is_ok());
        assert!(!result.unwrap()); // Should return false to indicate line should be processed
    }

    #[test]
    fn test_validate_diagram_header_skips_comments() {
        let mut first_line = false;
        
        // Test // comment
        let result = validate_diagram_header(
            "// this is a comment", 
            0, 
            &["architecture"], 
            &mut first_line
        );
        assert!(result.is_ok());
        assert!(result.unwrap()); // Should skip the line
        assert!(!first_line); // Should not mark as processed
        
        // Test %% comment
        let result = validate_diagram_header(
            "%% this is a comment", 
            0, 
            &["architecture"], 
            &mut first_line
        );
        assert!(result.is_ok());
        assert!(result.unwrap()); // Should skip the line
        assert!(!first_line); // Should not mark as processed
    }

    #[test]
    fn test_validate_diagram_header_skips_empty() {
        let mut first_line = false;
        
        // Test empty line
        let result = validate_diagram_header(
            "", 
            0, 
            &["architecture"], 
            &mut first_line
        );
        assert!(result.is_ok());
        assert!(result.unwrap()); // Should skip the line
        assert!(!first_line); // Should not mark as processed
        
        // Test whitespace only
        let result = validate_diagram_header(
            "   \t   ", 
            0, 
            &["architecture"], 
            &mut first_line
        );
        assert!(result.is_ok());
        assert!(result.unwrap()); // Should skip the line
        assert!(!first_line); // Should not mark as processed
    }

    #[test]
    fn test_should_skip_line() {
        assert!(should_skip_line(""));
        assert!(should_skip_line("   "));
        assert!(should_skip_line("\t\t"));
        assert!(should_skip_line("// comment"));
        assert!(should_skip_line("%% comment"));
        assert!(should_skip_line("  // spaced comment"));
        assert!(should_skip_line("  %% spaced comment"));
        assert!(!should_skip_line("actual content"));
        assert!(!should_skip_line("pie"));
        assert!(!should_skip_line("architecture"));
    }

    #[test]
    fn test_validate_multiple_headers() {
        let mut first_line = false;
        
        // Test state diagram v1
        let result = validate_diagram_header(
            "stateDiagram", 
            0, 
            &["stateDiagram", "stateDiagram-v2"], 
            &mut first_line
        );
        assert!(result.is_ok());
        assert!(result.unwrap());
        assert!(first_line);
        
        // Reset and test v2
        first_line = false;
        let result = validate_diagram_header(
            "stateDiagram-v2", 
            0, 
            &["stateDiagram", "stateDiagram-v2"], 
            &mut first_line
        );
        assert!(result.is_ok());
        assert!(result.unwrap());
        assert!(first_line);
    }

    #[test]
    fn test_validate_pie_header() {
        let mut first_line = false;
        let result = validate_diagram_header(
            "pie", 
            0, 
            &["pie"], 
            &mut first_line
        );
        assert!(result.is_ok());
        assert!(result.unwrap());
        assert!(first_line);
    }

    #[test]
    fn test_validate_header_with_content() {
        let mut first_line = false;
        let result = validate_diagram_header(
            "pie title Chart Name", 
            0, 
            &["pie"], 
            &mut first_line
        );
        assert!(result.is_ok());
        assert!(result.unwrap());
        assert!(first_line);
    }
}
