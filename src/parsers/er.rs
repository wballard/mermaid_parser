//! Entity Relationship diagram parser implementation

use crate::common::ast::{
    ErDiagram, Entity, Attribute, KeyType, ErRelationship, ErCardinality, 
    CardinalityValue, AccessibilityInfo,
};
use crate::error::{ParseError, Result};
use chumsky::prelude::*;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ERToken {
    ERDiagram,                // "erDiagram"
    EntityName(String),       // Entity identifier
    RelSymbol(String),        // ||--||, ||--o{, etc.
    Label(String),            // : "relationship label"
    LeftBrace,                // {
    RightBrace,               // }
    AttributeType(String),    // string, int, float, etc.
    AttributeName(String),    // Attribute identifier
    KeyType(KeyType),         // PK, FK, UK
    QuotedString(String),     // "comment"
    Comment(String),          // %% comment
    Colon,                    // :
    NewLine,
    Eof,
}

impl From<&ERToken> for String {
    fn from(token: &ERToken) -> Self {
        format!("{:?}", token)
    }
}

fn er_lexer<'src>() -> impl Parser<'src, &'src str, Vec<ERToken>, extra::Err<Simple<'src, char>>> {
    let whitespace = just(' ').or(just('\t')).repeated();
    
    let comment = just("%%")
        .then(none_of('\n').repeated())
        .map(|_| ERToken::Comment("".to_string()));
    
    let er_keyword = just("erDiagram")
        .map(|_| ERToken::ERDiagram);
    
    // Relationship symbols (order matters for overlapping patterns - longer first)
    let rel_symbols = choice((
        just("}o--o{").to("many-to-many"),
        just("}o--||").to("many-to-one"),
        just("||--o{").to("one-to-many"),
        just("||--||").to("one-to-one"),
        just("||--|{").to("one-to-one-or-more"),  // Moved before shorter patterns
        just("}|--|{").to("one-or-more-to-one-or-more"),
        just("}|--||").to("one-or-more-to-one"),
        just("}o..o{").to("many-to-many-optional"),
        just("}o..||").to("many-to-one-optional"),
        just("||..o{").to("one-to-many-optional"),
        just("||..||").to("one-to-one-optional"),
    ))
    .map(|s: &str| ERToken::RelSymbol(s.to_string()));
    
    // Key types
    let key_type = choice((
        just("PK").to(ERToken::KeyType(KeyType::PK)),
        just("FK").to(ERToken::KeyType(KeyType::FK)),
        just("UK").to(ERToken::KeyType(KeyType::UK)),
    ));
    
    // Attribute types
    let attr_types = choice((
        just("string"),
        just("int"),
        just("integer"),
        just("float"),
        just("double"),
        just("decimal"),
        just("boolean"),
        just("bool"),
        just("date"),
        just("datetime"),
        just("timestamp"),
        just("time"),
        just("blob"),
        just("text"),
        just("varchar"),
        just("char"),
    ))
    .map(|t: &str| ERToken::AttributeType(t.to_string()));
    
    // Quoted string
    let quoted_string = just('"')
        .ignore_then(
            none_of('"').repeated().collect::<String>()
        )
        .then_ignore(just('"'))
        .map(ERToken::QuotedString);
    
    // Entity/Attribute names (identifiers)
    let identifier = choice((
        // Hyphenated identifier like LINE-ITEM
        text::ident()
            .then(just('-'))
            .then(text::ident())
            .map(|((first, _), second)| format!("{}-{}", first, second)),
        // Regular identifier
        text::ident()
            .map(|s: &str| s.to_string()),
    ))
    .map(ERToken::EntityName);
    
    let left_brace = just('{').to(ERToken::LeftBrace);
    let right_brace = just('}').to(ERToken::RightBrace);
    let colon = just(':').to(ERToken::Colon);
    let newline = just('\n').to(ERToken::NewLine);
    
    // Combine all tokens
    let token = choice((
        comment,
        er_keyword,
        key_type,
        attr_types,
        rel_symbols,
        left_brace,
        right_brace,
        colon,
        quoted_string,
        identifier,
    ));
    
    // Handle whitespace and newlines
    whitespace
        .ignore_then(token)
        .or(newline)
        .repeated()
        .collect::<Vec<_>>()
}

