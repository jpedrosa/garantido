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
    // println!("md {:?}", md);
    let buf_reader = BufReader::with_capacity(md.len() as usize, file);
    let mut dims: u32 = 0;
    let mut good_count = 0;
    let mut max_len: f64 = 0.0;
    let mut max_word = String::new();
    let re = Regex::new("^[a-zæøåé]+\\-?[a-zæøåé]*$").unwrap();
    for line in buf_reader.lines() {
        let line = line?;
        let parts = line.split(" ").collect::<Vec<_>>();
        if parts.len() <= 3 {
            let word_lines = parts[0].parse::<u32>().unwrap();
            dims = parts[1].parse::<u32>().unwrap();
            println!("{} word lines, {} dimensions", word_lines, dims);
        } else if re.is_match(parts[0]) {
            let mut sum_of_squares: f64 = 0.0;
            for i in 1..dims as usize + 1 {
                let d = parts[i].parse::<f64>().unwrap();
                sum_of_squares += d * d;
            }
            let vlen = sum_of_squares.sqrt();
            good_count += 1;
            if vlen > max_len {
                max_len = vlen;
                max_word = parts[0].to_string();
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
