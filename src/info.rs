use std::{fmt::Display, net::SocketAddr};

pub struct Info {
    pub replicaof: Option<SocketAddr>,
    pub master_replid: String,
    pub master_repl_offset: usize,
}

impl Display for Info {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let role = if self.replicaof.is_some() {
            "slave"
        } else {
            "master"
        };

        let s: String = vec![
            ("role", role),
            ("master_replid", &self.master_replid),
            ("master_repl_offset", &self.master_repl_offset.to_string()),
        ]
        .iter()
        .map(|(key, value)| format!("{}:{}\n", key, value))
        .collect();

        f.write_str(&s)
    }
}
