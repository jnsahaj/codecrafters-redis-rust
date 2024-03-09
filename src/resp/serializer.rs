pub struct Serializer {}

impl Serializer {
    pub fn to_simple_string(s: &str) -> String {
        format!("+{}\r\n", s)
    }

    pub fn to_bulk_string(s: &str) -> String {
        if s.is_empty() {
            return format!("$-1\r\n");
        }

        format!("${}\r\n{}\r\n", s.len(), s)
    }
}
