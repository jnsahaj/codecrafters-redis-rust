pub struct Serializer {}

impl Serializer {
    pub fn to_simple_string(s: &str) -> String {
        format!("+{}\r\n", s)
    }
}
