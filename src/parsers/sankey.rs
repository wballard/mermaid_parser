//! Sankey diagram parser implementation

use crate::common::ast::{SankeyDiagram, SankeyLink, SankeyNode};
use crate::error::{ParseError, Result};
use chumsky::prelude::*;
use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq)]
pub enum SankeyToken {
    Header,               // "sankey-beta"
    Comma,                // ","
    NewLine,              // "\n" | "\r\n"
    QuotedText(String),   // "text"
    UnquotedText(String), // text
}

pub fn parse(input: &str) -> Result<SankeyDiagram> {
    let tokens =
        sankey_lexer()
            .parse(input)
            .into_result()
            .map_err(|e| ParseError::SyntaxError {
                message: "Failed to tokenize sankey diagram".to_string(),
                expected: vec![],
                found: format!("{:?}", e),
                line: 0,
                column: 0,
            })?;

    let result = sankey_parser()
        .parse(&tokens[..])
        .into_result()
        .map_err(|e| ParseError::SyntaxError {
            message: "Failed to parse sankey diagram".to_string(),
            expected: vec![],
            found: format!("{:?}", e),
            line: 0,
            column: 0,
        });
    result
}

fn sankey_lexer<'src>(
) -> impl Parser<'src, &'src str, Vec<SankeyToken>, extra::Err<Simple<'src, char>>> {
    let header = choice((just("sankey-beta"), just("sankey"))).map(|_| SankeyToken::Header);

    let comma = just(',').map(|_| SankeyToken::Comma);

    let newline = choice((just("\n"), just("\r\n"))).map(|_| SankeyToken::NewLine);

    let quoted_text = just('"')
        .ignore_then(
            choice((just("\"\"").to('"'), none_of('"').map(|c| c)))
                .repeated()
                .collect::<String>(),
        )
        .then_ignore(just('"'))
        .map(SankeyToken::QuotedText);

    // Unquoted text allows more characters including single quotes, hyphens, and slashes
    // but stops at commas, newlines, or tabs
    let unquoted_text = none_of(",\n\r\"\t")
        .repeated()
        .at_least(1)
        .collect::<String>()
        .map(|s: String| SankeyToken::UnquotedText(s.trim().to_string()));

    let whitespace = one_of(" \t").repeated();

    // Comment lines starting with %% and extending to end of line
    let comment = just("%%")
        .then(none_of('\n').repeated())
        .then(just('\n').or_not())
        .ignored();

    let token = choice((header, comma, newline, quoted_text, unquoted_text));

    // Skip any leading whitespace/newlines before the first token
    let leading_ws = choice((one_of(" \t\n\r").ignored(), comment)).repeated();

    leading_ws.ignore_then(
        choice((comment.map(|_| None), token.map(Some)))
            .padded_by(whitespace)
            .repeated()
            .collect::<Vec<_>>()
            .map(|tokens| tokens.into_iter().flatten().collect()),
    )
}

fn sankey_parser<'tokens, 'src: 'tokens>() -> impl Parser<
    'tokens,
    &'tokens [SankeyToken],
    SankeyDiagram,
    extra::Err<Simple<'tokens, SankeyToken>>,
