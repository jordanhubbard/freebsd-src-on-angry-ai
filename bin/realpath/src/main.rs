use std::env;
use std::io::{self, Write};
use std::path::Path;
use std::process;

fn usage() -> ! {
    let _ = io::stderr().write_all(b"usage: realpath [-q] [path ...]\n");
    process::exit(1);
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    let mut qflag = false;
    let mut paths: Vec<String> = Vec::new();

    let mut i = 0;
    while i < args.len() {
        if args[i] == "-q" {
            qflag = true;
            i += 1;
        } else if args[i].starts_with('-') {
            usage();
        } else {
            paths.push(args[i].clone());
            i += 1;
        }
    }

    if paths.is_empty() {
        paths.push(".".to_string());
    }

    let mut rval = 0;
    for path_str in &paths {
        match Path::new(path_str).canonicalize() {
            Ok(p) => {
                let s = p.to_string_lossy();
                println!("{}", s);
            }
            Err(_) => {
                if !qflag {
                    let _ = writeln!(io::stderr(), "realpath: {}: {}", path_str, io::Error::last_os_error());
                }
                rval = 1;
            }
        }
    }
    process::exit(rval);
}