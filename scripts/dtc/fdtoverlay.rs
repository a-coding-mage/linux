// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (c) 2017 Konsulko Group Inc. All rights reserved.
 *
 * Author:
 *	 Pantelis Antoniou <pantelis.antoniou@konsulko.com>
 */

use core::ffi::{c_char, c_int, c_void};

// C headers and project headers provide these declarations and common option macros.
// #define BUF_INCREMENT 65536
const BUF_INCREMENT: usize = 65536;

#[repr(C)]
pub struct Option {
    pub name: *const c_char,
    pub has_arg: c_int,
    pub flag: *mut c_int,
    pub val: c_int,
}

unsafe extern "C" {
    fn fdt_totalsize(fdt: *const c_void) -> u32;
    fn fdt_open_into(fdt: *const c_void, buf: *mut c_void, bufsize: c_int) -> c_int;
    fn fdt_path_offset(fdt: *const c_void, path: *const c_char) -> c_int;
    fn fdt_overlay_apply(fdt: *mut c_void, fdto: *mut c_void) -> c_int;
    fn fdt_strerror(errval: c_int) -> *const c_char;
    fn fdt_pack(fdt: *mut c_void) -> c_int;

    fn xmalloc(size: usize) -> *mut c_void;
    fn xrealloc(ptr: *mut c_void, size: usize) -> *mut c_void;
    fn utilfdt_read(filename: *const c_char, len: *mut usize) -> *mut c_char;
    fn utilfdt_write(filename: *const c_char, blob: *const c_char) -> c_int;
    fn util_getopt_long() -> c_int;
    fn usage(message: *const c_char) -> !;

    static mut optarg: *mut c_char;
    static mut optind: c_int;
}

// FDT and getopt constants supplied by libfdt and util.h.
const FDT_ERR_NOSPACE: c_int = 3;
const EOF_VALUE: c_int = -1;
const REQUIRED_ARGUMENT: c_int = 1;
const NO_ARGUMENT: c_int = 0;

/* Usage related data. */
static USAGE_SYNOPSIS: &[u8] = b"apply a number of overlays to a base blob\n\tfdtoverlay <options> [<overlay.dtbo> [<overlay.dtbo>]]\0";
static USAGE_SHORT_OPTS: &[u8] = b"i:o:v\0"; // USAGE_COMMON_SHORT_OPTS is supplied by util.h.
static USAGE_LONG_OPTS: [Option; 4] = [
    Option { name: b"input\0".as_ptr() as *const c_char, has_arg: REQUIRED_ARGUMENT, flag: core::ptr::null_mut(), val: b'i' as c_int },
    Option { name: b"output\0".as_ptr() as *const c_char, has_arg: REQUIRED_ARGUMENT, flag: core::ptr::null_mut(), val: b'o' as c_int },
    Option { name: b"verbose\0".as_ptr() as *const c_char, has_arg: NO_ARGUMENT, flag: core::ptr::null_mut(), val: b'v' as c_int },
    Option { name: core::ptr::null(), has_arg: 0, flag: core::ptr::null_mut(), val: 0 }, // USAGE_COMMON_LONG_OPTS
];
static USAGE_OPTS_HELP: [&[u8]; 3] = [
    b"Input base DT blob\0",
    b"Output DT blob\0",
    b"Verbose messages\0",
]; // USAGE_COMMON_OPTS_HELP

#[no_mangle]
pub static mut verbose: c_int = 0;

