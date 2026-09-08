// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (c) 2011 The Chromium OS Authors. All rights reserved.
 *
 * Portions from U-Boot cmd_fdt.c (C) Copyright 2007
 * Gerald Van Baren, Custom IDEAS, vanbaren@cideas.com
 * Based on code written by:
 *   Pantelis Antoniou <pantelis.antoniou@gmail.com> and
 *   Matthew McClintock <msm@freescale.com>
 */

use std::ffi::{c_char, c_int, c_void, CStr};
use std::ptr;

#[repr(C)]
pub enum display_mode {
    MODE_SHOW_VALUE,
    MODE_LIST_PROPS,
    MODE_LIST_SUBNODES,
}

#[repr(C)]
pub struct display_info {
    pub type_: c_int,
    pub size: c_int,
    pub mode: display_mode,
    pub default_val: *const c_char,
}

extern "C" {
    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
    fn printf(format: *const c_char, ... ) -> c_int;
    fn puts(s: *const c_char) -> c_int;
    fn exit(status: c_int) -> !;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn strlen(s: *const c_char) -> usize;
    fn getopt(argc: c_int, argv: *mut *mut c_char, optstring: *const c_char) -> c_int;
    static mut optarg: *mut c_char;
    static mut optind: c_int;
    static stderr: *mut c_void;

    fn fdt_strerror(err: c_int) -> *const c_char;
    fn fdt_first_property_offset(blob: *const c_void, node: c_int) -> c_int;
    fn fdt_get_property_by_offset(blob: *const c_void, offset: c_int, lenp: *mut c_int) -> *const fdt_property;
    fn fdt_string(blob: *const c_void, stroffset: u32) -> *const c_char;
    fn fdt_next_property_offset(blob: *const c_void, offset: c_int) -> c_int;
    fn fdt_next_tag(blob: *const c_void, node: c_int, nextoffset: *mut c_int) -> u32;
    fn fdt_get_name(blob: *const c_void, node: c_int, lenp: *mut c_int) -> *const c_char;
    fn fdt_getprop(blob: *const c_void, node: c_int, name: *const c_char, lenp: *mut c_int) -> *const c_void;
    fn fdt_path_offset(blob: *const c_void, path: *const c_char) -> c_int;
    fn util_is_printable_string(data: *const c_char, len: c_int) -> c_int;
    fn utilfdt_read(filename: *const c_char) -> *mut c_char;
    fn utilfdt_decode_type(arg: *const c_char, type_: *mut c_int, size: *mut c_int) -> c_int;
}

#[repr(C)]
struct fdt_property { tag: u32, len: u32, nameoff: u32 }

const FDT_ERR_NOTFOUND: c_int = 1;
const FDT_BEGIN_NODE: u32 = 1;
const FDT_END_NODE: u32 = 2;
const FDT_PROP: u32 = 3;
const FDT_END: u32 = 9;
const MAX_LEVEL: c_int = 32;

unsafe fn report_error(where_: *const c_char, err: c_int) {
    fprintf(stderr, b"Error at '%s': %s\n\0".as_ptr() as *const c_char, where_, fdt_strerror(err));
}

unsafe fn show_data(disp: *mut display_info, data: *const c_char, len: c_int) -> c_int {
    if len == 0 { return 0; }
    let is_string = (*disp).type_ == 's' as c_int || ((*disp).type_ == 0 && util_is_printable_string(data, len) != 0);
    if is_string {
        if *data.add((len - 1) as usize) != 0 {
            fprintf(stderr, b"Unterminated string\n\0".as_ptr() as *const c_char);
            return -1;
        }
        let mut s = data;
        while (s as usize).wrapping_sub(data as usize) < len as usize {
            if s != data { printf(b" \0".as_ptr() as *const c_char); }
            printf(b"%s\0".as_ptr() as *const c_char, s);
            s = s.add(strlen(s) + 1);
        }
        return 0;
    }
    let mut size = (*disp).size;
    if size == -1 { size = if len % 4 == 0 { 4 } else { 1 }; }
    else if len % size != 0 {
        fprintf(stderr, b"Property length must be a multiple of selected data size\n\0".as_ptr() as *const c_char);
        return -1;
    }
    let fmt = [b'%', if (*disp).type_ != 0 { (*disp).type_ as u8 } else { b'd' }, 0];
    let mut i = 0;
    let mut p = data as *const u8;
    while i < len {
        if i != 0 { printf(b" \0".as_ptr() as *const c_char); }
        let value: c_int = if size == 4 { u32::from_be(*(p as *const u32)) as c_int } else if size == 2 { ((*p as c_int) << 8) | *p.add(1) as c_int } else { *p as c_int };
        printf(fmt.as_ptr() as *const c_char, value);
        i += size; p = p.add(size as usize);
    }
    0
}

unsafe fn list_properties(blob: *const c_void, node: c_int) -> c_int {
    let mut prop = fdt_first_property_offset(blob, node);
    loop {
        if prop < 0 { return if prop == -FDT_ERR_NOTFOUND { 0 } else { prop }; }
        let data = fdt_get_property_by_offset(blob, prop, ptr::null_mut());
        let name = fdt_string(blob, u32::from_be((*data).nameoff));
        if !name.is_null() { puts(name); }
        prop = fdt_next_property_offset(blob, prop);
    }
}

