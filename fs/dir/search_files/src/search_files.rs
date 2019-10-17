// search_files.rs
// Copyright (c) 2019 Joao Pedrosa

use std::fs;


// Declares a closure parameter that accepts anything from ordinary functions
// to closures that may mutate something in their environment.
pub fn list(fp: &str, afn: &mut impl FnMut(&str, &str)) {
    if let Ok(dir) = fs::read_dir(fp) {
        for entry in dir {
            if let Ok(entry) = entry {
                if let Ok(md) = entry.metadata() {
                    if let Some(name) = entry.file_name().to_str() {
                        if md.is_dir() {
                            let ap = path::join(&fp, &name);
                            list(&ap, afn);
                        } else if md.is_file() {
                            afn(&name, &fp);
                        }
                    }
                }
            }
        }
    }
}

