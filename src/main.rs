use std::{
    fs,
    io::Write,
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};
use rust_http_server::{HttpMethod, Request, Response, ThreadPool};

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
    let request = match Request::new(&mut stream) {
        Ok(req) => req,
        Err(err) => {
            eprintln!("Bad request: {err}");
            let response = Response::new(400, "Bad Request".into(), err.into());
            let _ = stream.write_all(response.to_string().as_bytes());
            return;
        }
    };

    let peer = request.peer_addr();
    println!("Request from: {}:{}", peer.ip(), peer.port());

    let (status_code, reason, filename) = match (request.method(), request.path()) {
        (HttpMethod::Get, "/") => (200, "OK", "hello.html"),
        (HttpMethod::Get, "/sleep") => {
            thread::sleep(Duration::from_secs(8));
            (200, "OK", "hello.html")
        }
        _ => (404, "NOT FOUND", "404.html"),
    };

    let path = format!("static/{filename}");
    let contents = fs::read_to_string(path).unwrap(); // todo: error handling

    let response = Response::new(status_code, reason.into(), contents);
    stream.write_all(response.to_string().as_bytes()).unwrap(); // todo: error handling
}
