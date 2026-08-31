// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2019 Facebook

// C dependencies: <linux/bpf.h>, <stdint.h>, <bpf/bpf_helpers.h>,
// <bpf/bpf_core_read.h>

use core::ffi::c_void;
use core::mem::size_of_val;

extern "C" {
    fn bpf_core_read(dst: *mut c_void, sz: u32, src: *const c_void) -> i64;
}

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[repr(C)]
pub struct Data {
    pub in_: [i8; 256],
    pub out: [i8; 256],
}

#[no_mangle]
pub static mut data: Data = Data {
    in_: [0; 256],
    out: [0; 256],
};

#[repr(C)]
pub struct core_reloc_flavors {
    pub a: i32,
    pub b: i32,
    pub c: i32,
}

/* local flavor with reversed layout */
#[repr(C)]
pub struct core_reloc_flavors___reversed {
    pub c: i32,
    pub b: i32,
    pub a: i32,
}

#[repr(C)]
pub struct core_reloc_flavors___weird__anon_struct {
    pub b: i32,
}

#[repr(C)]
pub union core_reloc_flavors___weird__anon_union {
    pub a: i32,
    pub c: i32,
}

/* local flavor with nested/overlapping layout */
#[repr(C)]
pub struct core_reloc_flavors___weird {
    pub anon_struct: core_reloc_flavors___weird__anon_struct,
    /* a and c overlap in local flavor, but this should still work
     * correctly with target original flavor
     */
    pub anon_union: core_reloc_flavors___weird__anon_union,
}

macro_rules! CORE_READ {
    ($dst:expr, $src:expr) => {{
        bpf_core_read(
            $dst as *mut c_void,
            size_of_val(&*($dst)) as u32,
            $src as *const c_void,
        )
    }};
}

#[no_mangle]
#[link_section = "raw_tracepoint/sys_enter"]
pub unsafe extern "C" fn test_core_flavors(ctx: *mut c_void) -> i32 {
    let in_orig: *mut core_reloc_flavors = &mut data.in_ as *mut _ as *mut core_reloc_flavors;
    let in_rev: *mut core_reloc_flavors___reversed =
        &mut data.in_ as *mut _ as *mut core_reloc_flavors___reversed;
    let in_weird: *mut core_reloc_flavors___weird =
        &mut data.in_ as *mut _ as *mut core_reloc_flavors___weird;
    let out: *mut core_reloc_flavors = &mut data.out as *mut _ as *mut core_reloc_flavors;

    let _ = ctx;

    /* read a using weird layout */
    if CORE_READ!(
        &mut (*out).a as *mut i32,
        &(*in_weird).anon_union.a as *const i32
    ) != 0
    {
        return 1;
    }
    /* read b using reversed layout */
    if CORE_READ!(&mut (*out).b as *mut i32, &(*in_rev).b as *const i32) != 0 {
        return 1;
    }
    /* read c using original layout */
    if CORE_READ!(&mut (*out).c as *mut i32, &(*in_orig).c as *const i32) != 0 {
        return 1;
    }

    return 0;
}
