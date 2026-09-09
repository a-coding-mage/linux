// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * This file does the necessary interface mapping between the bootwrapper
 * device tree operations and the interface provided by shared source
 * files flatdevicetree.[ch].
 *
 * Copyright 2007 David Gibson, IBM Corporation.
 */

use core::ffi::{c_char, c_int, c_void};

const DEBUG: c_int = 0;
const EXPAND_GRANULARITY: c_int = 1024;

extern "C" {
    fn fdt_totalsize(fdt: *const c_void) -> c_int;
    fn fdt_open_into(fdt: *const c_void, buf: *mut c_void, size: c_int) -> c_int;
    fn fdt_path_offset(fdt: *const c_void, path: *const c_char) -> c_int;
    fn fdt_getprop(fdt: *const c_void, nodeoffset: c_int, name: *const c_char,
                   lenp: *mut c_int) -> *const c_void;
    fn fdt_setprop(fdt: *mut c_void, nodeoffset: c_int, name: *const c_char,
                   val: *const c_void, len: c_int) -> c_int;
    fn fdt_del_node(fdt: *mut c_void, nodeoffset: c_int) -> c_int;
    fn fdt_parent_offset(fdt: *const c_void, nodeoffset: c_int) -> c_int;
    fn fdt_add_subnode(fdt: *mut c_void, parentoffset: c_int, name: *const c_char) -> c_int;
    fn fdt_node_offset_by_prop_value(fdt: *const c_void, startoffset: c_int,
                                     propname: *const c_char, propval: *const c_void,
                                     proplen: c_int) -> c_int;
    fn fdt_node_offset_by_compatible(fdt: *const c_void, startoffset: c_int,
                                     compatible: *const c_char) -> c_int;
    fn fdt_get_path(fdt: *const c_void, nodeoffset: c_int, buf: *mut c_char,
                    buflen: c_int) -> c_int;
    fn fdt_pack(fdt: *mut c_void) -> c_int;
    fn fdt_strerror(errval: c_int) -> *const c_char;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn strlen(s: *const c_char) -> usize;
    fn malloc(size: usize) -> *mut c_void;
    fn printf(format: *const c_char, ...) -> c_int;
    fn exit();
    fn fatal(format: *const c_char, ...);
    fn min(a: c_int, b: c_int) -> c_int;
    fn _ALIGN(value: c_int, alignment: c_int) -> c_int;
}

#[repr(C)]
pub struct PlatformOps {
    pub realloc: unsafe extern "C" fn(*mut c_void, usize) -> *mut c_void,
}

#[repr(C)]
pub struct DtOps {
    pub finddevice: Option<unsafe extern "C" fn(*const c_char) -> *mut c_void>,
    pub getprop: Option<unsafe extern "C" fn(*const c_void, *const c_char, *mut c_void, c_int) -> c_int>,
    pub setprop: Option<unsafe extern "C" fn(*const c_void, *const c_char, *const c_void, c_int) -> c_int>,
    pub get_parent: Option<unsafe extern "C" fn(*const c_void) -> *mut c_void>,
    pub create_node: Option<unsafe extern "C" fn(*const c_void, *const c_char) -> *mut c_void>,
    pub find_node_by_prop_value: Option<unsafe extern "C" fn(*const c_void, *const c_char, *const c_char, c_int) -> *mut c_void>,
    pub find_node_by_compatible: Option<unsafe extern "C" fn(*const c_void, *const c_char) -> *mut c_void>,
    pub del_node: Option<unsafe extern "C" fn(*const c_void) -> c_int>,
    pub get_path: Option<unsafe extern "C" fn(*const c_void, *mut c_char, c_int) -> *mut c_char>,
    pub finalize: Option<unsafe extern "C" fn() -> usize>,
}

extern "C" {
    static mut fdt: *mut c_void;
    static mut buf: *mut c_void;
    static mut platform_ops: PlatformOps;
    static mut dt_ops: DtOps;
}

unsafe fn check_err(err: c_int) -> c_int {
    if (err < 0 && err != -FDT_ERR_NOTFOUND && err != -FDT_ERR_EXISTS)
        || (err < 0 && DEBUG != 0) {
        printf(b"%s():%d  %s\n\r\0".as_ptr() as *const c_char,
               b"check_err\0".as_ptr() as *const c_char, 0, fdt_strerror(err));
    }
    if err < 0 && err != -FDT_ERR_NOTFOUND && err != -FDT_ERR_EXISTS { exit(); }
    if err < 0 { -1 } else { 0 }
}

const FDT_ERR_NOTFOUND: c_int = 1;
const FDT_ERR_EXISTS: c_int = 2;
const FDT_ERR_NOSPACE: c_int = 3;

unsafe fn offset_devp(off: c_int) -> *mut c_void {
    if check_err(off) != 0 { core::ptr::null_mut() } else { (off as usize + 1) as *mut c_void }
}
unsafe fn devp_offset_find(devp: *const c_void) -> c_int { (devp as usize - 1) as c_int }
unsafe fn devp_offset(devp: *const c_void) -> c_int { if !devp.is_null() { (devp as usize - 1) as c_int } else { 0 } }

