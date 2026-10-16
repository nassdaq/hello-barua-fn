// What a function can see of the request, and how it shapes its answer.
// Deploy this folder (Folder: echo) and call its URL with any method, any
// path, any query:
//
//   GET  …/echo/books/45?lang=sw   → the request, as JSON
//   POST …/echo/text               → plain text, by content-type
//   GET  …/echo/missing            → a 404 the function chose
//   GET  …/echo/moved              → a 302 with a Location header
use barua_fn::{header, request, status};
use serde_json::json;
use std::io::Read;

fn main() {
    let req = request();
    let mut body = String::new();
    std::io::stdin().read_to_string(&mut body).ok();

    match req.path.as_str() {
        "/missing" => {
            status(404);
            print!("{}", json!({ "error": "nothing here", "path": req.path }));
        }
        "/moved" => {
            status(302);
            header("location", "/echo/");
            print!("moved");
        }
        "/text" => {
            header("content-type", "text/plain; charset=utf-8");
            header("x-echo", "hello");
            print!("you said: {}", body.trim());
        }
        _ => {
            print!(
                "{}",
                json!({
                    "method": req.method,
                    "path": req.path,
                    "segments": req.segments(),
                    "query": req.query,
                    "params": req.query_params(),
                    "content_type": req.content_type,
                    "source": req.source,
                    "headers": req.headers,
                    "body": body,
                })
            );
        }
    }
}
