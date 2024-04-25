use std::{io::{Read, Write}, net::TcpListener};

use http::httprequest::HttpRequest;

use crate::router::Router;


pub struct Server<'a> {
    pub address: &'a str,
    
}

impl<'a> Server<'a> {
    pub fn new(addr : &'a str) -> Server<'a> {
        Server {
            address : addr,
        }
        
    }

    pub fn run(&self) {
        let listener = TcpListener::bind(self.address).unwrap();
        println!("Server started at {}", self.address);
        for stream in listener.incoming() {
            let mut stream = stream.unwrap();
            println!("Connection established!");
            let mut buffer = [0; 1024];
            stream.read(&mut buffer).unwrap();
            let res = String::from_utf8(buffer.to_vec()).unwrap();
            
            
            let req = HttpRequest::from(res.as_str());
            Router::route(req, & mut stream);
            
        }
    }
}