//! Flowchart diagram parser implementation

use crate::common::ast::{
    AccessibilityInfo, EdgeType, FlowDirection, FlowEdge, FlowNode, FlowchartDiagram, NodeShape,
};
use crate::error::Result;
use chumsky::prelude::*;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FlowToken {
    // Keywords
    Graph,
    Flowchart,
    Subgraph,
    End,

    // Directions
    TB,
    TD,
    BT,
    RL,
    LR,

    // Node brackets
    LeftSquare,        // [
    RightSquare,       // ]
    LeftParen,         // (
    RightParen,        // )
    LeftBrace,         // {
    RightBrace,        // }
    LeftAngle,         // <
    RightAngle,        // >
    DoubleLeftSquare,  // [[
    DoubleRightSquare, // ]]
    DoubleLeftParen,   // ((
    DoubleRightParen,  // ))
    TripleLeftParen,   // (((
    TripleRightParen,  // )))
    DoubleLeftBrace,   // {{
    DoubleRightBrace,  // }}

    // Edge components
    Dash,     // -
    DashDash, // --
    Arrow,    // > or -->

    // Values
    NodeId(String),
    Text(String),

    Comment(String),
    NewLine,
    Eof,
}

impl From<&FlowToken> for String {
    fn from(token: &FlowToken) -> Self {
        format!("{:?}", token)
    }
}

fn flowchart_lexer<'src>(
) -> impl Parser<'src, &'src str, Vec<FlowToken>, extra::Err<Simple<'src, char>>> {
    let whitespace = just(' ').or(just('\t')).repeated();

    let comment = just("%%")
        .then(none_of('\n').repeated())
        .map(|_| FlowToken::Comment("".to_string()));

    let flowchart_keyword = just("flowchart").map(|_| FlowToken::Flowchart);

    let graph_keyword = just("graph").map(|_| FlowToken::Graph);

    // Directions
    let directions = choice((
        just("TB").to(FlowToken::TB),
        just("TD").to(FlowToken::TD),
        just("BT").to(FlowToken::BT),
        just("RL").to(FlowToken::RL),
        just("LR").to(FlowToken::LR),
    ));

    // Node shape brackets (order matters for overlapping patterns)
    let node_brackets = choice((
        just("(((").to(FlowToken::TripleLeftParen),
        just(")))").to(FlowToken::TripleRightParen),
        just("((").to(FlowToken::DoubleLeftParen),
        just("))").to(FlowToken::DoubleRightParen),
        just("[[").to(FlowToken::DoubleLeftSquare),
        just("]]").to(FlowToken::DoubleRightSquare),
        just("{{").to(FlowToken::DoubleLeftBrace),
        just("}}").to(FlowToken::DoubleRightBrace),
        just('[').to(FlowToken::LeftSquare),
        just(']').to(FlowToken::RightSquare),
        just('(').to(FlowToken::LeftParen),
        just(')').to(FlowToken::RightParen),
        just('{').to(FlowToken::LeftBrace),
        just('}').to(FlowToken::RightBrace),
        just('<').to(FlowToken::LeftAngle),
        just('>').to(FlowToken::RightAngle),
    ));

    // Edge patterns (order matters for overlapping patterns)
    let edge_patterns = choice((
        just("-->").to(FlowToken::Arrow),
        just("--").to(FlowToken::DashDash),
        just('-').to(FlowToken::Dash),
        just('>').to(FlowToken::RightAngle),
    ));

    // Text for node labels - will be handled differently
    let text_chars = none_of("]})\n>")
        .repeated()
        .at_least(1)
        .collect::<String>()
        .map(|s: String| FlowToken::Text(s.trim().to_string()));

    // Simple identifier
    let identifier = text::ident().map(|s: &str| FlowToken::NodeId(s.to_string()));

    let newline = just('\n').to(FlowToken::NewLine);

    // Combine all tokens (order matters for parsing)
    let token = choice((
        comment,
        flowchart_keyword,
        graph_keyword,
        directions,
        edge_patterns,
        node_brackets,
        identifier,
        text_chars, // Keep this last to avoid conflicts
    ));

    // Handle whitespace and newlines
    whitespace
        .ignore_then(token)
        .or(newline)
        .repeated()
        .collect::<Vec<_>>()
}

