// SPDX-License-Identifier: (GPL-2.0-or-later OR BSD-2-Clause)
/*
 * libfdt - Flat Device Tree manipulation
 * Copyright (C) 2006 David Gibson, IBM Corporation.
 */

use core::ffi::{c_char, c_void};
use core::{mem, ptr};

extern "C" {
    fn fdt_getprop_namelen_w(
        fdt: *mut c_void,
        nodeoffset: i32,
        name: *const c_char,
        namelen: i32,
        lenp: *mut i32,
    ) -> *mut c_void;
    fn fdt_getprop(
        fdt: *mut c_void,
        nodeoffset: i32,
        name: *const c_char,
        lenp: *mut i32,
    ) -> *const c_void;
    fn fdt_get_property_w(
        fdt: *mut c_void,
        nodeoffset: i32,
        name: *const c_char,
        lenp: *mut i32,
    ) -> *mut fdt_property;
    fn fdt_next_node(fdt: *mut c_void, offset: i32, depth: *mut i32) -> i32;
    fn fdt_offset_ptr_w(fdt: *mut c_void, offset: i32, len: i32) -> *mut c_void;
    fn strlen(s: *const c_char) -> usize;
}

#[repr(C)]
pub struct fdt_property {
    _private: [u8; 0],
}

const FDT_ERR_NOSPACE: i32 = 3;
const FDT_NOP: u32 = 4;

#[inline]
unsafe fn cpu_to_fdt32(x: u32) -> u32 {
    x.to_be()
}

pub unsafe fn fdt_setprop_inplace_namelen_partial(
    fdt: *mut c_void,
    nodeoffset: i32,
    name: *const c_char,
    namelen: i32,
    idx: u32,
    val: *const c_void,
    len: i32,
) -> i32 {
    let mut proplen: i32 = 0;
    let propval = fdt_getprop_namelen_w(fdt, nodeoffset, name, namelen, &mut proplen);
    if propval.is_null() {
        return proplen;
    }

    if (proplen as u32) < (len as u32).wrapping_add(idx) {
        return -FDT_ERR_NOSPACE;
    }

    ptr::copy_nonoverlapping(
        val as *const u8,
        (propval as *mut u8).add(idx as usize),
        len as usize,
    );
    0
}

pub unsafe fn fdt_setprop_inplace(
    fdt: *mut c_void,
    nodeoffset: i32,
    name: *const c_char,
    val: *const c_void,
    len: i32,
) -> i32 {
    let mut proplen: i32 = 0;
    let propval = fdt_getprop(fdt, nodeoffset, name, &mut proplen);
    if propval.is_null() {
        return proplen;
    }

    if proplen != len {
        return -FDT_ERR_NOSPACE;
    }

    fdt_setprop_inplace_namelen_partial(
        fdt,
        nodeoffset,
        name,
        strlen(name) as i32,
        0,
        val,
        len,
    )
}

unsafe fn fdt_nop_region_(start: *mut c_void, len: i32) {
    let mut p = start as *mut u32;
    let end = (start as *mut u8).add(len as usize);
    while (p as *mut u8) < end {
        *p = cpu_to_fdt32(FDT_NOP);
        p = p.add(1);
    }
}

pub unsafe fn fdt_nop_property(fdt: *mut c_void, nodeoffset: i32, name: *const c_char) -> i32 {
    let mut len: i32 = 0;
    let prop = fdt_get_property_w(fdt, nodeoffset, name, &mut len);
    if prop.is_null() {
        return len;
    }

    fdt_nop_region_(prop as *mut c_void, len + mem::size_of::<fdt_property>() as i32);
    0
}

pub unsafe fn fdt_node_end_offset_(fdt: *mut c_void, mut offset: i32) -> i32 {
    let mut depth: i32 = 0;
    while offset >= 0 && depth >= 0 {
        offset = fdt_next_node(fdt, offset, &mut depth);
    }
    offset
}

pub unsafe fn fdt_nop_node(fdt: *mut c_void, nodeoffset: i32) -> i32 {
    let endoffset = fdt_node_end_offset_(fdt, nodeoffset);
    if endoffset < 0 {
        return endoffset;
    }

    fdt_nop_region_(
        fdt_offset_ptr_w(fdt, nodeoffset, 0),
        endoffset - nodeoffset,
    );
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
