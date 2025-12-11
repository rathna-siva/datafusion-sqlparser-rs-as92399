use sqlparser::parser::Parser;
use sqlparser::dialect::GenericDialect;

fn main() {
    let input = "MATCH ()-[r]->() DELETE r";
    
    let dialect = GenericDialect {};
    let mut parser = Parser::new(&dialect).try_with_sql(input).expect("Failed to parse");
    
    match parser.parse_statements() {
        Ok(statements) => {
            for stmt in statements {
                println!("Original statement: {:?}", stmt);
                let desugared = stmt.desugar_cypher_to_sql();
                println!("Desugared statement: {:?}", desugared);
                println!("Desugared display: {}", desugared);
            }
        }
        Err(e) => println!("Parse error: {}", e),
    }
}
