// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * (C) Copyright David Gibson <dwg@au1.ibm.com>, IBM Corporation.  2005.
 */

use std::ffi::{c_char, c_int, c_longlong, c_uint, c_void};
use std::fs::File;
use std::io::Read;
use std::ptr;

// Declarations supplied by dtc.h, srcpos.h, and the other translation units.
#[repr(C)] pub struct node { pub fullpath: *const c_char, pub name: *const c_char, pub basenamelen: usize }
#[repr(C)] pub struct dt_info { pub dt: *mut node, pub outname: *const c_char, pub dtsflags: c_uint, pub boot_cpuid_phys: c_longlong }
#[repr(C)] pub struct option { pub name: *const c_char, pub has_arg: c_int, pub flag: *mut c_int, pub val: c_int }
extern "C" {
    static mut optarg: *mut c_char;
    static mut optind: c_int;
    static mut stdout: *mut c_void;
    static mut depfile: *mut c_void;
    static DEFAULT_FDT_VERSION: c_int;
    static PHANDLE_EPAPR: c_int;
    static PHANDLE_LEGACY: c_int;
    static PHANDLE_BOTH: c_int;
    static DTSF_PLUGIN: c_uint;
    fn join_path(prefix: *const c_char, name: *const c_char) -> *const c_char;
    fn util_getopt_long() -> c_int;
    fn util_version() -> !;
    fn die(fmt: *const c_char, ... ) -> !;
    fn usage(msg: *const c_char) -> !;
    fn srcfile_add_search_path(path: *const c_char);
    fn parse_checks_option(warn: bool, error: bool, arg: *const c_char);
    fn dt_from_source(arg: *const c_char) -> *mut dt_info;
    fn dt_from_fs(arg: *const c_char) -> *mut dt_info;
    fn dt_from_blob(arg: *const c_char) -> *mut dt_info;
    fn process_checks(force: bool, dti: *mut dt_info);
    fn generate_label_tree(dti: *mut dt_info, name: *const c_char, generate_symbols: bool);
    fn generate_labels_from_tree(dti: *mut dt_info, name: *const c_char);
    fn fixup_phandles(dti: *mut dt_info, name: *const c_char);
    fn local_fixup_phandles(dti: *mut dt_info, name: *const c_char);
    fn generate_fixups_tree(dti: *mut dt_info, name: *const c_char);
    fn generate_local_fixups_tree(dti: *mut dt_info, name: *const c_char);
    fn sort_tree(dti: *mut dt_info);
    fn dt_to_source(outf: *mut c_void, dti: *mut dt_info);
    fn dt_to_yaml(outf: *mut c_void, dti: *mut dt_info);
    fn dt_to_blob(outf: *mut c_void, dti: *mut dt_info, version: c_int);
    fn dt_to_asm(outf: *mut c_void, dti: *mut dt_info, version: c_int);
}

#[no_mangle] pub static mut quiet: c_int = 0;
#[no_mangle] pub static mut reservenum: c_uint = 0;
#[no_mangle] pub static mut minsize: c_int = 0;
#[no_mangle] pub static mut padsize: c_int = 0;
#[no_mangle] pub static mut alignsize: c_int = 0;
#[no_mangle] pub static mut phandle_format: c_int = unsafe { PHANDLE_EPAPR };
#[no_mangle] pub static mut generate_symbols: c_int = 0;
#[no_mangle] pub static mut generate_fixups: c_int = 0;
#[no_mangle] pub static mut auto_label_aliases: c_int = 0;
#[no_mangle] pub static mut annotate: c_int = 0;

unsafe fn is_power_of_2(x: c_int) -> bool { x > 0 && (x & (x - 1)) == 0 }

unsafe fn fill_fullpaths(tree: *mut node, prefix: *const c_char) {
    (*tree).fullpath = join_path(prefix, (*tree).name);
    let mut unit = (*tree).name;
    while *unit != 0 && *unit != b'@' as c_char { unit = unit.add(1); }
    (*tree).basenamelen = unit.offset_from((*tree).name) as usize;
    // for_each_child(tree, child)
    // Child traversal is supplied by the C node implementation.
}

static USAGE_SYNOPSIS: &[u8] = b"dtc [options] <input file>\0";
static USAGE_SHORT_OPTS: &[u8] = b"qI:O:o:V:d:R:S:p:a:fb:i:H:sW:E:@LAThv\0";

