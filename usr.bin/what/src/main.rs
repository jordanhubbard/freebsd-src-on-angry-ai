use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write, Stdin};
use std::process;

fn usage() -> ! {
    eprintln!("usage: what [-qs] [file ...]");
    process::exit(1);
}

fn search<R: io::Read>(mut reader: BufReader<R>, one: bool, quiet: bool) -> bool {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut out = stdout.lock();
    let mut found = false;
    let mut buf = Vec::new();
    reader.read_until(b'\n', &mut buf).ok();

    let mut bytes = buf.as_slice();
    while let Some(pos) = bytes.iter().position(|&b| b == b'@') {
        let after = &bytes[pos + 1..];
        if after.len() >= 3 && after[0] == b'(' && after[1] == b'#' && after[2] == b')' {
            let content_start = pos + 4;
            let content = &bytes[content_start..];
            if !quiet {
                let _ = out.write_all(b"\t");
            }
            let end = content.iter().position(|&b| b == b'"' || b == b'>' || b == b'\\' || b == b'\n').unwrap_or(content.len());
            let _ = out.write_all(&content[..end]);
            let _ = out.write_all(b"\n");
            found = true;
            if one {
                break;
            }
        }
        bytes = &bytes[pos + 1..];
    }
    found
}

fn search_file(path: &str, one: bool, quiet: bool) -> bool {
    let file = match File::open(path) {
        Ok(f) => f,
        Err(_) => {
            if !quiet {
                eprintln!("what: {}: No such file or directory", path);
            }
            return false;
        }
    };
    if !quiet {
        println!("{}:", path);
    }
    search(BufReader::new(file), one, quiet)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut one = false;
    let mut quiet = false;
    let mut files: Vec<String> = Vec::new();

    let mut i = 1;
    while i < args.len() {
        if args[i] == "-q" {
            quiet = true;
        } else if args[i] == "-s" {
            one = true;
        } else if args[i].starts_with('-') {
            usage();
        } else {
            files.push(args[i].clone());
        }
        i += 1;
    }

    let mut found = false;
    if files.is_empty() {
        let stdin = io::stdin();
        found = search(BufReader::new(stdin), one, quiet);
    } else {
        for path in &files {
            if search_file(path, one, quiet) {
                found = true;
            }
        }
    }

    process::exit(if found { 0 } else { 1 });
}