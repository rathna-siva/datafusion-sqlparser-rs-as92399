# Example: Creating a Statement::Insert

This document shows how to programmatically construct an `INSERT INTO` statement without parsing SQL.

## Simple INSERT with VALUES

```rust
use sqlparser::ast::*;
use sqlparser::ast::helpers::attached_token::AttachedToken;

// Create: INSERT INTO users (id, name) VALUES (1, 'John')

let insert_token = AttachedToken::empty();

// Define the columns
let columns = vec![
    Ident::new("id"),
    Ident::new("name"),
];

// Create the VALUES rows
let rows = vec![
    vec![
        Expr::Value(Value::Number("1".to_string(), false).into()),
        Expr::Value(Value::SingleQuotedString("John".to_string()).into()),
    ],
];

// Create SetExpr::Values
let values = SetExpr::Values(Values {
    explicit_row_keyword: false,
    rows,
});

// Create the Query
let query = Query {
    with: None,
    body: Box::new(values),
    order_by: None,
    limit_clause: None,
    fetch: None,
    locks: vec![],
    for_clause: None,
    settings: None,
    format_clause: None,
    pipe_operators: vec![],
};

// Create the Insert statement
let insert = Insert {
    insert_token,
    or: None,
    ignore: false,
    into: true,
    table: TableObject::TableName(
        ObjectName(vec![ObjectNamePart::Identifier(Ident::new("users"))])
    ),
    table_alias: None,
    columns,
    overwrite: false,
    source: Some(Box::new(query)),
    assignments: vec![],
    partitioned: None,
    after_columns: vec![],
    has_table_keyword: false,
    on: None,
    returning: None,
    replace_into: false,
    priority: None,
    insert_alias: None,
    settings: None,
    format_clause: None,
};

let statement = Statement::Insert(insert);

println!("{}", statement);
// Output: INSERT INTO users (id, name) VALUES (1, 'John')
```

## INSERT with SELECT subquery

```rust
use sqlparser::ast::*;
use sqlparser::ast::helpers::attached_token::AttachedToken;

// Create: INSERT INTO users (id, name) SELECT id, name FROM temp_users

let insert_token = AttachedToken::empty();

let columns = vec![
    Ident::new("id"),
    Ident::new("name"),
];

// Create SELECT query: SELECT id, name FROM temp_users
let select_token = AttachedToken::empty();
let projection = vec![
    SelectItem::UnnamedExpr(Expr::Identifier(Ident::new("id"))),
    SelectItem::UnnamedExpr(Expr::Identifier(Ident::new("name"))),
];

let table = TableFactor::Table {
    name: ObjectName(vec![ObjectNamePart::Identifier(Ident::new("temp_users"))]),
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

let select = Select {
    select_token,
    distinct: None,
    top: None,
    top_before_distinct: false,
    projection,
    exclude: None,
    into: None,
    from: vec![TableWithJoins {
        relation: table,
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

let insert = Insert {
    insert_token,
    or: None,
    ignore: false,
    into: true,
    table: TableObject::TableName(
        ObjectName(vec![ObjectNamePart::Identifier(Ident::new("users"))])
    ),
    table_alias: None,
    columns,
    overwrite: false,
    source: Some(Box::new(query)),
    assignments: vec![],
    partitioned: None,
    after_columns: vec![],
    has_table_keyword: false,
    on: None,
    returning: None,
    replace_into: false,
    priority: None,
    insert_alias: None,
    settings: None,
    format_clause: None,
};

let statement = Statement::Insert(insert);

println!("{}", statement);
// Output: INSERT INTO users (id, name) SELECT id, name FROM temp_users
```

## INSERT with multiple rows

