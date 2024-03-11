pub enum Command {
    Ping,
    Echo(String),
    Set(String, String, Option<usize>),
    Get(String),
    Info(String),
}
