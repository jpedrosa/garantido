// main.rs
// Copyright (c) 2019 Joao Pedrosa

// Implementation of a challenge found on the web:
// https://itnext.io/streams-for-the-win-a-performance-comparison-of-nodejs-methods-for-reading-large-datasets-pt-2-bcfa732fa40e

use aliasopts::{AliasOpts, CT};
use std::env;
use std::fs::File;
use std::io::{BufReader};
// imports the io traits with a glob.
use std::io::prelude::*;
use std::collections::HashMap;


fn print_help() {
    println!(
r#"Benchmark test.

Usage: drika <--file c:\path\to\file.txt>
    --file|-f         - Path to file to read the lines from.
    --help            - Shows this help.
"#);
}

fn check(file_path: &str,) -> std::io::Result<()> {
    let file = File::open(&file_path)?;
    let md = file.metadata()?;
    let cap = std::cmp::min(md.len() as usize, 1024 * 1024 * 500); // 500 MB
    let mut buf_reader = BufReader::with_capacity(cap, file);
    let mut buf = buf_reader.fill_buf()?;
    let mut len = buf.len();
    let mut line_count = 0;
    let mut names: HashMap<String, u32> = HashMap::new();
    let mut month_donations: HashMap<String, u32> = HashMap::new();
    let mut months_list: Vec<String> = Vec::new();
    let mut name_count = 0;
    let mut name_433 = String::new();
    let mut name_43244 = String::new();
    while len > 0 {
        let mut consume = 0;
        let mut col_count = 0;
        let mut name_start = 0;
        let mut i = 0;
        while i < len {
            let b = buf[i];
            if b == 10 {
                line_count += 1;
                consume = i + 1;
                col_count = 0;
                name_start = 0;
            } else if b == 124 { // |
                col_count += 1;
                if col_count == 4 { // date
                    let year = std::str::from_utf8(&buf[i + 1..i + 5])
                            .unwrap();
                    let month = std::str::from_utf8(&buf[i + 5..i + 7])
                            .unwrap();
                    i += 7;
                    let mut date = year.to_string();
                    date.push('-');
                    date.push_str(&month);
                    if let Some(c) = month_donations.get_mut(&date) {
                        *c += 1;
                    } else {
                        months_list.push(date.clone());
                        month_donations.insert(date, 1);
                    }
                } else if col_count == 7 { // name
                    name_start = i + 1;
                } else if col_count == 8 {
                    name_count += 1;
                    if name_count == 433 || name_count == 43244 {
                        let full_name = std::str::from_utf8(&buf[name_start..i])
                            .unwrap().to_string();
                        if name_count == 433 {
                            name_433 = full_name;
                        } else {
                            name_43244 = full_name;
                        }
                    }
                    for j in name_start..i {
                        if buf[j] == 44 { // ,
                            // Now skip spaces after the comma.
                            for m in j + 1..i {
                                if buf[m] != 32 { // space
                                    name_start = m;
                                    break;
                                }
                            }
                            break;
                        }
                    }
                    let mut name_end = i;
                    for j in name_start..i {
                        if buf[j] == 32 { // space
                            name_end = j;
                            break;
                        }
                    }
                    let name = std::str::from_utf8(&buf[name_start..name_end])
                        .unwrap();
                    if let Some(c) = names.get_mut(name) {
                        *c += 1;
                    } else {
                        names.insert(name.to_string(), 1);
                    }
                }
            }
            i += 1;
        }
        if consume == 0 {
            // Didn't get a newline even. Parse the last line and break.
            break;
        }
        buf_reader.consume(consume);
        buf = buf_reader.fill_buf()?;
        len = buf.len();
    }
    let mut top_name = String::new();
    let mut top_count: u32 = 0;
    for (name, count) in names.iter() {
        if count > &top_count {
            top_name = name.to_string();
            top_count = *count;
        }
    }
    println!("line_count {}", line_count);
    println!("Name 432: {}", name_433);
    println!("Name 43243: {}", name_43244);
    println!("Top name: {} {}", top_name, top_count);
    months_list.sort_by(|a, b| a.partial_cmp(b).unwrap());
    for date in months_list {
        println!("Donations per month and year: {} and donation count {}",
            date, month_donations[&date]);
    }
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
