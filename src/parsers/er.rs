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
    EntityAlias {             // alias[display name] or alias["display name"]
        alias: String,
        name: String,
    },
    RelSymbol(String),        // ||--||, ||--o{, etc.
    Label(String),            // : "relationship label"
    LeftBrace,                // {
    RightBrace,               // }
    AttributeType(String),    // string, int, float, etc.
    AttributeName(String),    // Attribute identifier
    KeyType(KeyType),         // PK, FK, UK
    QuotedString(String),     // "comment"
    Comment(String),          // %% comment
    AccTitle(String),         // accTitle: content
    AccDescr(String),         // accDescr: content
    Style(String),            // style directive content
    ClassDef(String),         // classDef directive content
    ClassAssignment {         // entity:::class
        entity: String,
        class: String,
    },
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
    
    // Accessibility directives
    let acc_title = just("accTitle:")
        .ignore_then(whitespace)
        .ignore_then(
            none_of('\n')
                .repeated()
                .collect::<String>()
        )
        .map(|content| ERToken::AccTitle(content.trim().to_string()));
    
    let acc_descr = just("accDescr:")
        .ignore_then(whitespace)
        .ignore_then(
            none_of('\n')
                .repeated()
                .collect::<String>()
        )
        .map(|content| ERToken::AccDescr(content.trim().to_string()));
    
    // Style directive - matches "style <id> <css-properties>"
    let style_directive = just("style")
        .ignore_then(whitespace.at_least(1))
        .ignore_then(
            none_of('\n')
                .repeated()
                .collect::<String>()
        )
        .map(|content| ERToken::Style(content.trim().to_string()));
    
    // ClassDef directive - matches "classDef <name> <css-properties>"
    let class_def_directive = just("classDef")
        .ignore_then(whitespace.at_least(1))
        .ignore_then(
            none_of('\n')
                .repeated()
                .collect::<String>()
        )
        .map(|content| ERToken::ClassDef(content.trim().to_string()));
    
    // Relationship symbols (order matters for overlapping patterns - longer first)
    let rel_symbols = choice((
        just("}o--o{").to("many-to-many"),
        just("}o--||").to("many-to-one"),
        just("||--o{").to("one-to-many"),
        just("||--o|").to("one-to-zero-or-one"),  // Added missing pattern  
        just("||--||").to("one-to-one"),
        just("||--|{").to("one-to-one-or-more"),  // Moved before shorter patterns
        just("}|--|{").to("one-or-more-to-one-or-more"),
        just("}|--||").to("one-or-more-to-one"),
        just("}|..|{").to("one-or-more-to-one-or-more-optional"),  // Added missing pattern
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
    
    // Attribute types with optional modifiers
    let base_attr_types = choice((
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
    ));
    
    // Attribute type with optional length (e.g., string(99)) or array notation (e.g., string[])
    let attr_types = base_attr_types
        .then(
            choice((
                // Array notation: string[]
                just("[]").to("[]".to_string()),
                // Length notation: string(99)
                just('(')
                    .ignore_then(text::int(10))
                    .then_ignore(just(')'))
                    .map(|n| format!("({})", n)),
            ))
            .or_not()
        )
        .map(|(base, modifier)| {
            if let Some(mod_str) = modifier {
                ERToken::AttributeType(format!("{}{}", base, mod_str))
            } else {
                ERToken::AttributeType(base.to_string())
            }
        });
    
    // Quoted string
    let quoted_string = just('"')
        .ignore_then(
            none_of('"').repeated().collect::<String>()
        )
        .then_ignore(just('"'))
        .map(ERToken::QuotedString);
    
    // Entity alias: alias[name] or alias["quoted name"]
    let entity_alias = text::ident()
        .then_ignore(just('['))
        .then(
            choice((
                // Quoted name: "Customer Account"
                just('"')
                    .ignore_then(none_of('"').repeated().collect::<String>())
                    .then_ignore(just('"')),
                // Unquoted name: Person
                none_of(']').repeated().collect::<String>()
            ))
        )
        .then_ignore(just(']'))
        .map(|(alias, name): (&str, String)| ERToken::EntityAlias {
            alias: alias.to_string(),
            name: name.trim().to_string(),
        });
    
    // Class assignment: entity:::class
    let class_assignment = choice((
        // Hyphenated entity with class: LINE-ITEM:::foo
        text::ident()
            .then_ignore(just('-'))
            .then(text::ident())
            .then_ignore(just(":::"))
            .then(text::ident())
            .map(|((first, second), class): ((&str, &str), &str)| ERToken::ClassAssignment {
                entity: format!("{}-{}", first, second),
                class: class.to_string(),
            }),
        // Regular entity with class: PERSON:::foo
        text::ident()
            .then_ignore(just(":::"))
            .then(text::ident())
            .map(|(entity, class): (&str, &str)| ERToken::ClassAssignment {
                entity: entity.to_string(),
                class: class.to_string(),
            }),
    ));
    
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
    
    // Combine all tokens (order matters - longer patterns first)
    let token = choice((
        comment,
        er_keyword,
        acc_title,     // Must come before identifier since "accTitle" could match as identifier
        acc_descr,     // Must come before identifier since "accDescr" could match as identifier
        style_directive, // Must come before identifier since "style" could match as identifier
        class_def_directive, // Must come before identifier since "classDef" could match as identifier
        key_type,
        attr_types,
        rel_symbols,
        left_brace,
        right_brace,
        colon,
        quoted_string,
        entity_alias,  // Must come before identifier since alias part could match as identifier
        class_assignment, // Must come before identifier since entity part could match as identifier
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
    
    // Parse entity name (can be EntityName, EntityAlias, or ClassAssignment)
    let entity_name = any().try_map(|t, span| {
        match t {
            ERToken::EntityName(name) => Ok(name),
            ERToken::EntityAlias { alias, name: _ } => Ok(alias), // Use alias for relationships
            ERToken::ClassAssignment { entity, class: _ } => Ok(entity), // Use entity name, ignore class for now
            _ => Err(Simple::new(Some(t.into()), span))
        }
    });
    
    // Parse entity alias specifically for entity definitions
    let entity_alias = any().try_map(|t, span| {
        match t {
            ERToken::EntityAlias { alias, name } => Ok((alias, name)),
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
    
    // Parse accessibility directives
    let acc_title = any().try_map(|t, span| {
        match t {
            ERToken::AccTitle(title) => Ok(title),
            _ => Err(Simple::new(Some(t.into()), span))
        }
    });
    
    let acc_descr = any().try_map(|t, span| {
        match t {
            ERToken::AccDescr(descr) => Ok(descr),
            _ => Err(Simple::new(Some(t.into()), span))
        }
    });
    
    // Parse style directives
    let style_directive = any().try_map(|t, span| {
        match t {
            ERToken::Style(content) => Ok(content),
            _ => Err(Simple::new(Some(t.into()), span))
        }
    });
    
    // Parse classDef directives
    let class_def_directive = any().try_map(|t, span| {
        match t {
            ERToken::ClassDef(content) => Ok(content),
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
    
    // Parse entity definition: ENTITY { attributes } or alias[Entity Name] { attributes }
    let entity_def = choice((
        // Entity with alias: alias[name] { attributes }
        entity_alias
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
            .map(|((alias, _display_name), attributes)| Entity { 
                name: alias, // Use alias as the entity identifier
                attributes 
            }),
        // Regular entity: ENTITY { attributes }
        entity_name
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
            .map(|(name, attributes)| Entity { name, attributes })
    ));
    
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
    let skip_token = any().filter(|t| !matches!(t, 
        ERToken::EntityName(_) | 
        ERToken::RelSymbol(_) | 
        ERToken::ClassAssignment { .. } |
        ERToken::EntityAlias { .. }
    ));
    
    // Parse diagram content - include accessibility directives, style, and classDef
    let content = choice((
        entity_def.map(|e| (Some(e), None, None, None, None, None)),
        relationship.map(|r| (None, Some(r), None, None, None, None)),
        acc_title.map(|t| (None, None, Some(t), None, None, None)),
        acc_descr.map(|d| (None, None, None, Some(d), None, None)),
        style_directive.map(|s| (None, None, None, None, Some(s), None)),
        class_def_directive.map(|c| (None, None, None, None, None, Some(c))),
        skip_token.map(|_| (None, None, None, None, None, None)),
    ))
        .repeated()
        .collect::<Vec<_>>();
    
    header
        .ignore_then(content)
        .map(|items| {
            let mut entities = HashMap::new();
            let mut relationships = Vec::new();
            let mut acc_title = None;
            let mut acc_descr = None;
            
            for (entity_opt, rel_opt, title_opt, descr_opt, _style_opt, _class_def_opt) in items {
                if let Some(entity) = entity_opt {
                    entities.insert(entity.name.clone(), entity);
                }
                if let Some(rel) = rel_opt {
                    relationships.push(rel);
                }
                if let Some(title) = title_opt {
                    acc_title = Some(title);
                }
                if let Some(descr) = descr_opt {
                    acc_descr = Some(descr);
                }
                // Style and classDef directives are parsed but not stored in the AST currently
            }
            
            ErDiagram {
                title: None,
                accessibility: AccessibilityInfo {
                    title: acc_title,
                    description: acc_descr,
                },
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
        "one-or-more-to-one-or-more-optional" | "}|..|{" => (
            ErCardinality { min: CardinalityValue::One, max: CardinalityValue::Many },
            ErCardinality { min: CardinalityValue::One, max: CardinalityValue::Many },
        ),
        "one-to-zero-or-one" | "||--o|" => (
            ErCardinality { min: CardinalityValue::One, max: CardinalityValue::One },
            ErCardinality { min: CardinalityValue::Zero, max: CardinalityValue::One },
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