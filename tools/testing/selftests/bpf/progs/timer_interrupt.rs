// SPDX-License-Identifier: GPL-2.0
// C dependencies translated as external Rust declarations:
// <vmlinux.h>, <bpf/bpf_helpers.h>, <bpf/bpf_tracing.h>, "bpf_experimental.h"

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";

pub const CLOCK_MONOTONIC: i32 = 1;

#[no_mangle]
pub static mut preempt_count: i32 = 0;
#[no_mangle]
pub static mut in_interrupt: i32 = 0;
#[no_mangle]
pub static mut in_interrupt_cb: i32 = 0;

#[repr(C)]
pub struct bpf_timer {
    _private: [u8; 0],
}

#[repr(C)]
pub struct elem {
    pub t: bpf_timer,
}

// Original C map declaration:
// struct {
//     __uint(type, BPF_MAP_TYPE_ARRAY);
//     __uint(max_entries, 1);
//     __type(key, int);
//     __type(value, struct elem);
// } array SEC(".maps");
//
// The concrete expansion of the BPF map-definition macros is supplied by
// bpf_helpers.h in the target build environment.
#[repr(C)]
pub struct array {
    _private: [u8; 0],
}

extern "C" {
    #[link_name = "array"]
    pub static mut array_map: array;

    pub fn get_preempt_count() -> i32;
    pub fn bpf_in_interrupt() -> i32;
    pub fn bpf_map_lookup_elem(map: *mut core::ffi::c_void, key: *const core::ffi::c_void)
        -> *mut core::ffi::c_void;
    pub fn bpf_timer_init(timer: *mut bpf_timer, map: *mut core::ffi::c_void, clockid: i32) -> i32;
    pub fn bpf_timer_set_callback(
        timer: *mut bpf_timer,
        callback: unsafe extern "C" fn(*mut core::ffi::c_void, *mut i32, *mut bpf_timer) -> i32,
    ) -> i32;
    pub fn bpf_timer_start(timer: *mut bpf_timer, nsecs: u64, flags: u64) -> i32;
}

unsafe extern "C" fn timer_in_interrupt(
    _map: *mut core::ffi::c_void,
    _key: *mut i32,
    _timer: *mut bpf_timer,
) -> i32 {
    preempt_count = get_preempt_count();
    in_interrupt_cb = bpf_in_interrupt();
    0
}

#[no_mangle]
#[link_section = "fentry/bpf_fentry_test1"]
pub unsafe extern "C" fn test_timer_interrupt() -> i32 {
    let timer: *mut bpf_timer;
    let mut key: i32 = 0;

    timer = bpf_map_lookup_elem(
        &mut array_map as *mut array as *mut core::ffi::c_void,
        &mut key as *mut i32 as *const core::ffi::c_void,
    ) as *mut bpf_timer;
    if timer.is_null() {
        return 0;
    }

    in_interrupt = bpf_in_interrupt();
    bpf_timer_init(
        timer,
        &mut array_map as *mut array as *mut core::ffi::c_void,
        CLOCK_MONOTONIC,
    );
    bpf_timer_set_callback(timer, timer_in_interrupt);
    bpf_timer_start(timer, 0, 0);
    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
