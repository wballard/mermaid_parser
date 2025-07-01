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
    pub classes: std::collections::HashMap<String, Class>,
    pub relationships: Vec<ClassRelationship>,
    pub notes: Vec<Note>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Class {
    pub name: String,
    pub stereotype: Option<Stereotype>,
    pub members: Vec<ClassMember>,
    pub annotations: Vec<String>,
    pub css_class: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Stereotype {
    Interface,
    Abstract,
    Service,
    Enumeration,
    Exception,
    Custom(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum ClassMember {
    Property(Property),
    Method(Method),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Property {
    pub name: String,
    pub prop_type: Option<String>,
    pub visibility: Visibility,
    pub is_static: bool,
    pub default_value: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Method {
    pub name: String,
    pub parameters: Vec<Parameter>,
    pub return_type: Option<String>,
    pub visibility: Visibility,
    pub is_static: bool,
    pub is_abstract: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Parameter {
    pub name: String,
    pub param_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Visibility {
    Public,    // +
    Private,   // -
    Protected, // #
    Package,   // ~
}

#[derive(Debug, Clone, PartialEq)]
pub struct ClassRelationship {
    pub from: String,
    pub to: String,
    pub relationship_type: ClassRelationshipType,
    pub from_cardinality: Option<String>,
    pub to_cardinality: Option<String>,
    pub label: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ClassRelationshipType {
    Inheritance,        // <|--
    Composition,        // *--
    Aggregation,        // o--
    Association,        // <--
    Link,              // --
    DashedLink,        // ..
    Dependency,        // <..
    Realization,       // <|..
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
    pub direction: FlowDirection,
    pub nodes: std::collections::HashMap<String, FlowNode>,
    pub edges: Vec<FlowEdge>,
    pub subgraphs: Vec<Subgraph>,
    pub styles: Vec<StyleDefinition>,
    pub class_defs: std::collections::HashMap<String, ClassDef>,
    pub clicks: Vec<ClickEvent>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum FlowDirection {
    TB, // Top to Bottom (same as TD)
    TD, // Top Down
    BT, // Bottom to Top
    RL, // Right to Left
    LR, // Left to Right
}

#[derive(Debug, Clone, PartialEq)]
pub struct FlowNode {
    pub id: String,
    pub text: Option<String>,
    pub shape: NodeShape,
    pub classes: Vec<String>,
    pub icon: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum NodeShape {
    Rectangle,           // [text]
    RoundedRectangle,   // (text)
    Stadium,            // ([text])
    Subroutine,         // [[text]]
    Cylinder,           // [(text)]
    Circle,             // ((text))
    Asymmetric,         // >text]
    Rhombus,            // {text}
    Hexagon,            // {{text}}
    Parallelogram,      // [/text/]
    ParallelogramAlt,   // [\text\]
    Trapezoid,          // [/text\]
    TrapezoidAlt,       // [\text/]
    DoubleCircle,       // (((text)))
}

#[derive(Debug, Clone, PartialEq)]
pub struct FlowEdge {
    pub from: String,
    pub to: String,
    pub edge_type: EdgeType,
    pub label: Option<String>,
    pub min_length: Option<i32>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum EdgeType {
    Arrow,              // -->
    DottedArrow,        // -.->
    ThickArrow,         // ==>
    OpenLink,           // ---
    DottedLink,         // -.-
    ThickLink,          // ===
    Invisible,          // ~~~
    CircleEdge,         // --o
    CrossEdge,          // --x
    MultiDirectional,   // <-->
}

#[derive(Debug, Clone, PartialEq)]
pub struct Subgraph {
    pub id: String,
    pub title: Option<String>,
    pub nodes: Vec<String>,     // Node IDs
    pub edges: Vec<FlowEdge>,
    pub subgraphs: Vec<Subgraph>, // Nested subgraphs
    pub direction: Option<FlowDirection>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StyleDefinition {
    pub target: StyleTarget,
    pub styles: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum StyleTarget {
    Node(String),
    Edge(String, String),
    Subgraph(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct ClassDef {
    pub name: String,
    pub styles: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ClickEvent {
    pub node_id: String,
    pub action: ClickAction,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ClickAction {
    Href(String, Option<String>), // URL, target
    Callback(String),             // Function name
    Both(String, String, Option<String>), // Callback, URL, target
}

#[derive(Debug, Clone, PartialEq)]
pub struct GanttDiagram {
    pub title: Option<String>,
    pub accessibility: AccessibilityInfo,
    pub date_format: Option<String>,
    pub axis_format: Option<String>,
    pub tick_interval: Option<String>,
    pub includes: Vec<String>,
    pub excludes: Vec<String>,
    pub today_marker: Option<String>,
    pub inclusive_end_dates: bool,
    pub top_axis: bool,
    pub weekdays: WeekdaySettings,
    pub sections: Vec<GanttSection>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GanttSection {
    pub name: String,
    pub tasks: Vec<GanttTask>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GanttTask {
    pub name: String,
    pub id: Option<String>,
    pub start_date: Option<String>,
    pub duration: Option<String>,
    pub dependencies: Vec<String>,
    pub status: TaskStatus,
    pub progress: Option<f32>,
    pub interactions: Vec<TaskInteraction>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TaskStatus {
    Active,
    Done,
    Critical,
    Milestone,
    None,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TaskInteraction {
    Click { task_id: String },
    Href { url: String },
    Call { function: String, args: Option<String> },
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct WeekdaySettings {
    pub start_day: Option<Weekday>,
    pub weekend: Vec<Weekday>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Weekday {
    Monday, Tuesday, Wednesday, Thursday, Friday, Saturday, Sunday,
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
    pub entities: std::collections::HashMap<String, Entity>,
    pub relationships: Vec<ErRelationship>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Entity {
    pub name: String,
    pub attributes: Vec<Attribute>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Attribute {
    pub name: String,
    pub attr_type: String,
    pub key_type: Option<KeyType>,
    pub comment: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum KeyType {
    PK,  // Primary Key
    FK,  // Foreign Key
    UK,  // Unique Key
}

#[derive(Debug, Clone, PartialEq)]
pub struct ErRelationship {
    pub left_entity: String,
    pub right_entity: String,
    pub left_cardinality: ErCardinality,
    pub right_cardinality: ErCardinality,
    pub label: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ErCardinality {
    pub min: CardinalityValue,
    pub max: CardinalityValue,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CardinalityValue {
    Zero,
    One,
    Many,
}

#[derive(Debug, Clone, PartialEq)]
pub struct C4Diagram {
    pub diagram_type: C4DiagramType,
    pub title: Option<String>,
    pub accessibility: AccessibilityInfo,
    pub elements: std::collections::HashMap<String, C4Element>,
    pub boundaries: Vec<C4Boundary>,
    pub relationships: Vec<C4Relationship>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum C4DiagramType {
    Context,
    Container,
    Component,
    Dynamic,
    Deployment,
}

#[derive(Debug, Clone, PartialEq)]
pub struct C4Element {
    pub id: String,
    pub element_type: C4ElementType,
    pub name: String,
    pub description: Option<String>,
    pub technology: Option<String>,
    pub tags: Vec<String>,
    pub is_external: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum C4ElementType {
    Person,
    System,
    SystemDb,
    SystemQueue,
    Container,
    ContainerDb,
    ContainerQueue,
    Component,
    ComponentDb,
    ComponentQueue,
    Node,
    DeploymentNode,
}

#[derive(Debug, Clone, PartialEq)]
pub struct C4Boundary {
    pub id: String,
    pub boundary_type: C4BoundaryType,
    pub label: String,
    pub tags: Vec<String>,
    pub elements: Vec<String>,  // Element IDs
    pub boundaries: Vec<C4Boundary>,  // Nested boundaries
}

#[derive(Debug, Clone, PartialEq)]
pub enum C4BoundaryType {
    System,
    Container,
    Enterprise,
    Generic,
}

#[derive(Debug, Clone, PartialEq)]
pub struct C4Relationship {
    pub from: String,
    pub to: String,
    pub label: Option<String>,
    pub technology: Option<String>,
    pub direction: C4RelationshipDirection,
    pub is_bidirectional: bool,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum C4RelationshipDirection {
    Default,
    Up,
    Down,
    Left,
    Right,
    Back,
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
    pub columns: Option<i32>,
    pub blocks: Vec<Block>,
    pub connections: Vec<BlockConnection>,
    pub styles: Vec<BlockStyleDefinition>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Block {
    Simple {
        id: String,
        label: Option<String>,
        shape: BlockShape,
    },
    Composite {
        id: String,
        label: Option<String>,
        blocks: Vec<Block>,
    },
    Space {
        size: Option<i32>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum BlockShape {
    Rectangle,      // Basic block
    RoundedRect,    // Rounded corners
    Rhombus,        // Diamond shape
    Circle,         // Circular
    Ellipse,        // Oval
    Cylinder,       // Database-style
    Custom(String), // Custom shape definition
}

#[derive(Debug, Clone, PartialEq)]
pub struct BlockConnection {
    pub from: String,
    pub to: String,
    pub label: Option<String>,
    pub arrow_type: BlockArrowType,
    pub style: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BlockArrowType {
    Normal,        // -->
    Dotted,        // -.->
    Thick,         // ==>
    Invisible,     // ~~~
    Bidirectional, // <-->
}

#[derive(Debug, Clone, PartialEq)]
pub struct BlockStyleDefinition {
    pub target: String,
    pub properties: Vec<BlockStyleProperty>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BlockStyleProperty {
    pub name: String,
    pub value: String,
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

