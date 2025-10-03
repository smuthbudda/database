#[derive(Clone, Debug, PartialEq)]
pub enum SQLOperator{
    EQUALS,
    GREATER_THAN,
    LESS_THAN,
    GREATER_THAN_OR_EQUALS,
    LESS_THAN_OR_EQUALS,
}

pub enum SQLFunction{
    COUNT,
    SUM,
    AVG,
    MAX,
    MIN,
}

#[derive(Clone, Debug, PartialEq)]
pub enum SQLType{
    INTEGER,
    STRING,
    FLOAT,
    TABLE
}

pub enum SQLKeyword {
    SELECT,
    FROM,
    WHERE,
    INSERT,
    INTO,
    VALUES,
    ALL,
    Create
}

impl SQLKeyword {
    pub fn as_str(&self) -> &str {
        match self {
            SQLKeyword::SELECT => "SELECT",
            SQLKeyword::FROM => "FROM",
            SQLKeyword::WHERE => "WHERE",
            SQLKeyword::INSERT => "INSERT",
            SQLKeyword::INTO => "INTO",
            SQLKeyword::VALUES => "VALUES",
            SQLKeyword::ALL => "*",
            SQLKeyword::Create => "CREATE",
        }
    }

    pub fn from_str(s: &str) -> Option<SQLKeyword> {
        match s.to_uppercase().as_str() {
            "SELECT" => Some(SQLKeyword::SELECT),
            "FROM" => Some(SQLKeyword::FROM),
            "WHERE" => Some(SQLKeyword::WHERE),
            "INSERT" => Some(SQLKeyword::INSERT),
            "INTO" => Some(SQLKeyword::INTO),
            "VALUES" => Some(SQLKeyword::VALUES),
            "*" => Some(SQLKeyword::ALL),
            _ => None,
        }
    }
}

