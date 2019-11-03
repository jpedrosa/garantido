// search_files.rs
// Copyright (c) 2019 Joao Pedrosa

use std::fs;
use std::fs::{Metadata};


// Declares a closure parameter that accepts anything from ordinary functions
// to closures that may mutate something in their environment.
// Lists files.
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


pub fn list_directories(fp: &str, afn: 
    &mut impl FnMut(&str, &str, &Metadata) -> bool) {
    if let Ok(dir) = fs::read_dir(fp) {
        for entry in dir {
            if let Ok(entry) = entry {
                if let Ok(md) = entry.metadata() {
                    if let Some(name) = entry.file_name().to_str() {
                        if md.is_dir() {
                            if afn(&name, &fp, &md) {
                                let ap = path::join(&fp, &name);
                                list_directories(&ap, afn);
                            }
                        }
                    }
                }
            }
        }
    }
}


pub fn find_filename(path: &str, recurse: bool, afn: 
    &mut impl FnMut(&str, &str) -> bool) -> bool {
    if let Ok(dir) = fs::read_dir(path) {
        for entry in dir {
            if let Ok(entry) = entry {
                if let Ok(md) = entry.metadata() {
                    if let Some(name) = entry.file_name().to_str() {
                        if md.is_dir() {
                            if recurse {
                                let ap = path::join(&path, &name);
                                if !find_filename(&ap, recurse, afn) {
                                    return false;
                                }
                            }
                        } else if afn(&name, &path) {
                            return false;
                        }
                    }
                }
            }
        }
    }
    true
}


pub fn list_filenames(path: &str, afn: &mut impl FnMut(&str, &str)) {
    if let Ok(dir) = fs::read_dir(path) {
        for entry in dir {
            if let Ok(entry) = entry {
                if let Ok(md) = entry.metadata() {
                    if let Some(name) = entry.file_name().to_str() {
                        if md.is_file() {
                            afn(&name, &path);
                        }
                    }
                }
            }
        }
    }
}
