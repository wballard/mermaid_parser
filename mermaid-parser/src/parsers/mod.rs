//! Individual parsers for each Mermaid diagram type

// Implemented parsers
// pub mod sankey;
// pub mod timeline;
// pub mod journey;
// pub mod sequence;

// Future parsers (placeholders)
// pub mod class;
// pub mod state;
// pub mod flowchart;
// pub mod gantt;
// pub mod pie;
// pub mod git;
// pub mod er;
// pub mod c4;
// pub mod mindmap;
// pub mod quadrant;
// pub mod xychart;
// pub mod kanban;
// pub mod block;
// pub mod architecture;
// pub mod packet;
// pub mod requirement;
// pub mod treemap;
// pub mod radar;

// Placeholder implementations for now
use crate::error::{ParseError, Result};
use crate::common::ast::*;

pub mod sankey {
    use super::*;
    
    pub fn parse(_input: &str) -> Result<SankeyDiagram> {
        Err(ParseError::UnsupportedDiagramType("sankey".to_string()))
    }
}

pub mod timeline {
    use super::*;
    
    pub fn parse(_input: &str) -> Result<TimelineDiagram> {
        Err(ParseError::UnsupportedDiagramType("timeline".to_string()))
    }
}

pub mod journey {
    use super::*;
    
    pub fn parse(_input: &str) -> Result<JourneyDiagram> {
        Err(ParseError::UnsupportedDiagramType("journey".to_string()))
    }
}

pub mod sequence {
    use super::*;
    
    pub fn parse(_input: &str) -> Result<SequenceDiagram> {
        Err(ParseError::UnsupportedDiagramType("sequence".to_string()))
    }
}

// Add other placeholder modules as needed...