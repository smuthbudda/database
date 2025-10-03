use std::collections::HashMap;
use crate::db::storage_engine::StorageEngine;
use crate::db::executor::ExecutionEngine;
use crate::db::parser::Parser;
use crate::db::sql_enums::SQLOperator;
use crate::db::query::{QueryPlan, Identifier};

mod db;

fn main() {
    // Try to load existing database from file
    let mut storage = match StorageEngine::load_from_file("jordan_db.db") {
        Ok(storage) => {
            println!("Loaded existing database from file");
            storage
        }
        Err(_) => {
            println!("Creating new database");
            StorageEngine::new("jordan_db".to_string())
        }
    };

    storage.set_database_name("jordan_db".to_string());
    let parser = Parser::new(storage.clone());
    // parser.p
    db_stuff(&mut storage);
}

fn db_stuff(storage: &mut StorageEngine) {


    // Create a table with columns if it doesn't exist
    if !storage.tables.contains_key("users") {
        storage.create_table("users", vec!["id".to_string(), "name".to_string(), "age".to_string()]);

        // Insert some sample data
        let mut row1 = HashMap::new();
        row1.insert("id".to_string(), "1".to_string());
        row1.insert("name".to_string(), "Alice".to_string());
        row1.insert("age".to_string(), "25".to_string());
        storage.insert_row("users", crate::db::schema::Row { data: row1 });

        let mut row2 = HashMap::new();
        row2.insert("id".to_string(), "2".to_string());
        row2.insert("name".to_string(), "Bob".to_string());
        row2.insert("age".to_string(), "30".to_string());
        storage.insert_row("users", crate::db::schema::Row { data: row2 });

        let mut row3 = HashMap::new();
        row3.insert("id".to_string(), "3".to_string());
        row3.insert("name".to_string(), "Charlie".to_string());
        row3.insert("age".to_string(), "35".to_string());
        storage.insert_row("users", crate::db::schema::Row { data: row3 });
    }

    // Demonstrate B-tree functionality
    println!("\n=== B-tree Index Information ===");
    if let Some(table) = storage.tables.get("users") {
        for (column, index) in &table.indexes {
            println!("Index for column '{}':", column);
            print!("  Traversal: ");
            index.traverse();
            println!();
        }
    }

    // Demonstrate index-based search
    println!("\n=== Index-based Search ===");
    let results = storage.search_by_index("users", SQLOperator::EQUALS, "age", 30);
    println!("Users with age 30: {:?}", results);

    // Demonstrate query execution
    println!("\n=== Query Execution ===");
    let executor = ExecutionEngine::new(storage.clone());
    let query_plan = QueryPlan {
        projection: vec![Identifier("name".to_string()), Identifier("age".to_string())],
        table: Identifier("users".to_string()),
    };

    match executor.execute(query_plan) {
        Ok(results) => {
            println!("Query results:");
            for row in results {
                println!("  {:?}", row);
            }
        }
        Err(e) => println!("Error: {:?}", e),
    }

    // Save database to file (this includes all B-trees)
    match executor.storage_engine.save_to_file("jordan_db.db") {
        Ok(_) => println!("\nDatabase saved to 'jordan_db.db'"),
        Err(e) => println!("Error saving database: {:?}", e),
    }
}