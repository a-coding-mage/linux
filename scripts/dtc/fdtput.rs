// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (c) 2011 The Chromium OS Authors. All rights reserved.
 */

use std::ffi::{c_char, c_int, c_void, CStr};
use std::ptr;

// Dependencies supplied by libfdt and util.h.
extern "C" {
    fn fdt_strerror(err: c_int) -> *const c_char;
    fn fdt_path_offset(blob: *const c_void, path: *const c_char) -> c_int;
    fn fdt_setprop(blob: *mut c_void, node: c_int, name: *const c_char,
                   val: *const c_char, len: c_int) -> c_int;
    fn fdt_subnode_offset_namelen(blob: *const c_void, parent: c_int,
                                  name: *const c_char, namelen: usize) -> c_int;
    fn fdt_add_subnode_namelen(blob: *mut c_void, parent: c_int,
                               name: *const c_char, namelen: usize) -> c_int;
    fn fdt_add_subnode(blob: *mut c_void, parent: c_int, name: *const c_char) -> c_int;
    fn utilfdt_read(filename: *const c_char) -> *mut c_char;
    fn utilfdt_write(filename: *const c_char, blob: *mut c_char) -> c_int;
    fn utilfdt_decode_type(arg: *const c_char, type_: *mut c_int, size: *mut c_int) -> c_int;
    fn cpu_to_fdt32(value: c_int) -> c_int;
    fn getopt(argc: c_int, argv: *mut *mut c_char, optstring: *const c_char) -> c_int;
    static mut optarg: *mut c_char;
    static mut optind: c_int;
}

/* These are the operations we support */
#[repr(C)]
enum oper_type {
    OPER_WRITE_PROP,
    OPER_CREATE_NODE,
}

#[repr(C)]
struct display_info {
    oper: oper_type,
    type_: c_int,
    size: c_int,
    verbose: c_int,
    auto_path: c_int,
}

unsafe fn report_error(name: *const c_char, mut namelen: c_int, err: c_int) {
    if namelen == -1 {
        namelen = libc_strlen(name) as c_int;
    }
    let error = fdt_strerror(err);
    eprintln!("Error at '{}': {}", c_string_prefix(name, namelen), CStr::from_ptr(error).to_string_lossy());
}

unsafe fn encode_value(disp: *mut display_info, mut arg: *mut *mut c_char,
                       mut arg_count: c_int, valuep: *mut *mut c_char,
                       value_len: *mut c_int) -> c_int {
    let mut value: *mut c_char = ptr::null_mut();
    let mut value_size = 0;
    let mut upto = 0;
    let mut fmt = [b'%', if (*disp).type_ != 0 { (*disp).type_ as u8 } else { b'd' }, 0];

    if (*disp).verbose != 0 { eprintln!("Decoding value:"); }
    while arg_count > 0 {
        let len = if (*disp).type_ == 's' as c_int {
            libc_strlen(*arg) as c_int + 1
        } else if (*disp).size == -1 { 4 } else { (*disp).size };
        if upto + len > value_size {
            value_size = upto + len + 500;
            value = libc_realloc(value as *mut c_void, value_size as usize) as *mut c_char;
            if value.is_null() { eprintln!("Out of mmory: cannot alloc {} bytes", value_size); return -1; }
        }
        let p = value.add(upto as usize);
        if (*disp).type_ == 's' as c_int {
            libc_memcpy(p as *mut c_void, *arg as *const c_void, len as usize);
            if (*disp).verbose != 0 { eprintln!("\tstring: '{}'", CStr::from_ptr(p).to_string_lossy()); }
        } else {
            let mut ival = 0;
            libc_sscanf(*arg, fmt.as_ptr() as *const c_char, &mut ival);
            if len == 4 { *(p as *mut c_int) = cpu_to_fdt32(ival); }
            else { *p = ival as u8 as c_char; }
            if (*disp).verbose != 0 { eprintln!("\t{}: {}", if (*disp).size == 1 { "byte" } else if (*disp).size == 2 { "short" } else { "int" }, ival); }
        }
        arg = arg.add(1); arg_count -= 1; upto += len;
    }
    *value_len = upto; *valuep = value;
    if (*disp).verbose != 0 { eprintln!("Value size {}", upto); }
    0
}

unsafe fn store_key_value(blob: *mut c_void, node_name: *const c_char,
                          property: *const c_char, buf: *const c_char, len: c_int) -> c_int {
    let node = fdt_path_offset(blob, node_name);
    if node < 0 { report_error(node_name, -1, node); return -1; }
    let err = fdt_setprop(blob, node, property, buf, len);
    if err != 0 { report_error(property, -1, err); return -1; }
    0
}

unsafe fn create_paths(blob: *mut c_void, in_path: *const c_char) -> c_int {
    let mut path = in_path;
    while *path as u8 == b'/' { path = path.add(1); }
    let mut offset = 0;
    while *path != 0 {
        let mut sep = path;
        while *sep != 0 && *sep as u8 != b'/' { sep = sep.add(1); }
        let n = sep.offset_from(path) as usize;
        let mut node = fdt_subnode_offset_namelen(blob, offset, path, n);
        if node == -FDT_ERR_NOTFOUND { node = fdt_add_subnode_namelen(blob, offset, path, n); }
        if node < 0 { report_error(path, n as c_int, node); return -1; }
        offset = node;
        path = if *sep == 0 { sep } else { sep.add(1) };
    }
    0
}

