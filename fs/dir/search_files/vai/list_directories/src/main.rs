// main.rs
// Copyright (c) 2019 Joao Pedrosa

use aliasopts::{AliasOpts, CT};
use std::env;
use std::fs::{Metadata};
// use regex::Regex;


fn print_help() {
    println!(
r#"Program to list directories containing a matching name.

Usage: list_directories --name <directory name> --path <starting path> <option>
    --name            - The name of the directory to match for.
    --path            - The starting path to search from.
    --look_inside     - Continue looking inside a match for nested matches. By
                default it will not look inside.
    --exclude_name    - Exclude name from showing in the resulting paths.
    --sort_by <name|(creation_time|ct)>
                      - Sort by name or creation time. Creation time goes from
                most recent to oldest.
    --help            - Shows this help.
"#);
}

#[derive(Debug)]
struct SearchItem {
    name: String,
    path: String,
    md: Metadata
}

fn main() {
    let a: Vec<String> = env::args().collect();
    let mut opts = AliasOpts::new();
    opts.add("--path", CT::CTString)
        .add("--name", CT::CTString)
        .add("--look_inside", CT::Flag)
        .add("--exclude_name", CT::Flag)
        .add("--sort_by", CT::CTString)
        .add("--help", CT::Flag)
        .parse(a[1..].to_vec());
    if let Some(_) = opts.get("--help") {
        print_help();
        return;
    }
    if opts.is_empty() {
        print_help();
        return;
    }
    let _name = opts.get("--name")
        .expect("Expected a --name parameter. Check --help for options.");
    let _path = opts.get("--path")
        .expect("Expected a --path parameter. Check --help for options.");
    let mut sort_by = "";
    if let Some(sb) = opts.get("--sort_by") {
        match sb.as_ref() {
            "name" => sort_by = "name",
            "creation_time"|"ct" => sort_by = "creation_time",
            _ => {
                eprintln!("Unexpected --sort_by value: \"{}\". \
                    Check --help for options.", sb);
                return;
            }
        }
    }
    let mut matches: Vec<SearchItem> = Vec::new();
    let mut look_inside = false;
    if let Some(_) = opts.get("--look_inside") {
        look_inside = true;
    }
    let mut lister = |name: &str, path: &str, md: &Metadata| -> bool {
        if name == _name {
            matches.push(SearchItem{
                name: name.to_string(),
                path: path.to_string(),
                md: md.to_owned()});
            return look_inside;
        }
        true
    };
    search_files::list_directories(&_path, &mut lister);
    if sort_by.len() > 0 {
        if sort_by == "creation_time" {
            matches.sort_by(|a, b| 
                b.md.created().unwrap().partial_cmp(
                    &a.md.created().unwrap()).unwrap());
        } else { // sort_by == "name" 
            matches.sort_by(|a, b| 
                a.path.partial_cmp(&b.path).unwrap());
        }
    }
    let mut exclude_name = false;
    if let Some(_) = opts.get("--exclude_name") {
        exclude_name = true;
    }
    let mut z = String::new();
    for item in matches {
        if exclude_name {
            z.push_str(&item.path);
        } else {
            z.push_str(&path::join(&item.path, &item.name));
        }
        z.push('\n');
    }
    if z.len() > 0 {
        // Remove trailing \n
        z.truncate(z.len() - 1);
    }
    println!("{}", z);
}
