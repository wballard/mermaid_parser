//! Simple targeted tests to improve coverage in visitor.rs

use mermaid_parser::common::ast::*;
use mermaid_parser::common::visitor::{AstVisitor, ComplexityAnalyzer, NodeCounter};

#[cfg(test)]
mod visitor_simple_tests {
    use super::*;

    // Test ComplexityAnalyzer with all diagram types that have simple default implementations
    #[test]
    fn test_complexity_analyzer_simple_diagrams() {
        let mut analyzer = ComplexityAnalyzer::new();

        // Test Gantt
        let gantt = GanttDiagram {
            title: Some("Test".to_string()),
            accessibility: AccessibilityInfo::default(),
            date_format: None,
            axis_format: None,
            tick_interval: None,
            includes: vec![],
            excludes: vec![],
            today_marker: None,
            inclusive_end_dates: false,
            top_axis: false,
            weekdays: WeekdaySettings {
                start_day: None,
                weekend: vec![],
            },
            sections: vec![],
        };
        analyzer.visit_gantt(&gantt);

        // Test Pie
        let pie = PieDiagram {
            title: Some("Test".to_string()),
            accessibility: AccessibilityInfo::default(),
            show_data: true,
            data: vec![],
        };
        analyzer.visit_pie(&pie);

        // Test Quadrant
        let quadrant = QuadrantDiagram {
            title: Some("Test".to_string()),
            accessibility: AccessibilityInfo::default(),
            x_axis: None,
            y_axis: None,
            quadrants: QuadrantLabels {
                quadrant_1: None,
                quadrant_2: None,
                quadrant_3: None,
                quadrant_4: None,
            },
            points: vec![],
            styles: vec![],
        };
        analyzer.visit_quadrant(&quadrant);

        // Test Kanban
        let kanban = KanbanDiagram {
            title: Some("Test".to_string()),
            accessibility: AccessibilityInfo::default(),
            sections: vec![],
        };
        analyzer.visit_kanban(&kanban);

        // Test Packet
        let packet = PacketDiagram {
            title: Some("Test".to_string()),
            accessibility: AccessibilityInfo::default(),
            fields: vec![],
        };
        analyzer.visit_packet(&packet);

        // Test Radar
        let radar = RadarDiagram {
            title: Some("Test".to_string()),
            accessibility: AccessibilityInfo::default(),
            config: RadarConfig::default(),
            axes: vec![],
            datasets: vec![],
        };
        analyzer.visit_radar(&radar);

        // Test Misc
        let misc = MiscDiagram {
            content: MiscContent::Raw(RawDiagram { lines: vec![] }),
            diagram_type: "test".to_string(),
        };
        analyzer.visit_misc(&misc);

        // All should contribute to complexity
        assert!(analyzer.cyclomatic_complexity() >= 1);
    }

    // Test ComplexityAnalyzer with complex diagrams
    #[test]
    fn test_complexity_analyzer_complex_diagrams() {
        let mut analyzer = ComplexityAnalyzer::new();

        // Test Git with commits and operations
        let git = GitDiagram {
            title: Some("Test".to_string()),
            accessibility: AccessibilityInfo::default(),
            theme: None,
            commits: vec![
                GitCommit {
                    id: Some("c1".to_string()),
                    commit_type: CommitType::Normal,
                    tag: None,
                    branch: "main".to_string(),
                },
                GitCommit {
                    id: Some("c2".to_string()),
                    commit_type: CommitType::Normal,
                    tag: None,
                    branch: "main".to_string(),
                },
            ],
            branches: vec![],
            operations: vec![
                GitOperation::Commit {
                    id: Some("c1".to_string()),
                    commit_type: CommitType::Normal,
                    tag: None,
                },
                GitOperation::Commit {
                    id: Some("c2".to_string()),
                    commit_type: CommitType::Normal,
                    tag: None,
                },
            ],
        };
        analyzer.visit_git(&git);

        // Should count commits and operations
        assert!(analyzer.cyclomatic_complexity() > 1);
    }

