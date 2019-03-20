#![allow(unused)]
use std::io;
use std::net::{ TcpListener, TcpStream };

fn handle_client(stream: TcpStream) {
    println!("{:?}", stream);
}

const TCPLISTENER: &str = "127.0.0.1:5060";
fn main() -> io::Result<()> {
    let listener = TcpListener::bind(TCPLISTENER)?;

    println!("TCP Server listening on: {}", TCPLISTENER);
    // accept connecitons and process them serially
    for stream in listener.incoming() {
        handle_client(stream?);
    }
    Ok(())
}