fn parse_node_shape(left_bracket: &FlowToken, right_bracket: &FlowToken) -> NodeShape {
    match (left_bracket, right_bracket) {
        (FlowToken::LeftSquare, FlowToken::RightSquare) => NodeShape::Rectangle,
        (FlowToken::LeftParen, FlowToken::RightParen) => NodeShape::RoundedRectangle,
        (FlowToken::LeftBrace, FlowToken::RightBrace) => NodeShape::Rhombus,
        (FlowToken::DoubleLeftParen, FlowToken::DoubleRightParen) => NodeShape::Circle,
        (FlowToken::TripleLeftParen, FlowToken::TripleRightParen) => NodeShape::DoubleCircle,
        (FlowToken::DoubleLeftSquare, FlowToken::DoubleRightSquare) => NodeShape::Subroutine,
        (FlowToken::DoubleLeftBrace, FlowToken::DoubleRightBrace) => NodeShape::Hexagon,
        _ => NodeShape::Rectangle, // Default
    }
}

fn parse_simple_node_and_edges(tokens: &[FlowToken]) -> (HashMap<String, FlowNode>, Vec<FlowEdge>) {
    let mut nodes = HashMap::new();
    let mut edges = Vec::new();
    let mut i = 0;

    while i < tokens.len() {
        match &tokens[i] {
            FlowToken::NodeId(node_id) => {
                // Check if this is a node definition: A[text...] or A{text...}, etc.
                if i + 2 < tokens.len() {
                    if let Some(left_bracket) = tokens.get(i + 1) {
                        if matches!(
                            left_bracket,
                            FlowToken::LeftSquare
                                | FlowToken::LeftParen
                                | FlowToken::LeftBrace
                                | FlowToken::DoubleLeftSquare
                                | FlowToken::DoubleLeftParen
                                | FlowToken::TripleLeftParen
                                | FlowToken::DoubleLeftBrace
                        ) {
                            // Collect text tokens and node ids until we find the closing bracket
                            let mut text_parts = Vec::new();
                            let mut j = i + 2;
                            let mut found_close = false;

                            while j < tokens.len() {
                                match &tokens[j] {
                                    FlowToken::NodeId(text) => {
                                        text_parts.push(text.clone());
                                        j += 1;
                                    }
                                    FlowToken::Text(text) => {
                                        text_parts.push(text.clone());
                                        j += 1;
                                    }
                                    bracket
                                        if matches!(
                                            bracket,
                                            FlowToken::RightSquare
                                                | FlowToken::RightParen
                                                | FlowToken::RightBrace
                                                | FlowToken::DoubleRightSquare
                                                | FlowToken::DoubleRightParen
                                                | FlowToken::TripleRightParen
                                                | FlowToken::DoubleRightBrace
                                        ) =>
                                    {
                                        let shape = parse_node_shape(left_bracket, bracket);
                                        let node_text = if text_parts.is_empty() {
                                            None
                                        } else {
                                            Some(text_parts.join(" "))
                                        };

                                        let node = FlowNode {
                                            id: node_id.clone(),
                                            text: node_text,
                                            shape,
                                            classes: Vec::new(),
                                            icon: None,
                                        };
                                        nodes.insert(node_id.clone(), node);
                                        found_close = true;
                                        i = j + 1;
                                        break;
                                    }
                                    _ => break,
                                }
                            }

                            if found_close {
                                continue;
                            }
                        }
                    }
                }

                // Check if this is an edge: A --> B
                if i + 2 < tokens.len() {
                    if let (Some(FlowToken::Arrow), Some(FlowToken::NodeId(target_id))) =
                        (tokens.get(i + 1), tokens.get(i + 2))
                    {
                        let edge = FlowEdge {
                            from: node_id.clone(),
                            to: target_id.clone(),
                            edge_type: EdgeType::Arrow,
                            label: None,
                            min_length: None,
                        };
                        edges.push(edge);
                        i += 3; // Skip the edge definition
                        continue;
                    }
                }

                i += 1;
            }
            _ => i += 1,
        }
    }

    (nodes, edges)
}

