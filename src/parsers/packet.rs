use crate::common::ast::{PacketDiagram, PacketField, AccessibilityInfo};
use crate::error::{ParseError, Result};

/// Simple string-based parser for packet diagrams
pub fn parse(input: &str) -> Result<PacketDiagram> {
    let lines: Vec<&str> = input.lines().collect();
    
    if lines.is_empty() {
        return Err(ParseError::EmptyInput);
    }
    
    let mut diagram = PacketDiagram {
        title: None,
        accessibility: AccessibilityInfo::default(),
        fields: Vec::new(),
    };
    
    let mut first_line_processed = false;
    
    for (line_num, line) in lines.iter().enumerate() {
        let trimmed = line.trim();
        
        // Skip empty lines and comments
        if trimmed.is_empty() || trimmed.starts_with("//") || trimmed.starts_with("%%") {
            continue;
        }
        
        // First meaningful line should start with "packet-beta" or "packet"
        if !first_line_processed {
            if !(trimmed.starts_with("packet-beta") || trimmed.starts_with("packet")) {
                return Err(ParseError::SyntaxError {
                    message: "Expected packet header".to_string(),
                    expected: vec!["packet-beta".to_string(), "packet".to_string()],
                    found: trimmed.to_string(),
                    line: line_num + 1,
                    column: 1,
                });
            }
            first_line_processed = true;
            continue;
        }
        
        // Handle title directive
        if trimmed.starts_with("title ") {
            let title = trimmed.strip_prefix("title ").unwrap().trim();
            diagram.title = Some(title.to_string());
            continue;
        }
        
        // Parse field definition: "0-15: \"Source Port\"" or "+16: \"Source Port\""
        if let Some(colon_pos) = trimmed.find(':') {
            let range_part = trimmed[..colon_pos].trim();
            let field_part = trimmed[colon_pos + 1..].trim();
            
            // Parse bit range - handle different formats
            let (start_bit, end_bit) = if range_part.starts_with('+') {
                // Additive format: "+16" means next 16 bits from current position
                let width_str = range_part.strip_prefix('+').unwrap().trim();
                let width = width_str.parse::<u32>().map_err(|_| ParseError::SyntaxError {
                    message: "Invalid bit width".to_string(),
                    expected: vec!["number".to_string()],
                    found: width_str.to_string(),
                    line: line_num + 1,
                    column: 1,
                })?;
                
                // Calculate start bit based on previous fields
                let current_bit = diagram.fields.iter()
                    .map(|f| f.end_bit + 1)
                    .max()
                    .unwrap_or(0);
                
                // Handle zero width (special case)
                if width == 0 {
                    (current_bit, current_bit)
                } else {
                    (current_bit, current_bit + width - 1)
                }
            } else if let Some(dash_pos) = range_part.find('-') {
                // Range: "0-15"
                let start_str = range_part[..dash_pos].trim();
                let end_str = range_part[dash_pos + 1..].trim();
                
                let start = start_str.parse::<u32>().map_err(|_| ParseError::SyntaxError {
                    message: "Invalid start bit number".to_string(),
                    expected: vec!["number".to_string()],
                    found: start_str.to_string(),
                    line: line_num + 1,
                    column: 1,
                })?;
                
                let end = end_str.parse::<u32>().map_err(|_| ParseError::SyntaxError {
                    message: "Invalid end bit number".to_string(),
                    expected: vec!["number".to_string()],
                    found: end_str.to_string(),
                    line: line_num + 1,
                    column: dash_pos + 2,
                })?;
                
                (start, end)
            } else {
                // Single bit: "106"
                let bit = range_part.parse::<u32>().map_err(|_| ParseError::SyntaxError {
                    message: "Invalid bit number".to_string(),
                    expected: vec!["number".to_string()],
                    found: range_part.to_string(),
                    line: line_num + 1,
                    column: 1,
                })?;
                
                (bit, bit)
            };
            
            // Parse field name (handle quoted strings, parentheses, and identifiers)
            let (name, is_optional) = if field_part.is_empty() {
                // Empty field name - skip this field
                continue;
            } else if field_part.starts_with('"') && field_part.ends_with('"') {
                // Quoted string: "Source Port"
                let name = field_part[1..field_part.len() - 1].to_string();
                (name, false)
            } else if field_part.starts_with('(') && field_part.ends_with(')') {
                // Optional field: (Options and Padding)
                let name = field_part[1..field_part.len() - 1].to_string();
                (name, true)
            } else {
                // Unquoted identifier: Source
                (field_part.to_string(), false)
            };
            
            diagram.fields.push(PacketField {
                start_bit,
                end_bit,
                name,
                is_optional,
            });
        } else {
            // Handle other packet directives or ignore unrecognized lines
            // Some test files have various directives like "packet structure", "packetTitle", etc.
            // We'll skip these for now to be more lenient
            continue;
        }
    }
    
    Ok(diagram)
}