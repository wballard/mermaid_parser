//! # Mermaid Parser
//!
//! A Rust parser for Mermaid diagrams using the Chumsky parsing library.
//! This crate focuses on parsing Mermaid syntax into Abstract Syntax Trees (ASTs)
//! without rendering graphics.
//!
//! ## Supported Diagram Types
//!
//! - **Sankey Diagrams**: Flow data visualization
//! - **Timeline Diagrams**: Chronological event sequences  
//! - **User Journey**: User experience mapping with satisfaction scores
//! - **Sequence Diagrams**: Message passing between actors
//! - **Class Diagrams**: Object-oriented relationships
//! - **State Diagrams**: State machine representations
//! - **Flowcharts**: General-purpose flow diagrams
//! - And many more...
//!
//! ## Example Usage
//!
//! ```rust
//! use mermaid_parser::{parse_diagram, DiagramType};
//!
//! let input = r#"
//! sankey-beta
//! A,B,10
//! B,C,5
//! "#;
//!
//! match parse_diagram(input) {
//!     Ok(DiagramType::Sankey(diagram)) => {
//!         println!("Found {} nodes and {} links",
//!                  diagram.nodes.len(), diagram.links.len());
//!     }
//!     Ok(_) => println!("Parsed a different diagram type"),
//!     Err(e) => eprintln!("Parse error: {}", e),
//! }
//! ```

pub mod common;
pub mod error;
pub mod parsers;

pub use common::ast::{DiagramType, KeyType, CardinalityValue};
pub use error::{ParseError, Result};

/// Parse a Mermaid diagram from text input
///
/// This function automatically detects the diagram type and parses
/// it into the appropriate AST representation.
///
/// # Arguments
///
/// * `input` - The Mermaid diagram text to parse
///
/// # Returns
///
/// Returns a `Result` containing the parsed diagram or a parse error.
///
/// # Example
///
/// ```rust
/// use mermaid_parser::parse_diagram;
///
/// let result = parse_diagram("sankey-beta\nA,B,10");
/// assert!(result.is_ok());
/// ```
pub fn parse_diagram(input: &str) -> Result<DiagramType> {
    // Detect diagram type from input
    let diagram_type = detect_diagram_type(input)?;

    // Parse based on detected type
    match diagram_type {
        "sankey" => parsers::sankey::parse(input).map(DiagramType::Sankey),
        "architecture" => parsers::architecture::parse(input).map(DiagramType::Architecture),
        "block" => parsers::block::parse(input).map(DiagramType::Block),
        "c4" => parsers::c4::parse(input).map(DiagramType::C4),
        "class" => parsers::class::parse(input).map(DiagramType::Class),
        "er" => parsers::er::parse(input).map(DiagramType::Er),
        "flowchart" => parsers::flowchart::parse(input).map(DiagramType::Flowchart),
        "gantt" => parsers::gantt::parse(input).map(DiagramType::Gantt),
        "git" => parsers::git::parse(input).map(DiagramType::Git),
        "kanban" => parsers::kanban::parse(input).map(DiagramType::Kanban),
        "mindmap" => parsers::mindmap::parse(input).map(DiagramType::Mindmap),
        "timeline" => parsers::timeline::parse(input).map(DiagramType::Timeline),
        "journey" => parsers::journey::parse(input).map(DiagramType::Journey),
        "sequence" => parsers::sequence::parse(input).map(DiagramType::Sequence),
        // Add other parsers as they're implemented
        _ => {
            // Try misc parser as a fallback for unknown diagram types
            parsers::misc::parse(input).map(DiagramType::Misc)
        }
    }
}

/// Detect the type of Mermaid diagram from input text
///
/// This function examines the first non-comment, non-whitespace line
/// to determine the diagram type.
fn detect_diagram_type(input: &str) -> Result<&'static str> {
    let first_line = input
        .lines()
        .map(|line| line.trim())
        .find(|line| !line.is_empty() && !line.starts_with("//") && !line.starts_with('#'))
        .ok_or(ParseError::EmptyInput)?;

    let first_word = first_line
        .split_whitespace()
        .next()
        .ok_or(ParseError::EmptyInput)?
        .to_lowercase()
        .trim_end_matches(':')
        .to_string();

    match first_word.as_str() {
        "sankey-beta" => Ok("sankey"),
        "timeline" => Ok("timeline"),
        "journey" => Ok("journey"),
        "sequencediagram" => Ok("sequence"),
        "classdiagram" => Ok("class"),
        "statediagram" | "statediagram-v2" => Ok("state"),
        "flowchart" | "graph" => Ok("flowchart"),
        "gantt" | "gantttestclick" => Ok("gantt"),
        "pie" => Ok("pie"),
        "gitgraph" => Ok("misc"), // Alternative gitGraph syntax handled by misc parser
        "info" => Ok("misc"),
        "erdiagram" | "erdiagramtitletext" => Ok("er"),
        "c4context" | "c4container" | "c4component" | "c4dynamic" | "c4deployment" => Ok("c4"),
        "mindmap" => Ok("mindmap"),
        "quadrant" => Ok("quadrant"),
        "xychart" => Ok("xychart"),
        "kanban" => Ok("kanban"),
        "block" => Ok("block"),
        "block-beta" => Ok("block"),
        "architecture" => Ok("architecture"),
        "packet" => Ok("packet"),
        "requirement" => Ok("requirement"),
        "sankey" => Ok("sankey"),
        "treemap" => Ok("treemap"),
        "radar" => Ok("radar"),
        _ => Ok("misc"), // Unknown diagram types are handled by misc parser
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diagram_type_detection() {
        assert_eq!(detect_diagram_type("sankey-beta\nA,B,10"), Ok("sankey"));
        assert_eq!(
            detect_diagram_type("timeline\ntitle My Day"),
            Ok("timeline")
        );
        assert_eq!(
            detect_diagram_type("journey\ntitle My Journey"),
            Ok("journey")
        );
        assert_eq!(
            detect_diagram_type("sequenceDiagram\nAlice->Bob: Hi"),
            Ok("sequence")
        );
        assert_eq!(
            detect_diagram_type("flowchart TD\nA --> B"),
            Ok("flowchart")
        );
        assert_eq!(detect_diagram_type("graph LR\nA --> B"), Ok("flowchart"));
    }

    #[test]
    fn test_detection_with_comments() {
        let input = r#"
        // This is a comment
        # Another comment
        
        timeline
        title My Day
        "#;
        assert_eq!(detect_diagram_type(input), Ok("timeline"));
    }

    #[test]
    fn test_empty_input() {
        assert!(detect_diagram_type("").is_err());
        assert!(detect_diagram_type("   \n  \n  ").is_err());
        assert!(detect_diagram_type("// Only comments\n# More comments").is_err());
    }

    #[test]
    fn test_unknown_diagram_type() {
        assert_eq!(detect_diagram_type("unknown_diagram_type"), Ok("misc"));
    }
}

