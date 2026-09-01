/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright (c) 2020 Facebook */

/* Dependencies from the original C source:
 * #include <linux/bpf.h>
 * #include <bpf/bpf_helpers.h>
 */

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(improper_ctypes)]

use core::ffi::c_void;

const BPF_MAP_TYPE_ARRAY: u32 = 2;
const BPF_MAP_TYPE_HASH_OF_MAPS: u32 = 13;
const BPF_MAP_TYPE_ARRAY_OF_MAPS: u32 = 12;
const BPF_MAP_TYPE_REUSEPORT_SOCKARRAY: u32 = 20;
const BPF_F_INNER_MAP: u32 = 0x1000;

#[repr(C)]
pub struct inner_map {
    pub type_: u32,
    pub max_entries: u32,
    pub key: i32,
    pub value: i32,
}

/* SEC(".maps") */
#[no_mangle]
pub static mut inner_map1: inner_map = inner_map {
    type_: BPF_MAP_TYPE_ARRAY,
    max_entries: 1,
    key: 0,
    value: 0,
};

/* SEC(".maps") */
#[no_mangle]
pub static mut inner_map2: inner_map = inner_map {
    type_: BPF_MAP_TYPE_ARRAY,
    max_entries: 1,
    key: 0,
    value: 0,
};

#[repr(C)]
pub struct inner_map_sz2 {
    pub type_: u32,
    pub max_entries: u32,
    pub key: i32,
    pub value: i32,
}

/* SEC(".maps") */
#[no_mangle]
pub static mut inner_map_sz2: inner_map_sz2 = inner_map_sz2 {
    type_: BPF_MAP_TYPE_ARRAY,
    max_entries: 2,
    key: 0,
    value: 0,
};

#[repr(C)]
pub struct outer_arr_value {
    pub type_: u32,
    /* changing max_entries to 2 will fail during load
     * due to incompatibility with inner_map definition */
    pub max_entries: u32,
    pub key: i32,
    pub value: i32,
}

#[repr(C)]
pub struct outer_arr {
    pub type_: u32,
    pub max_entries: u32,
    pub key: i32,
    pub value: i32,
    /* it's possible to use anonymous struct as inner map definition here */
    pub values: [*mut c_void; 3],
}

/* SEC(".maps") */
#[no_mangle]
pub static mut outer_arr: outer_arr = outer_arr {
    type_: BPF_MAP_TYPE_ARRAY_OF_MAPS,
    max_entries: 3,
    key: 0,
    value: 0,
    /* (void *) cast is necessary because we didn't use `struct inner_map`
     * in __inner(values, ...)
     * Actually, a conscious effort is required to screw up initialization
     * of inner map slots, which is a great thing!
     */
    values: [
        unsafe { &mut inner_map1 as *mut inner_map as *mut c_void },
        core::ptr::null_mut(),
        unsafe { &mut inner_map2 as *mut inner_map as *mut c_void },
    ],
};

#[repr(C)]
pub struct inner_map_sz3 {
    pub type_: u32,
    pub map_flags: u32,
    pub max_entries: u32,
    pub key: i32,
    pub value: i32,
}

/* SEC(".maps") */
#[no_mangle]
pub static mut inner_map3: inner_map_sz3 = inner_map_sz3 {
    type_: BPF_MAP_TYPE_ARRAY,
    map_flags: BPF_F_INNER_MAP,
    max_entries: 3,
    key: 0,
    value: 0,
};

/* SEC(".maps") */
#[no_mangle]
pub static mut inner_map4: inner_map_sz3 = inner_map_sz3 {
    type_: BPF_MAP_TYPE_ARRAY,
    map_flags: BPF_F_INNER_MAP,
    max_entries: 3,
    key: 0,
    value: 0,
};

#[repr(C)]
pub struct inner_map_sz4 {
    pub type_: u32,
    pub map_flags: u32,
    pub max_entries: u32,
    pub key: i32,
    pub value: i32,
}

/* SEC(".maps") */
#[no_mangle]
pub static mut inner_map5: inner_map_sz4 = inner_map_sz4 {
    type_: BPF_MAP_TYPE_ARRAY,
    map_flags: BPF_F_INNER_MAP,
    max_entries: 5,
    key: 0,
    value: 0,
};

#[repr(C)]
pub struct outer_arr_dyn_value {
    pub type_: u32,
    pub map_flags: u32,
    pub max_entries: u32,
    pub key: i32,
    pub value: i32,
}

#[repr(C)]
pub struct outer_arr_dyn {
    pub type_: u32,
    pub max_entries: u32,
    pub key: i32,
    pub value: i32,
    pub values: [*mut c_void; 3],
}

