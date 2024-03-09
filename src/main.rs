mod redis;
mod resp;
use std::{
    io::Read,
    net::{TcpListener, TcpStream},
    thread,
};

use crate::redis::Redis;

fn handle_client(mut stream: TcpStream) {
    let mut buf = [0u8; 1024];

    loop {
        let read_bytes = stream
            .read(&mut buf[..])
            .expect("Failed to read from client!");

        if read_bytes == 0 {
            return;
        }

        println!("reading {} bytes...", read_bytes);
        println!("data (raw):  {:?}", buf);
        println!("data (str):  {:?}", String::from_utf8_lossy(&buf));

        let mut redis = Redis::new(&stream);
        redis.eval(&buf[..]);
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    handle_client(stream);
                });
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
