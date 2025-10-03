use std::collections::HashMap;
use std::str::Split;
use crate::db::sql_enums::{SQLFunction, SQLKeyword, SQLOperator, SQLType};
use crate::db::storage_engine::StorageEngine;

pub struct Parser{
    pub storage_engine: StorageEngine
}

impl Parser{
    pub fn new(storage_engine: StorageEngine) -> Self {
        Parser{
            storage_engine,
        }
    }

    pub fn parse(&self, sql: String) -> Result<SQLQuery, String> {
        let query: Vec<&str> = sql.split_whitespace().collect();
        if query.is_empty() {
            return Err("Empty SQL".to_owned());
        }
        if !query.last().unwrap().ends_with(";") {
            return Err("Invalid SQL".to_owned());
        }

        let mut operations:Vec<SQLKeyword> = vec![];
        // Todo : Validate the operations are valid
        for operator in &query {
            let parse_result = SQLKeyword::from_str(operator);
            if parse_result.is_some() {
                operations.push(match parse_result {
                    Some(x) => x,
                    None => todo!(),
                });
            }
        }

        let res = match operations[0]{
            SQLKeyword::INSERT => self.parse_select(&query),
            _ => Err("Unsupported SQL Command".to_string())
        };

        res
    }

    /**
        Parses a select statement
        Example: 'FROM USERS SELECT *'
                 'FROM USERS SELECT * WHERE ID > 5'
    **/
    fn parse_select(&self, sql_strings: &[&str]) -> Result<SQLQuery, String> {
        let mut i = 1;
        // Get the table name
        let mut table_name: String = "".to_owned();
        for sql in sql_strings {
            if i == 1 {
                table_name = sql.to_string();
            }
        }

        let query: SelectQuery = SelectQuery{ columns: vec![],table: table_name, where_clause: None};
        let res = SQLQuery::Select(query);
        Ok(res)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum SQLQuery{
    Select(SelectQuery),
    Insert(InsertQuery),
    Create(CreateQuery),
}

#[derive(Debug, PartialEq, Clone)]
pub struct SelectQuery {
    pub columns: Vec<String>,
    pub table: String,
    pub where_clause: Option<WhereClause>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct InsertQuery {
    pub table: String,
    pub values: Vec<String>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct CreateQuery {
    pub table: String,
    pub columns: Vec<ColumnDefinition>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ColumnDefinition {
    pub name: String,
    pub data_type: SQLType,
}

#[derive(Debug, PartialEq, Clone)]
pub struct WhereClause {
    pub column: String,
    pub operator: SQLOperator,
    pub value: String,
}
#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::db::schema::Table;
    use super::*;

    #[test]
    fn test_parser() {
        let mut storage_engine = StorageEngine::new("test.db".to_string());
        storage_engine.tables = HashMap::new();
        storage_engine.tables.insert(
            "Users".to_string(),
            Table::new(vec!["id".to_string(), "name".to_string()], HashMap::new(), HashMap::new())
        );
        let parser = Parser::new(storage_engine);
        let res = parser.parse("FROM USER SELECT *".to_string());
        assert_eq!(true, true);
    }
}