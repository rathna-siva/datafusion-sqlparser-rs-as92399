# Edge Deletion Implementation

## Feature
Support for Cypher edge deletion pattern conversion to SQL DELETE statement.

## Input Examples
```cypher
MATCH ()-[r]->() DELETE r
MATCH (a)-[r]->(b) DELETE r
MATCH (x)-[rel:EATS]->(y) DELETE rel
```

## Output (Desugared SQL)
```sql
DELETE FROM edges
```

## Implementation Details

### Parser Changes (src/parser/mod.rs - `parse_cypher_match()`)

1. **Anonymous node support**: Modified parser to handle anonymous nodes `()` by assigning them a placeholder name `_anonymous`
   ```rust
   let first_var = if matches!(self.peek_token().token, Token::RParen) {
       "_anonymous".to_string()
   } else {
       self.parse_identifier()?.to_string()
   };
   ```

2. **Edge deletion detection**: After parsing the edge pattern `()-[r]->()`, the parser checks for a DELETE keyword:
   ```rust
   if self.peek_keyword(Keyword::DELETE) {
       self.expect_keyword(Keyword::DELETE)?;
       let _delete_var = self.parse_identifier()?.to_string();
       return Ok(Statement::CypherDelete {
           node_or_edge_name: edge_name,
           is_edge: true,              // Mark as edge deletion
           detach: false,
           label: None,
           properties: None,
       });
   }
   ```

### Desugaring (src/ast/mod.rs - `handle_delete_node()`)

The existing `handle_delete_node()` function already supports both node and edge deletion:
- When `is_edge: true`, it uses table name `"edges"` 
- When `is_edge: false`, it uses table name `"nodes"`

```rust
let table_name = if *is_edge {
    "edges".to_string()
} else {
    "nodes".to_string()
};
```

## Tested Patterns
✅ Anonymous edge patterns: `MATCH ()-[r]->() DELETE r`
✅ Named node patterns: `MATCH (a)-[r]->(b) DELETE r`
✅ Typed relationships: `MATCH (x)-[rel:EATS]->(y) DELETE rel`

All convert to: `DELETE FROM edges`