/* SEC(".maps") */
#[no_mangle]
pub static mut outer_arr_dyn: outer_arr_dyn = outer_arr_dyn {
    type_: BPF_MAP_TYPE_ARRAY_OF_MAPS,
    max_entries: 3,
    key: 0,
    value: 0,
    values: [
        unsafe { &mut inner_map3 as *mut inner_map_sz3 as *mut c_void },
        unsafe { &mut inner_map4 as *mut inner_map_sz3 as *mut c_void },
        unsafe { &mut inner_map5 as *mut inner_map_sz4 as *mut c_void },
    ],
};

#[repr(C)]
pub struct outer_hash {
    pub type_: u32,
    pub max_entries: u32,
    pub key: i32,
    /* Here everything works flawlessly due to reuse of struct inner_map
     * and compiler will complain at the attempt to use non-inner_map
     * references below. This is great experience.
     */
    pub values: [*mut inner_map; 5],
}

/* SEC(".maps") */
#[no_mangle]
pub static mut outer_hash: outer_hash = outer_hash {
    type_: BPF_MAP_TYPE_HASH_OF_MAPS,
    max_entries: 5,
    key: 0,
    values: [
        unsafe { &mut inner_map2 as *mut inner_map },
        core::ptr::null_mut(),
        core::ptr::null_mut(),
        core::ptr::null_mut(),
        unsafe { &mut inner_map1 as *mut inner_map },
    ],
};

#[repr(C)]
pub struct sockarr_sz1 {
    pub type_: u32,
    pub max_entries: u32,
    pub key: i32,
    pub value: i32,
}

/* SEC(".maps") */
#[no_mangle]
pub static mut sockarr_sz1: sockarr_sz1 = sockarr_sz1 {
    type_: BPF_MAP_TYPE_REUSEPORT_SOCKARRAY,
    max_entries: 1,
    key: 0,
    value: 0,
};

#[repr(C)]
pub struct sockarr_sz2 {
    pub type_: u32,
    pub max_entries: u32,
    pub key: i32,
    pub value: i32,
}

/* SEC(".maps") */
#[no_mangle]
pub static mut sockarr_sz2: sockarr_sz2 = sockarr_sz2 {
    type_: BPF_MAP_TYPE_REUSEPORT_SOCKARRAY,
    max_entries: 2,
    key: 0,
    value: 0,
};

#[repr(C)]
pub struct outer_sockarr_sz1 {
    pub type_: u32,
    pub max_entries: u32,
    pub key: i32,
    pub value: i32,
    pub values: [*mut c_void; 1],
}

/* SEC(".maps") */
#[no_mangle]
pub static mut outer_sockarr: outer_sockarr_sz1 = outer_sockarr_sz1 {
    type_: BPF_MAP_TYPE_ARRAY_OF_MAPS,
    max_entries: 1,
    key: 0,
    value: 0,
    values: [unsafe { &mut sockarr_sz1 as *mut sockarr_sz1 as *mut c_void }],
};

#[no_mangle]
pub static mut input: i32 = 0;

extern "C" {
    fn bpf_map_lookup_elem(map: *mut c_void, key: *const c_void) -> *mut c_void;
    fn bpf_map_update_elem(
        map: *mut c_void,
        key: *const c_void,
        value: *const c_void,
        flags: u64,
    ) -> i64;
}

/* SEC("raw_tp/sys_enter") */
#[no_mangle]
pub unsafe extern "C" fn handle__sys_enter(ctx: *mut c_void) -> i32 {
    let mut inner_map: *mut inner_map;
    let mut key: i32 = 0;
    let mut val: i32;

    let _ = ctx;

    inner_map = bpf_map_lookup_elem(
        &mut outer_arr as *mut outer_arr as *mut c_void,
        &key as *const i32 as *const c_void,
    ) as *mut inner_map;
    if inner_map.is_null() {
        return 1;
    }
    val = input;
    bpf_map_update_elem(
        inner_map as *mut c_void,
        &key as *const i32 as *const c_void,
        &val as *const i32 as *const c_void,
        0,
    );

    inner_map = bpf_map_lookup_elem(
        &mut outer_hash as *mut outer_hash as *mut c_void,
        &key as *const i32 as *const c_void,
    ) as *mut inner_map;
    if inner_map.is_null() {
        return 1;
    }
    val = input + 1;
    bpf_map_update_elem(
        inner_map as *mut c_void,
        &key as *const i32 as *const c_void,
        &val as *const i32 as *const c_void,
        0,
    );

    inner_map = bpf_map_lookup_elem(
        &mut outer_arr_dyn as *mut outer_arr_dyn as *mut c_void,
        &key as *const i32 as *const c_void,
    ) as *mut inner_map;
    if inner_map.is_null() {
        return 1;
    }
    val = input + 2;
    bpf_map_update_elem(
        inner_map as *mut c_void,
        &key as *const i32 as *const c_void,
        &val as *const i32 as *const c_void,
        0,
    );

    return 0;
}

/* SEC("license") */
#[no_mangle]
pub static mut _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
