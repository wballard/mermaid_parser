//! Architecture diagram parser implementation

use crate::common::ast::{
    ArchDirection, ArchEdge, ArchEdgeType, ArchitectureDiagram, EdgeEndpoint, Group, Junction,
    Port, Service, AccessibilityInfo,
};
use crate::error::{ParseError, Result};
use chumsky::prelude::*;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum ArchToken {
    ArchitectureBeta,        // "architecture-beta"
    Group,                   // "group"
    Service,                 // "service"
    Junction,                // "junction"
    In,                      // "in"
    LeftParen,               // (
    RightParen,              // )
    LeftSquare,              // [
    RightSquare,             // ]
    Colon,                   // :
    Dash,                    // -
    DashDash,                // --
    Dot,                     // .
    DotDot,                  // ..
    Arrow,                   // ->
    BiArrow,                 // <->
    PortL,                   // L
    PortR,                   // R
    PortT,                   // T
    PortB,                   // B
    Identifier(String),      // IDs and names
    Icon(String),            // Icon names
    Title(String),           // Quoted titles
    Comment(String),
    NewLine,
    Eof,
}

pub fn parse(input: &str) -> Result<ArchitectureDiagram> {
    let tokens = architecture_lexer()
        .parse(input)
        .into_result()
        .map_err(|e| ParseError::SyntaxError {
            message: "Failed to tokenize architecture diagram".to_string(),
            expected: vec![],
            found: format!("{:?}", e),
            line: 0,
            column: 0,
        })?;

    let result = architecture_parser()
        .parse(&tokens[..])
        .into_result()
        .map_err(|e| ParseError::SyntaxError {
            message: "Failed to parse architecture diagram".to_string(),
            expected: vec![],
            found: format!("{:?}", e),
            line: 0,
            column: 0,
        });
    result
}

fn architecture_lexer<'src>() -> impl Parser<'src, &'src str, Vec<ArchToken>, extra::Err<Simple<'src, char>>> {
    let comment = choice((
        just("%%").then(none_of('\n').repeated()),
        just("//").then(none_of('\n').repeated()),
    ))
    .map(|_| ArchToken::Comment("".to_string()));
    
    // Keywords - using just() instead of keyword() for hyphenated keywords
    let keywords = choice((
        just("architecture-beta").map(|_| ArchToken::ArchitectureBeta),
        text::keyword("group").map(|_| ArchToken::Group),
        text::keyword("service").map(|_| ArchToken::Service),
        text::keyword("junction").map(|_| ArchToken::Junction),
        text::keyword("in").map(|_| ArchToken::In),
    ));
    
    // Port specifiers - single uppercase letters
    let ports = one_of("LRTB").map(|c| match c {
        'L' => ArchToken::PortL,
        'R' => ArchToken::PortR,
        'T' => ArchToken::PortT,
        'B' => ArchToken::PortB,
        _ => unreachable!(),
    });
    
    // Edge types
    let edges = choice((
        just("<->").map(|_| ArchToken::BiArrow),
        just("->").map(|_| ArchToken::Arrow),
        just("--").map(|_| ArchToken::DashDash),
        just("..").map(|_| ArchToken::DotDot),
    ));
    
    // Icon in parentheses
    let icon = just('(')
        .ignore_then(
            none_of(')')
                .repeated()
                .at_least(1)
                .collect::<String>()
        )
        .then_ignore(just(')'))
        .map(ArchToken::Icon);
    
    // Title in square brackets
    let title = just('[')
        .ignore_then(
            none_of(']')
                .repeated()
                .collect::<String>()
        )
        .then_ignore(just(']'))
        .map(ArchToken::Title);
    
    // Identifier - after keywords
    let identifier = text::ident()
        .map(|s: &str| ArchToken::Identifier(s.to_string()));
    
    let colon = just(':').map(|_| ArchToken::Colon);
    let newline = text::newline().map(|_| ArchToken::NewLine);
    
    // Combine all tokens
    let token = choice((
        comment,
        keywords,
        edges,
        icon,
        title,
        ports,
        colon,
        identifier,
    ))
    .padded();
    
    // Parse many tokens
    token
        .or(newline)
        .repeated()
        .collect::<Vec<_>>()
}

fn architecture_parser<'tokens, 'src: 'tokens>() -> impl Parser<
    'tokens,
    &'tokens [ArchToken],
    ArchitectureDiagram,
    extra::Err<Simple<'tokens, ArchToken>>,