pub fn parse(input: &str) -> Result<FlowchartDiagram> {
    // First tokenize the input
    let tokens = flowchart_lexer().parse(input).into_result().map_err(|e| {
        crate::error::ParseError::LexError {
            message: format!("Lexer error: {:?}", e),
            line: 1,
            column: 1,
        }
    })?;

    // Parse the header to get direction
    let direction = if tokens.len() >= 2 {
        match (&tokens[0], &tokens[1]) {
            (FlowToken::Flowchart | FlowToken::Graph, FlowToken::TB) => FlowDirection::TB,
            (FlowToken::Flowchart | FlowToken::Graph, FlowToken::TD) => FlowDirection::TD,
            (FlowToken::Flowchart | FlowToken::Graph, FlowToken::BT) => FlowDirection::BT,
            (FlowToken::Flowchart | FlowToken::Graph, FlowToken::RL) => FlowDirection::RL,
            (FlowToken::Flowchart | FlowToken::Graph, FlowToken::LR) => FlowDirection::LR,
            _ => FlowDirection::TD, // Default
        }
    } else {
        FlowDirection::TD
    };

    // Skip header tokens and parse nodes and edges from the rest
    let remaining_tokens = if tokens.len() > 2 { &tokens[2..] } else { &[] };
    let (nodes, edges) = parse_simple_node_and_edges(remaining_tokens);

    Ok(FlowchartDiagram {
        title: None,
        accessibility: AccessibilityInfo::default(),
        direction,
        nodes,
        edges,
        subgraphs: Vec::new(),
        styles: Vec::new(),
        class_defs: HashMap::new(),
        clicks: Vec::new(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_flowchart_lexer() {
        let input = "flowchart TD";
        let tokens = flowchart_lexer().parse(input).into_result();

        assert!(tokens.is_ok(), "Failed to tokenize: {:?}", tokens);
        let tokens = tokens.unwrap();

        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0], FlowToken::Flowchart);
        assert_eq!(tokens[1], FlowToken::TD);
    }

    #[test]
    fn test_simple_flowchart() {
        let input = r#"flowchart TD
    A[Start] --> B[End]
"#;

        let result = parse(input);
        assert!(result.is_ok(), "Failed to parse: {:?}", result);

        let diagram = result.unwrap();
        assert_eq!(diagram.direction, FlowDirection::TD);
    }

    #[test]
    fn test_flowchart_directions() {
        let test_cases = vec![
            ("flowchart TB", FlowDirection::TB),
            ("flowchart TD", FlowDirection::TD),
            ("flowchart BT", FlowDirection::BT),
            ("flowchart LR", FlowDirection::LR),
            ("flowchart RL", FlowDirection::RL),
            ("graph LR", FlowDirection::LR),
        ];

        for (input, expected_direction) in test_cases {
            let result = parse(input);
            assert!(result.is_ok(), "Failed to parse: {}", input);

            let diagram = result.unwrap();
            assert_eq!(
                diagram.direction, expected_direction,
                "Wrong direction for: {}",
                input
            );
        }
    }

    #[test]
    fn test_basic_node_parsing() {
        let input = "flowchart TD\nA[Start Node]";
        let result = parse(input);
        assert!(result.is_ok(), "Failed to parse: {:?}", result);

        let diagram = result.unwrap();
        assert_eq!(diagram.direction, FlowDirection::TD);
        assert_eq!(diagram.nodes.len(), 1);

        let node = diagram.nodes.get("A").expect("Node A should exist");
        assert_eq!(node.id, "A");
        assert_eq!(node.text, Some("Start Node".to_string()));
        assert_eq!(node.shape, NodeShape::Rectangle);
    }

    #[test]
    fn test_basic_edge_parsing() {
        let input = "flowchart TD\nA --> B";
        let result = parse(input);
        assert!(result.is_ok(), "Failed to parse: {:?}", result);

        let diagram = result.unwrap();
        assert_eq!(diagram.edges.len(), 1);

        let edge = &diagram.edges[0];
        assert_eq!(edge.from, "A");
        assert_eq!(edge.to, "B");
        assert_eq!(edge.edge_type, EdgeType::Arrow);
    }
}
