use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_void};
use std::process;

type HesiodInitFn = unsafe extern "C" fn(*mut *mut c_void) -> c_int;
type HesiodResolveFn =
    unsafe extern "C" fn(*mut c_void, *const c_char, *const c_char) -> *mut *mut c_char;
type HesiodFreeListFn = unsafe extern "C" fn(*mut c_void, *mut *mut c_char);
type HesiodToBindFn =
    unsafe extern "C" fn(*mut c_void, *const c_char, *const c_char) -> *mut c_char;
type HesiodEndFn = unsafe extern "C" fn(*mut c_void);

struct HesiodFns {
    hesiod_init: HesiodInitFn,
    hesiod_resolve: HesiodResolveFn,
    hesiod_free_list: HesiodFreeListFn,
    hesiod_to_bind: HesiodToBindFn,
    hesiod_end: HesiodEndFn,
}

extern "C" {
    fn dlopen(filename: *const c_char, flag: c_int) -> *mut c_void;
    fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
    fn getopt(ac: c_int, av: *const *const c_char, optstr: *const c_char) -> c_int;
    static mut optind: c_int;
    fn free(p: *mut c_void);
    fn __error() -> *mut c_int;
}

fn load_hesiod() -> Option<HesiodFns> {
    unsafe {
        let handle = dlopen(std::ptr::null(), 2); // RTLD_NOW
        if handle.is_null() {
            return None;
        }

        let sym = |name: &str| -> *mut c_void {
            let c_name = CString::new(name).unwrap();
            dlsym(handle, c_name.as_ptr())
        };

        let init = std::mem::transmute::<*mut c_void, HesiodInitFn>(sym("hesiod_init"));
        let resolve = std::mem::transmute::<*mut c_void, HesiodResolveFn>(sym("hesiod_resolve"));
        let free_list =
            std::mem::transmute::<*mut c_void, HesiodFreeListFn>(sym("hesiod_free_list"));
        let to_bind = std::mem::transmute::<*mut c_void, HesiodToBindFn>(sym("hesiod_to_bind"));
        let end = std::mem::transmute::<*mut c_void, HesiodEndFn>(sym("hesiod_end"));

        Some(HesiodFns {
            hesiod_init: init,
            hesiod_resolve: resolve,
            hesiod_free_list: free_list,
            hesiod_to_bind: to_bind,
            hesiod_end: end,
        })
    }
}

fn usage() -> ! {
    eprintln!("usage: hesinfo [-bl] name type");
    eprintln!("\t-l selects long format");
    eprintln!("\t-b also does hes_to_bind conversion");
    process::exit(2);
}

fn get_errno() -> c_int {
    unsafe { *__error() }
}

fn main() {
    let args: Vec<CString> = std::env::args()
        .map(|s| CString::new(s).unwrap())
        .collect();
    let argc = args.len() as c_int;
    let argv: Vec<*const c_char> = args.iter().map(|s| s.as_ptr()).collect();

    let mut lflag = 0;
    let mut errflg = 0;
    let mut bflag = 0;

    let optstr = CString::new("lb").unwrap();
    unsafe {
        loop {
            let c = getopt(argc, argv.as_ptr(), optstr.as_ptr());
            if c == -1 {
                break;
            }
            match c as u8 {
                b'l' => lflag = 1,
                b'b' => bflag = 1,
                _ => errflg += 1,
            }
        }

        if argc - optind != 2 || errflg != 0 {
            usage();
        }

        let name = CStr::from_ptr(argv[optind as usize]).to_str().unwrap();
        let typ = CStr::from_ptr(argv[optind as usize + 1]).to_str().unwrap();

        let name_c = CString::new(name).unwrap();
        let typ_c = CString::new(typ).unwrap();

        let fns = match load_hesiod() {
            Some(f) => f,
            None => {
                eprintln!("hesinfo: cannot load hesiod functions");
                process::exit(1);
            }
        };

        let mut context: *mut c_void = std::ptr::null_mut();
        if (fns.hesiod_init)(&mut context) < 0 {
            let err = get_errno();
            if err == 78 {
                // ENOEXEC
                eprintln!("hesinfo: hesiod_init: Invalid Hesiod configuration file.");
            } else {
                eprint!("hesinfo: hesiod_init: ");
            }
        }

        if bflag != 0 {
            if lflag != 0 {
                eprint!("hes_to_bind({}, {}) expands to\n", name, typ);
            }
            let bindname = (fns.hesiod_to_bind)(context, name_c.as_ptr(), typ_c.as_ptr());
            if bindname.is_null() {
                if lflag != 0 {
                    eprint!("nothing\n");
                }
                let err = get_errno();
                if err == 2 {
                    // ENOENT
                    eprintln!("hesinfo: hesiod_to_bind: Unknown rhs-extension.");
                } else {
                    eprint!("hesinfo: hesiod_to_bind: ");
                }
                process::exit(1);
            }
            let bindname_str = CStr::from_ptr(bindname).to_str().unwrap();
            println!("{}", bindname_str);
            free(bindname as *mut c_void);
            if lflag != 0 {
                eprint!("which ");
            }
        }
        if lflag != 0 {
            eprint!("resolves to\n");
        }

        let list = (fns.hesiod_resolve)(context, name_c.as_ptr(), typ_c.as_ptr());
        if list.is_null() {
            if lflag != 0 {
                eprint!("nothing\n");
            }
            let err = get_errno();
            if err == 2 {
                // ENOENT
                eprintln!("hesinfo: hesiod_resolve: Hesiod name not found.");
            } else {
                eprint!("hesinfo: hesiod_resolve: ");
            }
            process::exit(1);
        }

        let mut p = list;
        loop {
            let entry = *p;
            if entry.is_null() {
                break;
            }
            let s = CStr::from_ptr(entry).to_str().unwrap();
            println!("{}", s);
            p = p.add(1);
        }

        (fns.hesiod_free_list)(context, list);
        (fns.hesiod_end)(context);
    }

    process::exit(0);
}