use std::{collections::HashMap, fs::File, io::{Read, Write}};
use serde::{Deserialize, Serialize};
use crate::db::{b_tree::BTree, schema::{Row, Table}};
use crate::db::sql_enums::SQLOperator;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Eq)]
pub struct StorageEngine {
    pub tables: HashMap<String, Table>,
    pub database_name: String
}

impl StorageEngine {
    pub fn new(database_name: String) -> Self {
        StorageEngine {
            tables: HashMap::new(),
            database_name
        }
    }

    pub fn set_database_name(&mut self, database_name: String)  {
        self.database_name = database_name;
    }

    pub fn create_table(&mut self, name: &str, columns: Vec<String>){
        let mut table = Table {
            columns: columns.clone(),
            rows: HashMap::new(),
            indexes: HashMap::new(),
        };
        
        // Create B-tree indexes for each column
        for column in columns {
            table.indexes.insert(column, BTree::new(5));
        }
        
        self.tables.insert(name.to_string(), table);
    }

    pub fn insert_row(&mut self, table_name: &str, row: Row) {
        if let Some(table) = self.tables.get_mut(table_name) {
            let row_id = table.rows.len();
            table.rows.insert(row_id, row.clone());
            
            // update the indexes 
            for (column, value) in &row.data{
                if let Some(index) = table.indexes.get_mut(column){
                    if let Ok(key) = value.parse::<u16>(){
                        index.insert(key);
                    }
                }
            }
        }
        
    }

    pub fn search_by_index(&self, table_name: &str, operator: SQLOperator, column: &str, value: u16) -> Vec<&Row> {
        let mut results = Vec::new();

        if let Some(table) = self.tables.get(table_name) {
            if let Some(index) = table.indexes.get(column) {
                if index.search(value) {
                    // Find all rows with this value
                    for row in table.rows.values() {
                        if let Some(row_value) = row.data.get(column) {
                            if let Ok(key) = row_value.parse::<u16>() {
                                if key == value {
                                    results.push(row);
                                }
                            }
                        }
                    }
                }
            }
        }
        
        results
    }

    /**
        Serialize the entire database (including B-trees) to a file
    **/
    pub fn save_to_file(&self, filename: &str) -> Result<(), std::io::Error> {
        let mut buffer = Vec::new();
        self.serialize(&mut buffer)?;
        
        let mut file = File::create(filename)?;
        file.write_all(&buffer)?;
        Ok(())
    }

    /**
        Load the entire database (including B-trees) from a file
    **/
    pub fn load_from_file(filename: &str) -> Result<Self, std::io::Error> {
        let mut file = File::open(filename)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        
        let storage: StorageEngine = bincode::deserialize(&buffer)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        
        Ok(storage)
    }


    /**
        Serialize the database object
    **/
    pub fn serialize (&self, buffer: &mut Vec<u8>) -> Result<(), std::io::Error>{
        buffer.clear();
        buffer.extend(bincode::serialize(&self).unwrap());
        Ok(())
    }

    pub fn deserialize (&self, buffer: &[u8]) -> Result<(), std::io::Error>{
        Ok((bincode::deserialize(buffer).unwrap()))
    }
}