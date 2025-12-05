use logos::Logos;
use regex::Regex;

#[derive(Logos, Debug, PartialEq, Clone)]
enum Token {
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*")]
    Ident,

    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[token("[")]
    LBracket,
    #[token("]")]
    RBracket,
    #[token(":")]
    Colon,
    #[token("-")]
    Dash,

    #[regex(r"\s+", logos::skip)]
    Whitespace,

}

#[derive(Debug, Clone)]
struct MyToken {
    tok: Token,
    text: String,
}

fn tokenize(input: &str) -> Vec<MyToken> {
    Token::lexer(input)
        .spanned()
        .filter_map(|(res, span)| {
            res.ok().map(|tok| MyToken {
                tok,
                text: input[span].to_string(),
            })
        })
        .collect()
}

fn main() {
    //eprintln!("Usage: transformer <cypher-query>");
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        //eprintln!("Requires two arguments at least : transformer <cypher-query>");
        return;
    } else {
        let cypher_query = args[1..].join(" ");
        //println!("Transforming Cypher query test: {}", cypher_query);

        /*
        let sql = if cypher_query.to_uppercase().starts_with("MATCH") {
            "SELECT * FROM nodes;".to_string()
        } else {
            format!("/* cypher: {} */\nSELECT 'untranslated' as placeholder;", cypher_query)
        };
        */

        let tokens = tokenize(&cypher_query);

        for token in &tokens {
            //println!("{:?} => {:?}", token.tok, token.text);
        }

        //println!("--- Accessing specific token by index ---");

        let i = 2;
        if let Some(token) = tokens.get(i) {
            //println!("tokens[{}] = {:?} => {:?}", i, token.tok, token.text);
        } else {
            //println!("No token at index {}", i);
        }

         //feature 1 : MATCH to SQL translation
        //parse into tokens of MATCH keyword, stuff in parentheses, etc.
        //if colon after n? then add label = word 
        //add : MATCH (a)-[:KNOWS]->(b) => SELECT * FROM nodes a JOIN edges e ON a.id = e.src_id JOIN nodes b ON e.dst_id = b.id WHERE e.type = 'KNOWS';
        //MATCH (a)-[r:TYPE]-(b) => WHERE e.type = 'TYPE'
        //MATCH (a)-[*2..4]->(b) => WHERE e.hops BETWEEN 2 AND 4
        //etc.  

        let tokens = tokenize(&cypher_query);

        let mut sql = String::new();

        // Get references to all tokens at the beginning
        let token_refs: Vec<&MyToken> = tokens.iter().collect();

        if token_refs.len() >= 4 {
            if token_refs[0].text == "MATCH" && token_refs[1].tok == Token::LParen {
                // Node with label case (needs 5 tokens)
                if token_refs.len() >= 6
                && token_refs[0].text.to_uppercase() == "MATCH"
                && token_refs[1].tok == Token::LParen
                && token_refs[3].tok == Token::RParen
                && token_refs[4].text.to_uppercase() == "RETURN"
                {
                    sql = "SELECT * FROM nodes;".to_string();
                }
                else if token_refs.len() >= 5
                    && token_refs[3].tok == Token::Colon
                    && token_refs[2].text == "n"
                    && token_refs[4].tok == Token::RParen
                {
                    sql = format!("SELECT * FROM nodes WHERE label = '{}';", token_refs[2].text);
                }
                // Node without label case (exactly 4 tokens)
                else if token_refs[3].tok == Token::RParen && token_refs[2].text == "n" && token_refs.len() == 4
                {
                    sql = "SELECT * FROM nodes;".to_string();
                }
            }
        }

        //println!("Generated SQL: {}", sql);






        //NEXT : WHERE clause translation
        //WHERE a.property = 'value' => WHERE a.property = 'value'
        
        //RETURN count(N) => SELECT COUNT(*)
        //RETURN a, b => SELECT a.*, b.*

        //CREATE (n:Person {name: "Bob"}) => INSERT INTO nodes (label, name) VALUES ('Person', 'Bob');

        //DELETE n => DELETE FROM nodes WHERE id = n.id;

        //DETACH DELETE n => DELETE FROM nodes WHERE id = n.id; DELETE FROM edges WHERE src_id = n.id OR dst_id = n.id;

        println!("{}", sql);
    }
    
}