unsafe fn expand_buf(minexpand: c_int) {
    let size = _ALIGN(fdt_totalsize(fdt) + minexpand, EXPAND_GRANULARITY);
    buf = (platform_ops.realloc)(buf, size as usize);
    if buf.is_null() { fatal(b"Couldn't find %d bytes to expand device tree\n\r\0".as_ptr() as *const c_char, size); }
    let rc = fdt_open_into(fdt, buf, size);
    if rc != 0 { fatal(b"Couldn't expand fdt into new buffer: %s\n\r\0".as_ptr() as *const c_char, fdt_strerror(rc)); }
    fdt = buf;
}

unsafe extern "C" fn fdt_wrapper_finddevice(path: *const c_char) -> *mut c_void { offset_devp(fdt_path_offset(fdt, path)) }
unsafe extern "C" fn fdt_wrapper_getprop(devp: *const c_void, name: *const c_char, out: *mut c_void, buflen: c_int) -> c_int {
    let mut len = 0; let p = fdt_getprop(fdt, devp_offset(devp), name, &mut len);
    if p.is_null() { return check_err(len); }
    memcpy(out, p, min(len, buflen) as usize); len
}
unsafe extern "C" fn fdt_wrapper_setprop(devp: *const c_void, name: *const c_char, value: *const c_void, len: c_int) -> c_int {
    let mut rc = fdt_setprop(fdt, devp_offset(devp), name, value, len);
    if rc == -FDT_ERR_NOSPACE { expand_buf(len + 16); rc = fdt_setprop(fdt, devp_offset(devp), name, value, len); }
    check_err(rc)
}
unsafe extern "C" fn fdt_wrapper_del_node(devp: *const c_void) -> c_int { fdt_del_node(fdt, devp_offset(devp)) }
unsafe extern "C" fn fdt_wrapper_get_parent(devp: *const c_void) -> *mut c_void { offset_devp(fdt_parent_offset(fdt, devp_offset(devp))) }
unsafe extern "C" fn fdt_wrapper_create_node(devp: *const c_void, name: *const c_char) -> *mut c_void {
    let mut offset = fdt_add_subnode(fdt, devp_offset(devp), name);
    if offset == -FDT_ERR_NOSPACE { expand_buf(strlen(name) as c_int + 16); offset = fdt_add_subnode(fdt, devp_offset(devp), name); }
    offset_devp(offset)
}
unsafe extern "C" fn fdt_wrapper_find_node_by_prop_value(prev: *const c_void, name: *const c_char, val: *const c_char, len: c_int) -> *mut c_void { offset_devp(fdt_node_offset_by_prop_value(fdt, devp_offset_find(prev), name, val as *const c_void, len)) }
unsafe extern "C" fn fdt_wrapper_find_node_by_compatible(prev: *const c_void, val: *const c_char) -> *mut c_void { offset_devp(fdt_node_offset_by_compatible(fdt, devp_offset_find(prev), val)) }
unsafe extern "C" fn fdt_wrapper_get_path(devp: *const c_void, out: *mut c_char, len: c_int) -> *mut c_char { let rc = fdt_get_path(fdt, devp_offset(devp), out, len); if check_err(rc) != 0 { core::ptr::null_mut() } else { out } }
unsafe extern "C" fn fdt_wrapper_finalize() -> usize { let rc = fdt_pack(fdt); if rc != 0 { fatal(b"Couldn't pack flat tree: %s\n\r\0".as_ptr() as *const c_char, fdt_strerror(rc)); } fdt as usize }

#[no_mangle]
pub unsafe extern "C" fn fdt_init(blob: *mut c_void) {
    dt_ops.finddevice = Some(fdt_wrapper_finddevice); dt_ops.getprop = Some(fdt_wrapper_getprop); dt_ops.setprop = Some(fdt_wrapper_setprop);
    dt_ops.get_parent = Some(fdt_wrapper_get_parent); dt_ops.create_node = Some(fdt_wrapper_create_node);
    dt_ops.find_node_by_prop_value = Some(fdt_wrapper_find_node_by_prop_value); dt_ops.find_node_by_compatible = Some(fdt_wrapper_find_node_by_compatible);
    dt_ops.del_node = Some(fdt_wrapper_del_node); dt_ops.get_path = Some(fdt_wrapper_get_path); dt_ops.finalize = Some(fdt_wrapper_finalize);
    fdt = blob; let bufsize = fdt_totalsize(fdt) + EXPAND_GRANULARITY; buf = malloc(bufsize as usize);
    if buf.is_null() { fatal(b"malloc failed. can't relocate the device tree\n\r\0".as_ptr() as *const c_char); }
    let err = fdt_open_into(fdt, buf, bufsize); if err != 0 { fatal(b"fdt_init(): %s\n\r\0".as_ptr() as *const c_char, fdt_strerror(err)); } fdt = buf;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