unsafe fn apply_one(base: *mut c_char, overlay: *const c_char, buf_len: *mut usize, name: *const c_char) -> *mut c_char {
    let mut tmp: *mut c_char = core::ptr::null_mut();
    let tmpo = xmalloc(fdt_totalsize(overlay as *const c_void) as usize) as *mut c_char;
    let mut ret: c_int;
    let mut has_symbols: bool;

    loop {
        tmp = xrealloc(tmp as *mut c_void, *buf_len) as *mut c_char;
        ret = fdt_open_into(base as *const c_void, tmp as *mut c_void, *buf_len as c_int);
        if ret != 0 {
            eprintln!("\nFailed to make temporary copy: {}", c_string(fdt_strerror(ret)));
            libc_free(tmpo as *mut c_void);
            libc_free(tmp as *mut c_void);
            return core::ptr::null_mut();
        }
        ret = fdt_path_offset(tmp as *const c_void, b"/__symbols__\0".as_ptr() as *const c_char);
        has_symbols = ret >= 0;
        core::ptr::copy_nonoverlapping(overlay as *const u8, tmpo as *mut u8, fdt_totalsize(overlay as *const c_void) as usize);
        ret = fdt_overlay_apply(tmp as *mut c_void, tmpo as *mut c_void);
        if ret == -FDT_ERR_NOSPACE {
            *buf_len += BUF_INCREMENT;
        }
        if ret != -FDT_ERR_NOSPACE { break; }
    }
    if ret != 0 {
        eprintln!("\nFailed to apply '{}': {}", c_string(name), c_string(fdt_strerror(ret)));
        if !has_symbols { eprintln!("base blob does not have a '/__symbols__' node, make sure you have compiled the base blob with '-@' option"); }
        libc_free(tmpo as *mut c_void);
        libc_free(tmp as *mut c_void);
        return core::ptr::null_mut();
    }
    libc_free(base as *mut c_void);
    libc_free(tmpo as *mut c_void);
    tmp
}

unsafe fn do_fdtoverlay(input_filename: *const c_char, output_filename: *const c_char, argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut buf_len = 0usize;
    let mut blob = utilfdt_read(input_filename, &mut buf_len);
    if blob.is_null() { eprintln!("\nFailed to read '{}'", c_string(input_filename)); return -1; }
    if fdt_totalsize(blob as *const c_void) as usize > buf_len { eprintln!("\nBase blob is incomplete ({} bytes read)", buf_len); libc_free(blob as *mut c_void); return -1; }
    let ovblob = xmalloc(core::mem::size_of::<*mut c_char>() * argc as usize) as *mut *mut c_char;
    core::ptr::write_bytes(ovblob, 0, argc as usize);
    for i in 0..argc as isize {
        let mut ov_len = 0usize;
        *ovblob.offset(i) = utilfdt_read(*argv.offset(i), &mut ov_len);
        if (*ovblob.offset(i)).is_null() || fdt_totalsize(*ovblob.offset(i) as *const c_void) as usize > ov_len { eprintln!("\nFailed to read overlay"); return -1; }
    }
    buf_len = fdt_totalsize(blob as *const c_void) as usize;
    for i in 0..argc as isize { blob = apply_one(blob, *ovblob.offset(i), &mut buf_len, *argv.offset(i)); if blob.is_null() { return -1; } }
    fdt_pack(blob as *mut c_void);
    let ret = utilfdt_write(output_filename, blob);
    for i in 0..argc as isize { libc_free(*ovblob.offset(i) as *mut c_void); }
    libc_free(ovblob as *mut c_void); libc_free(blob as *mut c_void); ret
}

unsafe fn c_string(p: *const c_char) -> String { if p.is_null() { String::new() } else { core::ffi::CStr::from_ptr(p).to_string_lossy().into_owned() } }
unsafe fn libc_free(p: *mut c_void) { extern "C" { fn free(ptr: *mut c_void); } free(p); }

#[no_mangle]
pub unsafe extern "C" fn main(mut argc: c_int, mut argv: *mut *mut c_char) -> c_int {
    let mut input_filename: *mut c_char = core::ptr::null_mut();
    let mut output_filename: *mut c_char = core::ptr::null_mut();
    loop {
        let opt = util_getopt_long();
        if opt == EOF_VALUE { break; }
        match opt {
            b'i' as c_int => input_filename = optarg,
            b'o' as c_int => output_filename = optarg,
            b'v' as c_int => verbose = 1,
            _ => {}, // case_USAGE_COMMON_FLAGS
        }
    }
    if input_filename.is_null() { usage(b"missing input file\0".as_ptr() as *const c_char); }
    if output_filename.is_null() { usage(b"missing output file\0".as_ptr() as *const c_char); }
    argv = argv.offset(optind as isize);
    argc -= optind;
    if argc <= 0 { usage(b"missing overlay file(s)\0".as_ptr() as *const c_char); }
    if verbose != 0 {
        println!("input  = {}", c_string(input_filename));
        println!("output = {}", c_string(output_filename));
        for i in 0..argc { println!("overlay[{}] = {}", i, c_string(*argv.offset(i as isize))); }
    }
    if do_fdtoverlay(input_filename, output_filename, argc, argv) != 0 { return 1; }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
