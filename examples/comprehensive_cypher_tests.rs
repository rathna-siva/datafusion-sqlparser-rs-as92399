use sqlparser::parser::Parser;
use sqlparser::dialect::GenericDialect;

fn main() {
    let test_cases = vec![
        // ===== CREATE NODE TESTS =====
        ("CREATE (b:Bug)", "Create simple Bug node"),
        ("CREATE (b:Bug {name: 'Ant'})", "Create Bug node with name property"),
        ("CREATE (b:Bug {name: 'Butterfly', color: 'Orange'})", "Create Bug node with name and color"),
        ("CREATE (b:Bug {color: 'Red'})", "Create Bug node with only color property"),
        
        // ===== MATCH NODE TESTS =====
        ("MATCH (b:Bug) RETURN b", "Match all Bug nodes"),
        ("MATCH (b:Bug {name: 'Ant'}) RETURN b", "Match Bug with specific name"),
        ("MATCH (b:Bug {name: 'Wasp', color: 'Black'}) RETURN b", "Match Bug with name and color"),
        ("MATCH (b:Bug {color: 'Red'}) RETURN b", "Match Bug with specific color"),
        
        // ===== MATCH EDGE TESTS =====
        ("MATCH (a)-[r]->(b) RETURN a, r, b", "Match all edges"),
        ("MATCH (a)-[r:EATS]->(b) RETURN a, r, b", "Match edges with EATS relationship"),
        ("MATCH (x)-[rel:HUNTS]->(y) RETURN x, rel, y", "Match edges with HUNTS relationship"),
        
        // ===== DELETE NODE TESTS =====
        ("MATCH (b:Bug {name: 'Moth'}) DELETE b", "Delete Bug node with specific name"),
        ("MATCH (b:Bug {color: 'Green'}) DELETE b", "Delete Bug node with specific color"),
        ("MATCH (b:Bug {name: 'Bee', color: 'Yellow'}) DELETE b", "Delete Bug with name and color"),
        ("MATCH (b:Bug) DETACH DELETE b", "Delete all Bug nodes with DETACH"),
        
        // ===== DELETE EDGE TESTS =====
        ("MATCH ()-[r]->() DELETE r", "Delete all edges"),
        ("MATCH ()-[r:EATS]->() DELETE r", "Delete all EATS edges"),
        ("MATCH ()-[r:HUNTS]->() DELETE r", "Delete all HUNTS edges"),
        
        // ===== CREATE RELATIONSHIP TESTS =====
        ("MATCH (a:Bug {name: 'Ant'}), (b:Bug {name: 'Bee'}) CREATE (a)-[:EATS]->(b)", "Create EATS relationship between specific bugs"),
        ("MATCH (x:Bug {name: 'Wasp'}), (y:Bug {name: 'Moth'}) CREATE (x)-[:HUNTS]->(y)", "Create HUNTS relationship between specific bugs"),
        ("MATCH (p:Bug {color: 'Red'}), (q:Bug {color: 'Blue'}) CREATE (p)-[:AVOIDS]->(q)", "Create AVOIDS relationship between colored bugs"),
    ];
    
    let dialect = GenericDialect {};
    
    println!("╔══════════════════════════════════════════════════════════════════════════╗");
    println!("║                   CYPHER TO SQL CONVERSION TESTS                          ║");
    println!("╚══════════════════════════════════════════════════════════════════════════╝\n");
    
    for (input, description) in test_cases {
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!("Test: {}", description);
        println!("Cypher: {}", input);
        
        let mut parser = Parser::new(&dialect).try_with_sql(input).expect("Failed to create parser");
        
        match parser.parse_statements() {
            Ok(statements) => {
                for stmt in statements {
                    let desugared = stmt.desugar_cypher_to_sql();
                    println!("SQL:    {}", desugared);
                }
            }
            Err(e) => println!("Error:  {}", e),
        }
        println!();
    }
    
    println!("╔══════════════════════════════════════════════════════════════════════════╗");
    println!("║                         ALL TESTS COMPLETED                              ║");
    println!("╚══════════════════════════════════════════════════════════════════════════╝");
}
