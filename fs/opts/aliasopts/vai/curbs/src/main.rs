// main.rs
// Copyright (c) 2019 Joao Pedrosa

use std::env;

fn lit(a: Vec<&str>) -> Vec<String> {
    a.iter().map(|s| s.to_string()).collect()
}

fn sample_url() {
    let a = lit(vec!["-u", "http://github.com"]);
    let mut opts = aliasopts::AliasOpts::new();
    opts.add("--url", aliasopts::CT::CTString)
        .add_alias("-u", "--url")
        .parse(a);
    assert_eq!(opts.get("--url"), Some("http://github.com".to_string()));
}

fn sample_file() {
    let a = lit(vec!["-f=shortcut/foo"]);
    let mut opts = aliasopts::AliasOpts::new();
    opts.add("--file", aliasopts::CT::CTString)
        .add_alias("-f", "--file")
        .parse(a);
    assert_eq!(opts.get("--file"), Some("shortcut/foo".to_string()));
}

fn sample_loose() {
    let a = lit(vec!["foo", "bar", "baz"]);
    let mut opts = aliasopts::AliasOpts::new();
    opts.parse(a);
    assert_eq!(opts.get_loose(2), Some("baz".to_string()));
}

fn sample_mode() {
    let a = lit(vec!["--mode", "foo", "bar"]);
    let mut opts = aliasopts::AliasOpts::new();
    opts.add("--mode", aliasopts::CT::CTString)
        .add_alias("-m", "--mode")
        .parse(a);
    assert_eq!(opts.get("--mode"), Some("foo".to_string()));
    assert_eq!(opts.get_loose(0), Some("bar".to_string()));
}

fn sample_funky_file() {
    let a = lit(vec!["--file", "foo", "-f", "bar", "--file", "baz"]);
    let mut opts = aliasopts::AliasOpts::new();
    opts.add("--file", aliasopts::CT::CTString)
        .add_alias("-f", "--file")
        .parse(a);
    assert_eq!(opts.get("--file"), Some("baz".to_string()));
}

fn sample_funky_file2() {
    let a = lit(vec!["--file", "-f", "foo"]);
    let mut opts = aliasopts::AliasOpts::new();
    opts.add("--file", aliasopts::CT::CTString)
        .add_alias("-f", "--file")
        .parse(a);
    assert_eq!(opts.get("--file"), Some("foo".to_string()));
}

fn sample_flag() {
    let a = lit(vec!["--file", "-f", "--enable-logging"]);
    let mut opts = aliasopts::AliasOpts::new();
    opts.add("--enable-logging", aliasopts::CT::Flag)
        .parse(a);
    assert_eq!(opts.get("--enable-logging"), Some("".to_string()));
}


fn main() {
    let a: Vec<String> = env::args().collect();
    println!("{:?}", &a[1..]);
    sample_url();
    sample_file();
    sample_loose();
    sample_mode();
    sample_funky_file();
    sample_funky_file2();
    sample_flag();
    let mut opts = aliasopts::AliasOpts::new();
    opts.add("--file", aliasopts::CT::CTString)
        .add_alias("-f", "--file")
        .parse(a);
    println!("opts: {:?}", opts);
    println!("--file: {:?}", opts.get("--file"));
}
