# Example: Creating a Statement::Query

This document shows how to programmatically construct a `Statement::Query` without parsing SQL.

## Simple SELECT * FROM table

```rust
use sqlparser::ast::*;
use sqlparser::ast::helpers::attached_token::AttachedToken;
use sqlparser::tokenizer::{Token, Span};

// Create a simple SELECT * FROM my_table statement

// 1. Create the SELECT token (using Token::Keyword for the SELECT keyword)
let select_token = AttachedToken::empty();

// 2. Create the projection (SELECT items)
// We want: SELECT *
let projection = vec![
    SelectItem::Wildcard(
        WildcardAdditionalOptions::default()
    ),
];

// 3. Create the table reference
// We want: FROM my_table
let table = TableFactor::Table {
    name: ObjectName(vec![ObjectNamePart::Identifier(Ident::new("my_table"))]),
    alias: None,
    args: None,
    with_hints: vec![],
    version: None,
    with_ordinality: false,
    partitions: vec![],
    json_path: None,
    sample: None,
    index_hints: vec![],
};

let table_with_joins = TableWithJoins {
    relation: table,
    joins: vec![],
};

let from = vec![table_with_joins];

// 4. Create the Select struct
let select = Select {
    select_token,
    distinct: None,
    top: None,
    top_before_distinct: false,
    projection,
    exclude: None,
    into: None,
    from,
    lateral_views: vec![],
    prewhere: None,
    selection: None,
    group_by: GroupByExpr::Expressions(vec![], vec![]),
    cluster_by: vec![],
    distribute_by: vec![],
    sort_by: vec![],
    having: None,
    named_window: vec![],
    qualify: None,
    window_before_qualify: false,
    value_table_mode: None,
    connect_by: None,
    flavor: SelectFlavor::Standard,
};

// 5. Wrap in SetExpr::Select
let set_expr = SetExpr::Select(Box::new(select));

// 6. Create the Query
let query = Query {
    with: None,
    body: Box::new(set_expr),
    order_by: None,
    limit_clause: None,
    fetch: None,
    locks: vec![],
    for_clause: None,
    settings: None,
    format_clause: None,
    pipe_operators: vec![],
};

// 7. Wrap in Statement::Query
let statement = Statement::Query(Box::new(query));

// Display the statement
println!("{}", statement);
// Output: SELECT * FROM my_table
```

## SELECT with WHERE clause

```rust
use sqlparser::ast::*;

let select = Select {
    select_token: /* ... */,
    distinct: None,
    top: None,
    top_before_distinct: false,
    projection: vec![
        SelectItem::UnnamedExpr(Expr::Identifier(Ident::new("id"))),
        SelectItem::UnnamedExpr(Expr::Identifier(Ident::new("name"))),
    ],
    exclude: None,
    into: None,
    from: vec![
        TableWithJoins {
            relation: TableFactor::Table {
                name: ObjectName(vec![ObjectNamePart::Identifier(Ident::new("users"))]),
                alias: None,
                args: None,
                with_hints: vec![],
                version: None,
                with_ordinality: false,
                partitions: vec![],
                json_path: None,
                sample: None,
                index_hints: vec![],
            },
            joins: vec![],
        },
    ],
    lateral_views: vec![],
    prewhere: None,
    // WHERE age > 18
    selection: Some(Expr::BinaryOp {
        left: Box::new(Expr::Identifier(Ident::new("age"))),
        op: BinaryOperator::Gt,
        right: Box::new(Expr::Value(Value::Number("18".to_string(), false))),
    }),
    group_by: GroupByExpr::Expressions(vec![], vec![]),
    cluster_by: vec![],
    distribute_by: vec![],
    sort_by: vec![],
    having: None,
    named_window: vec![],
    qualify: None,
    window_before_qualify: false,
    value_table_mode: None,
    connect_by: None,
    flavor: SelectFlavor::Standard,
};

let query = Query {
    with: None,
    body: Box::new(SetExpr::Select(Box::new(select))),
    order_by: None,
    limit_clause: None,
    fetch: None,
    locks: vec![],
    for_clause: None,
    settings: None,
    format_clause: None,
    pipe_operators: vec![],
};

let statement = Statement::Query(Box::new(query));
println!("{}", statement);
// Output: SELECT id, name FROM users WHERE age > 18
```

## SELECT with ORDER BY and LIMIT

```rust
use sqlparser::ast::*;

let select = Select {
    select_token: /* ... */,
    // ... other fields ...
    projection: vec![
        SelectItem::UnnamedExpr(Expr::Wildcard(AttachedToken::empty())),
    ],
    from: vec![
        TableWithJoins {
            relation: TableFactor::Table {
                name: ObjectName(vec![ObjectNamePart::Identifier(Ident::new("products"))]),
                alias: None,
                args: None,
                with_hints: vec![],
                version: None,
                with_ordinality: false,
                partitions: vec![],
                json_path: None,
                sample: None,
                index_hints: vec![],
            },
            joins: vec![],
        },
    ],
    // ... rest of Select fields ...
};

let query = Query {
    with: None,
    body: Box::new(SetExpr::Select(Box::new(select))),
    order_by: Some(OrderBy {
        kind: OrderByKind::Order,
        expressions: vec![
            OrderByExpr {
                expr: Expr::Identifier(Ident::new("price")),
                asc: Some(false),  // DESC
                nulls_first: None,
            },
        ],
    }),
    limit_clause: Some(LimitClause::Limit {
        quantity: Expr::Value(Value::Number("10".to_string(), false)),
        offset: None,
    }),
    fetch: None,
    locks: vec![],
    for_clause: None,
    settings: None,
    format_clause: None,
    pipe_operators: vec![],
};

let statement = Statement::Query(Box::new(query));
println!("{}", statement);
// Output: SELECT * FROM products ORDER BY price DESC LIMIT 10
```

