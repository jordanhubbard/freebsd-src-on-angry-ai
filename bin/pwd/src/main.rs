// SPDX-License-Identifier: BSD-3-Clause
//
// Copyright (c) 1991, 1993, 1994
//	The Regents of the University of California.  All rights reserved.
// Copyright (c) 2026 Dag-Erling Smørgrav
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
// OUT OF THE USE OF THIS SOFTWARE, EVEN WITH THE POSSIBILITY OF
// SUCH DAMAGE.

use std::env;
use std::io::{self, Write};
use std::os::unix::fs::MetadataExt;
use std::process;

fn getcwd_logical() -> Option<String> {
    let pwd = env::var("PWD").ok()?;
    if !pwd.starts_with('/') {
        return None;
    }

    // Check for /./ or /../ components
    for component in pwd.split('/').skip(1) {
        if component == "." || component == ".." {
            return None;
        }
    }

    // Check that $PWD refers to the current directory
    let pwd_meta = match std::fs::metadata(&pwd) {
        Ok(m) => m,
        Err(_) => return None,
    };
    let dot_meta = match std::fs::metadata(".") {
        Ok(m) => m,
        Err(_) => return None,
    };

    if pwd_meta.dev() != dot_meta.dev() || pwd_meta.ino() != dot_meta.ino() {
        return None;
    }

    Some(pwd)
}

fn getcwd_physical() -> Option<String> {
    env::current_dir().ok()?.to_str().map(|s| s.to_string())
}

fn warn(msg: &str) {
    let stderr = io::stderr();
    let mut handle = stderr.lock();
    let _ = write!(handle, "pwd: {}: {}\n", msg, io::Error::last_os_error());
}

fn usage() -> ! {
    let msg = b"usage: pwd [-L | -P]\n";
    let _ = io::stderr().write_all(msg);
    process::exit(1);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut logical = true;
    let mut optind = 1;

    while optind < args.len() {
        let arg = &args[optind];
        if arg.starts_with('-') && arg.len() > 1 {
            for ch in arg[1..].chars() {
                match ch {
                    'L' => logical = true,
                    'P' => logical = false,
                    _ => usage(),
                }
            }
        } else {
            break;
        }
        optind += 1;
    }

    if optind < args.len() {
        usage();
    }

    let pwd = if logical {
        getcwd_logical().or_else(|| getcwd_physical())
    } else {
        getcwd_physical()
    };

    match pwd {
        Some(p) => {
            let stdout = io::stdout();
            let mut handle = stdout.lock();
            if let Err(_) = writeln!(handle, "{}", p) {
                warn("stdout");
                process::exit(1);
            }
            if let Err(_) = handle.flush() {
                warn("stdout");
                process::exit(1);
            }
        }
        None => {
            warn(".");
            process::exit(1);
        }
    }

    process::exit(0);
}