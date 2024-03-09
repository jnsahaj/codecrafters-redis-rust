#[derive(Debug, Clone)]
pub enum DataType {
    SimpleString(String),
    BulkString(String),
    Array(Vec<DataType>),
}

impl DataType {
    pub fn try_into_string(&self) -> Result<String, String> {
        match self {
            DataType::SimpleString(s) => Ok(s.to_string()),
            DataType::BulkString(s) => Ok(s.to_string()),
            _other => Err(format!("Cannot convert {} into string", stringify!(other))),
        }
    }

    pub fn try_into_usize(&self) -> Result<usize, String> {
        let s = self.try_into_string()?;
        match s.parse::<usize>() {
            Ok(v) => Ok(v),
            _ => Err(format!("Cannot convert {} into usize", s)),
        }
    }
}