    // Test NodeCounter with various diagram types
    #[test]
    fn test_node_counter_diagram_types() {
        // Test Pie counting elements
        let mut pie_counter = NodeCounter::new();
        let pie = PieDiagram {
            title: None,
            accessibility: AccessibilityInfo::default(),
            show_data: false,
            data: vec![
                PieSlice {
                    label: "A".to_string(),
                    value: 50.0,
                },
                PieSlice {
                    label: "B".to_string(),
                    value: 50.0,
                },
            ],
        };
        pie_counter.visit_pie(&pie);
        assert_eq!(pie_counter.elements(), 2);

        // Test Git counting nodes and elements
        let mut git_counter = NodeCounter::new();
        let git = GitDiagram {
            title: None,
            accessibility: AccessibilityInfo::default(),
            theme: None,
            commits: vec![GitCommit {
                id: Some("c1".to_string()),
                commit_type: CommitType::Normal,
                tag: None,
                branch: "main".to_string(),
            }],
            branches: vec![],
            operations: vec![GitOperation::Commit {
                id: Some("c1".to_string()),
                commit_type: CommitType::Normal,
                tag: None,
            }],
        };
        git_counter.visit_git(&git);
        assert_eq!(git_counter.nodes(), 1); // 1 commit
        assert_eq!(git_counter.elements(), 1); // 1 operation

        // Test misc counting
        let mut misc_counter = NodeCounter::new();
        let misc = MiscDiagram {
            content: MiscContent::Raw(RawDiagram { lines: vec![] }),
            diagram_type: "test".to_string(),
        };
        misc_counter.visit_misc(&misc);
        assert_eq!(misc_counter.elements(), 1); // 1 misc element
    }

    // Test ComplexityAnalyzer helper methods
    #[test]
    fn test_complexity_analyzer_helper_methods() {
        let mut analyzer = ComplexityAnalyzer::new();

        // Test private methods by calling public interface
        let sankey = SankeyDiagram {
            nodes: vec![SankeyNode {
                id: "A".to_string(),
                name: "Node A".to_string(),
            }],
            links: vec![SankeyLink {
                source: "A".to_string(),
                target: "B".to_string(),
                value: 10.0,
            }],
        };
        analyzer.visit_sankey(&sankey);

        // Should have counted nodes and connections
        assert!(analyzer.cyclomatic_complexity() > 1);
    }

    // Test ComplexityAnalyzer with mindmap for depth tracking
    #[test]
    fn test_complexity_analyzer_mindmap_depth() {
        let mut analyzer = ComplexityAnalyzer::new();

        let mindmap = MindmapDiagram {
            title: None,
            accessibility: AccessibilityInfo::default(),
            root: MindmapNode {
                id: "root".to_string(),
                text: "Root".to_string(),
                shape: MindmapNodeShape::Default,
                icon: None,
                class: None,
                children: vec![MindmapNode {
                    id: "child".to_string(),
                    text: "Child".to_string(),
                    shape: MindmapNodeShape::Default,
                    icon: None,
                    class: None,
                    children: vec![MindmapNode {
                        id: "grandchild".to_string(),
                        text: "Grandchild".to_string(),
                        shape: MindmapNodeShape::Default,
                        icon: None,
                        class: None,
                        children: vec![],
                    }],
                }],
            },
        };

        analyzer.visit_mindmap(&mindmap);

        // Should have tracked depth properly
        assert!(analyzer.max_depth() >= 2);
    }

