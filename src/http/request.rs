#[allow(dead_code)]
/// Reprsents an HTTP Request
///
/// Contains the request line and the client address (ip address and)
///
/// # TODO
///
/// - Add headers and body
pub struct Request<'a> {
    request_line: RequestLine<'a>,
}

impl<'a> Request<'a> {
    /// Parses a raw HTTP request string into a [`Request`].
    ///
    /// Borrows slices directly from `raw` — zero heap allocations.
    /// The returned `Request<'a>` is valid for as long as `raw` is alive.
    ///
    /// # Errors
    ///
    /// Returns `Err` if:
    /// - `raw` contains no `\r\n` (missing request-line terminator)
    /// - The request line has fewer than three space-separated parts
    /// - The HTTP method is not one of `GET`, `POST`, `PUT`, `DELETE`
    pub fn parse(raw: &'a str) -> Result<Self, &'static str> {
        let first_line: &str = match raw.split_once("\r\n") {
            Some((first_line, _)) => first_line,
            None => return Err("Invalid HTTP request."),
        };

        let request_line = match RequestLine::new(first_line) {
            Ok(req_line) => req_line,
            Err(err) => return Err(err),
        };

        Ok(Self { request_line })
    }

    pub fn method(&self) -> &HttpMethod {
        &self.request_line.method
    }
    pub fn path(&self) -> &str {
        &self.request_line.path
    }
}

pub enum HttpMethod {
    Get,
    Post,
    Put,
    Delete,
}

#[allow(dead_code)]
struct RequestLine<'a> {
    method: HttpMethod,
    path: &'a str,
    version: &'a str,
}

impl<'a> RequestLine<'a> {
    fn new(line: &'a str) -> Result<Self, &'static str> {
        let mut parts = line.splitn(3, ' ');
        let http_method = parts.next().ok_or("Malformed request line")?;
        let path = parts.next().ok_or("Malformed request line")?;
        let version = parts.next().ok_or("Malformed request line")?;

        Ok(RequestLine {
            method: match http_method {
                "GET" => HttpMethod::Get,
                "POST" => HttpMethod::Post,
                "PUT" => HttpMethod::Put,
                "DELETE" => HttpMethod::Delete,
                _ => return Err("Method does not exist"),
            },
            path,
            version,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_valid_get_request_line() {
        let raw = "GET /index.html HTTP/1.1\r\nHost: localhost\r\nUser-Agent: curl\r\n\r\n";

        let request = Request::parse(raw).expect("a well-formed GET should parse");

        assert!(matches!(request.request_line.method, HttpMethod::Get));
        assert_eq!(request.request_line.path, "/index.html");
        assert_eq!(request.request_line.version, "HTTP/1.1");
    }

    #[test]
    fn parses_valid_post_request_line() {
        let raw = "POST /submit HTTP/1.1\r\nHost: localhost\r\nUser-Agent: curl\r\n\r\n";

        let request = Request::parse(raw).expect("a well-formed POST should parse");

        assert!(matches!(request.request_line.method, HttpMethod::Post));
        assert_eq!(request.request_line.path, "/submit");
        assert_eq!(request.request_line.version, "HTTP/1.1");
    }

    #[test]
    fn rejects_unknown_http_method() {
        let raw = "FOO /index.html HTTP/1.1\r\nHost: localhost\r\nUser-Agent: curl\r\n\r\n";

        let result = Request::parse(raw);

        assert!(result.is_err(), "an unknown HTTP method must be rejected");
    }

    #[test]
    fn rejects_noise_input() {
        let raw = "%%garbage--no-structure$$\r\nHost: localhost\r\nUser-Agent: curl\r\n\r\n";

        let result = Request::parse(raw);

        assert!(result.is_err(), "malformed input must be rejected");
    }

    #[test]
    fn rejects_request_line_with_missing_parts() {
        let raw = "GET /index.html\r\nHost: localhost\r\nUser-Agent: curl\r\n\r\n";

        let result = Request::parse(raw);

        assert!(
            result.is_err(),
            "a request line missing the version must be rejected"
        );
    }
}
