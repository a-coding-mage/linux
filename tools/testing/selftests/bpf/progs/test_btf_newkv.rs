// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2018 Facebook
// Original C dependencies:
// #include <linux/bpf.h>
// #include <bpf/bpf_helpers.h>
// #include "bpf_legacy.h"

#[repr(C)]
pub struct ipv_counts {
    pub v4: u32,
    pub v6: u32,
}

extern "C" {
    static BPF_MAP_TYPE_ARRAY: u32;

    fn bpf_map_lookup_elem(map: *mut core::ffi::c_void, key: *const core::ffi::c_void)
        -> *mut core::ffi::c_void;
}

#[repr(C)]
pub struct btf_map_def {
    pub type_: u32,
    pub max_entries: u32,
    pub key_size: u32,
    pub value_size: u32,
}

#[no_mangle]
#[link_section = ".maps"]
pub static mut btf_map: btf_map_def = btf_map_def {
    type_: unsafe { BPF_MAP_TYPE_ARRAY },
    max_entries: 4,
    key_size: core::mem::size_of::<i32>() as u32,
    value_size: core::mem::size_of::<ipv_counts>() as u32,
};

#[no_mangle]
#[inline(never)]
pub unsafe extern "C" fn test_long_fname_2() -> i32 {
    let counts: *mut ipv_counts;
    let key: i32 = 0;

    counts = bpf_map_lookup_elem(
        core::ptr::addr_of_mut!(btf_map).cast::<core::ffi::c_void>(),
        core::ptr::addr_of!(key).cast::<core::ffi::c_void>(),
    )
    .cast::<ipv_counts>();
    if counts.is_null() {
        return 0;
    }

    (*counts).v6 = (*counts).v6.wrapping_add(1);

    0
}

#[no_mangle]
#[inline(never)]
pub unsafe extern "C" fn test_long_fname_1() -> i32 {
    test_long_fname_2()
}

#[no_mangle]
#[link_section = "dummy_tracepoint"]
pub unsafe extern "C" fn _dummy_tracepoint(arg: *mut core::ffi::c_void) -> i32 {
    let _ = arg;
    test_long_fname_1()
}

#[no_mangle]
#[link_section = "license"]
pub static _license: [u8; 4] = *b"GPL\0";
