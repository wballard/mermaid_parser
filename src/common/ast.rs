//! Abstract Syntax Tree definitions for all Mermaid diagram types

/// Top-level enum representing all supported Mermaid diagram types
#[derive(Debug, Clone, PartialEq)]
pub enum DiagramType {
    /// Sankey flow diagrams
    Sankey(SankeyDiagram),
    /// Timeline diagrams
    Timeline(TimelineDiagram),
    /// User journey diagrams
    Journey(JourneyDiagram),
    /// Sequence diagrams
    Sequence(SequenceDiagram),
    /// Class diagrams
    Class(ClassDiagram),
    /// State diagrams
    State(StateDiagram),
    /// Flowchart diagrams
    Flowchart(FlowchartDiagram),
    /// Gantt charts
    Gantt(GanttDiagram),
    /// Pie charts
    Pie(PieDiagram),
    /// Git graphs
    Git(GitDiagram),
    /// Entity-relationship diagrams
    Er(ErDiagram),
    /// C4 architecture diagrams
    C4(C4Diagram),
    /// Mind maps
    Mindmap(MindmapDiagram),
    /// Quadrant charts
    Quadrant(QuadrantDiagram),
    /// XY charts
    XyChart(XyChartDiagram),
    /// Kanban boards
    Kanban(KanbanDiagram),
    /// Block diagrams
    Block(BlockDiagram),
    /// Architecture diagrams
    Architecture(ArchitectureDiagram),
    /// Packet diagrams
    Packet(PacketDiagram),
    /// Requirement diagrams
    Requirement(RequirementDiagram),
    /// Treemap diagrams
    Treemap(TreemapDiagram),
    /// Radar charts
    Radar(RadarDiagram),
}

/// Common accessibility information used across diagram types
#[derive(Debug, Clone, PartialEq, Default)]
pub struct AccessibilityInfo {
    pub title: Option<String>,
    pub description: Option<String>,
}

