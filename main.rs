#![allow(unused)]
use std::io;
use std::net::{ TcpListener, TcpStream };

fn handle_client(stream: TcpStream) {
    println!("{:?}", stream);
}

fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:5060")?;

    // accept connecitons and process them serially
    for stream in listener.incoming() {
        handle_client(stream?);
    }
    println!("Hello, world!");
    Ok(())
}
