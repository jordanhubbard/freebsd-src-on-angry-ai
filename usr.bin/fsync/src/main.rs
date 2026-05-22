use std::env;
use std::fs::File;
use std::io;
use std::process;

fn usage() {
    eprintln!("usage: fsync file ...");
    process::exit(64);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        usage();
    }

    let mut rval = 0;
    for arg in &args[1..] {
        match File::open(arg) {
            Ok(file) => {
                if file.sync_all().is_err() {
                    eprintln!("fsync: fsync {}: {}", arg, io::Error::last_os_error());
                    if rval == 0 {
                        rval = 71;
                    }
                }
            }
            Err(e) => {
                eprintln!("fsync: open {}: {}", arg, e);
                if rval == 0 {
                    rval = 66;
                }
            }
        }
    }
    process::exit(rval);
}