//! Individual parsers for each Mermaid diagram type

// Implemented parsers
pub mod architecture;
pub mod block;
pub mod c4;
pub mod class;
pub mod er;
pub mod flowchart;
pub mod gantt;
pub mod git;
pub mod kanban;
pub mod mindmap;
pub mod misc;
pub mod packet;
pub mod pie;
pub mod quadrant;
pub mod radar;
pub mod sankey;
pub mod state;

pub mod timeline;
pub mod requirement;
pub mod treemap;

// Future parsers (placeholders)  
// pub mod journey;
// pub mod sequence;
// pub mod xychart;

pub mod journey;
pub mod sequence;

// Add other placeholder modules as needed...
