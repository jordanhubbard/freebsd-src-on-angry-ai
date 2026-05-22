// SPDX-License-Identifier: BSD-3-Clause
//
// Copyright (c) 1988, 1993
//	The Regents of the University of California.  All rights reserved.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met:
// 1. Redistributions of source code must retain the above copyright
//    notice, this list of conditions and the following disclaimer.
// 2. Redistributions in binary form must reproduce the above copyright
//    notice, this list of conditions and the following disclaimer in the
//    documentation and/or other materials provided with the distribution.
// 3. Neither the name of the University nor the names of its contributors
//    may be used to endorse or promote products derived from this software
//    without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE REGENTS AND CONTRIBUTORS ``AS IS'' AND
// ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
// IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
// ARE DISCLAIMED.  IN NO EVENT SHALL THE REGENTS OR CONTRIBUTORS BE LIABLE
// FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
// DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS
// OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
// HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
// LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY
// OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF
// SUCH DAMAGE.

use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};
use std::os::unix::net::UnixStream;
use std::process;

const BSIZE: usize = 8 * 1024;

struct Entry {
    writer: Box<dyn Write>,
    name: String,
}

fn usage() {
    eprintln!("usage: tee [-ai] [file ...]");
    process::exit(1);
}

fn tee_open(path: &str, append: bool) -> Option<(Box<dyn Write>, String)> {
    let mut builder = OpenOptions::new();
    builder.write(true).create(true);
    if append {
        builder.append(true);
    } else {
        builder.truncate(true);
    }

    if let Ok(f) = builder.open(path) {
        return Some((Box::new(f), path.to_string()));
    }

    // Try as Unix socket if open failed with EOPNOTSUPP (38 on FreeBSD)
    if io::Error::last_os_error().raw_os_error() == Some(38) {
        if let Ok(stream) = UnixStream::connect(path) {
            return Some((Box::new(stream), path.to_string()));
        }
    }

    None
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut append = false;
    let mut ignore_sigint = false;
    let mut files: Vec<String> = Vec::new();

    let mut i = 1;
    while i < args.len() {
        if args[i] == "-a" {
            append = true;
        } else if args[i] == "-i" {
            ignore_sigint = true;
        } else if args[i].starts_with('-') {
            usage();
        } else {
            files.push(args[i].clone());
        }
        i += 1;
    }

    // Ignore SIGINT if -i flag is set
    if ignore_sigint {
        // Use std::os::unix::signal to ignore SIGINT
        // We need to use the raw syscall since we can't use external crates
        let sigint = 2; // SIGINT
        let sig_ign = 1; // SIG_IGN
        extern "C" {
            fn signal(signum: i32, handler: extern "C" fn()) -> i32;
        }
        extern "C" fn sig_ign_handler() {}
        unsafe {
            signal(sigint, sig_ign_handler);
        }
    }

    let mut entries: Vec<Entry> = Vec::new();

    // Add stdout
    entries.push(Entry {
        writer: Box::new(io::stdout()),
        name: "stdout".to_string(),
    });

    let mut exitval = 0;
    for f in &files {
        match tee_open(f, append) {
            Some((writer, name)) => {
                entries.push(Entry { writer, name });
            }
            None => {
                eprintln!("tee: {}: {}", f, io::Error::last_os_error());
                exitval = 1;
            }
        }
    }

    let mut buf = vec![0u8; BSIZE];
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    loop {
        match handle.read(&mut buf) {
            Ok(0) => break,
            Ok(n) => {
                for entry in &mut entries {
                    let mut remaining = n;
                    let mut offset = 0;
                    while remaining > 0 {
                        match entry.writer.write(&buf[offset..offset + remaining]) {
                            Ok(0) => {
                                eprintln!("tee: {}: write returned 0", entry.name);
                                exitval = 1;
                                break;
                            }
                            Ok(w) => {
                                remaining -= w;
                                offset += w;
                            }
                            Err(e) => {
                                eprintln!("tee: {}: {}", entry.name, e);
                                exitval = 1;
                                break;
                            }
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("tee: read: {}", e);
                process::exit(1);
            }
        }
    }

    process::exit(exitval);
}