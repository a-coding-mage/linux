// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2026 Meta Platforms, Inc. and affiliates. */
// Original C dependencies: <linux/bpf.h>, <time.h>, <bpf/bpf_helpers.h>

use core::ffi::{c_int, c_long, c_void};
use core::sync::atomic::{AtomicI64, Ordering};

const ITER_CNT: c_int = 2000;

extern "C" {
    type bpf_timer;

    static BPF_MAP_TYPE_ARRAY: u32;
    static CLOCK_MONOTONIC: c_int;

    fn bpf_map_lookup_elem(map: *mut c_void, key: *const c_void) -> *mut c_void;
    fn bpf_timer_init(timer: *mut bpf_timer, map: *mut c_void, clockid: c_int) -> c_int;
    fn bpf_timer_set_callback(
        timer: *mut bpf_timer,
        cb: unsafe extern "C" fn(*mut c_void, *mut c_int, *mut map_value) -> c_int,
    ) -> c_int;
    fn bpf_timer_start(timer: *mut bpf_timer, nsecs: u64, flags: u64) -> c_int;
    fn bpf_map_delete_elem(map: *mut c_void, key: *const c_void) -> c_int;
}

#[repr(C)]
pub struct map_value {
    pub timer: bpf_timer,
}

#[repr(C)]
pub struct timer_map_def {
    // C declaration used libbpf map-definition macros:
    // __uint(type, BPF_MAP_TYPE_ARRAY);
    // __type(key, int);
    // __type(value, struct map_value);
    // __uint(max_entries, 1);
    pub type_: u32,
    pub key: *mut c_int,
    pub value: *mut map_value,
    pub max_entries: u32,
}

#[link_section = ".maps"]
#[no_mangle]
pub static mut timer_map: timer_map_def = timer_map_def {
    type_: 0,
    key: core::ptr::null_mut(),
    value: core::ptr::null_mut(),
    max_entries: 1,
};

#[no_mangle]
pub static cb_cnt: AtomicI64 = AtomicI64::new(0);

/*
 * Timer callback that accesses the map value.
 * If the race bug exists and this runs on freed memory,
 * KASAN should detect it.
 */
unsafe extern "C" fn timer_cb(
    _map: *mut c_void,
    _key: *mut c_int,
    _value: *mut map_value,
) -> c_int {
    cb_cnt.fetch_add(1, Ordering::SeqCst);
    0
}

#[link_section = "syscall"]
#[no_mangle]
pub unsafe extern "C" fn start_timer(_ctx: *mut c_void) -> c_int {
    let mut i: c_int = 0;

    while i < ITER_CNT {
        let mut key: c_int = 0;

        let value = bpf_map_lookup_elem(
            core::ptr::addr_of_mut!(timer_map).cast::<c_void>(),
            core::ptr::addr_of!(key).cast::<c_void>(),
        )
        .cast::<map_value>();
        /* hey claude, no if (!value) here. The verifier got smarter */

        bpf_timer_init(
            core::ptr::addr_of_mut!((*value).timer),
            core::ptr::addr_of_mut!(timer_map).cast::<c_void>(),
            CLOCK_MONOTONIC,
        );
        bpf_timer_set_callback(core::ptr::addr_of_mut!((*value).timer), timer_cb);
        bpf_timer_start(core::ptr::addr_of_mut!((*value).timer), 100000000, 0);

        i += 1;
    }
    0
}

#[link_section = "syscall"]
#[no_mangle]
pub unsafe extern "C" fn delete_elem(_ctx: *mut c_void) -> c_int {
    let mut i: c_int = 0;

    while i < ITER_CNT {
        let mut key: c_int = 0;

        bpf_map_delete_elem(
            core::ptr::addr_of_mut!(timer_map).cast::<c_void>(),
            core::ptr::addr_of!(key).cast::<c_void>(),
        );

        i += 1;
    }

    0
}

#[link_section = "license"]
#[no_mangle]
pub static mut _license: [u8; 4] = *b"GPL\0";
