#[derive(Debug, Clone)]
pub enum DataType {
    SimpleString(String),
    BulkString(String),
    Array(Vec<DataType>),
}

impl DataType {
    pub fn cmp_string(&self, s: &str) -> bool {
        let a = match self {
            DataType::SimpleString(s) => s,
            DataType::BulkString(s) => s,
            _ => return false,
        };

        s == a
    }

    pub fn try_into_string(&self) -> Result<String, String> {
        match self {
            DataType::SimpleString(s) => Ok(s.to_string()),
            DataType::BulkString(s) => Ok(s.to_string()),
            other => Err(format!("Cannot convert {} into string", stringify!(other))),
        }
    }
}
