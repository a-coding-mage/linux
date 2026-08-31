// SPDX-License-Identifier: GPL-2.0-only
// C includes translated as external dependencies:
// <time.h>, <linux/bpf.h>, <bpf/bpf_helpers.h>

use core::ffi::{c_int, c_void};

type __u32 = u32;

// Constants supplied by the original included headers.
extern "C" {
    static BPF_MAP_TYPE_ARRAY: u32;
    static CLOCK_MONOTONIC: c_int;
}

#[repr(C)]
pub struct bpf_timer {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_spin_lock {
    _private: [u8; 0],
}

#[repr(C)]
struct timer {
    t: bpf_timer,
}

#[repr(C)]
struct lock {
    l: bpf_spin_lock,
}

#[repr(C)]
struct timers_map {
    // __uint(type, BPF_MAP_TYPE_ARRAY);
    r#type: *mut u32,
    // __uint(max_entries, 1);
    max_entries: *mut [u32; 1],
    // __type(key, __u32);
    key: *mut __u32,
    // __type(value, struct timer);
    value: *mut timer,
}

#[repr(C)]
struct locks_map {
    // __uint(type, BPF_MAP_TYPE_ARRAY);
    r#type: *mut u32,
    // __uint(max_entries, 1);
    max_entries: *mut [u32; 1],
    // __type(key, __u32);
    key: *mut __u32,
    // __type(value, struct lock);
    value: *mut lock,
}

extern "C" {
    fn bpf_map_lookup_elem(map: *mut c_void, key: *const c_void) -> *mut c_void;
    fn bpf_timer_init(timer: *mut bpf_timer, map: *mut c_void, clockid: c_int) -> c_int;
    fn bpf_timer_set_callback(
        timer: *mut bpf_timer,
        callback: extern "C" fn(*mut c_void, *mut c_int, *mut timer) -> c_int,
    ) -> c_int;
    fn bpf_timer_start(timer: *mut bpf_timer, nsecs: u64, flags: u64) -> c_int;
    fn bpf_timer_cancel(timer: *mut bpf_timer) -> c_int;
    fn bpf_spin_lock(lock: *mut bpf_spin_lock);
    fn bpf_spin_unlock(lock: *mut bpf_spin_lock);
}

#[link_section = ".maps"]
#[no_mangle]
static mut timers: timers_map = timers_map {
    r#type: unsafe { &BPF_MAP_TYPE_ARRAY as *const u32 as *mut u32 },
    max_entries: 1 as *mut [u32; 1],
    key: core::ptr::null_mut(),
    value: core::ptr::null_mut(),
};

#[link_section = ".maps"]
#[no_mangle]
static mut locks: locks_map = locks_map {
    r#type: unsafe { &BPF_MAP_TYPE_ARRAY as *const u32 as *mut u32 },
    max_entries: 1 as *mut [u32; 1],
    key: core::ptr::null_mut(),
    value: core::ptr::null_mut(),
};

extern "C" fn timer_cb(_map: *mut c_void, _key: *mut c_int, _timer: *mut timer) -> c_int {
    return 0;
}

unsafe fn timer_work() {
    let mut timer: *mut timer;
    let key: c_int = 0;

    timer = bpf_map_lookup_elem(
        &mut timers as *mut timers_map as *mut c_void,
        &key as *const c_int as *const c_void,
    ) as *mut timer;
    if !timer.is_null() {
        bpf_timer_init(
            &mut (*timer).t as *mut bpf_timer,
            &mut timers as *mut timers_map as *mut c_void,
            CLOCK_MONOTONIC,
        );
        bpf_timer_set_callback(&mut (*timer).t as *mut bpf_timer, timer_cb);
        bpf_timer_start(&mut (*timer).t as *mut bpf_timer, 10_000_000_000u64, 0);
        bpf_timer_cancel(&mut (*timer).t as *mut bpf_timer);
    }
}

unsafe fn spin_lock_work() {
    let key: c_int = 0;
    let mut lock: *mut lock;

    lock = bpf_map_lookup_elem(
        &mut locks as *mut locks_map as *mut c_void,
        &key as *const c_int as *const c_void,
    ) as *mut lock;
    if !lock.is_null() {
        bpf_spin_lock(&mut (*lock).l as *mut bpf_spin_lock);
        bpf_spin_unlock(&mut (*lock).l as *mut bpf_spin_lock);
    }
}

#[link_section = "?raw_tp/sys_enter"]
#[no_mangle]
pub unsafe extern "C" fn raw_tp_timer(_ctx: *mut c_void) -> c_int {
    timer_work();

    return 0;
}

#[link_section = "?tp/syscalls/sys_enter_nanosleep"]
#[no_mangle]
pub unsafe extern "C" fn tp_timer(_ctx: *mut c_void) -> c_int {
    timer_work();

    return 0;
}

#[link_section = "?kprobe"]
#[no_mangle]
pub unsafe extern "C" fn kprobe_timer(_ctx: *mut c_void) -> c_int {
    timer_work();

    return 0;
}

#[link_section = "?perf_event"]
#[no_mangle]
pub unsafe extern "C" fn perf_event_timer(_ctx: *mut c_void) -> c_int {
    timer_work();

    return 0;
}

#[link_section = "?raw_tp/sys_enter"]
#[no_mangle]
pub unsafe extern "C" fn raw_tp_spin_lock(_ctx: *mut c_void) -> c_int {
    spin_lock_work();

    return 0;
}

#[link_section = "?tp/syscalls/sys_enter_nanosleep"]
#[no_mangle]
pub unsafe extern "C" fn tp_spin_lock(_ctx: *mut c_void) -> c_int {
    spin_lock_work();

    return 0;
}

#[link_section = "?kprobe"]
#[no_mangle]
pub unsafe extern "C" fn kprobe_spin_lock(_ctx: *mut c_void) -> c_int {
    spin_lock_work();

    return 0;
}

#[link_section = "?perf_event"]
#[no_mangle]
pub unsafe extern "C" fn perf_event_spin_lock(_ctx: *mut c_void) -> c_int {
    spin_lock_work();

    return 0;
}

#[link_section = "license"]
#[no_mangle]
pub static LICENSE: [u8; 4] = *b"GPL\0";
