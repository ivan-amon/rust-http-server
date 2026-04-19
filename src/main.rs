use std::{
    fs,
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream}, thread, time::Duration,
};
use rust_http_server::ThreadPool;

const IP_ADDR: &str = "0.0.0.0";
const PORT: &str = "7878";
const NUM_THREADS: usize = 4;

fn main() {
    let addr = format!("{IP_ADDR}:{PORT}");
    let listener = TcpListener::bind(addr).unwrap(); // todo: error handling
    let pool = ThreadPool::new(NUM_THREADS);

    // infinite iterator:
    // - No requests: program sleeps, no cpu usage
    // - Requests arrives: iterator wakes up
    for stream in listener.incoming() {
        let stream = stream.unwrap(); // todo: error handling
        
        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let peer = stream.peer_addr().unwrap(); // todo: error handling
    println!("Request from: {}:{}", peer.ip(), peer.port());

    let buf_reader = BufReader::new(&stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")
        }
        _ => ("HTTP/1.1 400 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap(); // todo: error handling
    let length = contents.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap(); // todo: error handling
}
