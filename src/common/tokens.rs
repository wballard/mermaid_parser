//! Common token types used across multiple parsers

/// Common tokens that appear in multiple Mermaid diagram types
#[derive(Debug, Clone, PartialEq)]
pub enum CommonToken {
    /// Newline character(s)
    NewLine,
    
    /// End of file
    Eof,
    
    /// Title declaration
    Title(String),
    
    /// Accessibility title
    AccTitle,
    
    /// Accessibility title value
    AccTitleValue(String),
    
    /// Accessibility description
    AccDescr,
    
    /// Accessibility description value
    AccDescrValue(String),
    
    /// Multiline accessibility description
    AccDescrMultiline(String),
    
    /// Section declaration
    Section(String),
    
    /// Comment (ignored during parsing)
    Comment(String),
    
    /// Whitespace (usually ignored)
    Whitespace,
    
    /// Invalid token
    Invalid(char),
}

/// Span information for error reporting
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
    pub line: usize,
    pub column: usize,
}

impl Span {
    pub fn new(start: usize, end: usize, line: usize, column: usize) -> Self {
        Self { start, end, line, column }
    }
    
    pub fn single(position: usize, line: usize, column: usize) -> Self {
        Self { start: position, end: position + 1, line, column }
    }
}

/// Token with location information
#[derive(Debug, Clone, PartialEq)]
pub struct SpannedToken<T> {
    pub token: T,
    pub span: Span,
}

impl<T> SpannedToken<T> {
    pub fn new(token: T, span: Span) -> Self {
        Self { token, span }
    }
}