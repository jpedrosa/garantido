// main.rs
// Copyright (c) 2019 Joao Pedrosa

use aliasopts::{AliasOpts, CT};
use std::env;
use regex::Regex;
use std::fs;
use std::fs::File;
use std::io::{self, BufReader};
// imports the io traits with a glob.
use std::io::prelude::*;


fn print_help() {
    println!(
r#"Program to split file lines.

Usage: split_lines <--file c:\path\to\file.txt> <--range nfrom..nto> [option]
    --file|-f         - Path to file to read the lines from.
    --range|-r        - Range from line number up to line number (exclusive).
                E.g.: --range 0..5000  Where the first or last number can be
                ommitted. --range ..5000 is the same as 0..5000
                If the last number is ommitted it will go to the last line
                number found on the targeted file.
    --help            - Shows this help.
"#);
}

fn split_lines(file_path: &str, from_number: usize, to_number: usize) 
    -> std::io::Result<()> {
    let mut file = File::open(&file_path)?;
    let mut buf_reader = BufReader::new(file);
    let mut lines: Vec<u8> = Vec::with_capacity(1024 * 1024);
    let mut buf = buf_reader.fill_buf()?;
    let mut i = 0;
    let mut len = buf.len();
    while len > 0 && i < to_number {
        let mut start_at = 0;
        if i < from_number {
            for j in 0..len {
                if buf[j] == 10 {
                    i += 1;
                    if i >= from_number {
                        start_at = j + 1;
                        break;
                    }
                }
            }
        }
        if i >= from_number {
            let mut stop_at = len; 
            for j in start_at..stop_at {
                if buf[j] == 10 {
                    i += 1;
                    if i >= to_number {
                        stop_at = j + 1;
                        break;
                    }
                }
            }
            lines.extend_from_slice(&buf[start_at..stop_at]);
        }
        buf_reader.consume(len);
        buf = buf_reader.fill_buf()?;
        len = buf.len();
    }
    io::stdout().write_all(&lines);
    Ok(())
}

fn main() -> std::io::Result<()> {
    let a: Vec<String> = env::args().collect();
    let mut opts = AliasOpts::new();
    opts.add("--file", CT::CTString)
        .add_alias("-f", "--file")
        .add("--range", CT::CTString)
        .add_alias("-r", "--range")
        .add("--help", CT::Flag)
        .parse(a[1..].to_vec());
    if let Some(s) = opts.get("--range") {
        let mut from_number: usize = 0;
        let mut to_number: usize = std::usize::MAX;
        let re = Regex::new("^(\\d+)?\\.\\.(\\d+)?$").unwrap();
        let m = re.captures(&s)
            .expect("Expected --range n..n of types usize..usize.");
        if let Some(nfrom) = m.get(1) {
            from_number = nfrom.as_str().parse::<usize>()
                .expect("Expected --range n.. value to be of type usize.");
        }
        if let Some(nto) = m.get(2) {
            to_number = nto.as_str().parse::<usize>()
                .expect("Expected --range ..n value to be of type usize.");
            if to_number <= from_number {
                panic!("Expected --range n..to_number to_number to be greater \
                    than the from_number (n).");
            }
        }
        let file_path = opts.get("--file").expect("Expected --file value.");
        return split_lines(&file_path, from_number, to_number);
    }
    if let Some(_) = opts.get("--help") {
        print_help();
    }
    if opts.is_empty() {
        print_help();
    }
    Ok(())
}
