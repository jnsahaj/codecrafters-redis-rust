use std::{io::Write, net::TcpStream};

use crate::{
    resp::{command::Command, data_type::DataType, parser::Parser, serializer::Serializer},
    store::Store,
};

pub struct Redis {
    store: Store,
}

impl Redis {
    pub fn new() -> Self {
        Self {
            store: Store::new(),
        }
    }

    pub fn eval(&mut self, data: &[u8], stream: &mut TcpStream) {
        let mut parser = Parser::new(data);
        let dt = parser.parse().unwrap();
        let cmd = eval_dt(&dt);

        match cmd {
            Command::Ping => self.pong(stream),
            Command::Echo(s) => self.echo(stream, &s),
            Command::Set(k, v) => self.set(stream, &k, &v),
            Command::Get(s) => self.get(stream, &s),
        }
    }

    fn pong(&mut self, stream: &mut TcpStream) {
        self.stream_resp_write(stream, "PONG");
    }

    fn echo(&mut self, stream: &mut TcpStream, s: &str) {
        self.stream_resp_write(stream, s);
    }

    fn set(&mut self, stream: &mut TcpStream, k: &str, v: &str) {
        let _ = self.store.set(k, v, None);
        self.ok(stream);
    }

    fn get(&mut self, stream: &mut TcpStream, s: &str) {
        match self.store.get(s) {
            Some(v) => self.stream_resp_write(stream, &v),
            None => self.stream_resp_write(stream, ""),
        }
    }

    fn ok(&mut self, stream: &mut TcpStream) {
        self.stream_resp_write(stream, "OK");
    }

    fn stream_resp_write(&self, stream: &mut TcpStream, s: &str) {
        stream
            .write_all(Serializer::to_bulk_string(s).as_bytes())
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
            println!("ARRAY: {:?}", arr);
            if let Ok(s) = arr[0].try_into_string() {
                match &s[..] {
                    "echo" => return Command::Echo(arr[1].try_into_string().unwrap()),
                    "set" => {
                        return Command::Set(
                            arr[1].try_into_string().unwrap(),
                            arr[2].try_into_string().unwrap(),
                        )
                    }
                    "get" => return Command::Get(arr[1].try_into_string().unwrap()),
                    _ => (),
                }
            }

            eval_dt(&arr[0])
        }
    };

    cmd
}
