// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2021 Facebook */

// Dependencies from the original C source:
// #include "vmlinux.h"
// #include <bpf/bpf_helpers.h>
// #include <bpf/bpf_tracing.h>

#[repr(C)]
pub struct my_key {
    pub x: core::ffi::c_long,
}

#[repr(C)]
pub struct my_value {
    pub x: core::ffi::c_long,
}

// Original declaration:
// struct {
//     __uint(type, BPF_MAP_TYPE_HASH);
//     __type(key, struct my_key);
//     __type(value, struct my_value);
//     __uint(max_entries, 16);
// } map1 SEC(".maps");
#[repr(C)]
pub struct map1_def {
    pub type_: u32,
    pub key: *mut my_key,
    pub value: *mut my_value,
    pub max_entries: u32,
}

#[link_section = ".maps"]
#[no_mangle]
pub static mut map1: map1_def = map1_def {
    type_: BPF_MAP_TYPE_HASH,
    key: core::ptr::null_mut(),
    value: core::ptr::null_mut(),
    max_entries: 16,
};

 /* Matches map2 definition in linked_maps2.c. Order of the attributes doesn't
  * matter.
  */
#[repr(C)]
pub struct map2_t {
    pub max_entries: u32,
    pub key: *mut core::ffi::c_int,
    pub value: *mut core::ffi::c_int,
    pub type_: u32,
}

extern "C" {
    #[link_section = ".maps"]
    pub static mut map2: map2_t;
}

/* This should be the winning map definition, but we have no way of verifying,
 * so we just make sure that it links and works without errors
 */
// Original declaration used __weak SEC(".maps").
#[repr(C)]
pub struct map_weak_def {
    pub type_: u32,
    pub key: *mut core::ffi::c_int,
    pub value: *mut core::ffi::c_int,
    pub max_entries: u32,
}

#[link_section = ".maps"]
#[no_mangle]
pub static mut map_weak: map_weak_def = map_weak_def {
    type_: BPF_MAP_TYPE_ARRAY,
    key: core::ptr::null_mut(),
    value: core::ptr::null_mut(),
    max_entries: 16,
};

#[no_mangle]
pub static mut output_first1: core::ffi::c_int = 0;
#[no_mangle]
pub static mut output_second1: core::ffi::c_int = 0;
#[no_mangle]
pub static mut output_weak1: core::ffi::c_int = 0;

extern "C" {
    pub static BPF_MAP_TYPE_HASH: u32;
    pub static BPF_MAP_TYPE_ARRAY: u32;

    pub fn bpf_map_update_elem(
        map: *mut core::ffi::c_void,
        key: *const core::ffi::c_void,
        value: *const core::ffi::c_void,
        flags: u64,
    ) -> core::ffi::c_long;

    pub fn bpf_map_lookup_elem(
        map: *mut core::ffi::c_void,
        key: *const core::ffi::c_void,
    ) -> *mut core::ffi::c_void;
}

#[link_section = "raw_tp/sys_enter"]
#[no_mangle]
pub unsafe extern "C" fn handler_enter1() -> core::ffi::c_int {
    /* update values with key = 1 */
    let mut key: core::ffi::c_int = 1;
    let mut val: core::ffi::c_int = 1;
    let mut key_struct: my_key = my_key { x: 1 };
    let mut val_struct: my_value = my_value { x: 1000 };

    bpf_map_update_elem(
        &mut map1 as *mut _ as *mut core::ffi::c_void,
        &mut key_struct as *mut _ as *const core::ffi::c_void,
        &mut val_struct as *mut _ as *const core::ffi::c_void,
        0,
    );
    bpf_map_update_elem(
        &mut map2 as *mut _ as *mut core::ffi::c_void,
        &mut key as *mut _ as *const core::ffi::c_void,
        &mut val as *mut _ as *const core::ffi::c_void,
        0,
    );
    bpf_map_update_elem(
        &mut map_weak as *mut _ as *mut core::ffi::c_void,
        &mut key as *mut _ as *const core::ffi::c_void,
        &mut val as *mut _ as *const core::ffi::c_void,
        0,
    );

    return 0;
}

#[link_section = "raw_tp/sys_exit"]
#[no_mangle]
pub unsafe extern "C" fn handler_exit1() -> core::ffi::c_int {
    /* lookup values with key = 2, set in another file */
    let mut key: core::ffi::c_int = 2;
    let mut val: *mut core::ffi::c_int;
    let mut key_struct: my_key = my_key { x: 2 };
    let mut value_struct: *mut my_value;

    value_struct = bpf_map_lookup_elem(
        &mut map1 as *mut _ as *mut core::ffi::c_void,
        &mut key_struct as *mut _ as *const core::ffi::c_void,
    ) as *mut my_value;
    if !value_struct.is_null() {
        output_first1 = (*value_struct).x as core::ffi::c_int;
    }

    val = bpf_map_lookup_elem(
        &mut map2 as *mut _ as *mut core::ffi::c_void,
        &mut key as *mut _ as *const core::ffi::c_void,
    ) as *mut core::ffi::c_int;
    if !val.is_null() {
        output_second1 = *val;
    }

    val = bpf_map_lookup_elem(
        &mut map_weak as *mut _ as *mut core::ffi::c_void,
        &mut key as *mut _ as *const core::ffi::c_void,
    ) as *mut core::ffi::c_int;
    if !val.is_null() {
        output_weak1 = *val;
    }

    return 0;
}

#[link_section = "license"]
#[no_mangle]
pub static LICENSE: [core::ffi::c_char; 4] = [b'G' as core::ffi::c_char, b'P' as core::ffi::c_char, b'L' as core::ffi::c_char, 0];
