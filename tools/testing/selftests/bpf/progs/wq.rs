// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2024 Benjamin Tissoires
 */

// Dependencies from the original C source:
// #include "bpf_experimental.h"
// #include <bpf/bpf_helpers.h>
// #include "bpf_misc.h"
// #include "../test_kmods/bpf_testmod_kfunc.h"

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

pub type __u32 = u32;

pub const BPF_MAP_TYPE_HASH: u32 = 1;
pub const BPF_MAP_TYPE_ARRAY: u32 = 2;
pub const BPF_MAP_TYPE_LRU_HASH: u32 = 9;
pub const BPF_F_NO_PREALLOC: u32 = 1;

#[repr(C)]
pub struct bpf_timer {
    _bindgen_opaque_blob: [u8; 0],
}

#[repr(C)]
pub struct bpf_spin_lock {
    _bindgen_opaque_blob: [u8; 0],
}

#[repr(C)]
pub struct bpf_wq {
    _bindgen_opaque_blob: [u8; 0],
}

unsafe extern "C" {
    fn bpf_map_update_elem(map: *mut core::ffi::c_void, key: *mut i32, value: *mut core::ffi::c_void, flags: u64) -> i64;
    fn bpf_map_lookup_elem(map: *mut core::ffi::c_void, key: *mut i32) -> *mut core::ffi::c_void;
    fn bpf_wq_init(wq: *mut bpf_wq, map: *mut core::ffi::c_void, flags: u64) -> i32;
    fn bpf_wq_set_callback(
        wq: *mut bpf_wq,
        callback_fn: Option<unsafe extern "C" fn(*mut core::ffi::c_void, *mut i32, *mut core::ffi::c_void) -> i32>,
        flags: u64,
    ) -> i32;
    fn bpf_wq_start(wq: *mut bpf_wq, flags: u64) -> i32;
    fn bpf_kfunc_common_test();
    fn bpf_kfunc_call_test_sleepable();
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "license")]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[repr(C)]
pub struct hmap_elem {
    pub counter: i32,
    pub timer: bpf_timer, /* unused */
    pub lock: bpf_spin_lock, /* unused */
    pub work: bpf_wq,
}

#[repr(C)]
pub struct bpf_map_def {
    pub type_: u32,
    pub map_flags: u32,
    pub max_entries: u32,
    pub key_size: u32,
    pub value_size: u32,
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".maps")]
pub static mut hmap: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_HASH,
    map_flags: 0,
    max_entries: 1000,
    key_size: core::mem::size_of::<i32>() as u32,
    value_size: core::mem::size_of::<hmap_elem>() as u32,
};

#[unsafe(no_mangle)]
#[unsafe(link_section = ".maps")]
pub static mut hmap_malloc: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_HASH,
    map_flags: BPF_F_NO_PREALLOC,
    max_entries: 1000,
    key_size: core::mem::size_of::<i32>() as u32,
    value_size: core::mem::size_of::<hmap_elem>() as u32,
};

#[repr(C)]
pub struct elem {
    pub ok_offset: i32,
    pub w: bpf_wq,
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".maps")]
pub static mut array: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_ARRAY,
    map_flags: 0,
    max_entries: 2,
    key_size: core::mem::size_of::<i32>() as u32,
    value_size: core::mem::size_of::<elem>() as u32,
};

#[unsafe(no_mangle)]
#[unsafe(link_section = ".maps")]
pub static mut lru: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_LRU_HASH,
    map_flags: 0,
    max_entries: 4,
    key_size: core::mem::size_of::<i32>() as u32,
    value_size: core::mem::size_of::<elem>() as u32,
};

#[unsafe(no_mangle)]
pub static mut ok: __u32 = 0;
#[unsafe(no_mangle)]
pub static mut ok_sleepable: __u32 = 0;

unsafe fn test_elem_callback(
    map: *mut core::ffi::c_void,
    key: *mut i32,
    callback_fn: Option<unsafe extern "C" fn(*mut core::ffi::c_void, *mut i32, *mut core::ffi::c_void) -> i32>,
) -> i32 {
    let mut init: elem = core::mem::zeroed();
    let mut val: *mut elem;
    let mut wq: *mut bpf_wq;

    if (ok & (1 << *key)) != 0 || (ok_sleepable & (1 << *key)) != 0 {
        return -22;
    }

    if map == (&raw mut lru).cast::<core::ffi::c_void>()
        && bpf_map_update_elem(map, key, (&raw mut init).cast::<core::ffi::c_void>(), 0) != 0
    {
        return -1;
    }

    val = bpf_map_lookup_elem(map, key).cast::<elem>();
    if val.is_null() {
        return -2;
    }

    (*val).ok_offset = *key;

    wq = &raw mut (*val).w;
    if bpf_wq_init(wq, map, 0) != 0 {
        return -3;
    }

    if bpf_wq_set_callback(wq, callback_fn, 0) != 0 {
        return -4;
    }

    if bpf_wq_start(wq, 0) != 0 {
        return -5;
    }

    return 0;
}

