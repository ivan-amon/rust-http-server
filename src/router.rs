use std::{fs, thread, time::Duration};
use crate::http::{HttpMethod, Request, Response};

/// Dispatches a `Request` to the corresponding handler based on its HTTP method.
///
/// Returns a `400 Bad Request` `Response` for unsupported methods.
/// 
/// # TODO
/// 
/// Handle other HTTP Methods
pub fn dispatch(request: &Request<'_>) -> Response<'static> {
    let method = request.method();
    match method {
        HttpMethod::Get => get(request.path()),
        _ => Response::new(
            400,
            "BAD REQUEST",
            "<html><body><h1>400 Bad Request</h1></body></html>")
    }
}

/// Handles `GET` requests by mapping the path to a static file.
///
/// Returns `404 Not Found` for unknown paths. The `/sleep` path blocks the
/// thread for 8 seconds before responding.
fn get(path: &str) -> Response<'static> {
    let (status_code, reason_phrase, path) = match path {
        "/" => (200, "OK", "static/hello.html"),
        "/sleep" => {
            thread::sleep(Duration::from_secs(8));
            (200, "OK", "static/hello.html")
        }
        _ => (404, "NOT FOUND", "static/404.html")
    };

    let content = fs::read_to_string(path).unwrap();
    Response::new(status_code, reason_phrase, content)
}