// SPDX-License-Identifier: BSD-2-Clause
//
// Copyright (c) 2002 Marcel Moolenaar
// Copyright (c) 2022 Tobias C. Berner
// All rights reserved.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met:
//
// 1. Redistributions of source code must retain the above copyright
//    notice, this list of conditions and the following disclaimer.
// 2. Redistributions in binary form must reproduce the above copyright
//    notice, this list of conditions and the following disclaimer in the
//    documentation and/or other materials provided with the distribution.
//
// THIS SOFTWARE IS PROVIDED BY THE AUTHOR ``AS IS'' AND ANY EXPRESS OR
// IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES
// OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE DISCLAIMED.
// IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR ANY DIRECT, INDIRECT,
// INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT
// NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
// DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
// THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
// (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF
// THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

use std::fs::File;
use std::io::{self, Read, Write};
use std::process;

#[repr(C)]
#[derive(Clone, Copy)]
struct Uuid {
    time_low: u32,
    time_mid: u16,
    time_hi_and_version: u16,
    clock_seq_hi_and_reserved: u8,
    clock_seq_low: u8,
    node: [u8; 6],
}

impl Uuid {
    fn to_string(&self) -> String {
        format!(
            "{:08x}-{:04x}-{:04x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
            self.time_low,
            self.time_mid,
            self.time_hi_and_version,
            self.clock_seq_hi_and_reserved,
            self.clock_seq_low,
            self.node[0],
            self.node[1],
            self.node[2],
            self.node[3],
            self.node[4],
            self.node[5],
        )
    }

    fn to_compact_string(&self) -> String {
        format!(
            "{:08x}{:04x}{:04x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
            self.time_low,
            self.time_mid,
            self.time_hi_and_version,
            self.clock_seq_hi_and_reserved,
            self.clock_seq_low,
            self.node[0],
            self.node[1],
            self.node[2],
            self.node[3],
            self.node[4],
            self.node[5],
        )
    }
}

fn read_random(buf: &mut [u8]) {
    let mut f = File::open("/dev/urandom").unwrap_or_else(|e| {
        eprintln!("uuidgen: /dev/urandom: {}", e);
        process::exit(1);
    });
    if f.read_exact(buf).is_err() {
        eprintln!("uuidgen: failed to read from /dev/urandom");
        process::exit(1);
    }
}

fn get_node_id() -> [u8; 6] {
    let mut node = [0u8; 6];
    // Try to read from /dev/random for node ID, fallback to random
    read_random(&mut node);
    // Set multicast bit
    node[0] |= 0x01;
    node
}

fn uuidgen_v4(uuids: &mut [Uuid]) {
    let mut buf = vec![0u8; 16 * uuids.len()];
    read_random(&mut buf);

    for (i, uuid) in uuids.iter_mut().enumerate() {
        let base = i * 16;
        uuid.time_low = u32::from_be_bytes([buf[base], buf[base + 1], buf[base + 2], buf[base + 3]]);
        uuid.time_mid = u16::from_be_bytes([buf[base + 4], buf[base + 5]]);
        uuid.time_hi_and_version = u16::from_be_bytes([buf[base + 6], buf[base + 7]]);
        uuid.clock_seq_hi_and_reserved = buf[base + 8];
        uuid.clock_seq_low = buf[base + 9];
        uuid.node = [
            buf[base + 10],
            buf[base + 11],
            buf[base + 12],
            buf[base + 13],
            buf[base + 14],
            buf[base + 15],
        ];

        // Set version 4 bits in clock_seq_hi_and_reserved
        uuid.clock_seq_hi_and_reserved &= 0x3F;
        uuid.clock_seq_hi_and_reserved |= 0x80;

        // Set version 4 bits in time_hi_and_version
        uuid.time_hi_and_version &= 0x0FFF;
        uuid.time_hi_and_version |= 0x4000;
    }
}

fn uuidgen_v1(uuids: &mut [Uuid], node: &[u8; 6]) {
    let mut buf = vec![0u8; 10 * uuids.len()];
    read_random(&mut buf);

    for (i, uuid) in uuids.iter_mut().enumerate() {
        let base = i * 10;
        let clock_seq = u16::from_be_bytes([buf[base], buf[base + 1]]);
        let clock_seq = (clock_seq & 0x3FFF) | 0x8000;

        // Use current time for timestamp (simplified v1)
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap();
        let mut time_val = now.as_nanos() as u64;
        // Add some randomness to differentiate
        time_val += u64::from(buf[base + 2]) << 40
            | u64::from(buf[base + 3]) << 32
            | u64::from(buf[base + 4]) << 24
            | u64::from(buf[base + 5]) << 16
            | u64::from(buf[base + 6]) << 8
            | u64::from(buf[base + 7]);

        uuid.time_low = (time_val & 0xFFFFFFFF) as u32;
        uuid.time_mid = ((time_val >> 32) & 0xFFFF) as u16;
        uuid.time_hi_and_version = (((time_val >> 48) & 0x0FFF) | 0x1000) as u16;
        uuid.clock_seq_hi_and_reserved = (clock_seq >> 8) as u8;
        uuid.clock_seq_low = clock_seq as u8;
        uuid.node = *node;
    }
}

fn usage() -> ! {
    let msg = "usage: uuidgen [-1] [-r] [-n count] [-o filename]\n";
    let _ = io::stderr().write_all(msg.as_bytes());
    process::exit(1);
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    let mut iterate = false;
    let mut compact = false;
    let mut version = 1;
    let mut count: Option<usize> = None;
    let mut output_file: Option<String> = None;

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "-1" => iterate = true,
            "-c" => compact = true,
            "-r" => version = 4,
            "-n" => {
                if count.is_some() {
                    usage();
                }
                i += 1;
                if i >= args.len() {
                    usage();
                }
                let n: usize = match args[i].parse() {
                    Ok(n) if n >= 1 => n,
                    _ => usage(),
                };
                count = Some(n);
            }
            "-o" => {
                if output_file.is_some() {
                    eprintln!("uuidgen: multiple output files not allowed");
                    process::exit(1);
                }
                i += 1;
                if i >= args.len() {
                    usage();
                }
                output_file = Some(args[i].clone());
            }
            _ => usage(),
        }
        i += 1;
    }

    let count = count.unwrap_or(1);

    let mut writer: Box<dyn Write> = match &output_file {
        Some(path) => {
            let f = File::create(path).unwrap_or_else(|e| {
                eprintln!("uuidgen: {}: {}", path, e);
                process::exit(1);
            });
            Box::new(f)
        }
        None => Box::new(io::stdout()),
    };

    let node = get_node_id();
    let mut store = vec![Uuid {
        time_low: 0,
        time_mid: 0,
        time_hi_and_version: 0,
        clock_seq_hi_and_reserved: 0,
        clock_seq_low: 0,
        node: [0; 6],
    }; count];

    if !iterate {
        if version == 1 {
            uuidgen_v1(&mut store, &node);
        } else if version == 4 {
            uuidgen_v4(&mut store);
        } else {
            eprintln!("uuidgen: unsupported version");
            process::exit(1);
        }
    } else {
        for uuid in store.iter_mut() {
            if version == 1 {
                uuidgen_v1(std::slice::from_mut(uuid), &node);
            } else if version == 4 {
                uuidgen_v4(std::slice::from_mut(uuid));
            } else {
                eprintln!("uuidgen: unsupported version");
                process::exit(1);
            }
        }
    }

    for uuid in &store {
        let s = if compact {
            uuid.to_compact_string()
        } else {
            uuid.to_string()
        };
        let _ = writeln!(writer, "{}", s);
    }

    let _ = writer.flush();
}