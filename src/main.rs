use rust_http_server::{Request, Response, ThreadPool, router};
use std::{
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
};

const IP_ADDR: &str = "0.0.0.0";
const PORT: &str = "7878";
const NUM_THREADS: usize = 4;

fn main() {
    let addr = format!("{IP_ADDR}:{PORT}");
    let listener = TcpListener::bind(addr)
        .expect("Failed to bind to address: port may be in use or insufficient permissions");
    let pool = ThreadPool::new(NUM_THREADS);

    // infinite iterator:
    // - No requests: program sleeps, no cpu usage
    // - Requests arrives: iterator wakes up
    for stream in listener.incoming() {
        let stream = match stream {
            Ok(stream) => stream,
            Err(err) => {
                eprintln!("Failed to accept connection: {err}");
                continue;
            }
        };

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
        let bytes_read = match reader.read_line(&mut raw_request) {
            Ok(n) => n,
            Err(err) => {
                eprintln!("Failed to read from stream: {err}");
                return;
            }
        };
        if bytes_read == 0 {
            // Connection Closed
            return;
        }
        if raw_request.ends_with("\r\n\r\n") {
            // End of Headers
            break;
        }
    }

    let request = match Request::parse(&raw_request) {
        Ok(req) => req,
        Err(err) => { // Error parsing HTTP Request
            let response = Response::new(
                400,
                "BAD REQUEST",
                format!("{err}"),
            );
            if let Err(err) = stream.write_all(response.to_string().as_bytes()) {
                eprintln!("Failed to write response: {err}");
            };
            return;
        }
    };

    // Send HTTP Response (write on TcpStream)
    let response = router::dispatch(&request);
    if let Err(err) = stream.write_all(response.to_string().as_bytes()) {
        eprintln!("Failed to write response: {err}");
    }
}
