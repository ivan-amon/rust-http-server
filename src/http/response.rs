use std::{borrow::Cow, fmt};

/// Reprsents an HTTP Response
/// 
/// Contains the status line
/// 
/// # TODO
/// 
/// - Add headers and body
pub struct Response<'a> {
    status_line: StatusLine,
    headers: Headers,
    body: Body<'a>,
}

impl<'a> Response<'a> {
    /// Builds a new [`Response`].
    ///
    /// `content` accepts `&str` or `String` via `Cow<'a, str>`, avoiding
    /// massive reallocs:
    /// - `&'static str` → [`Cow::Borrowed`], zero allocations.
    /// - `String` (e.g. file contents) → [`Cow::Owned`], reuses the caller's
    ///   buffer instead of cloning it.
    pub fn new(status_code: usize, reason_phrase: &'static str, content: impl Into<Cow<'a, str>>) -> Self {
        let content = content.into();
        let content_length = content.len();
        
        let status_line = StatusLine { 
            version: "HTTP/1.1",
            status_code,
            reason_phrase
        };
        let headers = Headers { content_length, };
        let body = Body { content };

        Response { status_line, headers, body }
    }
}

impl<'a> fmt::Display for Response<'a> {
    /// Formats the [`Response`] as raw HTTP/1.1 wire text:
    /// status line, `Content-Length` header, blank line and body.
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
    version: &'static str,
    status_code: usize,
    reason_phrase: &'static str,
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
struct Body<'a> {
    content: Cow<'a, str>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_status_line() {
        let r = Response::new(200, "OK".into(), "hello");
        assert!(r.to_string().starts_with("HTTP/1.1 200 OK\r\n"));
    }

    #[test]
    fn test_content_length_matches_body() {
        let content = "hola mundo";
        let r = Response::new(200, "OK".into(), content);
        assert!(r.to_string().contains(&format!("Content-Length: {}", content.len())));
    }

    #[test]
    fn test_display_has_blank_line_separator() {
        let r = Response::new(200, "OK".into(), "body");
        assert!(r.to_string().contains("\r\n\r\n"));
    }

    #[test]
    fn test_body_in_output() {
        let r = Response::new(200, "OK".into(), "<h1>Test</h1>");
        assert!(r.to_string().ends_with("<h1>Test</h1>"));
    }

    #[test]
    fn test_404_response() {
        let r = Response::new(404, "Not Found".into(), "not found");
        assert!(r.to_string().starts_with("HTTP/1.1 404 Not Found\r\n"));
    }

    #[test]
    fn test_full_response() {
        let content = "<h1>Test</h1>";
        let r = Response::new(200, "OK".into(), content);
        let expected = format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
            content.len(),
            content
        );
        assert_eq!(r.to_string(), expected);
    }
}