```rust
use sqlparser::ast::*;
use sqlparser::ast::helpers::attached_token::AttachedToken;

// Create: INSERT INTO users (id, name, email) VALUES 
//         (1, 'Alice', 'alice@example.com'),
//         (2, 'Bob', 'bob@example.com')

let insert_token = AttachedToken::empty();

let columns = vec![
    Ident::new("id"),
    Ident::new("name"),
    Ident::new("email"),
];

// Create multiple VALUE rows
let rows = vec![
    vec![
        Expr::Value(Value::Number("1".to_string(), false).into()),
        Expr::Value(Value::SingleQuotedString("Alice".to_string()).into()),
        Expr::Value(Value::SingleQuotedString("alice@example.com".to_string()).into()),
    ],
    vec![
        Expr::Value(Value::Number("2".to_string(), false).into()),
        Expr::Value(Value::SingleQuotedString("Bob".to_string()).into()),
        Expr::Value(Value::SingleQuotedString("bob@example.com".to_string()).into()),
    ],
];

let values = SetExpr::Values(Values {
    explicit_row_keyword: false,
    rows,
});

let query = Query {
    with: None,
    body: Box::new(values),
    order_by: None,
    limit_clause: None,
    fetch: None,
    locks: vec![],
    for_clause: None,
    settings: None,
    format_clause: None,
    pipe_operators: vec![],
};

let insert = Insert {
    insert_token,
    or: None,
    ignore: false,
    into: true,
    table: TableObject::TableName(
        ObjectName(vec![ObjectNamePart::Identifier(Ident::new("users"))])
    ),
    table_alias: None,
    columns,
    overwrite: false,
    source: Some(Box::new(query)),
    assignments: vec![],
    partitioned: None,
    after_columns: vec![],
    has_table_keyword: false,
    on: None,
    returning: None,
    replace_into: false,
    priority: None,
    insert_alias: None,
    settings: None,
    format_clause: None,
};

let statement = Statement::Insert(insert);

println!("{}", statement);
// Output: INSERT INTO users (id, name, email) VALUES 
//         (1, 'Alice', 'alice@example.com'), 
//         (2, 'Bob', 'bob@example.com')
```

## Key Points

1. **Statement::Insert** wraps an `Insert` struct
2. **Insert** contains:
   - `table`: The target table (using `TableObject::TableName`)
   - `columns`: The list of columns to insert into
   - `source`: Optional `Query` containing VALUES or SELECT
   - `insert_token`: The INSERT keyword token
   - Other optional fields: `or`, `ignore`, `on`, `returning`, etc.

3. **For VALUES inserts**:
   - Use `SetExpr::Values(Values { rows, ... })`
   - Each row is a `Vec<Expr>`

4. **For SELECT inserts**:
   - Use `SetExpr::Select(Box::new(select))`
   - Same as creating a regular SELECT query

5. Key required fields for Insert:
   - `insert_token: AttachedToken::empty()` - Use empty for programmatic creation
   - `table: TableObject::TableName(ObjectName(...))` - Specify the target table
   - `columns: Vec<Ident>` - List of columns to insert
   - `source: Some(Box::new(query))` - The VALUES or SELECT query
   - `into: true` - Include the INTO keyword

6. Optional fields typically used:
   - `returning: Some(vec![...])` - For RETURNING clause (PostgreSQL)
   - `on: Some(OnInsert::...)` - For conflict handling
   - `ignore: true` - For MySQL INSERT IGNORE
   - `replace_into: true` - For MySQL REPLACE INTO

## Quick Helper: INSERT VALUES

```rust
fn insert_values(
    table_name: &str,
    column_names: Vec<&str>,
    row_values: Vec<Vec<Expr>>,
) -> Statement {
    use sqlparser::ast::*;
    use sqlparser::ast::helpers::attached_token::AttachedToken;

    let columns = column_names
        .into_iter()
        .map(|name| Ident::new(name))
        .collect();

    let values = SetExpr::Values(Values {
        explicit_row_keyword: false,
        rows: row_values,
    });

    let query = Query {
        with: None,
        body: Box::new(values),
        order_by: None,
        limit_clause: None,
        fetch: None,
        locks: vec![],
        for_clause: None,
        settings: None,
        format_clause: None,
        pipe_operators: vec![],
    };

    let insert = Insert {
        insert_token: AttachedToken::empty(),
        or: None,
        ignore: false,
        into: true,
        table: TableObject::TableName(
            ObjectName(vec![ObjectNamePart::Identifier(Ident::new(table_name))])
        ),
        table_alias: None,
        columns,
        overwrite: false,
        source: Some(Box::new(query)),
        assignments: vec![],
        partitioned: None,
        after_columns: vec![],
        has_table_keyword: false,
        on: None,
        returning: None,
        replace_into: false,
        priority: None,
        insert_alias: None,
        settings: None,
        format_clause: None,
    };

    Statement::Insert(insert)
}

// Usage:
let values = vec![vec![
    Expr::Value(Value::Number("1".to_string(), false).into()),
    Expr::Value(Value::SingleQuotedString("Alice".to_string()).into()),
]];

let stmt = insert_values("users", vec!["id", "name"], values);
println!("{}", stmt);
// Output: INSERT INTO users (id, name) VALUES (1, 'Alice')
```
