use mermaid_parser::common::ast::ParticipantType;
use mermaid_parser::parsers::sequence;

#[test]
fn test_simple_sequence() {
    let input = r#"sequenceDiagram
    participant Alice
    participant Bob
    Alice->Bob: Hello Bob
    Bob-->>Alice: Hello Alice!"#;

    let result = sequence::parse(input);
    assert!(result.is_ok());
    let diagram = result.unwrap();

    // Should have 2 explicitly declared participants
    let explicit_participants = diagram
        .participants
        .iter()
        .filter(|p| p.actor == "Alice" || p.actor == "Bob")
        .count();
    assert_eq!(explicit_participants, 2);
    assert_eq!(diagram.statements.len(), 2);
}

#[test]
fn test_actor_declaration() {
    let input = r#"sequenceDiagram
    actor Alice
    participant Bob as B
    Alice->B: Message"#;

    let result = sequence::parse(input);
    assert!(result.is_ok());
    let diagram = result.unwrap();

    // Print debug info
    println!("Participants: {:?}", diagram.participants);

    // Should have 2 declared participants (Alice and Bob)
    // B is an alias for Bob, not a separate participant
    let alice = diagram.participants.iter().find(|p| p.actor == "Alice");
    let bob = diagram.participants.iter().find(|p| p.actor == "Bob");

    assert!(alice.is_some());
    assert!(bob.is_some());

    assert_eq!(alice.unwrap().participant_type, ParticipantType::Actor);
    assert_eq!(bob.unwrap().participant_type, ParticipantType::Participant);
    assert_eq!(bob.unwrap().alias, Some("B".to_string()));
}

#[test]
fn test_arrow_types() {
    let tests = vec![
        ("->", "SolidOpen"),
        ("->>", "SolidClosed"),
        ("-->", "DottedOpen"),
        ("-->>", "DottedClosed"),
        ("-x", "Cross"),
        ("-)", "Point"),
    ];

    for (arrow_str, _expected_type) in tests {
        let input = format!("sequenceDiagram\n    Alice{}Bob: Test", arrow_str);
        let result = sequence::parse(&input);
        assert!(result.is_ok(), "Failed to parse arrow type: {}", arrow_str);
    }
}

#[test]
fn test_loop_statement() {
    let input = r#"sequenceDiagram
    participant Alice
    participant Bob
    loop Every minute
        Alice->Bob: Check status
        Bob-->>Alice: OK
    end"#;

    let result = sequence::parse(input);
    assert!(result.is_ok());
}

#[test]
fn test_alt_statement() {
    let input = r#"sequenceDiagram
    participant Alice
    participant Bob
    alt Success case
        Alice->Bob: Request
        Bob-->>Alice: Success
    else Failure case
        Bob-->>Alice: Error
    end"#;

    let result = sequence::parse(input);
    assert!(result.is_ok());
}

#[test]
fn test_note_positions() {
    let input = r#"sequenceDiagram
    participant Alice
    participant Bob
    note left of Alice: Left note
    note right of Bob: Right note
    note over Alice,Bob: Over both"#;

    let result = sequence::parse(input);
    assert!(result.is_ok());
}

#[test]
fn test_activation() {
    let input = r#"sequenceDiagram
    participant Alice
    participant Bob
    Alice->Bob: Request
    activate Bob
    Bob-->>Alice: Processing
    deactivate Bob"#;

    let result = sequence::parse(input);
    assert!(result.is_ok());
}

#[test]
fn test_autonumber() {
    let input = r#"sequenceDiagram
    autonumber
    participant Alice
    participant Bob
    Alice->Bob: Step 1
    Bob-->>Alice: Step 2"#;

    let result = sequence::parse(input);
    assert!(result.is_ok());
}

#[test]
fn test_title() {
    let input = r#"sequenceDiagram
    title My Sequence Diagram
    participant Alice
    Alice->Alice: Self message"#;

    let result = sequence::parse(input);
    assert!(result.is_ok());
    let diagram = result.unwrap();
    assert_eq!(diagram.title, Some("My Sequence Diagram".to_string()));
}

#[test]
fn test_complex_example() {
    let input = r#"sequenceDiagram
    title Authentication Flow
    participant User
    participant Browser
    participant Server
    participant Database
    
    User->>Browser: Enter credentials
    Browser->>Server: POST /login
    activate Server
    Server->>Database: Query user
    activate Database
    Database-->>Server: User data
    deactivate Database
    
    alt Valid credentials
        Server-->>Browser: 200 OK + Token
        Browser-->>User: Login successful
    else Invalid credentials
        Server-->>Browser: 401 Unauthorized
        Browser-->>User: Login failed
    end
    deactivate Server
    
    note right of Server: Validate and generate JWT"#;

    let result = sequence::parse(input);
    assert!(result.is_ok());
}
