use std::{self, io::{Read, Write}, net::{TcpListener, TcpStream}};

#[async_std::main]
async fn main() {
    let server = TcpListener::bind("127.0.0.1:24404").unwrap();
    server.set_ttl(ttl)
    for ret in server.incoming() {
        match ret {
            Ok(stream) => handle_stream(stream).await,
            Err(e) => eprintln!("{}", e),
        }
    }
}

async fn handle_stream (mut stream: TcpStream) {
    let mut buf = [0u8;1024];
    stream.read(&mut buf).unwrap();
    stream.write(&buf).unwrap();
}