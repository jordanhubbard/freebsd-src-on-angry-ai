// Copyright (c) 2014 Pietro Cerutti <gahr@FreeBSD.org>
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

use std::collections::BTreeSet;
use std::ffi::CStr;
use std::io::{self, Write};
use std::process;

extern "C" {
    fn setutxent();
    fn getutxent() -> *mut utmpx;
    fn endutxent();
}

const USER_PROCESS: i16 = 7;

#[repr(C)]
struct utmpx {
    ut_type: i16,
    ut_pad: i16,
    ut_pid: i32,
    ut_line: [i8; 16],
    ut_id: [i8; 8],
    ut_user: [i8; 32],
    ut_host: [i8; 128],
    ut_session: i32,
    ut_tv: timeval,
    ut_addr_v6: [i32; 4],
    __ut_unused: [i8; 20],
}

#[repr(C)]
struct timeval {
    tv_sec: i64,
    tv_usec: i64,
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        let stderr = io::stderr();
        let mut handle = stderr.lock();
        let _ = handle.write_all(b"usage: users\n");
        process::exit(1);
    }

    let mut names = BTreeSet::new();

    unsafe {
        setutxent();
        loop {
            let ut = getutxent();
            if ut.is_null() {
                break;
            }
            if (*ut).ut_type == USER_PROCESS {
                let user = CStr::from_ptr((*ut).ut_user.as_ptr());
                if let Ok(s) = user.to_str() {
                    if !s.is_empty() {
                        names.insert(s.to_string());
                    }
                }
            }
        }
        endutxent();
    }

    if !names.is_empty() {
        let stdout = io::stdout();
        let mut handle = stdout.lock();
        let names: Vec<&String> = names.iter().collect();
        for (i, name) in names.iter().enumerate() {
            if i > 0 {
                let _ = handle.write_all(b" ");
            }
            let _ = handle.write_all(name.as_bytes());
        }
        let _ = handle.write_all(b"\n");
    }
}