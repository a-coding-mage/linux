// SPDX-License-Identifier: GPL-2.0
// Translated from C implementation source: create-read.c

use std::env;
use std::ffi::CString;
use std::os::raw::{c_char, c_int, c_uint, c_void};
use std::process;

type ModeT = c_uint;
type SizeT = usize;
type SsizeT = isize;

const EXIT_FAILURE: i32 = 1;
const EXIT_SUCCESS: i32 = 0;

const O_RDWR: c_int = 0o00000002;
const O_CREAT: c_int = 0o00000100;

unsafe extern "C" {
    fn open(pathname: *const c_char, flags: c_int, mode: ModeT) -> c_int;
    fn read(fd: c_int, buf: *mut c_void, count: SizeT) -> SsizeT;
    fn close(fd: c_int) -> c_int;
    fn perror(s: *const c_char);
}

fn main() {
    let argv: Vec<String> = env::args().collect();
    let argc = argv.len() as c_int;
    let path: CString;
    let mut buf = [0 as c_char; 4];
    let fd: c_int;
    let rc: SsizeT;

    if argc < 2 {
        eprintln!("usage: {} <path>", argv[0]);
        process::exit(EXIT_FAILURE);
    }

    path = CString::new(argv[1].as_str()).unwrap();

    /* create a test variable */
    unsafe {
        fd = open(path.as_ptr(), O_RDWR | O_CREAT, 0o600);
        if fd < 0 {
            let msg = CString::new("open(O_WRONLY)").unwrap();
            perror(msg.as_ptr());
            process::exit(EXIT_FAILURE);
        }

        rc = read(fd, buf.as_mut_ptr() as *mut c_void, buf.len());
        if rc != 0 {
            eprintln!("Reading a new var should return EOF");
            close(fd);
            process::exit(EXIT_FAILURE);
        }

        close(fd);
    }
    process::exit(EXIT_SUCCESS);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
