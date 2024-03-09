use std::{io::Write, net::TcpStream};

use crate::resp::{command::Command, data_type::DataType, parser::Parser, serializer::Serializer};

pub struct Redis<'a> {
    stream: &'a TcpStream,
}

impl<'a> Redis<'a> {
    pub fn new(stream: &'a TcpStream) -> Self {
        Self { stream: &stream }
    }

    pub fn eval(&mut self, data: &[u8]) {
        let mut parser = Parser::new(data);
        let dt = parser.parse().unwrap();
        let cmd = eval_dt(dt);

        match cmd {
            Command::Ping => self.pong(),
            Command::Echo(s) => self.echo(&s),
        }
    }

    fn pong(&mut self) {
        self.stream
            .write_all(Serializer::to_simple_string("PONG").as_bytes())
            .expect("Failed to write to stream!");
    }

    fn echo(&mut self, s: &str) {
        self.stream
            .write_all(Serializer::to_simple_string(s).as_bytes())
            .expect("Failed to write to stream!");
    }
}

fn eval_dt(dt: DataType) -> Command {
    let cmd = match dt {
        DataType::SimpleString(s) => match &s[..] {
            "ping" => Command::Ping,
            _ => todo!(),
        },
        DataType::BulkString(s) => match &s[..] {
            "ping" => Command::Ping,
            _ => todo!(),
        },
        DataType::Array(arr) => {
            if arr[0].cmp_string("echo") {
                return Command::Echo(arr[1].try_into_string().unwrap());
            };

            if arr[0].cmp_string("ping") {
                return Command::Ping;
            };

            Command::Ping
        }
    };

    cmd
}
