use std::env;
use std::os::unix::process::CommandExt;
use std::process;

const CC: &str = "/usr/bin/cc";
const ARGS_PREPENDED: &[&str] = &["-std=iso9899:199409", "-pedantic"];

fn usage() -> ! {
    eprintln!(
        "usage: c89 [-cEgOs] [-D name[=value]] ... [-I directory] ... [-L directory] ...\n\
         [-o outfile] [-U name] ... operand ...\n\
         \n\
         where operand is one or more of file.c, file.o, file.a\n\
         or -llibrary"
    );
    process::exit(1);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let argc = args.len();

    if argc < 2 {
        eprintln!("missing operand");
        usage();
    }

    // Build argv: CC, prepended args, then parsed original args[1..]
    let mut argv: Vec<String> = Vec::with_capacity(2 + ARGS_PREPENDED.len() + argc);
    argv.push(CC.to_string());
    for arg in ARGS_PREPENDED {
        argv.push((*arg).to_string());
    }

    // Parse options to handle -l specially (starts operands)
    let valid_opts = "cD:EgI:l:L:o:OsU:";
    let mut i = 1;
    let mut found_operand = false;

    while i < argc {
        let arg = &args[i];
        if !arg.starts_with('-') || arg == "-" {
            found_operand = true;
            argv.push(arg.clone());
            i += 1;
            continue;
        }

        let opt_char = arg[1..].chars().next();
        match opt_char {
            Some(c) if valid_opts.contains(c) => {
                if c == 'l' {
                    found_operand = true;
                    argv.push(arg.clone());
                    i += 1;
                    while i < argc {
                        argv.push(args[i].clone());
                        i += 1;
                    }
                    break;
                }

                let opt_pos = valid_opts.find(c).unwrap();
                let takes_arg = opt_pos + 1 < valid_opts.len()
                    && valid_opts.as_bytes()[opt_pos + 1] == b':';

                if takes_arg {
                    let after_flag = &arg[2..];
                    if after_flag.is_empty() {
                        if i + 1 < argc {
                            argv.push(arg.clone());
                            i += 1;
                            argv.push(args[i].clone());
                            i += 1;
                        } else {
                            eprintln!("missing operand");
                            usage();
                        }
                    } else {
                        argv.push(arg.clone());
                        i += 1;
                    }
                } else {
                    argv.push(arg.clone());
                    i += 1;
                }
            }
            _ => {
                found_operand = true;
                argv.push(arg.clone());
                i += 1;
            }
        }
    }

    if !found_operand {
        eprintln!("missing operand");
        usage();
    }

    // Use exec via CommandExt
    let mut cmd = process::Command::new(CC);
    for arg in &argv[1..] {
        cmd.arg(arg);
    }

    let err = cmd.exec();
    eprintln!("execv({}): {}", CC, err);
    process::exit(1);
}