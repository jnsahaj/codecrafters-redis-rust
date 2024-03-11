use std::{
    io::Write,
    net::{SocketAddr, TcpStream},
};

use crate::{
    info::Info,
    resp::{
        command::Command,
        data_type::{DataType, RespSerializable},
        parser::Parser,
    },
    store::Store,
};

pub struct Redis {
    info: Info,
    store: Store,
}

impl Redis {
    pub fn new(replicaof: Option<SocketAddr>) -> Self {
        Self {
            info: Info {
                replicaof,
                master_repl_offset: 0,
                master_replid: "8371b4fb1155b71f4a04d3e1bc3e18c4a990aeeb".into(),
            },
            store: Store::new(),
        }
    }

    pub fn eval(&mut self, data: &[u8], stream: &mut TcpStream) {
        let mut parser = Parser::new(data);
        let dt = parser.parse().unwrap();
        let cmd = eval_dt(&dt);

        match cmd {
            Command::Ping => self.echo(stream, "PONG"),
            Command::Echo(s) => self.echo(stream, &s),
            Command::Set(k, v, e) => self.set(stream, &k, &v, e),
            Command::Get(s) => self.get(stream, &s),
            Command::Info(s) => self.info(stream, &s),
        }
    }

    fn echo(&mut self, stream: &mut TcpStream, s: &str) {
        self.stream_resp_write(stream, s);
    }

    fn info(&mut self, stream: &mut TcpStream, s: &str) {
        match s {
            "replication" => self.stream_resp_write(stream, &self.info.to_string()),
            _ => todo!(),
        }
    }

    fn set(
        &mut self,
        stream: &mut TcpStream,
        k: &str,
        v: &str,
        expire_in_millisecs: Option<usize>,
    ) {
        let _ = self.store.set(k, v, expire_in_millisecs);
        self.echo(stream, "OK");
    }

    fn get(&mut self, stream: &mut TcpStream, s: &str) {
        match self.store.get(s) {
            Some(v) => self.stream_resp_write(stream, &v),
            None => self.stream_resp_write(stream, ""),
        }
    }

    fn stream_resp_write(&self, stream: &mut TcpStream, s: &str) {
        stream
            .write_all(DataType::BulkString(s.into()).serialize().as_bytes())
            .expect("Failed to write to stream!");
    }
}

fn eval_dt(dt: &DataType) -> Command {
    let cmd = match dt {
        DataType::SimpleString(s) | DataType::BulkString(s) => match &s[..] {
            "ping" => Command::Ping,
            _ => todo!(),
        },
        DataType::Array(arr) => {
            if let Ok(s) = arr[0].try_into_string() {
                match &s[..] {
                    "echo" => return Command::Echo(arr[1].try_into_string().unwrap()),
                    "set" => {
                        let key = arr[1].try_into_string().unwrap();
                        let value = arr[2].try_into_string().unwrap();
                        let mut exp = None;

                        if let Some(px) = arr.get(3) {
                            if px.try_into_string().unwrap() == "px" {
                                exp = Some(arr[4].try_into_usize().unwrap());
                            }
                        }
                        return Command::Set(key, value, exp);
                    }
                    "get" => return Command::Get(arr[1].try_into_string().unwrap()),
                    "info" => return Command::Info(arr[1].try_into_string().unwrap()),
                    _ => (),
                }
            }

            eval_dt(&arr[0])
        }
    };

    cmd
}