unsafe fn test_hmap_elem_callback(
    map: *mut core::ffi::c_void,
    key: *mut i32,
    callback_fn: Option<unsafe extern "C" fn(*mut core::ffi::c_void, *mut i32, *mut core::ffi::c_void) -> i32>,
) -> i32 {
    let mut init: hmap_elem = core::mem::zeroed();
    let mut val: *mut hmap_elem;
    let mut wq: *mut bpf_wq;

    if (ok & (1 << *key)) != 0 || (ok_sleepable & (1 << *key)) != 0 {
        return -22;
    }

    if bpf_map_update_elem(map, key, (&raw mut init).cast::<core::ffi::c_void>(), 0) != 0 {
        return -1;
    }

    val = bpf_map_lookup_elem(map, key).cast::<hmap_elem>();
    if val.is_null() {
        return -2;
    }

    wq = &raw mut (*val).work;
    if bpf_wq_init(wq, map, 0) != 0 {
        return -3;
    }

    if bpf_wq_set_callback(wq, callback_fn, 0) != 0 {
        return -4;
    }

    if bpf_wq_start(wq, 0) != 0 {
        return -5;
    }

    return 0;
}

/* callback for non sleepable workqueue */
unsafe extern "C" fn wq_callback(
    _map: *mut core::ffi::c_void,
    key: *mut i32,
    _value: *mut core::ffi::c_void,
) -> i32 {
    bpf_kfunc_common_test();
    ok |= 1 << *key;
    return 0;
}

/* callback for sleepable workqueue */
unsafe extern "C" fn wq_cb_sleepable(
    _map: *mut core::ffi::c_void,
    key: *mut i32,
    value: *mut core::ffi::c_void,
) -> i32 {
    let data: *mut elem = value.cast::<elem>();
    let offset: i32 = (*data).ok_offset;

    if *key != offset {
        return 0;
    }

    bpf_kfunc_call_test_sleepable();
    ok_sleepable |= 1 << offset;
    return 0;
}

// SEC("tc")
/* test that workqueues can be used from an array */
// __retval(0)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_call_array_sleepable(_ctx: *mut core::ffi::c_void) -> i64 {
    let mut key: i32 = 0;

    return test_elem_callback(
        (&raw mut array).cast::<core::ffi::c_void>(),
        &raw mut key,
        Some(wq_cb_sleepable),
    ) as i64;
}

// SEC("syscall")
/* Same test than above but from a sleepable context. */
// __retval(0)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_syscall_array_sleepable(_ctx: *mut core::ffi::c_void) -> i64 {
    let mut key: i32 = 1;

    return test_elem_callback(
        (&raw mut array).cast::<core::ffi::c_void>(),
        &raw mut key,
        Some(wq_cb_sleepable),
    ) as i64;
}

// SEC("tc")
/* test that workqueues can be used from a hashmap */
// __retval(0)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_call_hash_sleepable(_ctx: *mut core::ffi::c_void) -> i64 {
    let mut key: i32 = 2;

    return test_hmap_elem_callback(
        (&raw mut hmap).cast::<core::ffi::c_void>(),
        &raw mut key,
        Some(wq_callback),
    ) as i64;
}

// SEC("tc")
/* test that workqueues can be used from a hashmap with NO_PREALLOC. */
// __retval(0)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_call_hash_malloc_sleepable(_ctx: *mut core::ffi::c_void) -> i64 {
    let mut key: i32 = 3;

    return test_hmap_elem_callback(
        (&raw mut hmap_malloc).cast::<core::ffi::c_void>(),
        &raw mut key,
        Some(wq_callback),
    ) as i64;
}

// SEC("tc")
/* test that workqueues can be used from a LRU map */
// __retval(0)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_call_lru_sleepable(_ctx: *mut core::ffi::c_void) -> i64 {
    let mut key: i32 = 4;

    return test_elem_callback(
        (&raw mut lru).cast::<core::ffi::c_void>(),
        &raw mut key,
        Some(wq_callback),
    ) as i64;
}

// SEC("tc")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_map_no_btf(_ctx: *mut core::ffi::c_void) -> i64 {
    let mut val: *mut elem;
    let mut wq: *mut bpf_wq;
    let mut key: i32 = 42;

    val = bpf_map_lookup_elem(
        (&raw mut array).cast::<core::ffi::c_void>(),
        &raw mut key,
    )
    .cast::<elem>();
    if val.is_null() {
        return -2;
    }

    wq = &raw mut (*val).w;
    if bpf_wq_init(wq, (&raw mut array).cast::<core::ffi::c_void>(), 0) != 0 {
        return -3;
    }
    return 0;
}
