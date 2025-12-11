use sqlparser::parser::Parser;
use sqlparser::dialect::GenericDialect;

fn main() {
    let input = "MATCH (x)-[rel:EATS]->(y) DELETE rel";
    
    let dialect = GenericDialect {};
    let mut parser = Parser::new(&dialect).try_with_sql(input).expect("Failed to parse");
    
    match parser.parse_statements() {
        Ok(statements) => {
            for stmt in statements {
                let desugared = stmt.desugar_cypher_to_sql();
                println!("Input:  {}", input);
                println!("Output: {}", desugared);
            }
        }
        Err(e) => println!("Error: {}", e),
    }
}
