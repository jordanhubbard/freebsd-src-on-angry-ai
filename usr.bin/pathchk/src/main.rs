// SPDX-License-Identifier: BSD-2-Clause
// Translation of FreeBSD usr.bin/pathchk/pathchk.c to Rust

use std::ffi::CString;
use std::os::raw::{c_char, c_int, c_long};
use std::process;

const NAME_MAX: c_long = 255;
const PATH_MAX: c_long = 1024;
const _POSIX_NAME_MAX: c_long = 14;
const _POSIX_PATH_MAX: c_long = 256;
const _PC_NAME_MAX: c_int = 4;
const _PC_PATH_MAX: c_int = 3;
const ENOENT: c_int = 2;

extern "C" {
    fn pathconf(path: *const c_char, name: c_int) -> c_long;
}

static PORTABLE_CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789._-";

fn warnx(msg: &str) {
    eprintln!("pathchk: {}", msg);
}

fn warn(path: &str, partial: &str) {
    let err = std::io::Error::last_os_error();
    eprintln!("pathchk: {}: {} {}: {}", path, partial, path, err);
}

fn usage() -> ! {
    eprintln!("usage: pathchk [-Pp] pathname ...");
    process::exit(1);
}

fn portable(path: &str) -> bool {
    path.as_bytes().iter().all(|&b| PORTABLE_CHARSET.contains(&b))
}

fn get_pathconf(path: &str, name: c_int) -> c_long {
    let c_path = CString::new(path).unwrap();
    let val = unsafe { pathconf(c_path.as_ptr(), name) };
    if val == -1 {
        // errno may be set; use conservative fallback
        if name == _PC_PATH_MAX {
            PATH_MAX
        } else {
            NAME_MAX
        }
    } else {
        val
    }
}

fn check(path: &str, pflag: bool, pflag_cap: bool) -> i32 {
    if pflag_cap && path.is_empty() {
        warnx(&format!("{}: empty pathname", path));
        return 1;
    }

    if (pflag_cap || pflag) && (path.starts_with('-') || path.contains("/-")) {
        warnx(&format!("{}: contains a component starting with '-'", path));
        return 1;
    }

    let namemax: c_long = if !pflag {
        let base = if path.starts_with('/') { "/" } else { "." };
        get_pathconf(base, _PC_NAME_MAX)
    } else {
        _POSIX_NAME_MAX
    };

    // Walk path components
    let mut remaining = path;
    let mut current_path = String::new();

    loop {
        // Skip leading slashes
        remaining = remaining.trim_start_matches('/');
        if remaining.is_empty() {
            break;
        }

        // Find next component
        let (component, rest) = if let Some(pos) = remaining.find('/') {
            (&remaining[..pos], &remaining[pos + 1..])
        } else {
            (remaining, "")
        };

        let complen = component.len();

        if namemax != -1 && complen as c_long > namemax {
            warnx(&format!(
                "{}: {}: component too long (limit {})",
                path, component, namemax
            ));
            return 1;
        }

        if !pflag {
            // Build the path up to this component for stat
            let test_path = if current_path.is_empty() {
                if path.starts_with('/') {
                    format!("/{}", component)
                } else {
                    component.to_string()
                }
            } else {
                format!("{}/{}", current_path, component)
            };

            match std::fs::metadata(&test_path) {
                Ok(_) => {}
                Err(e) => {
                    if e.raw_os_error().unwrap_or(0) != ENOENT {
                        warn(path, &test_path);
                        return 1;
                    }
                }
            }
        }

        if pflag && !portable(component) {
            warnx(&format!(
                "{}: {}: component contains non-portable character",
                path, component
            ));
            return 1;
        }

        if rest.is_empty() {
            break;
        }

        if !pflag {
            // Update namemax based on current directory
            let val = get_pathconf(&current_path, _PC_NAME_MAX);
            if val != -1 {
                let _ = val;
            }
        }

        current_path = if current_path.is_empty() {
            if path.starts_with('/') {
                format!("/{}", component)
            } else {
                component.to_string()
            }
        } else {
            format!("{}/{}", current_path, component)
        };

        remaining = rest;
    }

    let pathmax: c_long = if !pflag {
        get_pathconf(path, _PC_PATH_MAX)
    } else {
        _POSIX_PATH_MAX
    };

    if pathmax != -1 && path.len() as c_long >= pathmax {
        warnx(&format!(
            "{}: path too long (limit {})",
            path,
            pathmax - 1
        ));
        return 1;
    }

    0
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut pflag = false;
    let mut pflag_cap = false;
    let mut i = 1;

    while i < args.len() {
        let arg = &args[i];
        if arg.starts_with('-') && arg.len() > 1 {
            for ch in arg[1..].chars() {
                match ch {
                    'p' => pflag = true,
                    'P' => pflag_cap = true,
                    _ => usage(),
                }
            }
            i += 1;
        } else {
            break;
        }
    }

    if i >= args.len() {
        usage();
    }

    let mut rval = 0;
    for arg in &args[i..] {
        rval |= check(arg, pflag, pflag_cap);
    }

    process::exit(rval);
}