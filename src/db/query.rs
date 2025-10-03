use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Hash)]
pub struct Identifier(pub String);
impl From<String> for Identifier {
    fn from(s: String) -> Self {
        Identifier(s)
    }
}


#[derive(Debug, PartialEq, Clone)]
pub struct QueryPlan {
    pub projection: Vec<Identifier>,
    pub table: Identifier
}

pub struct QueryPlanner{

}

impl QueryPlanner {
    pub fn new() -> Self {
        QueryPlanner{}
    }

    pub fn plan(&self){

    }
}
