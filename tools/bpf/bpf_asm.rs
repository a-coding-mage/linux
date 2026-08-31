// SPDX-License-Identifier: GPL-2.0-only
/*
 * Minimal BPF assembler
 *
 * Instead of libpcap high-level filter expressions, it can be quite
 * useful to define filters in low-level BPF assembler (that is kept
 * close to Steven McCanne and Van Jacobson's original BPF paper).
 * In particular for BPF JIT implementors, JIT security auditors, or
 * just for defining BPF expressions that contain extensions which are
 * not supported by compilers.
 *
 * How to get into it:
 *
 * 1) read Documentation/networking/filter.rst
 * 2) Run `bpf_asm [-c] <filter-prog file>` to translate into binary
 *    blob that is loadable with xt_bpf, cls_bpf et al. Note: -c will
 *    pretty print a C-like construct.
 *
 * Copyright 2013 Daniel Borkmann <borkmann@redhat.com>
 */

use std::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

extern "C" {
    static mut stdin: *mut FILE;

    fn strncmp(s1: *const c_char, s2: *const c_char, n: usize) -> c_int;
    fn fopen(pathname: *const c_char, mode: *const c_char) -> *mut FILE;

    fn bpf_asm_compile(fp: *mut FILE, cstyle: bool);
}

unsafe fn main_impl(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut fp: *mut FILE = stdin;
    let mut cstyle: bool = false;
    let mut i: c_int;

    i = 1;
    while i < argc {
        if strncmp(b"-c\0".as_ptr() as *const c_char, *argv.offset(i as isize), 2) == 0 {
            cstyle = true;
            i += 1;
            continue;
        }

        fp = fopen(*argv.offset(i as isize), b"r\0".as_ptr() as *const c_char);
        if fp.is_null() {
            fp = stdin;
            i += 1;
            continue;
        }

        break;
    }

    bpf_asm_compile(fp, cstyle);

    0
}

#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let _ = std::mem::size_of::<c_void>();
    main_impl(argc, argv)
}
