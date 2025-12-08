use std::process::Command;

#[test]
fn test_match_all_nodes() {
    let output = Command::new("./target/release/transformer")
        .arg("MATCH (n) RETURN n")
        .output()
        .expect("Failed to execute transformer");
    
    let sql = String::from_utf8(output.stdout).unwrap();
    assert_eq!(sql.trim(), "SELECT * FROM nodes;");
}

#[test]
fn test_match_with_label() {
    let output = Command::new("./target/release/transformer")
        .arg("MATCH (n:Bug) RETURN n")
        .output()
        .expect("Failed to execute transformer");
    
    let sql = String::from_utf8(output.stdout).unwrap();
    assert_eq!(sql.trim(), "SELECT * FROM nodes WHERE label = 'Bug';");
}

#[test]
fn test_return_string() {
    let output = Command::new("./target/release/transformer")
        .arg("RETURN 'hello' AS message")
        .output()
        .expect("Failed to execute transformer");
    
    let sql = String::from_utf8(output.stdout).unwrap();
    assert_eq!(sql.trim(), "SELECT 'hello' AS message;");
}

#[test]
fn test_return_number() {
    let output = Command::new("./target/release/transformer")
        .arg("RETURN 42 AS answer")
        .output()
        .expect("Failed to execute transformer");
    
    let sql = String::from_utf8(output.stdout).unwrap();
    assert_eq!(sql.trim(), "SELECT 42 AS answer;");
}

#[test]
fn test_create_simple() {
    let output = Command::new("./target/release/transformer")
        .arg("CREATE (n:Bug)")
        .output()
        .expect("Failed to execute transformer");
    
    let sql = String::from_utf8(output.stdout).unwrap();
    assert!(sql.contains("INSERT INTO nodes"));
    assert!(sql.contains("Bug"));
}

#[test]
fn test_create_with_properties() {
    let output = Command::new("./target/release/transformer")
        .arg("CREATE (n:Bug {name: 'Ant'})")
        .output()
        .expect("Failed to execute transformer");
    
    let sql = String::from_utf8(output.stdout).unwrap();
    assert!(sql.contains("INSERT INTO nodes"));
    assert!(sql.contains("\"name\""));
    assert!(sql.contains("Ant"));
}