// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2026 Meta Platforms, Inc. and affiliates. */

/* Translated from:
 * #include <vmlinux.h>
 * #include <stdbool.h>
 * #include <bpf/bpf_helpers.h>
 * #include <bpf/bpf_tracing.h>
 * #include "bpf_misc.h"
 */

pub const ENOENT: i32 = 2;
pub const EEXIST: i32 = 17;

/* External BPF constants supplied by BPF headers in the original source. */
extern "C" {
    static BPF_NOEXIST: u64;
    static BPF_EXIST: u64;
    static BPF_ANY: u64;
}

#[link_section = "license"]
#[no_mangle]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[no_mangle]
pub static mut err: i32 = 0;

#[repr(C)]
pub struct elem {
    pub arr: [i8; 128],
    pub val: i32,
}

#[repr(C)]
pub struct rhmap_def {
    _private: u8,
}

/* Original BPF map declaration:
 * struct {
 *     __uint(type, BPF_MAP_TYPE_RHASH);
 *     __uint(map_flags, BPF_F_NO_PREALLOC);
 *     __uint(max_entries, 128);
 *     __type(key, int);
 *     __type(value, struct elem);
 * } rhmap SEC(".maps");
 */
#[link_section = ".maps"]
#[no_mangle]
pub static mut rhmap: rhmap_def = rhmap_def { _private: 0 };

extern "C" {
    fn bpf_map_lookup_elem(map: *mut rhmap_def, key: *const i32) -> *mut elem;
    fn bpf_map_update_elem(
        map: *mut rhmap_def,
        key: *const i32,
        value: *const elem,
        flags: u64,
    ) -> i32;
    fn bpf_map_delete_elem(map: *mut rhmap_def, key: *const i32) -> i32;
}

#[link_section = "syscall"]
#[no_mangle]
pub unsafe extern "C" fn test_rhash_lookup_update(ctx: *mut core::ffi::c_void) -> i32 {
    let mut key: i32 = 5;
    let empty: elem = elem {
        val: 3,
        arr: [0; 128],
    };
    let mut e: *mut elem;

    let _ = ctx;
    err = 1;
    e = bpf_map_lookup_elem(&mut rhmap, &key);
    if !e.is_null() {
        return 1;
    }

    err = bpf_map_update_elem(&mut rhmap, &key, &empty, BPF_NOEXIST);
    if err != 0 {
        return 1;
    }

    e = bpf_map_lookup_elem(&mut rhmap, &key);
    if e.is_null() || (*e).val != empty.val {
        err = 2;
        return 2;
    }

    err = 0;
    return 0;
}

#[link_section = "syscall"]
#[no_mangle]
pub unsafe extern "C" fn test_rhash_update_delete(ctx: *mut core::ffi::c_void) -> i32 {
    let mut key: i32 = 6;
    let empty: elem = elem {
        val: 4,
        arr: [0; 128],
    };
    let mut e: *mut elem;

    let _ = ctx;
    err = 1;
    e = bpf_map_lookup_elem(&mut rhmap, &key);
    if !e.is_null() {
        return 1;
    }

    err = bpf_map_update_elem(&mut rhmap, &key, &empty, BPF_NOEXIST);
    if err != 0 {
        return 2;
    }

    err = bpf_map_delete_elem(&mut rhmap, &key);
    if err != 0 {
        return 3;
    }

    e = bpf_map_lookup_elem(&mut rhmap, &key);
    if !e.is_null() {
        err = 4;
        return 4;
    }

    err = 0;
    return 0;
}

#[link_section = "syscall"]
#[no_mangle]
pub unsafe extern "C" fn test_rhash_update_elements(ctx: *mut core::ffi::c_void) -> i32 {
    let mut key: i32 = 0;
    let mut empty: elem = elem {
        val: 4,
        arr: [0; 128],
    };
    let mut e: *mut elem;
    let mut i: i32;

    let _ = ctx;
    err = 1;

    i = 0;
    while i < 128 {
        key = i;
        e = bpf_map_lookup_elem(&mut rhmap, &key);
        if !e.is_null() {
            return 1;
        }

        empty.val = key;
        err = bpf_map_update_elem(&mut rhmap, &key, &empty, BPF_NOEXIST);
        if err != 0 {
            return 2;
        }

        e = bpf_map_lookup_elem(&mut rhmap, &key);
        if e.is_null() || (*e).val != key {
            err = 4;
            return 4;
        }

        i += 1;
    }

    i = 0;
    while i < 128 {
        key = i;
        err = bpf_map_delete_elem(&mut rhmap, &key);
        if err != 0 {
            return 3;
        }

        e = bpf_map_lookup_elem(&mut rhmap, &key);
        if !e.is_null() {
            err = 5;
            return 5;
        }

        i += 1;
    }

    err = 0;
    return 0;
}

