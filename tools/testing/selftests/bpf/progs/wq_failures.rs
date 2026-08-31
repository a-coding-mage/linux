// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2024 Benjamin Tissoires
 */

// C dependencies:
// #include "bpf_experimental.h"
// #include <bpf/bpf_helpers.h>
// #include "bpf_misc.h"
// #include "../test_kmods/bpf_testmod_kfunc.h"

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

type c_void = core::ffi::c_void;
type c_int = i32;
type c_long = i64;
type u64 = u64;
type u32 = u32;

const BPF_MAP_TYPE_ARRAY: u32 = 2;
const BPF_MAP_TYPE_LRU_HASH: u32 = 9;
const BPF_F_TEST_STATE_FREQ: u32 = 1;

#[repr(C)]
pub struct bpf_wq {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct elem {
    pub w: bpf_wq,
}

#[repr(C)]
pub struct bpf_map_def {
    pub type_: u32,
    pub max_entries: u32,
}

unsafe extern "C" {
    fn bpf_kfunc_common_test();
    fn bpf_kfunc_call_test_sleepable();
    fn bpf_map_lookup_elem(map: *const c_void, key: *const c_void) -> *mut c_void;
    fn bpf_wq_init(wq: *mut bpf_wq, map: *const c_void, flags: u64) -> c_int;
    fn bpf_wq_set_callback(
        wq: *mut c_void,
        callback: unsafe extern "C" fn(*mut c_void, *mut c_int, *mut c_void) -> c_int,
        flags: u64,
    ) -> c_int;
    fn bpf_get_prandom_u32() -> u32;
}

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[unsafe(link_section = ".maps")]
#[unsafe(no_mangle)]
pub static mut array: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_ARRAY,
    max_entries: 2,
};

#[unsafe(link_section = ".maps")]
#[unsafe(no_mangle)]
pub static mut lru: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_LRU_HASH,
    max_entries: 4,
};

/* callback for non sleepable workqueue */
unsafe extern "C" fn wq_callback(
    _map: *mut c_void,
    _key: *mut c_int,
    _value: *mut c_void,
) -> c_int {
    unsafe {
        bpf_kfunc_common_test();
    }
    0
}

/* callback for sleepable workqueue */
unsafe extern "C" fn wq_cb_sleepable(
    _map: *mut c_void,
    _key: *mut c_int,
    _value: *mut c_void,
) -> c_int {
    unsafe {
        bpf_kfunc_call_test_sleepable();
    }
    0
}

#[unsafe(link_section = "tc")]
#[unsafe(no_mangle)]
/* test that bpf_wq_init takes a map as a second argument
 */
// __log_level(2)
// __flag(BPF_F_TEST_STATE_FREQ)
// __failure
// __msg(": (85) call bpf_wq_init#") /* anchor message */
// __msg("pointer in R2 isn't map pointer")
pub unsafe extern "C" fn test_wq_init_nomap(_ctx: *mut c_void) -> c_long {
    let mut wq: *mut bpf_wq;
    let mut val: *mut elem;
    let mut key: c_int = 0;

    unsafe {
        val = bpf_map_lookup_elem(
            core::ptr::addr_of!(array).cast::<c_void>(),
            (&raw const key).cast::<c_void>(),
        )
        .cast::<elem>();
    }
    if val.is_null() {
        return -1;
    }

    unsafe {
        wq = (&raw mut (*val).w).cast::<bpf_wq>();
        if bpf_wq_init(wq, (&raw const key).cast::<c_void>(), 0) != 0 {
            return -3;
        }
    }

    0
}

#[unsafe(link_section = "tc")]
#[unsafe(no_mangle)]
/* test that the workqueue is part of the map in bpf_wq_init
 */
