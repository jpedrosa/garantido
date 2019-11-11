
use aliasopts::{AliasOpts, CT};
use std::env;
use std::io::{self, Write};

#[cfg(windows)]
const LINE_ENDING: &'static str = "\r\n";
#[cfg(not(windows))]
const LINE_ENDING: &'static str = "\n";

fn print_help() {
    println!(
r#"Program to echo newlines on Windows.

Usage: necho <"text\nother text"> [option]
    -n                - Omits printing the trailing newline.
    -wn               - Prints newline in the Windows format \r\n.
    -pn               - Prints newline in the Posix format \n.
    --help            - Shows this help.
"#);
}

fn main() -> std::io::Result<()> {
    let a: Vec<String> = env::args().collect();
    let mut opts = AliasOpts::new();
    opts.add("-n", CT::Flag)
        .add("-wn", CT::Flag)
        .add("-pn", CT::Flag)
        .add("--help", CT::Flag)
        .parse(a[1..].to_vec());
    if let Some(_) = opts.get("--help") {
        print_help();
    } else if opts.is_empty() {
        print_help();
    } else {
        let mut print_newline = true;
        let mut newline = LINE_ENDING;
        let mut z = String::new();
        if let Some(_) = opts.get("-n") {
            print_newline = false;
        } 
        if let Some(_) = opts.get("-wn") {
            newline = "\r\n";
        } else if let Some(_) = opts.get("-pn") {
            newline = "\n";
        }
        for i in 0..opts.loose_len() {
            if i > 0 {
                z.push_str(newline);
            }
            let s = opts.get_loose(i).unwrap();
            if s.contains('\\') {
                let mut escape = false;
                for c in s.chars() {
                    if c == '\\' {
                        if escape {
                            escape = false;
                            z.push('\\');
                        } else {
                            escape = true;
                        }
                    } else if c == 'n' && escape {
                        z.push('\n');
                        escape = false;
                    } else {
                        z.push(c);
                        if escape {
                            escape = false;
                        }
                    }
                }
            } else {
                z.push_str(&s);
            }
        }
        if print_newline {
            z.push_str(&newline);
        }
        print!("{}", z);
        io::stdout().flush()?;
    }
    Ok(())
}