unsafe fn create_node(blob: *mut c_void, node_name: *mut c_char) -> c_int {
    let mut p = node_name.add(libc_strlen(node_name));
    while p != node_name && *p != b'/' as c_char { p = p.sub(1); }
    if p == node_name { report_error(node_name, -1, -FDT_ERR_BADPATH); return -1; }
    *p = 0;
    let mut node = 0;
    if p > node_name { node = fdt_path_offset(blob, node_name); if node < 0 { report_error(node_name, -1, node); return -1; } }
    node = fdt_add_subnode(blob, node, p.add(1));
    if node < 0 { report_error(p.add(1), -1, node); return -1; }
    0
}

unsafe fn do_fdtput(disp: *mut display_info, filename: *const c_char,
                    mut arg: *mut *mut c_char, mut arg_count: c_int) -> c_int {
    let blob = utilfdt_read(filename); if blob.is_null() { return -1; }
    let mut ret = 0;
    match (*disp).oper {
        oper_type::OPER_WRITE_PROP => {
            if (*disp).auto_path != 0 && create_paths(blob as *mut c_void, *arg) != 0 { return -1; }
            let mut value = ptr::null_mut(); let mut len = 0;
            if encode_value(disp, arg.add(2), arg_count - 2, &mut value, &mut len) != 0 || store_key_value(blob as *mut c_void, *arg, *arg.add(1), value, len) != 0 { ret = -1; }
        }
        oper_type::OPER_CREATE_NODE => while ret >= 0 && arg_count > 0 { ret = if (*disp).auto_path != 0 { create_paths(blob as *mut c_void, *arg) } else { create_node(blob as *mut c_void, *arg) }; arg = arg.add(1); arg_count -= 1; },
    }
    if ret >= 0 { ret = utilfdt_write(filename, blob); }
    libc_free(blob as *mut c_void); ret
}

const FDT_ERR_NOTFOUND: c_int = 1; // supplied by libfdt
const FDT_ERR_BADPATH: c_int = 5; // supplied by libfdt

extern "C" {
    fn libc_strlen(s: *const c_char) -> usize;
    fn libc_realloc(p: *mut c_void, size: usize) -> *mut c_void;
    fn libc_memcpy(dst: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn libc_sscanf(s: *const c_char, fmt: *const c_char, ...) -> c_int;
    fn libc_free(p: *mut c_void);
}

fn c_string_prefix(s: *const c_char, len: c_int) -> String {
    unsafe { std::slice::from_raw_parts(s as *const u8, len as usize).iter().map(|&c| c as char).collect() }
}

const USAGE_MSG: &str = "fdtput - write a property value to a device tree\n\nThe command line arguments are joined together into a single value.\n\nUsage:\n\tfdtput <options> <dt file> <node> <property> [<value>...]\n\tfdtput -c <options> <dt file> [<node>...]\nOptions:\n\t-c\t\tCreate nodes if they don't already exist\n\t-p\t\tAutomatically create nodes as needed for the node path\n\t-t <type>\tType of data\n\t-v\t\tVerbose: display each value decoded from command line\n\t-h\t\tPrint this help\n\n";

unsafe fn usage(msg: *const c_char) -> ! {
    if !msg.is_null() { eprintln!("Error: {}\n", CStr::from_ptr(msg).to_string_lossy()); }
    eprint!("{}", USAGE_MSG);
    std::process::exit(2)
}

#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, mut argv: *mut *mut c_char) -> c_int {
    let mut disp = display_info { oper: oper_type::OPER_WRITE_PROP, type_: 0, size: -1, verbose: 0, auto_path: 0 };
    let mut filename: *mut c_char = ptr::null_mut();
    loop {
        let c = getopt(argc, argv, b"chpt:v\0".as_ptr() as *const c_char);
        if c == -1 { break; }
        match c as u8 {
            b'c' => disp.oper = oper_type::OPER_CREATE_NODE,
            b'h' | b'?' => usage(ptr::null()),
            b'p' => disp.auto_path = 1,
            b't' => if utilfdt_decode_type(optarg, &mut disp.type_, &mut disp.size) != 0 { usage(b"Invalid type string\0".as_ptr() as *const c_char); },
            b'v' => disp.verbose = 1,
            _ => (),
        }
    }
    if optind < argc { filename = *argv.add(optind as usize); optind += 1; }
    if filename.is_null() { usage(b"Missing filename\0".as_ptr() as *const c_char); }
    argv = argv.add(optind as usize);
    let remaining = argc - optind;
    if matches!(disp.oper, oper_type::OPER_WRITE_PROP) {
        if remaining < 1 { usage(b"Missing node\0".as_ptr() as *const c_char); }
        if remaining < 2 { usage(b"Missing property\0".as_ptr() as *const c_char); }
    }
    if do_fdtput(&mut disp, filename, argv, remaining) != 0 { 1 } else { 0 }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