#[link_section = "syscall"]
#[no_mangle]
pub unsafe extern "C" fn test_rhash_update_exist(ctx: *mut core::ffi::c_void) -> i32 {
    let mut key: i32 = 10;
    let val1: elem = elem {
        val: 100,
        arr: [0; 128],
    };
    let val2: elem = elem {
        val: 200,
        arr: [0; 128],
    };
    let mut e: *mut elem;
    let mut ret: i32;

    let _ = ctx;
    err = 1;

    /* BPF_EXIST on non-existent key should fail with -ENOENT */
    ret = bpf_map_update_elem(&mut rhmap, &key, &val1, BPF_EXIST);
    if ret != -ENOENT {
        return 1;
    }

    /* Insert element first */
    ret = bpf_map_update_elem(&mut rhmap, &key, &val1, BPF_NOEXIST);
    if ret != 0 {
        return 2;
    }

    /* Verify initial value */
    e = bpf_map_lookup_elem(&mut rhmap, &key);
    if e.is_null() || (*e).val != 100 {
        return 3;
    }

    /* BPF_EXIST on existing key should succeed and update value */
    ret = bpf_map_update_elem(&mut rhmap, &key, &val2, BPF_EXIST);
    if ret != 0 {
        return 4;
    }

    /* Verify value was updated */
    e = bpf_map_lookup_elem(&mut rhmap, &key);
    if e.is_null() || (*e).val != 200 {
        return 5;
    }

    /* Cleanup */
    bpf_map_delete_elem(&mut rhmap, &key);
    err = 0;
    return 0;
}

#[link_section = "syscall"]
#[no_mangle]
pub unsafe extern "C" fn test_rhash_update_any(ctx: *mut core::ffi::c_void) -> i32 {
    let mut key: i32 = 11;
    let val1: elem = elem {
        val: 111,
        arr: [0; 128],
    };
    let val2: elem = elem {
        val: 222,
        arr: [0; 128],
    };
    let mut e: *mut elem;
    let mut ret: i32;

    let _ = ctx;
    err = 1;

    /* BPF_ANY on non-existent key should insert */
    ret = bpf_map_update_elem(&mut rhmap, &key, &val1, BPF_ANY);
    if ret != 0 {
        return 1;
    }

    e = bpf_map_lookup_elem(&mut rhmap, &key);
    if e.is_null() || (*e).val != 111 {
        return 2;
    }

    /* BPF_ANY on existing key should update */
    ret = bpf_map_update_elem(&mut rhmap, &key, &val2, BPF_ANY);
    if ret != 0 {
        return 3;
    }

    e = bpf_map_lookup_elem(&mut rhmap, &key);
    if e.is_null() || (*e).val != 222 {
        return 4;
    }

    /* Cleanup */
    bpf_map_delete_elem(&mut rhmap, &key);
    err = 0;
    return 0;
}

#[link_section = "syscall"]
#[no_mangle]
pub unsafe extern "C" fn test_rhash_noexist_duplicate(ctx: *mut core::ffi::c_void) -> i32 {
    let mut key: i32 = 12;
    let val: elem = elem {
        val: 600,
        arr: [0; 128],
    };
    let mut ret: i32;

    let _ = ctx;
    err = 1;

    /* Insert element */
    ret = bpf_map_update_elem(&mut rhmap, &key, &val, BPF_NOEXIST);
    if ret != 0 {
        return 1;
    }

    /* Try to insert again with BPF_NOEXIST - should fail with -EEXIST */
    ret = bpf_map_update_elem(&mut rhmap, &key, &val, BPF_NOEXIST);
    if ret != -EEXIST {
        return 2;
    }

    /* Cleanup */
    bpf_map_delete_elem(&mut rhmap, &key);
    err = 0;
    return 0;
}

#[link_section = "syscall"]
#[no_mangle]
pub unsafe extern "C" fn test_rhash_delete_nonexistent(ctx: *mut core::ffi::c_void) -> i32 {
    let mut key: i32 = 99999;
    let mut ret: i32;

    let _ = ctx;
    err = 1;

    /* Delete non-existent key should return -ENOENT */
    ret = bpf_map_delete_elem(&mut rhmap, &key);
    if ret != -ENOENT {
        return 1;
    }

    err = 0;
    return 0;
}