// __log_level(2)
// __flag(BPF_F_TEST_STATE_FREQ)
// __failure
// __msg(": (85) call bpf_wq_init#") /* anchor message */
// __msg("workqueue pointer in R1 map_uid=0 doesn't match map pointer in R2 map_uid=0")
pub unsafe extern "C" fn test_wq_init_wrong_map(_ctx: *mut c_void) -> c_long {
    let mut wq: *mut bpf_wq;
    let mut val: *mut elem;
    let mut key: c_int = 0;

    unsafe {
        val = bpf_map_lookup_elem(
            core::ptr::addr_of!(array).cast::<c_void>(),
            (&raw const key).cast::<c_void>(),
        )
        .cast::<elem>();
    }
    if val.is_null() {
        return -1;
    }

    unsafe {
        wq = (&raw mut (*val).w).cast::<bpf_wq>();
        if bpf_wq_init(wq, core::ptr::addr_of!(lru).cast::<c_void>(), 0) != 0 {
            return -3;
        }
    }

    0
}

#[unsafe(link_section = "?tc")]
#[unsafe(no_mangle)]
// __log_level(2)
// __failure
/* check that the first argument of bpf_wq_set_callback()
 * is a correct bpf_wq pointer.
 */
// __msg(": (85) call bpf_wq_set_callback#") /* anchor message */
// __msg("R1 doesn't point to a map value")
pub unsafe extern "C" fn test_wrong_wq_pointer(_ctx: *mut c_void) -> c_long {
    let mut key: c_int = 0;
    let mut wq: *mut bpf_wq;

    unsafe {
        wq = bpf_map_lookup_elem(
            core::ptr::addr_of!(array).cast::<c_void>(),
            (&raw const key).cast::<c_void>(),
        )
        .cast::<bpf_wq>();
    }
    if wq.is_null() {
        return 1;
    }

    unsafe {
        if bpf_wq_init(wq, core::ptr::addr_of!(array).cast::<c_void>(), 0) != 0 {
            return 2;
        }
    }

    unsafe {
        if bpf_wq_set_callback(
            (&raw mut wq).cast::<c_void>(),
            wq_callback,
            0,
        ) != 0
        {
            return 3;
        }
    }

    -22
}

#[unsafe(link_section = "?tc")]
#[unsafe(no_mangle)]
// __log_level(2)
// __failure
/* check that the first argument of bpf_wq_set_callback()
 * is a correct bpf_wq pointer.
 */
// __msg(": (85) call bpf_wq_set_callback#") /* anchor message */
// __msg("off 1 doesn't point to 'struct bpf_wq' that is at 0")
pub unsafe extern "C" fn test_wrong_wq_pointer_offset(_ctx: *mut c_void) -> c_long {
    let mut key: c_int = 0;
    let mut wq: *mut bpf_wq;

    unsafe {
        wq = bpf_map_lookup_elem(
            core::ptr::addr_of!(array).cast::<c_void>(),
            (&raw const key).cast::<c_void>(),
        )
        .cast::<bpf_wq>();
    }
    if wq.is_null() {
        return 1;
    }

    unsafe {
        if bpf_wq_init(wq, core::ptr::addr_of!(array).cast::<c_void>(), 0) != 0 {
            return 2;
        }
    }

    unsafe {
        if bpf_wq_set_callback(
            (wq.cast::<u8>()).add(1).cast::<c_void>(),
            wq_cb_sleepable,
            0,
        ) != 0
        {
            return 3;
        }
    }

    -22
}

#[unsafe(link_section = "tc")]
#[unsafe(no_mangle)]
// __log_level(2)
// __failure
// __msg(": (85) call bpf_wq_init#")
// __msg("R1 doesn't have constant offset. bpf_wq has to be at the constant offset")
pub unsafe extern "C" fn test_bad_wq_off(_ctx: *mut c_void) -> c_long {
    let mut val: *mut elem;
    let mut wq: *mut bpf_wq;
    let mut key: c_int = 42;
    let mut unknown: u64;

    unsafe {
        val = bpf_map_lookup_elem(
            core::ptr::addr_of!(array).cast::<c_void>(),
            (&raw const key).cast::<c_void>(),
        )
        .cast::<elem>();
    }
    if val.is_null() {
        return -2;
    }

    unsafe {
        unknown = bpf_get_prandom_u32() as u64;
        wq = (&raw mut (*val).w).add(unknown as usize);
        if bpf_wq_init(wq, core::ptr::addr_of!(array).cast::<c_void>(), 0) != 0 {
            return -3;
        }
    }
    0
}
