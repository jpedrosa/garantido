// path.rs
// Copyright (c) 2019 Joao Pedrosa

// static WIN_SEP_BYTES: &[u8; 1] = &[92];
// static POSIX_SEP_BYTES: &[u8; 1] = &[93];
// static WIN_SEP_STR: &[u8; 1] = &"\\";
// static POSIX_SEP_STR: &[u8; 1] = &"/";
// static SEP: char = std::path::MAIN_SEPARATOR;
// Hacky way to do conditional assignment in a global constant.
static CONDITION: usize = (std::path::MAIN_SEPARATOR == '\\') as usize;
static SEP_BYTES: &[u8; 1] = [&[93], &[92]][CONDITION];
// static SEP_BYTE: u8 = [93, 92][CONDITION];
static SEP_STR: &str = [&"/", &"\\"][CONDITION];

// pub fn join_bytes(a: &[u8], b: &[u8]) -> Vec<u8> {
//     let alen = a.len();
//     let blen = b.len();
//     if (alen > 0 && (a[alen -1] == 92 || a[alen -1] == 93)) ||
//         (blen > 0 && (b[0] == 92 || b[0] == 93)) {
//         return [a, b].concat().to_vec();
//     }
//     return [a, SEP_BYTES, b].concat().to_vec();
// }

pub fn join(a: &str, b: &str) -> String {
    if a.ends_with("/") || a.ends_with("\\") || b.starts_with("/") ||
        b.starts_with("\\") {
        return [a, b].concat().to_string();
    } else {
        return [a, SEP_STR, b].concat().to_string();
    }
}
