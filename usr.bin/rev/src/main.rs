// SPDX-License-Identifier: BSD-3-Clause
//
// Copyright (c) 1987, 1992, 1993
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

use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write, Stdin};
use std::process;

fn usage() {
    eprintln!("usage: rev [file ...]");
    process::exit(1);
}

fn process_reader<R: BufRead>(mut reader: R, filename: &str) -> i32 {
    let stdout = io::stdout();
    let mut out = stdout.lock();
    let mut rval = 0;

    for line_result in reader.lines() {
        match line_result {
            Ok(line) => {
                let reversed: String = line.chars().rev().collect();
                if let Err(e) = writeln!(out, "{}", reversed) {
                    eprintln!("rev: {}: {}", filename, e);
                    rval = 1;
                }
            }
            Err(e) => {
                eprintln!("rev: {}: {}", filename, e);
                rval = 1;
            }
        }
    }

    rval
}

fn main() {
    let args: Vec<String> = env::args().collect();

    // rev accepts no options; any argument starting with '-' is an error
    let mut files: Vec<String> = Vec::new();
    for arg in &args[1..] {
        if arg.starts_with('-') {
            usage();
        }
        files.push(arg.clone());
    }

    let mut rval = 0;

    if files.is_empty() {
        // Read from stdin
        let stdin: Stdin = io::stdin();
        let reader = BufReader::new(stdin);
        rval = process_reader(reader, "stdin");
    } else {
        for filename in &files {
            match File::open(filename) {
                Ok(f) => {
                    let reader = BufReader::new(f);
                    rval |= process_reader(reader, filename);
                }
                Err(e) => {
                    eprintln!("rev: {}: {}", filename, e);
                    rval = 1;
                }
            }
        }
    }

    process::exit(rval);
}