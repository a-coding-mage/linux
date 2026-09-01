// SPDX-License-Identifier: GPL-2.0-only
/* Copyright (c) 2017 Facebook
 */
// C dependencies: <stddef.h>, <linux/bpf.h>, <bpf/bpf_helpers.h>, "bpf_misc.h"

type __u32 = u32;
type __u64 = u64;

const BPF_MAP_TYPE_ARRAY: __u32 = 2;

#[repr(C)]
pub struct test_map_id {
    type_: __u32,
    max_entries: __u32,
    key: __u32,
    value: __u64,
}

#[link_section = ".maps"]
#[no_mangle]
pub static mut test_map_id: test_map_id = test_map_id {
    type_: BPF_MAP_TYPE_ARRAY,
    max_entries: 1,
    key: 0,
    value: 0,
};

extern "C" {
    fn bpf_map_lookup_elem(map: *mut core::ffi::c_void, key: *const core::ffi::c_void) -> *mut core::ffi::c_void;
    fn __sink(value: *mut __u64);
}

#[link_section = "raw_tp/sys_enter"]
#[no_mangle]
pub unsafe extern "C" fn test_obj_id(ctx: *mut core::ffi::c_void) -> i32 {
    let mut key: __u32 = 0;
    let mut value: *mut __u64;

    value = bpf_map_lookup_elem(
        core::ptr::addr_of_mut!(test_map_id) as *mut core::ffi::c_void,
        core::ptr::addr_of_mut!(key) as *const core::ffi::c_void,
    ) as *mut __u64;
    __sink(value);

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