fn er_parser<'src>() -> impl Parser<'src, &'src [ERToken], ErDiagram, extra::Err<Simple<'src, ERToken>>> {
    // Parse erDiagram header
    let header = just(ERToken::ERDiagram)
        .then_ignore(
            any().filter(|t| matches!(t, ERToken::NewLine))
                .repeated()
        );
    
    // Parse entity name
    let entity_name = any().try_map(|t, span| {
        match t {
            ERToken::EntityName(name) => Ok(name),
            _ => Err(Simple::new(Some(t.into()), span))
        }
    });
    
    // Parse attribute type
    let attr_type = any().try_map(|t, span| {
        match t {
            ERToken::AttributeType(typ) => Ok(typ),
            _ => Err(Simple::new(Some(t.into()), span))
        }
    });
    
    // Parse key type
    let key_type = any().try_map(|t, span| {
        match t {
            ERToken::KeyType(kt) => Ok(kt),
            _ => Err(Simple::new(Some(t.into()), span))
        }
    });
    
    // Parse quoted string
    let quoted_string = any().try_map(|t, span| {
        match t {
            ERToken::QuotedString(s) => Ok(s),
            _ => Err(Simple::new(Some(t.into()), span))
        }
    });
    
    // Parse attribute: type name [key_type] ["comment"]
    let attribute = attr_type
        .then(entity_name)  // Attribute names use same parser as entity names
        .then(key_type.or_not())
        .then(quoted_string.or_not())
        .map(|(((attr_type, name), key_type), comment)| {
            Attribute {
                name,
                attr_type,
                key_type,
                comment,
            }
        });
    
    // Parse entity definition: ENTITY { attributes }
    let entity_def = entity_name
        .then_ignore(just(ERToken::LeftBrace))
        .then(
            any().filter(|t| matches!(t, ERToken::NewLine))
                .repeated()
                .ignore_then(attribute)
                .then_ignore(
                    any().filter(|t| matches!(t, ERToken::NewLine))
                        .repeated()
                )
                .repeated()
                .collect::<Vec<_>>()
        )
        .then_ignore(just(ERToken::RightBrace))
        .map(|(name, attributes)| Entity { name, attributes });
    
    // Parse relationship symbol and convert to cardinality
    let rel_symbol = any().try_map(|t, span| {
        match t {
            ERToken::RelSymbol(symbol) => Ok(symbol),
            _ => Err(Simple::new(Some(t.into()), span))
        }
    });
    
    // Parse a simple relationship: ENTITY1 ||--o{ ENTITY2 : label
    let relationship = entity_name
        .then(rel_symbol)
        .then(entity_name)
        .then(
            just(ERToken::Colon)
                .ignore_then(entity_name)
                .or_not()
        )
        .map(|(((left_entity, symbol), right_entity), label)| {
            let (left_card, right_card) = parse_cardinality(&symbol);
            ErRelationship {
                left_entity,
                right_entity,
                left_cardinality: left_card,
                right_cardinality: right_card,
                label,
            }
        });
    
    // Skip newlines and other tokens
    let skip_token = any().filter(|t| !matches!(t, ERToken::EntityName(_) | ERToken::RelSymbol(_)));
    
    // Parse diagram content
    let content = choice((
        entity_def.map(|e| (Some(e), None)),
        relationship.map(|r| (None, Some(r))),
        skip_token.map(|_| (None, None)),
    ))
        .repeated()
        .collect::<Vec<_>>();
    
    header
        .ignore_then(content)
        .map(|items| {
            let mut entities = HashMap::new();
            let mut relationships = Vec::new();
            
            for (entity_opt, rel_opt) in items {
                if let Some(entity) = entity_opt {
                    entities.insert(entity.name.clone(), entity);
                }
                if let Some(rel) = rel_opt {
                    relationships.push(rel);
                }
            }
            
            ErDiagram {
                title: None,
                accessibility: AccessibilityInfo::default(),
                entities,
                relationships,
            }
        })
}