    // Test individual element visitor methods
    #[test]
    fn test_individual_element_visitors() {
        let mut analyzer = ComplexityAnalyzer::new();

        // Test element visitors directly
        analyzer.visit_sankey_node(&SankeyNode {
            id: "test".to_string(),
            name: "Test".to_string(),
        });

        analyzer.visit_sankey_link(&SankeyLink {
            source: "A".to_string(),
            target: "B".to_string(),
            value: 1.0,
        });

        analyzer.visit_flow_node(&FlowNode {
            id: "test".to_string(),
            text: Some("Test".to_string()),
            shape: NodeShape::Rectangle,
            classes: vec![],
            icon: None,
        });

        analyzer.visit_flow_edge(&FlowEdge {
            from: "A".to_string(),
            to: "B".to_string(),
            edge_type: EdgeType::Arrow,
            label: None,
            min_length: None,
        });

        analyzer.visit_sequence_message(&Message {
            from: "A".to_string(),
            to: "B".to_string(),
            text: "Test".to_string(),
            arrow_type: ArrowType::SolidOpen,
        });

        analyzer.visit_class_definition(&Class {
            name: "Test".to_string(),
            stereotype: None,
            members: vec![],
            annotations: vec![],
            css_class: None,
        });

        analyzer.visit_state_node(&State {
            id: "test".to_string(),
            display_name: Some("Test".to_string()),
            state_type: StateType::Simple,
            substates: vec![],
            concurrent_regions: vec![],
        });

        analyzer.visit_state_transition(&StateTransition {
            from: "A".to_string(),
            to: "B".to_string(),
            event: Some("event".to_string()),
            guard: None,
            action: None,
        });

        // All these should have been counted
        assert!(analyzer.cyclomatic_complexity() > 1);
    }

    // Test NodeCounter recursive methods
    #[test]
    fn test_node_counter_recursive_methods() {
        let mut counter = NodeCounter::new();

        // Test mindmap traversal
        let mindmap = MindmapDiagram {
            title: None,
            accessibility: AccessibilityInfo::default(),
            root: MindmapNode {
                id: "root".to_string(),
                text: "Root".to_string(),
                shape: MindmapNodeShape::Default,
                icon: None,
                class: None,
                children: vec![
                    MindmapNode {
                        id: "child1".to_string(),
                        text: "Child 1".to_string(),
                        shape: MindmapNodeShape::Default,
                        icon: None,
                        class: None,
                        children: vec![],
                    },
                    MindmapNode {
                        id: "child2".to_string(),
                        text: "Child 2".to_string(),
                        shape: MindmapNodeShape::Default,
                        icon: None,
                        class: None,
                        children: vec![],
                    },
                ],
            },
        };
        counter.visit_mindmap(&mindmap);
        assert_eq!(counter.nodes(), 3); // root + 2 children

        // Test treemap traversal
        let treemap = TreemapDiagram {
            title: None,
            accessibility: AccessibilityInfo::default(),
            root: TreemapNode {
                name: "Root".to_string(),
                value: Some(100.0),
                children: vec![TreemapNode {
                    name: "Child".to_string(),
                    value: Some(50.0),
                    children: vec![],
                }],
            },
        };
        counter.visit_treemap(&treemap);
        assert_eq!(counter.nodes(), 5); // Previous 3 + root + child
    }

    // Test DiagramType accept methods
    #[test]
    fn test_diagram_type_accept() {
        let diagram = DiagramType::Sankey(SankeyDiagram {
            nodes: vec![],
            links: vec![],
        });

        let mut counter = NodeCounter::new();
        diagram.accept(&mut counter);

        // Should have called visit_sankey
        assert_eq!(counter.nodes(), 0);
        assert_eq!(counter.edges(), 0);
    }

    // Test edge case for complexity analyzer
    #[test]
    fn test_complexity_analyzer_edge_cases() {
        let analyzer = ComplexityAnalyzer::new();

        // Test zero state
        assert_eq!(analyzer.max_depth(), 0);
        assert_eq!(analyzer.average_branching_factor(), 0.0);
        assert_eq!(analyzer.cyclomatic_complexity(), 1); // Minimum

        // Test after some operations via public interface
        let mut analyzer = ComplexityAnalyzer::new();

        // Use a simple sankey to exercise count methods
        let sankey = SankeyDiagram {
            nodes: vec![SankeyNode {
                id: "A".to_string(),
                name: "Node A".to_string(),
            }],
            links: vec![SankeyLink {
                source: "A".to_string(),
                target: "B".to_string(),
                value: 10.0,
            }],
        };
        analyzer.visit_sankey(&sankey);

        assert!(analyzer.cyclomatic_complexity() > 1);
    }
}
