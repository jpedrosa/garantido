// main.rs
// Copyright (c) 2019 Joao Pedrosa

use aliasopts::{AliasOpts, CT};
use std::env;
use regex::Regex;


fn print_help() {
    println!(
r#"Program to inspect some Windows data and drive some automation.

Usage: vip <option>
    --move_cursor x,y - Moves the cursor to the x,y absolute pixel position.
    --screen_size     - Prints the width and height sizes of the primary screen.
    --virtual_screen_size - Prints the width and height sizes of the combined
        screens in case of multiple monitors.
    --cursor_position - Prints the position of the cursor.
    --help            - Shows this help.
"#);
}

fn main() {
    let a: Vec<String> = env::args().collect();
    let mut opts = AliasOpts::new();
    opts.add("--move_cursor", CT::CTString)
        .add("--screen_size", CT::Flag)
        .add("--virtual_screen_size", CT::Flag)
        .add("--cursor_position", CT::Flag)
        .add("--help", CT::Flag)
        .parse(a[1..].to_vec());
    if let Some(s) = opts.get("--move_cursor") {
        let re = Regex::new("^(-?\\d+),(-?\\d+)$").unwrap();
        if let Some(m) = re.captures(&s) {
            if let Ok(x) = m.get(1).unwrap().as_str().parse::<i32>() {
                if let Ok(y) = m.get(2).unwrap().as_str().parse::<i32>() {
                    dynamo::move_cursor(x, y);
                    println!("mmm {:?} x {:?} y {:?}", m, x, y);
                }
            }
        } else {
            eprintln!(
"Error: invalid --move_cursor parameter \"{}\". \
Expected a parameter like 10,30", s);
        }
    }
    if let Some(_) = opts.get("--cursor_position") {
        println!("Cursor position: {:?}", dynamo::cursor_position());
    }
    if let Some(_) = opts.get("--screen_size") {
        println!("Screen size: {:?}", dynamo::screen_size());
    }
    if let Some(_) = opts.get("--virtual_screen_size") {
        println!("Virtual screen size: {:?}", dynamo::virtual_screen_size());
    }
    if opts.is_empty() {
        print_help();
    }
}