fn parse_cardinality(symbol: &str) -> (ErCardinality, ErCardinality) {
    match symbol {
        "one-to-one" | "||--||" => (
            ErCardinality { min: CardinalityValue::One, max: CardinalityValue::One },
            ErCardinality { min: CardinalityValue::One, max: CardinalityValue::One },
        ),
        "one-to-many" | "||--o{" => (
            ErCardinality { min: CardinalityValue::One, max: CardinalityValue::One },
            ErCardinality { min: CardinalityValue::Zero, max: CardinalityValue::Many },
        ),
        "many-to-one" | "}o--||" => (
            ErCardinality { min: CardinalityValue::Zero, max: CardinalityValue::Many },
            ErCardinality { min: CardinalityValue::One, max: CardinalityValue::One },
        ),
        "many-to-many" | "}o--o{" => (
            ErCardinality { min: CardinalityValue::Zero, max: CardinalityValue::Many },
            ErCardinality { min: CardinalityValue::Zero, max: CardinalityValue::Many },
        ),
        "one-to-one-or-more" | "||--|{" => (
            ErCardinality { min: CardinalityValue::One, max: CardinalityValue::One },
            ErCardinality { min: CardinalityValue::One, max: CardinalityValue::Many },
        ),
        _ => (
            ErCardinality { min: CardinalityValue::Zero, max: CardinalityValue::Many },
            ErCardinality { min: CardinalityValue::Zero, max: CardinalityValue::Many },
        ),
    }
}

