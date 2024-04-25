use std::{io::{Read, Write}, net::TcpStream};
use std::str;

fn main() {
    
    let mut stream = TcpStream::connect("localhost:3000").unwrap();
    stream.write(b"Hello, server!").unwrap();
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    println!("Server response: {}", str::from_utf8(&buffer).unwrap());


 
}
