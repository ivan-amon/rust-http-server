use std::fmt;

/// Reprsents an HTTP Response
/// 
/// Contains the status line
/// 
/// # TODO
/// 
/// - Add headers and body
pub struct Response {
    status_line: StatusLine,
    headers: Headers,
    body: Body,
}

impl Response {
    pub fn new(status_code: usize, reason_phrase: String, content: String) -> Self {
        let status_line = StatusLine { 
            version: String::from("HTTP/1.1"),
            status_code,
            reason_phrase
        };
        let content_length = content.len();
        let headers = Headers {
            content_length,
        };
        let body = Body {
            content
        };

        Response { status_line, headers, body }
    }
}

impl fmt::Display for Response {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} {} {}\r\nContent-Length: {}\r\n\r\n{}",
            self.status_line.version,
            self.status_line.status_code,
            self.status_line.reason_phrase,
            self.headers.content_length,
            self.body.content
        )
    }
}

struct StatusLine {
    version: String,
    status_code: usize,
    reason_phrase: String,
}

/// Represents the headers of an HTTP Response
/// 
/// # Todo:
/// 
/// - Add 'Content-Type' to send more file types instead of just HTML
struct Headers {
    content_length: usize,
}

/// Represents the body of an HTTP Response
///
/// At the moment out server only sends HTML files
struct Body {
    content: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_status_line() {
        let r = Response::new(200, "OK".into(), "hello".into());
        assert!(r.to_string().starts_with("HTTP/1.1 200 OK\r\n"));
    }

    #[test]
    fn test_content_length_matches_body() {
        let content = "hola mundo";
        let r = Response::new(200, "OK".into(), content.into());
        assert!(r.to_string().contains(&format!("Content-Length: {}", content.len())));
    }

    #[test]
    fn test_display_has_blank_line_separator() {
        let r = Response::new(200, "OK".into(), "body".into());
        assert!(r.to_string().contains("\r\n\r\n"));
    }

    #[test]
    fn test_body_in_output() {
        let r = Response::new(200, "OK".into(), "<h1>Test</h1>".into());
        assert!(r.to_string().ends_with("<h1>Test</h1>"));
    }

    #[test]
    fn test_404_response() {
        let r = Response::new(404, "Not Found".into(), "not found".into());
        assert!(r.to_string().starts_with("HTTP/1.1 404 Not Found\r\n"));
    }

    #[test]
    fn test_full_response() {
        let content = "<h1>Test</h1>";
        let r = Response::new(200, "OK".into(), content.into());
        let expected = format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
            content.len(),
            content
        );
        assert_eq!(r.to_string(), expected);
    }
}