> + Clone {
    // Skip comments and newlines before architecture-beta
    select! {
        ArchToken::Comment(_) => (),
        ArchToken::NewLine => (),
    }
    .repeated()
    .ignore_then(just(&ArchToken::ArchitectureBeta))
    .then_ignore(
        select! {
            ArchToken::NewLine => ()
        }
        .repeated()
    )
    .then(
        any()
            .repeated()
            .collect::<Vec<_>>()
    )
    .map(|(_, tokens)| {
            let mut services = HashMap::new();
            let mut groups = HashMap::new();
            let mut junctions = HashMap::new();
            let mut edges = Vec::new();
            let mut i = 0;
            
            while i < tokens.len() {
                match &tokens[i] {
                    ArchToken::Comment(_) | ArchToken::NewLine => {
                        i += 1;
                    }
                    ArchToken::Group => {
                        if let Some((group, consumed)) = parse_group(&tokens[i..]) {
                            groups.insert(group.id.clone(), group);
                            i += consumed;
                        } else {
                            i += 1;
                        }
                    }
                    ArchToken::Service => {
                        if let Some((service, consumed)) = parse_service(&tokens[i..]) {
                            services.insert(service.id.clone(), service);
                            i += consumed;
                        } else {
                            i += 1;
                        }
                    }
                    ArchToken::Junction => {
                        if let Some((junction, consumed)) = parse_junction(&tokens[i..]) {
                            junctions.insert(junction.id.clone(), junction);
                            i += consumed;
                        } else {
                            i += 1;
                        }
                    }
                    ArchToken::Identifier(id) => {
                        // Try to parse edge
                        if let Some((edge, consumed)) = parse_edge(&tokens[i..], id) {
                            edges.push(edge);
                            i += consumed;
                        } else {
                            i += 1;
                        }
                    }
                    _ => {
                        i += 1;
                    }
                }
            }
            
            ArchitectureDiagram {
                title: None,
                accessibility: AccessibilityInfo::default(),
                direction: ArchDirection::TB, // Default
                services,
                groups,
                junctions,
                edges,
            }
        })
}

fn parse_group(tokens: &[ArchToken]) -> Option<(Group, usize)> {
    if tokens.len() < 4 {
        return None;
    }
    
    let mut i = 1; // Skip "group"
    
    let id = match &tokens[i] {
        ArchToken::Identifier(id) => id.clone(),
        _ => return None,
    };
    i += 1;
    
    let icon = match &tokens[i] {
        ArchToken::Icon(icon) => {
            i += 1;
            Some(icon.clone())
        }
        _ => None,
    };
    
    let title = match &tokens[i] {
        ArchToken::Title(title) => {
            i += 1;
            title.clone()
        }
        _ => id.clone(),
    };
    
    // Check for nested group
    let in_group = if i + 1 < tokens.len() && matches!(&tokens[i], ArchToken::In) {
        i += 1;
        match &tokens[i] {
            ArchToken::Identifier(parent) => {
                i += 1;
                Some(parent.clone())
            }
            _ => None,
        }
    } else {
        None
    };
    
    Some((
        Group {
            id,
            icon,
            title,
            in_group,
        },
        i,
    ))
}

fn parse_service(tokens: &[ArchToken]) -> Option<(Service, usize)> {
    if tokens.len() < 4 {
        return None;
    }
    
    let mut i = 1; // Skip "service"
    
    let id = match &tokens[i] {
        ArchToken::Identifier(id) => id.clone(),
        _ => return None,
    };
    i += 1;
    
    let icon = match &tokens[i] {
        ArchToken::Icon(icon) => {
            i += 1;
            Some(icon.clone())
        }
        _ => None,
    };
    
    let title = match &tokens[i] {
        ArchToken::Title(title) => {
            i += 1;
            title.clone()
        }
        _ => id.clone(),
    };
    
    // Check for group membership
    let in_group = if i + 1 < tokens.len() && matches!(&tokens[i], ArchToken::In) {
        i += 1;
        match &tokens[i] {
            ArchToken::Identifier(group) => {
                i += 1;
                Some(group.clone())
            }
            _ => None,
        }
    } else {
        None
    };
    
    Some((
        Service {
            id,
            icon,
            title,
            in_group,
        },
        i,
    ))
}

