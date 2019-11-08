// main.rs
// Copyright (c) 2019 Joao Pedrosa

use aliasopts::{AliasOpts, CT};
use std::env;
use regex::Regex;
use std::fs::File;
use std::io::{BufReader};
// imports the io traits with a glob.
use std::io::prelude::*;


fn print_help() {
    println!(
r#"Benchmark test.

Usage: asgard <--file c:\path\to\file.txt>
    --file|-f         - Path to file to read the lines from.
    --help            - Shows this help.
"#);
}

fn check(file_path: &str,) -> std::io::Result<()> {
    let file = File::open(&file_path)?;
    let md = file.metadata()?;
    let buf_reader = BufReader::with_capacity(md.len() as usize, file);
    let mut dims: u32 = 0;
    let mut good_count = 0;
    let mut max_len: f64 = 0.0;
    let mut max_word = String::new();
    let re = Regex::new("^[a-zæøåé]+\\-?[a-zæøåé]*$").unwrap();
    let mut got_header = false;
    for line in buf_reader.lines() {
        let line = line?;
        let mut parts = line.split(" ");
        let k = parts.next().unwrap();
        if !got_header {
            got_header = true;
            let word_lines = k.parse::<u32>().unwrap();
            dims = parts.next().unwrap().parse::<u32>().unwrap();
            println!("{} word lines, {} dimensions", word_lines, dims);
        } else if re.is_match(&k) {
            let mut sum_of_squares: f64 = 0.0;
            while let Some(p) = parts.next() {
                let d = p.parse::<f64>().unwrap();
                sum_of_squares += d * d;
            }
            let vlen = sum_of_squares.sqrt();
            good_count += 1;
            if vlen > max_len {
                max_len = vlen;
                max_word = k.to_string();
            }
        }
    }
    println!("\naccepted words: {}", good_count);
    println!("max_len={}, for '{}'", max_len, max_word);
    Ok(())
}

fn main() -> std::io::Result<()> {
    let a: Vec<String> = env::args().collect();
    let mut opts = AliasOpts::new();
    opts.add("--file", CT::CTString)
        .add_alias("-f", "--file")
        .add("--help", CT::Flag)
        .parse(a[1..].to_vec());
    if let Some(fp) = opts.get("--file") {
        return check(&fp);
    }
    if let Some(_) = opts.get("--help") {
        print_help();
    }
    if opts.is_empty() {
        print_help();
    }
    Ok(())
}
