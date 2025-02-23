use std::net::TcpListener;
use http::httprequest::HttpRequest;
use super::router::Router;
use std::io::{Read, Write};
use std::str;

pub struct Server<'a> {
    socket_addr: &'a str,
}

impl<'a> Server<'a> {
    pub fn new(socket_addr: &'a str) -> Self {
        Server { socket_addr }
    }

    pub fn run(&self) {
        let connection_listener = TcpListener::bind(self.socket_addr);
        println!("running on {}", self.socket_addr);

        for stream in connection_listener.incoming() {
            let mut stream = stream.unwrap();
            println!("Connection established");
            let mut read_buff = [0, 200];
            stream.read(&mut read_buff).unwrap();
            let req: HttpRequest = String::from_utf8(read_buff.to_vec()).unwrap().into();
            Router::route(req, &mut stream);
        }
    }
}