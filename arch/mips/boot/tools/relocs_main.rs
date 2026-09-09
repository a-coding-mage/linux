// SPDX-License-Identifier: GPL-2.0

use core::ffi::{c_char, c_int, c_void};
use core::ptr;

// C headers and relocs.h provide these declarations and constants.
const EI_NIDENT: usize = 16;
const EI_CLASS: usize = 4;
const ELFCLASS64: u8 = 2;

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn die(fmt: *mut c_char, ...);
    fn vfprintf(stream: *mut FILE, format: *const c_char, ap: *mut c_void) -> c_int;
    fn exit(status: c_int) -> !;
    fn fopen(path: *const c_char, mode: *const c_char) -> *mut FILE;
    fn fread(ptr: *mut c_void, size: usize, count: usize, stream: *mut FILE) -> usize;
    fn rewind(stream: *mut FILE);
    fn fclose(stream: *mut FILE) -> c_int;
    fn strcmp(lhs: *const c_char, rhs: *const c_char) -> c_int;
    fn strerror(errnum: c_int) -> *const c_char;
    static mut errno: c_int;

    fn process_32(
        fp: *mut FILE,
        as_text: c_int,
        as_bin: c_int,
        show_reloc_info: c_int,
        keep_relocs: c_int,
    );
    fn process_64(
        fp: *mut FILE,
        as_text: c_int,
        as_bin: c_int,
        show_reloc_info: c_int,
        keep_relocs: c_int,
    );
}

unsafe fn usage() -> ! {
    die(b"relocs [--reloc-info|--text|--bin|--keep] vmlinux\n\0".as_ptr() as *mut c_char);
    exit(1)
}

pub unsafe fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut show_reloc_info: c_int = 0;
    let mut as_text: c_int = 0;
    let mut as_bin: c_int = 0;
    let mut keep_relocs: c_int = 0;
    let mut fname: *mut c_char = ptr::null_mut();
    let mut fp: *mut FILE;
    let mut i: c_int;
    let mut e_ident = [0u8; EI_NIDENT];

    i = 1;
    while i < argc {
        let arg = *argv.add(i as usize);

        if *arg as u8 == b'-' {
            if strcmp(arg, b"--reloc-info\0".as_ptr() as *const c_char) == 0 {
                show_reloc_info = 1;
                i += 1;
                continue;
            }
            if strcmp(arg, b"--text\0".as_ptr() as *const c_char) == 0 {
                as_text = 1;
                i += 1;
                continue;
            }
            if strcmp(arg, b"--bin\0".as_ptr() as *const c_char) == 0 {
                as_bin = 1;
                i += 1;
                continue;
            }
            if strcmp(arg, b"--keep\0".as_ptr() as *const c_char) == 0 {
                keep_relocs = 1;
                i += 1;
                continue;
            }
        } else if fname.is_null() {
            fname = arg;
            i += 1;
            continue;
        }
        usage();
    }
    if fname.is_null() {
        usage();
    }

    fp = fopen(fname, b"r+\0".as_ptr() as *const c_char);
    if fp.is_null() {
        die(
            b"Cannot open %s: %s\n\0".as_ptr() as *mut c_char,
            fname,
            strerror(errno),
        );
    }

    if fread(
        e_ident.as_mut_ptr() as *mut c_void,
        1,
        EI_NIDENT,
        fp,
    ) != EI_NIDENT {
        die(
            b"Cannot read %s: %s\0".as_ptr() as *mut c_char,
            fname,
            strerror(errno),
        );
    }

    rewind(fp);
    if e_ident[EI_CLASS] == ELFCLASS64 {
        process_64(fp, as_text, as_bin, show_reloc_info, keep_relocs);
    } else {
        process_32(fp, as_text, as_bin, show_reloc_info, keep_relocs);
    }
    fclose(fp);
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
