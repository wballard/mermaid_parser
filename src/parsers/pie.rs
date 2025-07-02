use crate::common::ast::{PieDiagram, PieSlice, AccessibilityInfo};
use crate::error::{ParseError, Result};

/// Simple string-based parser for pie chart diagrams
pub fn parse(input: &str) -> Result<PieDiagram> {
    let lines: Vec<&str> = input.lines().collect();
    
    if lines.is_empty() {
        return Err(ParseError::EmptyInput);
    }
    
    let mut diagram = PieDiagram {
        title: None,
        accessibility: AccessibilityInfo::default(),
        show_data: false,
        data: Vec::new(),
    };
    
    let mut first_line_processed = false;
    let mut in_multiline_acc_descr = false;
    let mut multiline_content = Vec::new();
    
    for (line_num, line) in lines.iter().enumerate() {
        let trimmed = line.trim();
        
        // Skip empty lines, comments, and lines with only whitespace
        if trimmed.is_empty() || trimmed.starts_with("//") || trimmed.starts_with("%%") {
            continue;
        }
        
        // Handle multiline accessibility content
        if in_multiline_acc_descr {
            if trimmed == "}" {
                // End of multiline block
                in_multiline_acc_descr = false;
                if !multiline_content.is_empty() {
                    diagram.accessibility.description = Some(multiline_content.join(" "));
                    multiline_content.clear();
                }
                continue;
            } else if !trimmed.is_empty() && !trimmed.starts_with("//") && !trimmed.starts_with("%%") {
                // Accumulate content
                multiline_content.push(trimmed.to_string());
                continue;
            } else {
                continue;
            }
        }
        
        // Skip lines that are just whitespace or escape sequences like \t
        if trimmed.chars().all(|c| c.is_whitespace()) || trimmed == "\\t" {
            continue;
        }
        
        // Handle lines starting with \t followed by directives
        let effective_trimmed = if trimmed.starts_with("\\t") {
            trimmed.strip_prefix("\\t").unwrap_or(trimmed)
        } else {
            trimmed
        };
        
        // First meaningful line should start with "pie"
        if !first_line_processed {
            if !trimmed.starts_with("pie") {
                return Err(ParseError::SyntaxError {
                    message: "Expected pie header".to_string(),
                    expected: vec!["pie".to_string()],
                    found: trimmed.to_string(),
                    line: line_num + 1,
                    column: 1,
                });
            }
            
            // Handle various pie header formats
            if trimmed.starts_with("pie title ") {
                // Inline title: "pie title Chart Name"
                let title = trimmed.strip_prefix("pie title ").unwrap().trim();
                diagram.title = Some(title.to_string());
            } else if trimmed == "pie showData" {
                // pie with showData
                diagram.show_data = true;
            } else if trimmed.starts_with("pie accTitle:") {
                // pie accTitle: content
                let acc_title = trimmed.strip_prefix("pie accTitle:").unwrap().trim();
                diagram.accessibility.title = Some(acc_title.to_string());
            } else if trimmed.starts_with("pie accDescr:") {
                // pie accDescr: content  
                let acc_descr = trimmed.strip_prefix("pie accDescr:").unwrap().trim();
                diagram.accessibility.description = Some(acc_descr.to_string());
            } else if trimmed.starts_with("pie accDescr {") {
                // pie accDescr { ... } - start multiline block
                in_multiline_acc_descr = true;
            } else if trimmed.starts_with("pie ") {
                // "pie chart" or other text after pie - treat as title
                let title = trimmed.strip_prefix("pie ").unwrap().trim();
                if !title.is_empty() {
                    diagram.title = Some(title.to_string());
                }
            }
            // else: just "pie" keyword alone
            
            first_line_processed = true;
            continue;
        }
        
        // Handle other directives
        if effective_trimmed.starts_with("title ") {
            let title = effective_trimmed.strip_prefix("title ").unwrap().trim();
            diagram.title = Some(title.to_string());
        } else if effective_trimmed == "showData" {
            diagram.show_data = true;
        } else if effective_trimmed.starts_with("accTitle:") {
            let acc_title = effective_trimmed.strip_prefix("accTitle:").unwrap().trim();
            diagram.accessibility.title = Some(acc_title.to_string());
        } else if effective_trimmed.starts_with("accDescr:") {
            let acc_descr = effective_trimmed.strip_prefix("accDescr:").unwrap().trim();
            diagram.accessibility.description = Some(acc_descr.to_string());
        } else if effective_trimmed.starts_with("accDescr {") {
            // accDescr { ... } - start multiline block (standalone version)
            in_multiline_acc_descr = true;
        } else if let Some(colon_pos) = effective_trimmed.find(':') {
            // Parse data entry: "Label" : value
            let label_part = effective_trimmed[..colon_pos].trim();
            let value_part = effective_trimmed[colon_pos + 1..].trim();
            
            // Extract label (remove quotes if present)
            let label = if label_part.starts_with('"') && label_part.ends_with('"') {
                label_part[1..label_part.len() - 1].to_string()
            } else {
                label_part.to_string()
            };
            
            // Parse value
            match value_part.parse::<f64>() {
                Ok(value) => {
                    diagram.data.push(PieSlice { label, value });
                }
                Err(_) => {
                    return Err(ParseError::SyntaxError {
                        message: "Invalid numeric value".to_string(),
                        expected: vec!["number".to_string()],
                        found: value_part.to_string(),
                        line: line_num + 1,
                        column: colon_pos + 2,
                    });
                }
            }
        } else {
            return Err(ParseError::SyntaxError {
                message: "Expected data entry or directive".to_string(),
                expected: vec!["\"label\" : value".to_string(), "title".to_string(), "showData".to_string()],
                found: effective_trimmed.to_string(),
                line: line_num + 1,
                column: 1,
            });
        }
    }
    
    // Accept pie charts with just the header - they might be placeholders or incomplete
    
    Ok(diagram)
}