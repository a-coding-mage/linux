// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2021 Facebook */

/* Original C dependencies:
 * #include "vmlinux.h"
 * #include <bpf/bpf_helpers.h>
 * #include <bpf/bpf_tracing.h>
 */

/* modifiers and typedefs are ignored when comparing key/value types */
#[repr(C)]
pub struct my_key {
    pub x: core::ffi::c_long,
}

pub type key_type = my_key;

#[repr(C)]
pub struct my_value {
    pub x: core::ffi::c_long,
}

pub type value_type = my_value;

const BPF_MAP_TYPE_HASH: u32 = 1;
const BPF_MAP_TYPE_ARRAY: u32 = 2;

#[repr(C)]
pub struct bpf_map_def {
    pub type_: u32,
    pub max_entries: u32,
}

unsafe extern "C" {
    #[link_name = "map1"]
    #[unsafe(link_section = ".maps")]
    pub static mut map1: bpf_map_def;

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

#[unsafe(link_section = ".maps")]
#[unsafe(no_mangle)]
pub static mut map2: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_ARRAY,
    max_entries: 8,
};

/* this definition will lose, but it has to exactly match the winner */
#[unsafe(link_section = ".maps")]
#[unsafe(no_mangle)]
pub static mut map_weak: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_ARRAY,
    max_entries: 16,
};

#[unsafe(no_mangle)]
pub static mut output_first2: core::ffi::c_int = 0;
#[unsafe(no_mangle)]
pub static mut output_second2: core::ffi::c_int = 0;
#[unsafe(no_mangle)]
pub static mut output_weak2: core::ffi::c_int = 0;

#[unsafe(link_section = "raw_tp/sys_enter")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handler_enter2() -> core::ffi::c_int {
    /* update values with key = 2 */
    let mut key: core::ffi::c_int = 2;
    let mut val: core::ffi::c_int = 2;
    let mut key_struct: key_type = key_type { x: 2 };
    let mut val_struct: value_type = value_type { x: 2000 };

    unsafe {
        bpf_map_update_elem(
            &raw mut map1 as *mut core::ffi::c_void,
            &raw const key_struct as *const core::ffi::c_void,
            &raw const val_struct as *const core::ffi::c_void,
            0,
        );
        bpf_map_update_elem(
            &raw mut map2 as *mut core::ffi::c_void,
            &raw const key as *const core::ffi::c_void,
            &raw const val as *const core::ffi::c_void,
            0,
        );
        bpf_map_update_elem(
            &raw mut map_weak as *mut core::ffi::c_void,
            &raw const key as *const core::ffi::c_void,
            &raw const val as *const core::ffi::c_void,
            0,
        );
    }

    0
}

#[unsafe(link_section = "raw_tp/sys_exit")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handler_exit2() -> core::ffi::c_int {
    /* lookup values with key = 1, set in another file */
    let mut key: core::ffi::c_int = 1;
    let mut val: *mut core::ffi::c_int;
    let mut key_struct: key_type = key_type { x: 1 };
    let mut value_struct: *mut value_type;

    unsafe {
        value_struct = bpf_map_lookup_elem(
            &raw mut map1 as *mut core::ffi::c_void,
            &raw const key_struct as *const core::ffi::c_void,
        ) as *mut value_type;
        if !value_struct.is_null() {
            output_first2 = (*value_struct).x as core::ffi::c_int;
        }

        val = bpf_map_lookup_elem(
            &raw mut map2 as *mut core::ffi::c_void,
            &raw const key as *const core::ffi::c_void,
        ) as *mut core::ffi::c_int;
        if !val.is_null() {
            output_second2 = *val;
        }

        val = bpf_map_lookup_elem(
            &raw mut map_weak as *mut core::ffi::c_void,
            &raw const key as *const core::ffi::c_void,
        ) as *mut core::ffi::c_int;
        if !val.is_null() {
            output_weak2 = *val;
        }
    }

    0
}

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static LICENSE: [u8; 4] = *b"GPL\0";
