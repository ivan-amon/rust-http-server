use rust_http_server::{Request, ThreadPool, router};
use std::{
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
};

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
    let mut reader = BufReader::new(&mut stream);
    let mut raw_request = String::new();

    // todo: add body to raw request, not needed for GET method
    loop {
        let bytes_read = reader.read_line(&mut raw_request).unwrap();
        if bytes_read == 0 { // Connection Closed
            return;
        }
        if raw_request.ends_with("\r\n\r\n") { // End of Headers
            break;
        }
    }

    let request = match Request::parse(&raw_request) {
        Ok(r) => r,
        Err(_) => return, // todo: send 400 Bad Request
    };

    let response = router::dispatch(&request);
    stream.write_all(response.to_string().as_bytes()).unwrap(); // todo: error handling
}