fn parse_junction(tokens: &[ArchToken]) -> Option<(Junction, usize)> {
    if tokens.len() < 2 {
        return None;
    }
    
    let mut i = 1; // Skip "junction"
    
    let id = match &tokens[i] {
        ArchToken::Identifier(id) => id.clone(),
        _ => return None,
    };
    i += 1;
    
    // Check for group membership
    let in_group = if i + 1 < tokens.len() && matches!(&tokens[i], ArchToken::In) {
        i += 1;
        match &tokens[i] {
            ArchToken::Identifier(group) => {
                i += 1;
                Some(group.clone())
            }
            _ => None,
        }
    } else {
        None
    };
    
    Some((
        Junction {
            id,
            in_group,
        },
        i,
    ))
}

fn parse_edge(tokens: &[ArchToken], from_id: &str) -> Option<(ArchEdge, usize)> {
    if tokens.len() < 3 {
        return None;
    }
    
    let mut i = 1; // Skip from_id
    
    // Check for two possible formats:
    // 1. source:port -- port:target (with colons)
    // 2. source port--port target (without colons)
    
    let from_port = if matches!(&tokens[i], ArchToken::Colon) {
        // Format 1: source:port
        i += 1;
        match &tokens[i] {
            ArchToken::PortL => {
                i += 1;
                Some(Port::Left)
            }
            ArchToken::PortR => {
                i += 1;
                Some(Port::Right)
            }
            ArchToken::PortT => {
                i += 1;
                Some(Port::Top)
            }
            ArchToken::PortB => {
                i += 1;
                Some(Port::Bottom)
            }
            _ => None,
        }
    } else if matches!(&tokens[i], ArchToken::PortL | ArchToken::PortR | ArchToken::PortT | ArchToken::PortB) {
        // Format 2: source port (space separated)
        let port = match &tokens[i] {
            ArchToken::PortL => Port::Left,
            ArchToken::PortR => Port::Right,
            ArchToken::PortT => Port::Top,
            ArchToken::PortB => Port::Bottom,
            _ => unreachable!(),
        };
        i += 1;
        Some(port)
    } else {
        None
    };
    
    // Parse edge type
    let edge_type = match &tokens[i] {
        ArchToken::DashDash => {
            i += 1;
            ArchEdgeType::Solid
        }
        ArchToken::DotDot => {
            i += 1;
            ArchEdgeType::Dotted
        }
        ArchToken::Arrow => {
            i += 1;
            ArchEdgeType::Arrow
        }
        ArchToken::BiArrow => {
            i += 1;
            ArchEdgeType::BiArrow
        }
        _ => return None,
    };
    
    // Parse to port and target
    let (to_port, to_id) = if matches!(&tokens[i], ArchToken::PortL | ArchToken::PortR | ArchToken::PortT | ArchToken::PortB) {
        let port = match &tokens[i] {
            ArchToken::PortL => Port::Left,
            ArchToken::PortR => Port::Right,
            ArchToken::PortT => Port::Top,
            ArchToken::PortB => Port::Bottom,
            _ => unreachable!(),
        };
        i += 1;
        
        // Check for colon format
        if i < tokens.len() && matches!(&tokens[i], ArchToken::Colon) {
            i += 1;
            match &tokens[i] {
                ArchToken::Identifier(id) => {
                    i += 1;
                    (Some(port), id.clone())
                }
                _ => return None,
            }
        } else if i < tokens.len() {
            // Space format: port target
            match &tokens[i] {
                ArchToken::Identifier(id) => {
                    i += 1;
                    (Some(port), id.clone())
                }
                _ => return None,
            }
        } else {
            return None;
        }
    } else {
        match &tokens[i] {
            ArchToken::Identifier(id) => {
                i += 1;
                (None, id.clone())
            }
            _ => return None,
        }
    };
    
    Some((
        ArchEdge {
            from: EdgeEndpoint {
                id: from_id.to_string(),
                port: from_port,
            },
            to: EdgeEndpoint {
                id: to_id,
                port: to_port,
            },
            label: None,
            edge_type,
        },
        i,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_architecture() {
        let input = r#"architecture-beta
    service api[API Service]
    service db(database)[Database]
    
    api -- db
"#;

        let result = parse(input);
        assert!(result.is_ok(), "Failed to parse: {:?}", result);

        let diagram = result.unwrap();
        assert_eq!(diagram.services.len(), 2);
        assert_eq!(diagram.edges.len(), 1);

        let api = &diagram.services["api"];
        assert_eq!(api.title, "API Service");
        assert!(api.icon.is_none());

        let db = &diagram.services["db"];
        assert_eq!(db.icon, Some("database".to_string()));
    }

    #[test]
    fn test_architecture_with_groups() {
        let input = r#"architecture-beta
    group api[API]
    group public[Public API] in api
    group private[Private API] in api
    
    service serv1(server)[Server] in public
    service serv2(server)[Server] in private
    service db(database)[Database] in private
    
    serv1 -- serv2
    serv2 -- db
"#;

        let result = parse(input);
        assert!(result.is_ok(), "Failed to parse: {:?}", result);

        let diagram = result.unwrap();
        assert_eq!(diagram.groups.len(), 3);
        assert_eq!(diagram.services.len(), 3);
        assert_eq!(diagram.edges.len(), 2);

        let public_group = &diagram.groups["public"];
        assert_eq!(public_group.in_group, Some("api".to_string()));

        let serv1 = &diagram.services["serv1"];
        assert_eq!(serv1.in_group, Some("public".to_string()));
    }

    #[test]
    fn test_architecture_with_ports() {
        let input = r#"architecture-beta
    service api[API]
    service db[Database]
    service cache[Cache]
    
    api:L -- R:db
    api:B -- T:cache
    db:L -> R:cache
"#;

        let result = parse(input);
        assert!(result.is_ok(), "Failed to parse: {:?}", result);

        let diagram = result.unwrap();
        assert_eq!(diagram.edges.len(), 3);

        let edge1 = &diagram.edges[0];
        assert_eq!(edge1.from.id, "api");
        assert_eq!(edge1.from.port, Some(Port::Left));
        assert_eq!(edge1.to.id, "db");
        assert_eq!(edge1.to.port, Some(Port::Right));

        let edge3 = &diagram.edges[2];
        assert_eq!(edge3.edge_type, ArchEdgeType::Arrow);
    }

    #[test]
    fn test_architecture_with_junctions() {
        let input = r#"architecture-beta
    service api[API]
    service db[Database]
    junction junc1
    
    api -- junc1
    junc1 -- db
"#;

        let result = parse(input);
        assert!(result.is_ok(), "Failed to parse: {:?}", result);

        let diagram = result.unwrap();
        assert_eq!(diagram.junctions.len(), 1);
        assert_eq!(diagram.edges.len(), 2);

        let junction = &diagram.junctions["junc1"];
        assert_eq!(junction.id, "junc1");
    }

    #[test]
    fn test_architecture_edge_types() {
        let input = r#"architecture-beta
    service a[A]
    service b[B]
    service c[C]
    service d[D]
    
    a -- b
    b .. c
    c -> d
    d <-> a
"#;

        let result = parse(input);
        assert!(result.is_ok(), "Failed to parse: {:?}", result);

        let diagram = result.unwrap();
        assert_eq!(diagram.edges.len(), 4);

        assert_eq!(diagram.edges[0].edge_type, ArchEdgeType::Solid);
        assert_eq!(diagram.edges[1].edge_type, ArchEdgeType::Dotted);
        assert_eq!(diagram.edges[2].edge_type, ArchEdgeType::Arrow);
        assert_eq!(diagram.edges[3].edge_type, ArchEdgeType::BiArrow);
    }

    #[test]
    fn test_architecture_comments() {
        let input = r#"architecture-beta
    %% This is a comment
    service api[API]
    %% Another comment
    service db[Database]
    
    api -- db
"#;

        let result = parse(input);
        assert!(result.is_ok(), "Failed to parse: {:?}", result);

        let diagram = result.unwrap();
        assert_eq!(diagram.services.len(), 2);
        assert_eq!(diagram.edges.len(), 1);
    }

    #[test]
    fn test_real_world_architecture() {
        // Test with actual mermaid sample
        let input = std::fs::read_to_string("test/architecture/architecture_spec_ts_001.mermaid")
            .expect("Failed to read test file");

        let result = parse(&input);
        assert!(result.is_ok(), "Failed to parse real-world example: {:?}", result);

        let diagram = result.unwrap();
        assert_eq!(diagram.groups.len(), 1); // api group
        assert_eq!(diagram.services.len(), 5); // db, disk1, disk2, server, gateway
        assert_eq!(diagram.edges.len(), 4); // 4 connections
    }
}