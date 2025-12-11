use sqlparser::parser::Parser;
use sqlparser::dialect::GenericDialect;

fn main() {
    let test_cases = vec![
        "MATCH (a)-[r]->(b) RETURN a, r, b",
        "MATCH (a)-[r:EATS]->(b) RETURN a, r, b",
    ];
    
    let dialect = GenericDialect {};
    
    for input in test_cases {
        println!("Input:  {}", input);
        let mut parser = Parser::new(&dialect).try_with_sql(input).expect("Failed to parse");
        
        match parser.parse_statements() {
            Ok(statements) => {
                for stmt in statements {
                    let desugared = stmt.desugar_cypher_to_sql();
                    println!("Output: {}", desugared);
                }
            }
            Err(e) => println!("Error: {}", e),
        }
        println!();
    }
}
