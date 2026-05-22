// SPDX-License-Identifier: BSD-2-Clause
//
// Copyright (c) 2009 Ed Schouten <ed@FreeBSD.org>
// All rights reserved.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met:
// 1. Redistributions of source code must retain the above copyright
//    notice, this list of conditions and the following disclaimer.
// 2. Redistributions in binary form must reproduce the above copyright
//    notice, this list of conditions and the following disclaimer in the
//    documentation and/or other materials provided with the distribution.
//
// THIS SOFTWARE IS PROVIDED BY THE AUTHOR AND CONTRIBUTORS ``AS IS'' AND
// ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
// IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
// ARE DISCLAIMED.  IN NO EVENT SHALL THE AUTHOR OR CONTRIBUTORS BE LIABLE
// FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
// DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS
// OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
// HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
// LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY
// OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF
// SUCH DAMAGE.

use std::env;
use std::ffi::CString;
use std::io::{self, Write};
use std::process;

extern "C" {
    fn revoke(filename: *const i8) -> i32;
}

fn usage() -> ! {
    let stderr = io::stderr();
    let mut handle = stderr.lock();
    let _ = handle.write_all(b"usage: revoke file ...\n");
    process::exit(1);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        usage();
    }

    let mut error = false;
    for arg in &args[1..] {
        let c_arg = match CString::new(arg.as_bytes()) {
            Ok(s) => s,
            Err(_) => {
                eprintln!("{}: invalid argument", arg);
                error = true;
                continue;
            }
        };
        unsafe {
            if revoke(c_arg.as_ptr()) != 0 {
                let err = std::io::Error::last_os_error();
                eprintln!("{}: {}", arg, err);
                error = true;
            }
        }
    }

    if error {
        process::exit(1);
    }
}