## Key Points

1. **Statement::Query** wraps a `Box<Query>`
2. **Query** contains:
   - `with`: Optional CTEs (WITH clause)
   - `body`: A `SetExpr` (typically `SetExpr::Select` for simple queries)
   - `order_by`: Optional ORDER BY
   - `limit_clause`: Optional LIMIT
   - Other optional clauses (FETCH, locks, FOR XML, etc.)

3. **SetExpr::Select** wraps a `Box<Select>`
4. **Select** contains all the parts of a SELECT statement:
   - `projection`: The SELECT list items
   - `from`: Table references
   - `selection`: WHERE clause
   - `group_by`: GROUP BY expressions
   - `having`: HAVING clause
   - And many more optional clauses

5. Use `AttachedToken` for tokens that need position tracking (like SELECT keyword)
6. Use `ObjectName(vec![ObjectNamePart::Identifier(Ident)])` for qualified names
7. Use `Expr` for expressions (identifiers, literals, binary operations, etc.)
8. Use `TableFactor::Table` for table references (requires all 10 fields including `partitions`, `json_path`, `sample`, `index_hints`)
9. Use `TableWithJoins` to wrap table factors with join information
10. Use `GroupByExpr::Expressions(vec![Expr], vec![GroupByWithModifier])` for GROUP BY

## Quick Helper: Minimal SELECT

```rust
fn minimal_select_all_from(table_name: &str) -> Statement {
    use sqlparser::ast::*;
    use sqlparser::ast::helpers::attached_token::AttachedToken;
    
    Statement::Query(Box::new(Query {
        with: None,
        body: Box::new(SetExpr::Select(Box::new(Select {
            select_token: AttachedToken::empty(),
            distinct: None,
            top: None,
            top_before_distinct: false,
            projection: vec![SelectItem::Wildcard(WildcardAdditionalOptions::default())],
            exclude: None,
            into: None,
            from: vec![TableWithJoins {
                relation: TableFactor::Table {
                    name: ObjectName(vec![ObjectNamePart::Identifier(Ident::new(table_name))]),
                    alias: None,
                    args: None,
                    with_hints: vec![],
                    version: None,
                    with_ordinality: false,
                    partitions: vec![],
                    json_path: None,
                    sample: None,
                    index_hints: vec![],
                },
                joins: vec![],
            }],
            lateral_views: vec![],
            prewhere: None,
            selection: None,
            group_by: GroupByExpr::Expressions(vec![], vec![]),
            cluster_by: vec![],
            distribute_by: vec![],
            sort_by: vec![],
            having: None,
            named_window: vec![],
            qualify: None,
            window_before_qualify: false,
            value_table_mode: None,
            connect_by: None,
            flavor: SelectFlavor::Standard,
        }))),
        order_by: None,
        limit_clause: None,
        fetch: None,
        locks: vec![],
        for_clause: None,
        settings: None,
        format_clause: None,
        pipe_operators: vec![],
    }))
}

// Usage:
let stmt = minimal_select_all_from("employees");
println!("{}", stmt);
// Output: SELECT * FROM employees
```

## Helper with WHERE clause (for Cypher MATCH conversion)

```rust
/// Helper to convert Cypher MATCH patterns to SQL queries
/// Example: MATCH (n:Bug) RETURN n
///          becomes: SELECT * FROM nodes WHERE label = 'Bug'
fn select_all_from_with_where(table_name: &str, where_condition: Expr) -> Statement {
    use sqlparser::ast::*;
    use sqlparser::ast::helpers::attached_token::AttachedToken;
    
    Statement::Query(Box::new(Query {
        with: None,
        body: Box::new(SetExpr::Select(Box::new(Select {
            select_token: AttachedToken::empty(),
            distinct: None,
            top: None,
            top_before_distinct: false,
            projection: vec![SelectItem::Wildcard(WildcardAdditionalOptions::default())],
            exclude: None,
            into: None,
            from: vec![TableWithJoins {
                relation: TableFactor::Table {
                    name: ObjectName(vec![ObjectNamePart::Identifier(Ident::new(table_name))]),
                    alias: None,
                    args: None,
                    with_hints: vec![],
                    version: None,
                    with_ordinality: false,
                    partitions: vec![],
                    json_path: None,
                    sample: None,
                    index_hints: vec![],
                },
                joins: vec![],
            }],
            lateral_views: vec![],
            prewhere: None,
            selection: Some(where_condition),  // The WHERE clause
            group_by: GroupByExpr::Expressions(vec![], vec![]),
            cluster_by: vec![],
            distribute_by: vec![],
            sort_by: vec![],
            having: None,
            named_window: vec![],
            qualify: None,
            window_before_qualify: false,
            value_table_mode: None,
            connect_by: None,
            flavor: SelectFlavor::Standard,
        }))),
        order_by: None,
        limit_clause: None,
        fetch: None,
        locks: vec![],
        for_clause: None,
        settings: None,
        format_clause: None,
        pipe_operators: vec![],
    }))
}

// Usage for: MATCH (n:Bug) RETURN n
let where_expr = Expr::BinaryOp {
    left: Box::new(Expr::Identifier(Ident::new("label"))),
    op: BinaryOperator::Eq,
    right: Box::new(Expr::Value(Value::SingleQuotedString("Bug".to_string()))),
};
let stmt = select_all_from_with_where("nodes", where_expr);
println!("{}", stmt);
// Output: SELECT * FROM nodes WHERE label = 'Bug'
```
