// SPDX-License-Identifier: BSD-2-Clause
// Translation of FreeBSD usr.bin/c99/c99.c to Rust

use std::env;
use std::os::unix::process::CommandExt;
use std::process;

fn usage() -> ! {
    eprintln!(
        "{}\n{}",
        "usage: c99 [-cEgs] [-D name[=value]] ... [-I directory] ... [-L directory] ...",
        "       [-o outfile] [-O optlevel] [-U name] ... operand ..."
    );
    process::exit(1);
}

fn addlib(cmd_args: &mut Vec<String>, lib: &str) {
    match lib {
        "pthread" => {
            cmd_args.push("-pthread".to_string());
        }
        "rt" => {
            // librt functionality is in libc or unimplemented.
        }
        "xnet" => {
            // xnet functionality is in libc.
        }
        _ => {
            cmd_args.push("-l".to_string());
            cmd_args.push(lib.to_string());
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut cmd_args: Vec<String> = Vec::new();

    cmd_args.push("/usr/bin/cc".to_string());
    cmd_args.push("-std=iso9899:1999".to_string());
    cmd_args.push("-pedantic".to_string());

    let mut i = 1;
    let mut found_lib = false;

    while i < args.len() {
        let arg = &args[i];

        if arg == "-l" || arg.starts_with("-l") {
            if arg == "-l" {
                i += 1;
                if i >= args.len() {
                    usage();
                }
                addlib(&mut cmd_args, &args[i]);
                i += 1;
            } else {
                addlib(&mut cmd_args, &arg[2..]);
                i += 1;
            }
            found_lib = true;
            break;
        }

        if arg.starts_with('-') {
            let ch = arg.chars().nth(1).unwrap_or('\0');
            match ch {
                'c' | 'D' | 'E' | 'g' | 'I' | 'L' | 'o' | 'O' | 's' | 'U' => {
                    cmd_args.push(arg.clone());
                    i += 1;
                }
                _ => {
                    usage();
                }
            }
        } else {
            cmd_args.push(arg.clone());
            i += 1;
        }
    }

    if !found_lib {
        while i < args.len() {
            let arg = &args[i];
            if arg == "-l" {
                i += 1;
                if i >= args.len() {
                    usage();
                }
                addlib(&mut cmd_args, &args[i]);
                i += 1;
            } else if arg.starts_with("-l") {
                addlib(&mut cmd_args, &arg[2..]);
                i += 1;
            } else {
                cmd_args.push(arg.clone());
                i += 1;
            }
        }
    }

    let mut cmd = process::Command::new(&cmd_args[0]);
    for arg in &cmd_args[1..] {
        cmd.arg(arg);
    }
    let err = cmd.exec();
    eprintln!("c99: /usr/bin/cc: {}", err);
    process::exit(1);
}