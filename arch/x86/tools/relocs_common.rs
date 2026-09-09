// SPDX-License-Identifier: GPL-2.0
// Translated from relocs.h / relocs_common.c.

use std::ffi::{c_char, c_int, c_void};

// External C declarations supplied by the surrounding build.
#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

unsafe extern "C" {
    static mut stderr: *mut FILE;
    fn vfprintf(stream: *mut FILE, format: *mut c_char, ap: *mut c_void) -> c_int;
    fn exit(status: c_int) -> !;
    fn strcmp(lhs: *const c_char, rhs: *const c_char) -> c_int;
    fn fopen(path: *const c_char, mode: *const c_char) -> *mut FILE;
    fn fread(ptr: *mut c_void, size: usize, count: usize, stream: *mut FILE) -> usize;
    fn rewind(stream: *mut FILE);
    fn fclose(stream: *mut FILE) -> c_int;
    fn strerror(errnum: c_int) -> *const c_char;
    fn process_64(
        fp: *mut FILE,
        use_real_mode: c_int,
        as_text: c_int,
        show_absolute_syms: c_int,
        show_absolute_relocs: c_int,
        show_reloc_info: c_int,
    );
    fn process_32(
        fp: *mut FILE,
        use_real_mode: c_int,
        as_text: c_int,
        show_absolute_syms: c_int,
        show_absolute_relocs: c_int,
        show_reloc_info: c_int,
    );
}

const EI_NIDENT: usize = 16;
const EI_CLASS: usize = 4;
const ELFCLASS64: u8 = 2;

// The C variadic interface is retained for ABI compatibility.
pub unsafe extern "C" fn die(fmt: *mut c_char, mut args: ...) {
    vfprintf(stderr, fmt, &mut args as *mut _ as *mut c_void);
    exit(1);
}

unsafe fn usage() {
    die(
        b"relocs [--abs-syms|--abs-relocs|--reloc-info|--text|--realmode] vmlinux\n"
            .as_ptr() as *mut c_char,
    );
}

pub unsafe extern "C" fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut show_absolute_syms: c_int = 0;
    let mut show_absolute_relocs: c_int = 0;
    let mut show_reloc_info: c_int = 0;
    let mut as_text: c_int = 0;
    let mut use_real_mode: c_int = 0;
    let mut fname: *const c_char = std::ptr::null();
    let mut fp: *mut FILE;
    let mut e_ident = [0u8; EI_NIDENT];

    let mut i = 1;
    while i < argc {
        let arg = *argv.add(i as usize);
        if *arg == b'-' as c_char {
            if strcmp(arg, b"--abs-syms\0".as_ptr() as *const c_char) == 0 {
                show_absolute_syms = 1;
                i += 1;
                continue;
            }
            if strcmp(arg, b"--abs-relocs\0".as_ptr() as *const c_char) == 0 {
                show_absolute_relocs = 1;
                i += 1;
                continue;
            }
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
            if strcmp(arg, b"--realmode\0".as_ptr() as *const c_char) == 0 {
                use_real_mode = 1;
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
    fp = fopen(fname, b"r\0".as_ptr() as *const c_char);
    if fp.is_null() {
        die(b"Cannot open %s: %s\n\0".as_ptr() as *mut c_char, fname, strerror(*libc_errno()));
    }
    if fread(e_ident.as_mut_ptr() as *mut c_void, 1, EI_NIDENT, fp) != EI_NIDENT {
        die(b"Cannot read %s: %s\0".as_ptr() as *mut c_char, fname, strerror(*libc_errno()));
    }
    rewind(fp);
    if e_ident[EI_CLASS] == ELFCLASS64 {
        process_64(fp, use_real_mode, as_text, show_absolute_syms, show_absolute_relocs, show_reloc_info);
    } else {
        process_32(fp, use_real_mode, as_text, show_absolute_syms, show_absolute_relocs, show_reloc_info);
    }
    fclose(fp);
    0
}

unsafe extern "C" {
    fn libc_errno() -> *mut c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