unsafe fn list_subnodes(blob: *const c_void, mut node: c_int) -> c_int {
    let mut nextoffset = 0; let mut level = 0; let depth = 1;
    while level >= 0 {
        let tag = fdt_next_tag(blob, node, &mut nextoffset);
        match tag {
            FDT_BEGIN_NODE => { let mut pathp = fdt_get_name(blob, node, ptr::null_mut()); if level <= depth { if pathp.is_null() { pathp = b"/* NULL pointer error */\0".as_ptr() as *const c_char; } if *pathp == 0 { pathp = b"/\0".as_ptr() as *const c_char; } if level == 1 { puts(pathp); } } level += 1; if level >= MAX_LEVEL { puts(b"Nested too deep, aborting.\0".as_ptr() as *const c_char); return 1; } }
            FDT_END_NODE => { level -= 1; if level == 0 { level = -1; } }
            FDT_END => return 1,
            FDT_PROP => {},
            _ => { if level <= depth { printf(b"Unknown tag 0x%08X\n\0".as_ptr() as *const c_char, tag); } return 1; }
        }
        node = nextoffset;
    }
    0
}

unsafe fn show_data_for_item(blob: *const c_void, disp: *mut display_info, node: c_int, property: *const c_char) -> c_int {
    let mut len = 0; let mut err = 0;
    match (*disp).mode {
        display_mode::MODE_LIST_PROPS => err = list_properties(blob, node),
        display_mode::MODE_LIST_SUBNODES => err = list_subnodes(blob, node),
        display_mode::MODE_SHOW_VALUE => {
            let value = fdt_getprop(blob, node, property, &mut len);
            if !value.is_null() { if show_data(disp, value as *const c_char, len) != 0 { err = -1; } else { puts(b"\0".as_ptr() as *const c_char); } }
            else if !(*disp).default_val.is_null() { puts((*disp).default_val); }
            else { report_error(property, len); err = -1; }
        }
    }
    err
}

unsafe fn do_fdtget(disp: *mut display_info, filename: *const c_char, args: *mut *mut c_char, arg_count: c_int, args_per_step: c_int) -> c_int {
    let blob = utilfdt_read(filename); if blob.is_null() { return -1; }
    let mut i = 0;
    while i + args_per_step <= arg_count {
        let node = fdt_path_offset(blob as *const c_void, *args.add(i as usize));
        if node < 0 { if !(*disp).default_val.is_null() { puts((*disp).default_val); i += args_per_step; continue; } report_error(*args.add(i as usize), node); return -1; }
        let prop = if args_per_step == 1 { ptr::null() } else { *args.add((i + 1) as usize) };
        if show_data_for_item(blob as *const c_void, disp, node, prop) != 0 { return -1; }
        i += args_per_step;
    }
    0
}

static USAGE_MSG: &[u8] = b"fdtget - read values from device tree\n\nEach value is printed on a new line.\n\nUsage:\n\tfdtget <options> <dt file> [<node> <property>]...\n\tfdtget -p <options> <dt file> [<node> ]...\nOptions:\n\t-t <type>\tType of data\n\t-p\t\tList properties for each node\n\t-l\t\tList subnodes for each node\n\t-d\t\tDefault value to display when the property is missing\n\t-h\t\tPrint this help\n\n\0";

unsafe fn usage(msg: *const c_char) -> ! {
    if !msg.is_null() { fprintf(stderr, b"Error: %s\n\n\0".as_ptr() as *const c_char, msg); }
    fprintf(stderr, b"%s\0".as_ptr() as *const c_char, USAGE_MSG.as_ptr()); exit(2)
}

#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, mut argv: *mut *mut c_char) -> c_int {
    let mut filename: *mut c_char = ptr::null_mut();
    let mut disp: display_info = std::mem::zeroed(); disp.size = -1; disp.mode = display_mode::MODE_SHOW_VALUE;
    let mut args_per_step = 2;
    loop {
        let c = getopt(argc, argv, b"d:hlpt:\0".as_ptr() as *const c_char); if c == -1 { break; }
        match c as u8 {
            b'h' | b'?' => usage(ptr::null()),
            b't' => if utilfdt_decode_type(optarg, &mut disp.type_, &mut disp.size) != 0 { usage(b"Invalid type string\0".as_ptr() as *const c_char); },
            b'p' => { disp.mode = display_mode::MODE_LIST_PROPS; args_per_step = 1; },
            b'l' => { disp.mode = display_mode::MODE_LIST_SUBNODES; args_per_step = 1; },
            b'd' => disp.default_val = optarg,
            _ => {}
        }
    }
    if optind < argc { filename = *argv.add(optind as usize); optind += 1; }
    if filename.is_null() { usage(b"Missing filename\0".as_ptr() as *const c_char); }
    argv = argv.add(optind as usize); let count = argc - optind;
    if count == 0 { return 0; }
    if args_per_step == 2 && count % 2 != 0 { usage(b"Must have an even number of arguments\0".as_ptr() as *const c_char); }
    if do_fdtget(&mut disp, filename, argv, count, args_per_step) != 0 { return 1; } 0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
