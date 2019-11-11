
use std::fmt;

mod iniciante;


fn split_lines(s: &str) -> Vec<String> {
    s.lines().map(|s| s.to_string()).collect::<Vec<_>>()
}

fn split_columns(s: &str) -> Vec<String> {
    s.split_whitespace().map(|s| s.to_string()).collect::<Vec<_>>()
}

fn assert_run(afn: impl Fn(Vec<String>) -> String, s: &str, expected: &str) {
    assert_eq!(&afn(split_lines(&s)), expected);
}

fn main() {
    println!("Hello, world!");
}
