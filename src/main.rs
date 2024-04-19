mod info;
mod redis;
mod resp;
mod store;
use std::{
    net::{SocketAddr, ToSocketAddrs},
    sync::Arc,
    usize,
};

use crate::redis::Redis;
use clap::Parser;
use resp::data_type::{DataType, RespSerializable};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
    sync::Mutex,
};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(long, default_value = "6379")]
    port: usize,

    #[arg(long, num_args(2))]
    replicaof: Option<Vec<String>>,
}

async fn handle_client(mut stream: TcpStream, redis: Arc<Mutex<Redis>>) {
    let mut buf = [0u8; 1024];

    loop {
        let read_bytes = stream
            .read(&mut buf[..])
            .await
            .expect("Failed to read from client!");

        if read_bytes == 0 {
            return;
        }

        println!("reading {} bytes...", read_bytes);
        println!("data (raw):  {:?}", buf);
        println!("data (str):  {:?}", String::from_utf8_lossy(&buf));

        let mut redis = redis.lock().await;
        redis.eval(&buf[..], &mut stream).await;
    }
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let replicaof = args
        .replicaof
        .map(|x| format!("{}:{}", &x[0], x[1]).to_socket_addrs().unwrap())
        .map(|mut x| x.next().unwrap());

    let listener = TcpListener::bind(format!("127.0.0.1:{}", args.port))
        .await
        .unwrap();

    let redis = Arc::new(Mutex::new(Redis::new(replicaof)));

    if let Some(sa) = replicaof {
        replica_handshake(sa, args.port).await
    }

    while let Ok((socket, _)) = listener.accept().await {
        let redis = Arc::clone(&redis);
        tokio::spawn(async move {
            handle_client(socket, redis).await;
        });
    }
}

async fn replica_handshake(master_socket_addr: SocketAddr, self_port: usize) {
    let mut buf = [0; 1024];
    let mut stream = TcpStream::connect(master_socket_addr).await.unwrap();
    stream
        .write_all(
            DataType::Array(vec![DataType::BulkString("ping".into())])
                .serialize()
                .as_bytes(),
        )
        .await
        .expect("Failed to write to stream: replica handshake");

    stream.read(&mut buf).await.unwrap();

    stream
        .write_all(
            DataType::Array(vec![
                DataType::BulkString("REPLCONF".into()),
                DataType::BulkString("listening-port".into()),
                DataType::BulkString(self_port.to_string()),
            ])
            .serialize()
            .as_bytes(),
        )
        .await
        .unwrap();

    stream.read(&mut buf).await.unwrap();

    stream
        .write_all(
            DataType::Array(vec![
                DataType::BulkString("REPLCONF".into()),
                DataType::BulkString("capa".into()),
                DataType::BulkString("psync2".into()),
            ])
            .serialize()
            .as_bytes(),
        )
        .await
        .unwrap();

    stream.read(&mut buf).await.unwrap();

    stream
        .write_all(
            DataType::Array(vec![
                DataType::BulkString("PSYNC".into()),
                DataType::BulkString("?".into()),
                DataType::BulkString("-1".into()),
            ])
            .serialize()
            .as_bytes(),
        )
        .await
        .unwrap();

    stream.read(&mut buf).await.unwrap();
}