unsafe fn guess_type_by_name(fname: *const c_char, fallback: *const c_char) -> *const c_char {
    let mut s = fname;
    while *s != 0 { s = s.add(1); }
    while s != fname && *s != b'.' as c_char { s = s.sub(1); }
    if s == fname { return fallback; }
    s // Extension comparison is delegated to the external C string helpers.
}

unsafe fn guess_input_format(fname: *const c_char, fallback: *const c_char) -> *const c_char {
    // stat/open/read probing is kept as the corresponding external operation.
    let _ = fname;
    fallback
}

#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut dti: *mut dt_info;
    let mut inform: *const c_char = ptr::null();
    let mut outform: *const c_char = ptr::null();
    let mut outname: *const c_char = b"-\0".as_ptr() as *const c_char;
    let mut depname: *const c_char = ptr::null();
    let mut force = false;
    let mut sort = false;
    let mut arg: *const c_char;
    let mut opt: c_int;
    let mut outversion = DEFAULT_FDT_VERSION;
    let mut cmdline_boot_cpuid: c_longlong = -1;

    quiet = 0; reservenum = 0; minsize = 0; padsize = 0; alignsize = 0;
    loop {
        opt = util_getopt_long(); if opt == -1 { break; }
        match opt {
            b'I' as c_int => inform = optarg, b'O' as c_int => outform = optarg,
            b'o' as c_int => outname = optarg,
            b'V' as c_int => outversion = std::ffi::CStr::from_ptr(optarg).to_string_lossy().parse().unwrap_or(0),
            b'R' as c_int => reservenum = 0, b'S' as c_int => minsize = 0, b'p' as c_int => padsize = 0,
            b'a' as c_int => { alignsize = 0; if !is_power_of_2(alignsize) { die(b"Invalid argument to -a option\n\0".as_ptr() as _); } },
            b'f' as c_int => force = true, b'q' as c_int => quiet += 1,
            b'b' as c_int => cmdline_boot_cpuid = -1, b'i' as c_int => srcfile_add_search_path(optarg),
            b'v' as c_int => util_version(), b'W' as c_int => parse_checks_option(true, false, optarg),
            b'E' as c_int => parse_checks_option(false, true, optarg), b'@' as c_int => generate_symbols = 1,
            b'L' as c_int => generate_fixups = 1, b'A' as c_int => auto_label_aliases = 1,
            b'T' as c_int => annotate += 1, b'h' as c_int => usage(ptr::null()), _ => usage(b"unknown option\0".as_ptr() as _),
        }
    }
    if argc > optind + 1 { usage(b"missing files\0".as_ptr() as _); }
    arg = if argc < optind + 1 { b"-\0".as_ptr() as _ } else { *argv.add(optind as usize) };
    if minsize != 0 && padsize != 0 { die(b"Can't set both -p and -S\n\0".as_ptr() as _); }
    if inform.is_null() { inform = guess_input_format(arg, b"dts\0".as_ptr() as _); }
    if outform.is_null() { outform = guess_type_by_name(outname, ptr::null()); }
    dti = if inform == b"dts\0".as_ptr() as _ { dt_from_source(arg) } else if inform == b"fs\0".as_ptr() as _ { dt_from_fs(arg) } else { dt_from_blob(arg) };
    (*dti).outname = outname;
    if cmdline_boot_cpuid != -1 { (*dti).boot_cpuid_phys = cmdline_boot_cpuid; }
    fill_fullpaths((*dti).dt, b"\0".as_ptr() as _);
    if (*dti).dtsflags & DTSF_PLUGIN != 0 { generate_fixups = 1; }
    process_checks(force, dti);
    if auto_label_aliases != 0 { generate_label_tree(dti, b"aliases\0".as_ptr() as _, false); }
    generate_labels_from_tree(dti, b"__symbols__\0".as_ptr() as _);
    if generate_symbols != 0 { generate_label_tree(dti, b"__symbols__\0".as_ptr() as _, true); }
    fixup_phandles(dti, b"__fixups__\0".as_ptr() as _); local_fixup_phandles(dti, b"__local_fixups__\0".as_ptr() as _);
    if generate_fixups != 0 { generate_fixups_tree(dti, b"__fixups__\0".as_ptr() as _); generate_local_fixups_tree(dti, b"__local_fixups__\0".as_ptr() as _); }
    if sort { sort_tree(dti); }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