// Sankey Diagrams
#[derive(Debug, Clone, PartialEq)]
pub struct SankeyDiagram {
    pub nodes: Vec<SankeyNode>,
    pub links: Vec<SankeyLink>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SankeyNode {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SankeyLink {
    pub source: String,
    pub target: String,
    pub value: f64,
}

// Timeline Diagrams
#[derive(Debug, Clone, PartialEq)]
pub struct TimelineDiagram {
    pub title: Option<String>,
    pub accessibility: AccessibilityInfo,
    pub sections: Vec<TimelineSection>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TimelineSection {
    pub name: String,
    pub items: Vec<TimelineItem>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TimelineItem {
    Period(String),
    Event(String),
}

// Journey Diagrams
#[derive(Debug, Clone, PartialEq)]
pub struct JourneyDiagram {
    pub title: Option<String>,
    pub accessibility: AccessibilityInfo,
    pub sections: Vec<JourneySection>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct JourneySection {
    pub name: String,
    pub tasks: Vec<JourneyTask>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct JourneyTask {
    pub name: String,
    pub score: i32,
    pub actors: Vec<String>,
}

// Sequence Diagrams
#[derive(Debug, Clone, PartialEq)]
pub struct SequenceDiagram {
    pub title: Option<String>,
    pub accessibility: AccessibilityInfo,
    pub participants: Vec<Participant>,
    pub statements: Vec<SequenceStatement>,
    pub autonumber: Option<AutoNumber>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Participant {
    pub actor: String,
    pub alias: Option<String>,
    pub participant_type: ParticipantType,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParticipantType {
    Participant,
    Actor,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SequenceStatement {
    Message(Message),
    Note(Note),
    Loop(Loop),
    Alt(Alternative),
    Opt(Optional),
    Par(Parallel),
    Critical(Critical),
    Activate(String),
    Deactivate(String),
    Create(Participant),
    Destroy(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Message {
    pub from: String,
    pub to: String,
    pub text: String,
    pub arrow_type: ArrowType,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ArrowType {
    SolidOpen,
    SolidClosed,
    DottedOpen,
    DottedClosed,
    Cross,
    Point,
    BiDirectionalSolid,
    BiDirectionalDotted,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Note {
    pub position: NotePosition,
    pub actor: String,
    pub text: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum NotePosition {
    LeftOf,
    RightOf,
    Over,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Loop {
    pub condition: String,
    pub statements: Vec<SequenceStatement>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Alternative {
    pub condition: String,
    pub statements: Vec<SequenceStatement>,
    pub else_branch: Option<ElseBranch>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ElseBranch {
    pub condition: Option<String>,
    pub statements: Vec<SequenceStatement>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Optional {
    pub condition: String,
    pub statements: Vec<SequenceStatement>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Parallel {
    pub branches: Vec<ParallelBranch>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ParallelBranch {
    pub condition: Option<String>,
    pub statements: Vec<SequenceStatement>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Critical {
    pub condition: String,
    pub statements: Vec<SequenceStatement>,
    pub options: Vec<CriticalOption>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CriticalOption {
    pub condition: String,
    pub statements: Vec<SequenceStatement>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AutoNumber {
    pub start: Option<i32>,
    pub step: Option<i32>,
    pub visible: bool,
}

// Placeholder types for other diagram types
// These will be expanded as parsers are implemented

#[derive(Debug, Clone, PartialEq)]
pub struct ClassDiagram {
    pub title: Option<String>,
    pub accessibility: AccessibilityInfo,
    // TODO: Add class diagram specific fields
}

#[derive(Debug, Clone, PartialEq)]
pub struct StateDiagram {
    pub title: Option<String>,
    pub accessibility: AccessibilityInfo,
    // TODO: Add state diagram specific fields
}

#[derive(Debug, Clone, PartialEq)]
pub struct FlowchartDiagram {
    pub title: Option<String>,
    pub accessibility: AccessibilityInfo,
    // TODO: Add flowchart specific fields
}

#[derive(Debug, Clone, PartialEq)]
pub struct GanttDiagram {
    pub title: Option<String>,
    pub accessibility: AccessibilityInfo,
    // TODO: Add gantt specific fields
}

#[derive(Debug, Clone, PartialEq)]
pub struct PieDiagram {
    pub title: Option<String>,
    pub accessibility: AccessibilityInfo,
    // TODO: Add pie chart specific fields
}

#[derive(Debug, Clone, PartialEq)]
pub struct GitDiagram {
    pub title: Option<String>,
    pub accessibility: AccessibilityInfo,
    // TODO: Add git graph specific fields
}

#[derive(Debug, Clone, PartialEq)]
pub struct ErDiagram {
    pub title: Option<String>,
    pub accessibility: AccessibilityInfo,
    // TODO: Add ER diagram specific fields
}

#[derive(Debug, Clone, PartialEq)]
pub struct C4Diagram {
    pub title: Option<String>,
    pub accessibility: AccessibilityInfo,
    // TODO: Add C4 diagram specific fields
}

#[derive(Debug, Clone, PartialEq)]
pub struct MindmapDiagram {
    pub title: Option<String>,
    pub accessibility: AccessibilityInfo,
    // TODO: Add mindmap specific fields
}

#[derive(Debug, Clone, PartialEq)]
pub struct QuadrantDiagram {
    pub title: Option<String>,
    pub accessibility: AccessibilityInfo,
    // TODO: Add quadrant chart specific fields
}

#[derive(Debug, Clone, PartialEq)]
pub struct XyChartDiagram {
    pub title: Option<String>,
    pub accessibility: AccessibilityInfo,
    // TODO: Add XY chart specific fields
}

#[derive(Debug, Clone, PartialEq)]
pub struct KanbanDiagram {
    pub title: Option<String>,
    pub accessibility: AccessibilityInfo,
    // TODO: Add kanban specific fields
}

#[derive(Debug, Clone, PartialEq)]
pub struct BlockDiagram {
    pub title: Option<String>,
    pub accessibility: AccessibilityInfo,
    // TODO: Add block diagram specific fields
}

#[derive(Debug, Clone, PartialEq)]
pub struct ArchitectureDiagram {
    pub title: Option<String>,
    pub accessibility: AccessibilityInfo,
    pub direction: ArchDirection,
    pub services: std::collections::HashMap<String, Service>,
    pub groups: std::collections::HashMap<String, Group>,
    pub junctions: std::collections::HashMap<String, Junction>,
    pub edges: Vec<ArchEdge>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ArchDirection {
    TB, // Top to Bottom
    BT, // Bottom to Top
    LR, // Left to Right
    RL, // Right to Left
}

#[derive(Debug, Clone, PartialEq)]
pub struct Service {
    pub id: String,
    pub icon: Option<String>,
    pub title: String,
    pub in_group: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Group {
    pub id: String,
    pub icon: Option<String>,
    pub title: String,
    pub in_group: Option<String>, // For nested groups
}

#[derive(Debug, Clone, PartialEq)]
pub struct Junction {
    pub id: String,
    pub in_group: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ArchEdge {
    pub from: EdgeEndpoint,
    pub to: EdgeEndpoint,
    pub label: Option<String>,
    pub edge_type: ArchEdgeType,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EdgeEndpoint {
    pub id: String,
    pub port: Option<Port>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Port {
    Left,   // L
    Right,  // R
    Top,    // T
    Bottom, // B
}

#[derive(Debug, Clone, PartialEq)]
pub enum ArchEdgeType {
    Solid,      // --
    Dotted,     // ..
    Arrow,      // ->
    BiArrow,    // <->
}

#[derive(Debug, Clone, PartialEq)]
pub struct PacketDiagram {
    pub title: Option<String>,
    pub accessibility: AccessibilityInfo,
    // TODO: Add packet diagram specific fields
}

#[derive(Debug, Clone, PartialEq)]
pub struct RequirementDiagram {
    pub title: Option<String>,
    pub accessibility: AccessibilityInfo,
    // TODO: Add requirement diagram specific fields
}

#[derive(Debug, Clone, PartialEq)]
pub struct TreemapDiagram {
    pub title: Option<String>,
    pub accessibility: AccessibilityInfo,
    // TODO: Add treemap specific fields
}

#[derive(Debug, Clone, PartialEq)]
pub struct RadarDiagram {
    pub title: Option<String>,
    pub accessibility: AccessibilityInfo,
    // TODO: Add radar chart specific fields
}