pub fn parse(input: &str) -> Result<ErDiagram> {
    let tokens = er_lexer()
        .parse(input)
        .into_result()
        .map_err(|e| ParseError::SyntaxError {
            message: "Failed to tokenize ER diagram".to_string(),
            expected: vec![],
            found: format!("{:?}", e),
            line: 0,
            column: 0,
        })?;
    
    let result = er_parser()
        .parse(&tokens[..])
        .into_result()
        .map_err(|e| ParseError::SyntaxError {
            message: "Failed to parse ER diagram".to_string(),
            expected: vec![],
            found: format!("{:?}", e),
            line: 0,
            column: 0,
        });
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer_er_keyword() {
        let input = "erDiagram";
        let tokens = er_lexer()
            .parse(input)
            .into_result();
        
        assert!(tokens.is_ok(), "Failed to tokenize: {:?}", tokens);
        let tokens = tokens.unwrap();
        
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0], ERToken::ERDiagram);
    }

    #[test]
    fn test_lexer_relationship_symbols() {
        let input = "CUSTOMER ||--o{ ORDER";
        let tokens = er_lexer()
            .parse(input)
            .into_result();
        
        assert!(tokens.is_ok(), "Failed to tokenize: {:?}", tokens);
        let tokens = tokens.unwrap();
        
        // Should have: CUSTOMER, ||--o{, ORDER
        assert!(tokens.len() >= 3, "Expected at least 3 tokens, got: {:?}", tokens);
        assert_eq!(tokens[0], ERToken::EntityName("CUSTOMER".to_string()));
        assert_eq!(tokens[1], ERToken::RelSymbol("one-to-many".to_string()));
        assert_eq!(tokens[2], ERToken::EntityName("ORDER".to_string()));
    }

    #[test]
    fn test_lexer_entity_with_attributes() {
        let input = r#"CUSTOMER {
        string name PK
    }"#;
        let tokens = er_lexer()
            .parse(input)
            .into_result();
        
        assert!(tokens.is_ok(), "Failed to tokenize: {:?}", tokens);
        let tokens = tokens.unwrap();
        
        // Should include: CUSTOMER, {, newline, string, name, PK, newline, }
        let expected_tokens = vec![
            ERToken::EntityName("CUSTOMER".to_string()),
            ERToken::LeftBrace,
            ERToken::NewLine,
            ERToken::AttributeType("string".to_string()),
            ERToken::EntityName("name".to_string()),
            ERToken::KeyType(KeyType::PK),
            ERToken::NewLine,
            ERToken::RightBrace,
        ];
        
        assert_eq!(tokens.len(), expected_tokens.len(), "Token count mismatch. Got: {:?}", tokens);
        for (i, (expected, actual)) in expected_tokens.iter().zip(tokens.iter()).enumerate() {
            assert_eq!(expected, actual, "Token mismatch at index {}", i);
        }
    }

    #[test]
    fn test_simple_er_diagram() {
        let input = r#"erDiagram
    CUSTOMER ||--o{ ORDER : places
"#;

        let result = parse(input);
        assert!(result.is_ok(), "Failed to parse: {:?}", result);
        
        let diagram = result.unwrap();
        assert_eq!(diagram.relationships.len(), 1);
        
        let rel = &diagram.relationships[0];
        assert_eq!(rel.left_entity, "CUSTOMER");
        assert_eq!(rel.right_entity, "ORDER");
        assert_eq!(rel.label, Some("places".to_string()));
    }

    #[test]
    fn test_parser_multiple_relationships() {
        let input = r#"erDiagram
    CUSTOMER ||--o{ ORDER : places
    ORDER ||--|{ LINE-ITEM : contains
"#;
        
        let result = parse(input);
        assert!(result.is_ok(), "Failed to parse: {:?}", result);
        
        let diagram = result.unwrap();
        assert_eq!(diagram.relationships.len(), 2);
        
        let rel1 = &diagram.relationships[0];
        assert_eq!(rel1.left_entity, "CUSTOMER");
        assert_eq!(rel1.right_entity, "ORDER");
        assert_eq!(rel1.label, Some("places".to_string()));
        
        let rel2 = &diagram.relationships[1];
        assert_eq!(rel2.left_entity, "ORDER");
        assert_eq!(rel2.right_entity, "LINE-ITEM");
        assert_eq!(rel2.label, Some("contains".to_string()));
    }

    #[test]
    fn test_parser_entity_with_attributes() {
        let input = r#"erDiagram
    CUSTOMER {
        string name PK
        int customerId
        string address
    }
"#;
        
        let result = parse(input);
        assert!(result.is_ok(), "Failed to parse: {:?}", result);
        
        let diagram = result.unwrap();
        assert_eq!(diagram.entities.len(), 1);
        
        let customer = &diagram.entities["CUSTOMER"];
        assert_eq!(customer.name, "CUSTOMER");
        assert_eq!(customer.attributes.len(), 3);
        
        let name_attr = &customer.attributes[0];
        assert_eq!(name_attr.name, "name");
        assert_eq!(name_attr.attr_type, "string");
        assert_eq!(name_attr.key_type, Some(KeyType::PK));
        
        let id_attr = &customer.attributes[1];
        assert_eq!(id_attr.name, "customerId");
        assert_eq!(id_attr.attr_type, "int");
        assert_eq!(id_attr.key_type, None);
    }

    #[test]
    fn test_parser_complete_er_diagram() {
        let input = r#"erDiagram
    CUSTOMER ||--o{ ORDER : places
    CUSTOMER {
        string name PK
        string customerId PK
        string address
        string phoneNumber
    }
    ORDER ||--|{ LINE-ITEM : contains
    ORDER {
        int orderId PK
        string customerId FK
        date orderDate
        string status
    }
    LINE-ITEM {
        string productId FK
        int quantity
        float pricePerUnit
    }
"#;
        
        let result = parse(input);
        assert!(result.is_ok(), "Failed to parse: {:?}", result);
        
        let diagram = result.unwrap();
        assert_eq!(diagram.entities.len(), 3);
        assert_eq!(diagram.relationships.len(), 2);
        
        // Check CUSTOMER entity
        let customer = &diagram.entities["CUSTOMER"];
        assert_eq!(customer.attributes.len(), 4);
        
        // Check ORDER entity
        let order = &diagram.entities["ORDER"];
        assert_eq!(order.attributes.len(), 4);
        assert_eq!(order.attributes[1].key_type, Some(KeyType::FK));
        
        // Check relationships
        assert_eq!(diagram.relationships[0].left_entity, "CUSTOMER");
        assert_eq!(diagram.relationships[0].right_entity, "ORDER");
    }

    #[test]
    fn test_real_er_file() {
        let input = std::fs::read_to_string("test/er/entityRelationshipDiagram_md_001.mermaid").unwrap();
        // Remove metadata comments
        let input = input.lines()
            .filter(|line| !line.starts_with("//"))
            .collect::<Vec<_>>()
            .join("\n")
            .trim()
            .to_string();
        
        let result = parse(&input);
        assert!(result.is_ok(), "Failed to parse real file: {:?}", result);
        
        let diagram = result.unwrap();
        // Just verify it parses successfully and has content
        assert!(!diagram.entities.is_empty() || !diagram.relationships.is_empty());
    }
}