> + Clone {
    let field = select! {
        SankeyToken::QuotedText(text) => text.clone(),
        SankeyToken::UnquotedText(text) => text.clone(),
    };

    let record = field // source
        .then_ignore(just(&SankeyToken::Comma))
        .then(field) // target
        .then_ignore(just(&SankeyToken::Comma))
        .then(field) // value
        .map(|((source, target), value_str)| {
            let value = value_str.trim().parse::<f64>().unwrap_or(0.0);
            SankeyLink {
                source: source.trim().to_string(),
                target: target.trim().to_string(),
                value,
            }
        });

    let csv_line = record.then_ignore(just(&SankeyToken::NewLine).or_not());

    let blank_line = just(&SankeyToken::NewLine);

    let content_line = choice((csv_line.map(Some), blank_line.map(|_| None)));

    just(&SankeyToken::Header)
        .then_ignore(just(&SankeyToken::NewLine).or_not()) // Header might be at EOF
        .then_ignore(just(&SankeyToken::NewLine).repeated()) // Allow blank lines after header
        .then(content_line.repeated().collect::<Vec<_>>())
        .map(|(_, lines)| {
            let links: Vec<SankeyLink> = lines.into_iter().flatten().collect();

            let mut nodes = HashSet::new();
            for link in &links {
                nodes.insert(link.source.clone());
                nodes.insert(link.target.clone());
            }

            SankeyDiagram {
                nodes: nodes
                    .into_iter()
                    .map(|name| SankeyNode {
                        id: name.clone(),
                        name,
                    })
                    .collect(),
                links,
            }
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;

    #[test]
    fn test_simple_sankey() {
        let input = r#"sankey-beta
A,B,10
B,C,5
"#;

        let result = parse(input);
        assert!(result.is_ok());

        let diagram = result.unwrap();
        assert_eq!(diagram.nodes.len(), 3);
        assert_eq!(diagram.links.len(), 2);
        assert_eq!(diagram.links[0].value, 10.0);
        assert_eq!(diagram.links[1].value, 5.0);
    }

    #[test]
    fn test_quoted_fields() {
        let input = r#"sankey-beta
"Source Node","Target Node",25.5
"Another ""Quoted"" Source",Destination,15.0
"#;

        let result = parse(input);
        assert!(result.is_ok());

        let diagram = result.unwrap();
        assert_eq!(diagram.links[0].source, "Source Node");
        assert_eq!(diagram.links[0].target, "Target Node");
        assert_eq!(diagram.links[1].source, "Another \"Quoted\" Source");
    }

    #[test]
    fn test_with_blank_lines() {
        let input = r#"sankey-beta

A,B,10

B,C,5
"#;

        let result = parse(input);
        assert!(result.is_ok());

        let diagram = result.unwrap();
        assert_eq!(diagram.links.len(), 2);
    }

    #[test]
    fn test_real_sankey_files() {
        let test_dir = Path::new("test/sankey");
        if test_dir.exists() {
            for entry in fs::read_dir(test_dir).unwrap() {
                let entry = entry.unwrap();
                let path = entry.path();
                if path.extension().and_then(|s| s.to_str()) == Some("mermaid") {
                    let content = fs::read_to_string(&path)
                        .unwrap_or_else(|_| panic!("Failed to read file: {:?}", path));

                    // Remove metadata comments
                    let content = content
                        .lines()
                        .filter(|line| !line.trim().starts_with("//"))
                        .collect::<Vec<_>>()
                        .join("\n");

                    let result = parse(&content);
                    if let Err(e) = &result {
                        eprintln!("Error parsing file {:?}: {:?}", path, e);
                        eprintln!("Content after removing comments:\n{}", content);
                    }
                    assert!(result.is_ok(), "Failed to parse file: {:?}", path);

                    let diagram = result.unwrap();
                    // Some test files might have just the header with no data
                    if !diagram.links.is_empty() {
                        assert!(
                            !diagram.nodes.is_empty(),
                            "No nodes found in file: {:?}",
                            path
                        );

                        // Verify all links reference existing nodes
                        let node_names: HashSet<_> =
                            diagram.nodes.iter().map(|n| &n.name).collect();

                        for link in &diagram.links {
                            assert!(
                                node_names.contains(&link.source),
                                "Link source '{}' not found in nodes for file: {:?}",
                                link.source,
                                path
                            );
                            assert!(
                                node_names.contains(&link.target),
                                "Link target '{}' not found in nodes for file: {:?}",
                                link.target,
                                path
                            );
                            assert!(
                                link.value >= 0.0,
                                "Link value should be non-negative for file: {:?}",
                                path
                            );
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn test_whitespace_handling() {
        let input = r#"sankey-beta
  A  ,  B  ,  10  
C,D,20
"#;

        let result = parse(input);
        assert!(result.is_ok());

        let diagram = result.unwrap();
        assert_eq!(diagram.links[0].source, "A");
        assert_eq!(diagram.links[0].target, "B");
        assert_eq!(diagram.links[0].value, 10.0);
    }

    #[test]
    fn test_special_characters_in_unquoted() {
        let input = r#"sankey-beta
Bio-conversion,Liquid,0.597
Agricultural 'waste',Bio-conversion,124.729
"#;

        let result = parse(input);
        assert!(result.is_ok());

        let diagram = result.unwrap();
        assert_eq!(diagram.links[0].source, "Bio-conversion");
        assert_eq!(diagram.links[1].source, "Agricultural 'waste'");
        assert_eq!(diagram.links[1].target, "Bio-conversion");
        assert_eq!(diagram.links[1].value, 124.729);
    }

    #[test]
    fn test_comment_lines() {
        let input = r#"sankey-beta

%% source,target,value
Electricity grid,Over generation / exports,104.453
Electricity grid,Heating and cooling - homes,113.726
"#;

        let result = parse(input);
        assert!(result.is_ok());

        let diagram = result.unwrap();
        assert_eq!(diagram.links.len(), 2);
        assert_eq!(diagram.links[0].source, "Electricity grid");
        assert_eq!(diagram.links[0].target, "Over generation / exports");
    }

    #[test]
    fn test_trailing_newline() {
        // This simulates the real file content after removing // comments
        let input = "\nsankey-beta\n\n%% source,target,value\nElectricity grid,Over generation / exports,104.453\nElectricity grid,Heating and cooling - homes,113.726\nElectricity grid,H2 conversion,27.14\n";

        let result = parse(input);
        assert!(result.is_ok(), "Failed to parse with error: {:?}", result);

        let diagram = result.unwrap();
        assert_eq!(diagram.links.len(), 3);
    }

    #[test]
    fn test_empty_sankey() {
        // Test sankey with just the header (no data)
        let input = "\nsankey\n";

        let result = parse(input);
        assert!(
            result.is_ok(),
            "Failed to parse empty sankey with error: {:?}",
            result
        );

        let diagram = result.unwrap();
        assert_eq!(diagram.links.len(), 0);
        assert_eq!(diagram.nodes.len(), 0);
    }
}
