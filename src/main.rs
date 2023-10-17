use std::fs;
use async_std::net::TcpListener;
use async_std::net::TcpStream;
use async_std::task;
use async_std::task::spawn;
use async_std::prelude::*;
use futures::StreamExt;
use std::time::Duration;
use std::marker::Unpin;
use async_std::io::{Read, Write};

#[async_std::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").await.unwrap();
    // for stream in listener.incoming() {
    //     let stream = stream.unwrap();
    //     handle_connection(stream).await;
    // }
    listener.incoming().for_each_concurrent(None, |tcpstream| async move {
        let tcpstream  = tcpstream.unwrap();
        spawn(handle_connection(tcpstream ));
    }).await;
}

async fn handle_connection(mut stream: impl Read + Write + Unpin) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).await.unwrap();
    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";
    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else if buffer.starts_with(sleep){
        task::sleep(Duration::from_secs(5)).await;
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    }
    else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };
    let content = fs::read_to_string(filename).unwrap();
    let response = format!("{status_line}{content}");
    stream.write(response.as_bytes()).await.unwrap();
    stream.flush().await.unwrap();
}
