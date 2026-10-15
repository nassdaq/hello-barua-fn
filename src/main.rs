// A function for Barua: read the request on stdin, print the answer on stdout.
// Deploy this repository at https://fn.barua.app and it becomes a URL.
//
//   POST {"name": "Asha"}   →   {"message": "Hello, Asha!", ...}
//
// Set GREETING in the function's environment to change the word.
use std::io::{self, Read, Write};

fn field<'a>(json: &'a str, key: &str) -> Option<&'a str> {
    let k = format!("\"{}\"", key);
    let rest = &json[json.find(&k)? + k.len()..];
    let rest = &rest[rest.find('"')? + 1..];
    Some(&rest[..rest.find('"')?])
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).ok();
    let input = input.trim();
    let name = field(input, "name").unwrap_or(if input.is_empty() { "world" } else { input });
    let name = name.replace('"', "'");
    let from_env = std::env::var("GREETING").ok();
    let greeting = from_env.clone().unwrap_or_else(|| "Hello".to_string());
    let out = format!(
        "{{\"message\":\"{}, {}!\",\"lang\":\"rust\",\"greeting_from_env\":{}}}",
        greeting, name, from_env.is_some()
    );
    io::stdout().write_all(out.as_bytes()).ok();
}
