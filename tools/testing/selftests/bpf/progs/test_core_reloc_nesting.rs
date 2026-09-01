// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2019 Facebook

// C dependencies: <linux/bpf.h>, <stdint.h>, <bpf/bpf_helpers.h>,
// <bpf/bpf_core_read.h>

use core::ffi::c_void;

extern "C" {
    fn bpf_core_read(dst: *mut c_void, sz: u32, src: *const c_void) -> i64;
}

#[link_section = "license"]
#[no_mangle]
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
#[derive(Copy, Clone)]
pub struct core_reloc_nesting_substruct {
    pub a: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union core_reloc_nesting_subunion {
    pub b: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union core_reloc_nesting_a_union {
    pub a: core_reloc_nesting_substruct,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct core_reloc_nesting_b_struct {
    pub b: core_reloc_nesting_subunion,
}

/* int a.a.a and b.b.b accesses */
#[repr(C)]
pub struct core_reloc_nesting {
    pub a: core_reloc_nesting_a_union,
    pub b: core_reloc_nesting_b_struct,
}

unsafe fn CORE_READ<T>(dst: *mut T, src: *const T) -> i64 {
    bpf_core_read(
        dst as *mut c_void,
        core::mem::size_of::<T>() as u32,
        src as *const c_void,
    )
}

#[link_section = "raw_tracepoint/sys_enter"]
#[no_mangle]
pub unsafe extern "C" fn test_core_nesting(ctx: *mut c_void) -> i32 {
    let in_: *mut core_reloc_nesting = data.in_.as_mut_ptr() as *mut c_void as *mut core_reloc_nesting;
    let out: *mut core_reloc_nesting = data.out.as_mut_ptr() as *mut c_void as *mut core_reloc_nesting;

    let _ = ctx;

    if CORE_READ(
        &mut (*out).a.a.a as *mut i32,
        &(*in_).a.a.a as *const i32,
    ) != 0
    {
        return 1;
    }
    if CORE_READ(
        &mut (*out).b.b.b as *mut i32,
        &(*in_).b.b.b as *const i32,
    ) != 0
    {
        return 1;
    }

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
