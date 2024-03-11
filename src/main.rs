mod info;
mod redis;
mod resp;
mod store;
use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream, ToSocketAddrs},
    sync::{Arc, Mutex},
    thread,
};

use clap::Parser;
use resp::data_type::{DataType, RespSerializable};

use crate::redis::Redis;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(long, default_value = "6379")]
    port: usize,

    #[arg(long, num_args(2))]
    replicaof: Option<Vec<String>>,
}

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
    let args = Args::parse();

    let replicaof = args
        .replicaof
        .map(|x| format!("{}:{}", &x[0], x[1]).to_socket_addrs().unwrap())
        .map(|mut x| x.next().unwrap());

    let listener = TcpListener::bind(format!("127.0.0.1:{}", args.port)).unwrap();
    let redis = Arc::new(Mutex::new(Redis::new(replicaof)));

    if let Some(sa) = replicaof {
        let mut stream = TcpStream::connect(sa).unwrap();
        stream
            .write_all(
                DataType::Array(vec![DataType::BulkString("ping".into())])
                    .serialize()
                    .as_bytes(),
            )
            .expect("Failed to write to stream: Handshake");
    }

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
