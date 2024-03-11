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

const NULL_RESP_STRING: &str = "$-1\r\n";
const NULL_RESP_ARRAY: &str = "*0\r\n";

pub trait RespSerializable {
    fn serialize(&self) -> String;
}

impl RespSerializable for DataType {
    fn serialize(&self) -> String {
        match self {
            DataType::SimpleString(s) => {
                if s.is_empty() {
                    return NULL_RESP_STRING.to_string();
                }
                format!("+{}\r\n", s)
            }
            DataType::BulkString(s) => {
                if s.is_empty() {
                    return NULL_RESP_STRING.to_string();
                }

                format!("${}\r\n{}\r\n", s.len(), s)
            }
            DataType::Array(arr) => {
                if arr.is_empty() {
                    return NULL_RESP_ARRAY.to_string();
                }

                format!(
                    "*{}\r\n{}",
                    arr.len(),
                    arr.iter().map(|dt| dt.serialize()).collect::<String>()
                )
            }
        }
    }
}
