use std::{fs, io::{BufReader, prelude::*}, net::{TcpListener, TcpStream}};

fn main() {

    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    // infinite iterator:
    // - No requests: program sleeps, no cpu usage
    // - Requests arrives: iterator wakes up
    for stream in listener.incoming() {
        let stream = stream.unwrap(); // todo: error hangling
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let peer = stream.peer_addr().unwrap(); // todo: error handling
    println!("Request from: {}:{}", peer.ip(), peer.port());

    let status_line = "HTTP/1.1 200 OK";
    let contents = fs::read_to_string("hello.html").unwrap(); // todo: error hangling
    let length = contents.len();
    let response = 
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap(); // todo: error handling
}
