use std::{io::{Read, Write}, net::TcpListener};
fn main() {
    
    let listener = TcpListener::bind("localhost:3000").unwrap();
    println!("Server started at localhost:3000");
    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        println!("Connection established!");
        let mut buffer = [0; 1024];
        stream.read(&mut buffer).unwrap();
        println!("Received: {}", String::from_utf8_lossy(&buffer));
        stream.write(&buffer).unwrap();
    }
    
}
