use std::collections::HashMap;
use serde::{Deserialize, Serialize};

use crate::db::b_tree::BTree;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Eq)]
pub struct Table{
    pub columns: Vec<String>,
    pub rows: HashMap<usize, Row>,
    pub indexes: HashMap<String, BTree>,  
}

impl Table{
    pub fn new(columns: Vec<String>, rows: HashMap<usize, Row>, indexes: HashMap<String, BTree>) -> Self{
        Table{columns, rows, indexes}
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Eq)]
pub struct Row{
    pub data: HashMap<String, String>
}

