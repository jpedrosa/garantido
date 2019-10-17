// main.rs
// Copyright (c) 2019 Joao Pedrosa


use std::time::{Instant};

// fn lister(name: &str, path: &str) {
//     println!("name: {}  path: {}", name, path);
// }

// fn tryMuch(afn: fnMut ()) {
//     println!("before");
//     fnMut();
//     println!("after");
// }

fn main() {
    let mut total = 0;
    // let mut lister = |name: &str, path: &str| {
    //     total += 1;
    //     println!("name: {}  path: {}", name, path);
    // };
    let mut lister = |name: &str, path: &str| {
        total += 1;
        // println!("name: {}  path: {}", name, path);
    };
    let time = Instant::now();
    search_files::list("c:\\construir\\", &mut lister);
    let ms = time.elapsed().as_millis();
    // search_files::list("c:\\t_\\", &mut lister);
    // search_files::list("c:\\t_", lister);
    println!("Hello, world!");
    println!("total files: {}", total);
    println!("Elapsed: {} ms", ms);
}
