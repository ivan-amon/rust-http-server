use std::io::{BufRead, BufReader};
use std::net::{SocketAddr, TcpStream};

#[allow(dead_code)]
/// Reprsents an HTTP Request
/// 
/// Contains the request line and the client address (ip address and)
/// 
/// # TODO
/// 
/// - Add headers and body
pub struct Request {
    request_line: RequestLine,
    peer_addr: SocketAddr,
}

impl Request {
    pub fn new(stream: &mut TcpStream) -> Result<Self, &'static str> {
        let buf_reader = BufReader::new(&mut *stream);
        let first_line = buf_reader.lines().next().unwrap().unwrap(); // todo: error handling

        let request_line = match RequestLine::new(&first_line) {
            Ok(req) => req,
            Err(err) => return Err(err),
        };
        

        Ok(Request { 
            request_line, 
            peer_addr: stream.peer_addr().unwrap(),
        })
    }

    pub fn peer_addr(&self) -> SocketAddr { self.peer_addr }
    pub fn method(&self) -> &HttpMethod { &self.request_line.method }
    pub fn path(&self) -> &str { &self.request_line.path }
}

pub enum HttpMethod {
    Get,
    Post,
    Put,
    Delete,
}

#[allow(dead_code)]
struct RequestLine {
    method: HttpMethod,
    path: String,
    version: String,
}

impl RequestLine {
    fn new(line: &str) -> Result<Self, &'static str> {
        let line_parts: Vec<&str> = line.split_whitespace().take(3).collect();
        if line_parts.len() < 3 {
            return Err("Malformed request line");
        }

        let http_method = line_parts[0];
        let path = line_parts[1];
        let version = line_parts[2];

        Ok(RequestLine {
            method: match http_method {
                "GET" => HttpMethod::Get,
                "POST" => HttpMethod::Post,
                "PUT" => HttpMethod::Put,
                "DELETE" => HttpMethod::Delete,
                _        => return Err("Method does not exist"),
            },
            path: String::from(path),
            version: String::from(version),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use std::net::{TcpListener, TcpStream};
    use std::thread;

    fn stream_with(payload: &[u8]) -> TcpStream {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap();
        let bytes = payload.to_vec();
        thread::spawn(move || {
            let mut client = TcpStream::connect(addr).unwrap();
            client.write_all(&bytes).unwrap();
        });
        let (server_stream, _) = listener.accept().unwrap();
        server_stream
    }

    #[test]
    fn parses_valid_get_request_line() {
        let mut stream = stream_with(b"GET /index.html HTTP/1.1\r\n\r\n");

        let request = Request::new(&mut stream).expect("a well-formed GET should parse");

        assert!(matches!(request.request_line.method, HttpMethod::Get));
        assert_eq!(request.request_line.path, "/index.html");
        assert_eq!(request.request_line.version, "HTTP/1.1");
    }

    #[test]
    fn parses_valid_post_request_line() {
        let mut stream = stream_with(b"POST /submit HTTP/1.1\r\n\r\n");

        let request = Request::new(&mut stream).expect("a well-formed POST should parse");

        assert!(matches!(request.request_line.method, HttpMethod::Post));
        assert_eq!(request.request_line.path, "/submit");
        assert_eq!(request.request_line.version, "HTTP/1.1");
    }

    #[test]
    fn rejects_unknown_http_method() {
        let mut stream = stream_with(b"FOO /index.html HTTP/1.1\r\n\r\n");

        let result = Request::new(&mut stream);

        assert!(result.is_err(), "an unknown HTTP method must be rejected");
    }

    #[test]
    fn rejects_noise_input() {
        let mut stream = stream_with(b"%%garbage--no-structure$$\r\n\r\n");

        let result = Request::new(&mut stream);

        assert!(result.is_err(), "malformed input must be rejected");
    }

    #[test]
    fn rejects_request_line_with_missing_parts() {
        let mut stream = stream_with(b"GET /index.html\r\n\r\n");

        let result = Request::new(&mut stream);

        assert!(result.is_err(), "a request line missing the version must be rejected");
    }

    #[test]
    fn stream_remains_usable_after_parsing() {
        let mut stream = stream_with(b"GET /index.html HTTP/1.1\r\nHost: localhost\r\n\r\n");

        let _request = Request::new(&mut stream).expect("a well-formed GET should parse");

        let _still_mine: &mut TcpStream = &mut stream;
    }
}





