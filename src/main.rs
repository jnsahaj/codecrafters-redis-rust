mod redis;
mod resp;
mod store;
use std::{
    io::Read,
    net::{TcpListener, TcpStream},
    sync::{Arc, Mutex},
    thread,
};

use crate::redis::Redis;

fn handle_client(mut stream: TcpStream, redis: Arc<Mutex<Redis>>) {
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

        let mut redis = redis.lock().unwrap();
        redis.eval(&buf[..], &mut stream);
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();
    let redis = Arc::new(Mutex::new(Redis::new()));

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let redis = Arc::clone(&redis);
                thread::spawn(move || {
                    handle_client(stream, redis);
                });
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
