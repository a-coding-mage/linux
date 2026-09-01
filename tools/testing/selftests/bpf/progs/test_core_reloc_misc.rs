// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2019 Facebook

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]

use core::ffi::c_void;

// C dependencies: <linux/bpf.h>, <stdint.h>, <bpf/bpf_helpers.h>,
// and <bpf/bpf_core_read.h>.

extern "C" {
    fn bpf_core_read(dst: *mut c_void, sz: u32, src: *const c_void) -> i64;
}

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[repr(C)]
pub struct data_t {
    pub r#in: [i8; 256],
    pub out: [i8; 256],
}

#[no_mangle]
pub static mut data: data_t = data_t {
    r#in: [0; 256],
    out: [0; 256],
};

#[repr(C)]
pub struct core_reloc_misc_output {
    pub a: i32,
    pub b: i32,
    pub c: i32,
}

#[repr(C)]
pub struct core_reloc_misc___a {
    pub a1: i32,
    pub a2: i32,
}

#[repr(C)]
pub struct core_reloc_misc___b {
    pub b1: i32,
    pub b2: i32,
}

/* fixed two first members, can be extended with new fields */
#[repr(C)]
pub struct core_reloc_misc_extensible {
    pub a: i32,
    pub b: i32,
}

unsafe fn CORE_READ<T>(dst: *mut T, src: *const T) -> i64 {
    bpf_core_read(
        dst as *mut c_void,
        core::mem::size_of::<T>() as u32,
        src as *const c_void,
    )
}

#[no_mangle]
#[link_section = "raw_tracepoint/sys_enter"]
pub unsafe extern "C" fn test_core_misc(ctx: *mut c_void) -> i32 {
    let in_a: *mut core_reloc_misc___a = core::ptr::addr_of_mut!(data.r#in) as *mut c_void
        as *mut core_reloc_misc___a;
    let in_b: *mut core_reloc_misc___b = core::ptr::addr_of_mut!(data.r#in) as *mut c_void
        as *mut core_reloc_misc___b;
    let in_ext: *mut core_reloc_misc_extensible = core::ptr::addr_of_mut!(data.r#in) as *mut c_void
        as *mut core_reloc_misc_extensible;
    let out: *mut core_reloc_misc_output = core::ptr::addr_of_mut!(data.out) as *mut c_void
        as *mut core_reloc_misc_output;

    let _ = ctx;

    /* record two different relocations with the same accessor string */
    if CORE_READ(core::ptr::addr_of_mut!((*out).a), core::ptr::addr_of!((*in_a).a1)) != 0
        || CORE_READ(core::ptr::addr_of_mut!((*out).b), core::ptr::addr_of!((*in_b).b1)) != 0
    {
        /* accessor: 0:0 */
        /* accessor: 0:0 */
        return 1;
    }

    /* Validate relocations capture array-only accesses for structs with
     * fixed header, but with potentially extendable tail. This will read
     * first 4 bytes of 2nd element of in_ext array of potentially
     * variably sized struct core_reloc_misc_extensible. */
    if CORE_READ(
        core::ptr::addr_of_mut!((*out).c),
        in_ext.add(2) as *const core_reloc_misc_extensible as *const i32,
    ) != 0
    {
        /* accessor: 2 */
        return 1;
    }

    return 0;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
