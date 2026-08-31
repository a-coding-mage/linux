// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2026 Meta Platforms, Inc. and affiliates. */
// C dependencies:
// #include <vmlinux.h>
// #include <bpf/bpf_helpers.h>
// #include <bpf/bpf_tracing.h>

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::c_void;

const CLOCK_MONOTONIC: i32 = 1;
const BPF_MAP_TYPE_ARRAY: u32 = 2;

#[repr(C)]
pub struct bpf_timer {
    _private: [u8; 0],
}

#[repr(C)]
pub struct hrtimer {
    _private: [u8; 0],
}

pub type hrtimer_mode = u32;

#[repr(C)]
pub struct elem {
    pub timer: bpf_timer,
}

#[repr(C)]
pub struct timer_map_def {
    pub type_: u32,
    pub max_entries: u32,
    // C used __type(key, int) and __type(value, struct elem).
}

unsafe extern "C" {
    fn bpf_map_lookup_elem(map: *mut c_void, key: *const c_void) -> *mut c_void;
    fn bpf_timer_init(timer: *mut bpf_timer, map: *mut c_void, clockid: i32) -> i32;
    fn bpf_timer_set_callback(
        timer: *mut bpf_timer,
        callback: unsafe extern "C" fn(*mut c_void, *mut i32, *mut elem) -> i32,
    ) -> i32;
    fn bpf_timer_start(timer: *mut bpf_timer, nsecs: u64, flags: u64) -> i32;
}

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[unsafe(link_section = ".maps")]
#[unsafe(no_mangle)]
pub static mut timer_map: timer_map_def = timer_map_def {
    type_: BPF_MAP_TYPE_ARRAY,
    max_entries: 1,
};

#[unsafe(no_mangle)]
pub static mut in_timer_start: i32 = 0;
#[unsafe(no_mangle)]
pub static mut tp_called: i32 = 0;

unsafe extern "C" fn timer_cb(_map: *mut c_void, _key: *mut i32, _value: *mut elem) -> i32 {
    0
}

#[unsafe(link_section = "tp_btf/hrtimer_start")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn tp_hrtimer_start(
    _hrtimer: *mut hrtimer,
    _mode: hrtimer_mode,
    was_armed: bool,
) -> i32 {
    let timer: *mut bpf_timer;
    let key: i32 = 0;

    if unsafe { core::ptr::read_volatile(core::ptr::addr_of!(in_timer_start)) } == 0 || !was_armed
    {
        return 0;
    }

    unsafe {
        core::ptr::write_volatile(core::ptr::addr_of_mut!(tp_called), 1);
    }
    timer = unsafe {
        bpf_map_lookup_elem(
            core::ptr::addr_of_mut!(timer_map).cast::<c_void>(),
            core::ptr::addr_of!(key).cast::<c_void>(),
        )
        .cast::<bpf_timer>()
    };

    /*
     * Call bpf_timer_start() from the tracepoint within hrtimer logic
     * on the same timer to make sure it doesn't deadlock.
     */
    unsafe {
        bpf_timer_start(timer, 1000000000, 0);
    }
    0
}

#[unsafe(link_section = "syscall")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn start_timer(_ctx: *mut c_void) -> i32 {
    let timer: *mut bpf_timer;
    let key: i32 = 0;

    timer = unsafe {
        bpf_map_lookup_elem(
            core::ptr::addr_of_mut!(timer_map).cast::<c_void>(),
            core::ptr::addr_of!(key).cast::<c_void>(),
        )
        .cast::<bpf_timer>()
    };
    /* claude may complain here that there is no NULL check. Ignoring it. */
    unsafe {
        bpf_timer_init(
            timer,
            core::ptr::addr_of_mut!(timer_map).cast::<c_void>(),
            CLOCK_MONOTONIC,
        );
        bpf_timer_set_callback(timer, timer_cb);
    }

    /*
     * call hrtimer_start() twice, so that 2nd call does
     * trace_hrtimer_start(was_armed=1) tracepoint.
     */
    unsafe {
        core::ptr::write_volatile(core::ptr::addr_of_mut!(in_timer_start), 1);
        bpf_timer_start(timer, 1000000000, 0);
        bpf_timer_start(timer, 1000000000, 0);
        core::ptr::write_volatile(core::ptr::addr_of_mut!(in_timer_start), 0);
    }
    0
}
