use logos::Logos;
use regex::Regex;
use std::collections::HashMap;

#[derive(Logos, Debug, PartialEq, Clone)]
enum Token {
    // Identifiers (keywords are still idents)
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*")]
    Ident,

    // Cypher string literal: 'text' (including spaces and special chars)
    #[regex(r"'[^']*'")]
    StringLiteral,

    // Numbers (integers and decimals)
    #[regex(r"[0-9]+\.?[0-9]*")]
    Number,

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
    #[token("{")]
    LBrace,
    #[token("}")]
    RBrace,
    #[token(",")]
    Comma,

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
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        return;
    }

    let cypher_query = args[1..].join(" ");

    let tokens = tokenize(&cypher_query);
    let token_refs: Vec<&MyToken> = tokens.iter().collect();

    let mut sql = String::new();

    if token_refs.len() >= 1 {
        if token_refs.len() >= 4 && token_refs[0].text.to_uppercase() == "MATCH" {
            if token_refs[1].tok == Token::LParen {
                // MATCH (n:Label) RETURN ...
                if token_refs.len() >= 8
                    && token_refs[3].tok == Token::Colon
                    && token_refs[5].tok == Token::RParen
                    && token_refs[6].text.to_uppercase() == "RETURN"
                {
                    let name = &token_refs[4].text;
                    sql = format!("SELECT * FROM nodes WHERE label = '{}';", name);
                }
                // MATCH (n) RETURN
                else if token_refs.len() >= 6
                    && token_refs[3].tok == Token::RParen
                    && token_refs[4].text.to_uppercase() == "RETURN"
                {
                    sql = "SELECT * FROM nodes;".to_string();
                }
            }
        }

        else if token_refs[0].text.to_uppercase() == "RETURN" {
            // RETURN 'value' AS name (string)
            if token_refs.len() == 4
                && token_refs[1].tok == Token::StringLiteral
                && token_refs[2].text.to_uppercase() == "AS"
                && token_refs[3].tok == Token::Ident
            {
                let value = &token_refs[1].text;
                let alias = &token_refs[3].text;
                sql = format!("SELECT {} AS {};", value, alias);
            }
            // RETURN number AS name
            else if token_refs.len() == 4
                && token_refs[1].tok == Token::Number
                && token_refs[2].text.to_uppercase() == "AS"
                && token_refs[3].tok == Token::Ident
            {
                let value = &token_refs[1].text;
                let alias = &token_refs[3].text;
                sql = format!("SELECT {} AS {};", value, alias);
            }
            // RETURN 'value' (no alias)
            else if token_refs.len() == 2
                && token_refs[1].tok == Token::StringLiteral
            {
                sql = format!("SELECT {};", token_refs[1].text);
            }
            // RETURN number (no alias)
            else if token_refs.len() == 2
                && token_refs[1].tok == Token::Number
            {
                sql = format!("SELECT {};", token_refs[1].text);
            }
        }

        else if token_refs[0].text.to_uppercase() == "CREATE" {
            // CREATE (n:Label)
            if token_refs.len() >= 6
                && token_refs[1].tok == Token::LParen
                && token_refs[3].tok == Token::Colon
                && token_refs[5].tok == Token::RParen
            {
                let label = &token_refs[4].text;
                sql = format!("INSERT INTO nodes (label, properties) VALUES ('{}', '{{}}'); SELECT * FROM nodes WHERE id = last_insert_rowid();", label);
            } else if token_refs.len() >= 4
                && token_refs[1].tok == Token::LParen
                && token_refs[3].tok == Token::RParen
            {
                sql = "INSERT INTO nodes (label, properties) VALUES ('', '{}'); SELECT * FROM nodes WHERE id = last_insert_rowid();".to_string();
            } else if token_refs.len() >= 11 
                && token_refs[1].tok == Token::LParen
                && token_refs[3].tok == Token::Colon
                && token_refs[5].tok == Token::LBrace
            {
                let label = &token_refs[4].text;
                
                // Find the closing brace
                let mut brace_end = 0;
                for i in 6..token_refs.len() {
                    if token_refs[i].tok == Token::RBrace {
                        brace_end = i;
                        if i + 1 >= token_refs.len() || token_refs[i+1].tok != Token::RParen {
                            // Invalid syntax
                            println!("");
                            return;
                        }
                        break;
                    }
                }
                
                if brace_end == 0 {
                    // No closing brace found
                    println!("");
                    return;
                }
                
                // Parse properties between braces
                let mut properties = std::collections::HashMap::new();
                let mut i = 6;
                
                while i < brace_end {
                    // Expect: key : value [, key : value ...]
                    if i + 2 < brace_end 
                        && token_refs[i].tok == Token::Ident 
                        && token_refs[i+1].tok == Token::Colon
                    {
                        let key = &token_refs[i].text;
                        let value = if token_refs[i+2].tok == Token::StringLiteral {
                            // Remove quotes from string literal
                            token_refs[i+2].text.trim_matches('\'').to_string()
                        } else if token_refs[i+2].tok == Token::Number {
                            token_refs[i+2].text.clone()
                        } else {
                            token_refs[i+2].text.clone()
                        };
                        
                        properties.insert(key.clone(), value);
                        i += 3; // Move past key : value
                        
                        // Skip comma if present
                        if i < brace_end && token_refs[i].tok == Token::Comma {
                            i += 1;
                        }
                    } else {
                        i += 1;
                    }
                }
                
                // Convert properties to JSON string
                let json_props = properties.iter()
                    .map(|(k, v)| format!("\"{}\":\"{}\"", k, v))
                    .collect::<Vec<String>>()
                    .join(",");
                
                sql = format!(
                    "INSERT INTO nodes (label, properties) VALUES ('{}', '{{{}}}'); SELECT * FROM nodes WHERE id = last_insert_rowid();",
                    label, json_props
                );
            }
        }
    }

    println!("{}", sql);
}