# Programmatically Constructing DELETE Statements

This document demonstrates how to construct SQL DELETE statements programmatically using the AST without parsing.

## Example 1: Simple DELETE with WHERE Clause

Delete all rows from a table matching a condition:

```rust
use sqlparser::ast::*;

let delete_stmt = Statement::Delete {
    table_name: ObjectName(vec![Ident {
        value: "users".to_string(),
        quote_style: None,
    }]),
    using: None,
    selection: Some(Expr::BinaryOp {
        left: Box::new(Expr::Identifier(Ident {
            value: "age".to_string(),
            quote_style: None,
        })),
        op: BinaryOperator::Gt,
        right: Box::new(Expr::Value(Value::Number("65".to_string(), false))),
    }),
    order_by: vec![],
    limit: None,
    returning: None,
};

// Equivalent SQL: DELETE FROM users WHERE age > 65
```

## Example 2: DELETE with Multiple Conditions

Delete with AND operator combining multiple conditions:

```rust
use sqlparser::ast::*;

let delete_stmt = Statement::Delete {
    table_name: ObjectName(vec![Ident {
        value: "products".to_string(),
        quote_style: None,
    }]),
    using: None,
    selection: Some(Expr::BinaryOp {
        left: Box::new(Expr::BinaryOp {
            left: Box::new(Expr::Identifier(Ident {
                value: "status".to_string(),
                quote_style: None,
            })),
            op: BinaryOperator::Equal,
            right: Box::new(Expr::Value(Value::SingleQuotedString("inactive".to_string()))),
        }),
        op: BinaryOperator::And,
        right: Box::new(Expr::BinaryOp {
            left: Box::new(Expr::Identifier(Ident {
                value: "stock".to_string(),
                quote_style: None,
            })),
            op: BinaryOperator::Equal,
            right: Box::new(Expr::Value(Value::Number("0".to_string(), false))),
        }),
    }),
    using: None,
    order_by: vec![],
    limit: None,
    returning: None,
};

// Equivalent SQL: DELETE FROM products WHERE status = 'inactive' AND stock = 0
```

## Example 3: DELETE All Rows (No WHERE Clause)

Delete all rows from a table:

```rust
use sqlparser::ast::*;

let delete_stmt = Statement::Delete {
    table_name: ObjectName(vec![Ident {
        value: "logs".to_string(),
        quote_style: None,
    }]),
    using: None,
    selection: None,  // No WHERE clause - deletes all rows
    order_by: vec![],
    limit: None,
    returning: None,
};

// Equivalent SQL: DELETE FROM logs
```

## Example 4: DELETE with Qualified Table Name

Delete from a table in a specific schema:

```rust
use sqlparser::ast::*;

let delete_stmt = Statement::Delete {
    table_name: ObjectName(vec![
        Ident {
            value: "public".to_string(),
            quote_style: None,
        },
        Ident {
            value: "users".to_string(),
            quote_style: None,
        },
    ]),
    using: None,
    selection: Some(Expr::BinaryOp {
        left: Box::new(Expr::Identifier(Ident {
            value: "deleted_at".to_string(),
            quote_style: None,
        })),
        op: BinaryOperator::IsNotNull,
        right: Box::new(Expr::Value(Value::Null)),
    }),
    order_by: vec![],
    limit: None,
    returning: None,
};

// Equivalent SQL: DELETE FROM public.users WHERE deleted_at IS NOT NULL
```

## Example 5: DELETE with RETURNING Clause (PostgreSQL)

Delete and return the deleted rows:

```rust
use sqlparser::ast::*;

let delete_stmt = Statement::Delete {
    table_name: ObjectName(vec![Ident {
        value: "orders".to_string(),
        quote_style: None,
    }]),
    using: None,
    selection: Some(Expr::BinaryOp {
        left: Box::new(Expr::Identifier(Ident {
            value: "status".to_string(),
            quote_style: None,
        })),
        op: BinaryOperator::Equal,
        right: Box::new(Expr::Value(Value::SingleQuotedString("cancelled".to_string()))),
    }),
    order_by: vec![],
    limit: None,
    returning: Some(vec![
        SelectItem::UnnamedExpr(Expr::Identifier(Ident {
            value: "id".to_string(),
            quote_style: None,
        })),
        SelectItem::UnnamedExpr(Expr::Identifier(Ident {
            value: "order_date".to_string(),
            quote_style: None,
        })),
    ]),
};

// Equivalent SQL: DELETE FROM orders WHERE status = 'cancelled' RETURNING id, order_date
```

## Key Points

### Delete Statement Structure
```rust
Statement::Delete {
    table_name: ObjectName,           // Table to delete from
    using: Option<Vec<TableWithJoins>>, // Tables for JOIN conditions (optional)
    selection: Option<Expr>,          // WHERE clause expression (optional)
    order_by: Vec<OrderByExpr>,       // ORDER BY clause (optional)
    limit: Option<Expr>,              // LIMIT clause (optional)
    returning: Option<Vec<SelectItem>>, // RETURNING clause (optional)
}
```

### Creating Binary Conditions with AND/OR
```rust
Expr::BinaryOp {
    left: Box::new(condition1),
    op: BinaryOperator::And,    // or BinaryOperator::Or
    right: Box::new(condition2),
}
```

### Common Comparison Operators
- `BinaryOperator::Equal` - `=`
- `BinaryOperator::NotEqual` - `<>`
- `BinaryOperator::Gt` - `>`
- `BinaryOperator::Lt` - `<`
- `BinaryOperator::GtEq` - `>=`
- `BinaryOperator::LtEq` - `<=`
- `BinaryOperator::And` - `AND`
- `BinaryOperator::Or` - `OR`

### Value Types
- `Value::Number(String, bool)` - Numeric literal
- `Value::SingleQuotedString(String)` - String literal
- `Value::Null` - NULL value

## Helper Function

```rust
fn delete_where(
    table_name: &str,
    condition: Expr,
) -> Statement {
    Statement::Delete {
        table_name: ObjectName(vec![Ident {
            value: table_name.to_string(),
            quote_style: None,
        }]),
        using: None,
        selection: Some(condition),
        order_by: vec![],
        limit: None,
        returning: None,
    }
}

// Usage:
let condition = Expr::BinaryOp {
    left: Box::new(Expr::Identifier(Ident {
        value: "age".to_string(),
        quote_style: None,
    })),
    op: BinaryOperator::Gt,
    right: Box::new(Expr::Value(Value::Number("65".to_string(), false))),
};

let stmt = delete_where("users", condition);
```
