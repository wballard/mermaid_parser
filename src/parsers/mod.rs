//! Individual parsers for each Mermaid diagram type

// Implemented parsers
pub mod sankey;
pub mod architecture;
pub mod block;
pub mod c4;
pub mod class;
pub mod er;
pub mod flowchart;
pub mod gantt;
pub mod git;

// Future parsers (placeholders)
// pub mod timeline;
// pub mod journey;
// pub mod sequence;
// pub mod state;
// pub mod flowchart;
// pub mod gantt;
// pub mod pie;
// pub mod git;
// pub mod c4;
// pub mod mindmap;
// pub mod quadrant;
// pub mod xychart;
// pub mod kanban;
// pub mod block;
// pub mod packet;
// pub mod requirement;
// pub mod treemap;
// pub mod radar;

// Placeholder implementations for now
use crate::common::ast::*;
use crate::error::{ParseError, Result};

pub mod timeline {
    use super::*;

    pub fn parse(_input: &str) -> Result<TimelineDiagram> {
        Err(ParseError::UnsupportedDiagramType("timeline".to_string()))
    }
}

pub mod journey;

pub mod sequence {
    use super::*;

    pub fn parse(_input: &str) -> Result<SequenceDiagram> {
        Err(ParseError::UnsupportedDiagramType("sequence".to_string()))
    }
}

// Add other placeholder modules as needed...

