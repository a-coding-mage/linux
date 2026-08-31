// SPDX-License-Identifier: GPL-2.0
/* Copyright 2026 Google LLC */

/*
 * C dependencies:
 *   "vmlinux.h"
 *   <bpf/bpf_helpers.h>
 *   <bpf/bpf_core_read.h>
 *   "bpf_experimental.h"
 *   "bpf_misc.h"
 *   "wakeup_source.h"
 */

pub const MAX_LOOP_ITER: i32 = 1000;
pub const RB_SIZE: u32 = 16384 * 4;

#[repr(C)]
pub struct rb {
    /* __uint(type, BPF_MAP_TYPE_RINGBUF); */
    /* __uint(max_entries, RB_SIZE); */
}

/* SEC(".maps") */
#[no_mangle]
pub static mut rb: rb = rb {};

#[repr(C)]
pub struct bpf_ws_lock {
    _private: [u8; 0],
}

extern "C" {
    #[link_name = "bpf_wakeup_sources_read_lock"]
    pub fn bpf_wakeup_sources_read_lock() -> *mut bpf_ws_lock;
    #[link_name = "bpf_wakeup_sources_read_unlock"]
    pub fn bpf_wakeup_sources_read_unlock(lock: *mut bpf_ws_lock);
    #[link_name = "bpf_wakeup_sources_get_head"]
    pub fn bpf_wakeup_sources_get_head() -> *mut list_head;

    pub fn bpf_core_read(dst: *mut core::ffi::c_void, size: u32, src: *const core::ffi::c_void) -> i64;
    pub fn bpf_ringbuf_reserve(
        ringbuf: *mut rb,
        size: u64,
        flags: u64,
    ) -> *mut core::ffi::c_void;
    pub fn bpf_ringbuf_submit(data: *mut core::ffi::c_void, flags: u64);
    pub fn bpf_ktime_get_ns() -> s64;
    pub fn bpf_probe_read_kernel_str(
        dst: *mut core::ffi::c_char,
        size: u32,
        unsafe_ptr: *const core::ffi::c_void,
    ) -> i64;
}

/* Types and constants are supplied by the translated equivalents of the included headers. */
type s64 = i64;

/* SEC("syscall") */
/* __success __retval(0) */
#[no_mangle]
pub unsafe extern "C" fn iterate_wakeupsources(ctx: *mut core::ffi::c_void) -> i32 {
    let head: *mut list_head = bpf_wakeup_sources_get_head();
    let mut pos: *mut list_head = head;
    let lock: *mut bpf_ws_lock;
    let mut i: i32;

    lock = bpf_wakeup_sources_read_lock();
    if lock.is_null() {
        return 0;
    }

    i = 0;
    while i < MAX_LOOP_ITER {
        if bpf_core_read(
            &mut pos as *mut *mut list_head as *mut core::ffi::c_void,
            core::mem::size_of_val(&pos) as u32,
            &(*pos).next as *const *mut list_head as *const core::ffi::c_void,
        ) != 0
            || pos.is_null()
            || pos == head
        {
            break;
        }

        let e: *mut wakeup_event_t = bpf_ringbuf_reserve(
            &mut rb as *mut rb,
            core::mem::size_of::<wakeup_event_t>() as u64,
            0,
        ) as *mut wakeup_event_t;

        if e.is_null() {
            break;
        }

        let ws: *mut wakeup_source = bpf_core_cast(
            (pos as *mut u8).offset(-(bpf_core_field_offset!(wakeup_source, entry) as isize))
                as *mut core::ffi::c_void,
        );
        let mut active_time: s64 = 0;
        let active: bool = BPF_CORE_READ_BITFIELD!(ws, active);
        let autosleep_enable: bool = BPF_CORE_READ_BITFIELD!(ws, autosleep_enabled);
        let last_time: s64 = (*ws).last_time;
        let mut max_time: s64 = (*ws).max_time;
        let mut prevent_sleep_time: s64 = (*ws).prevent_sleep_time;
        let mut total_time: s64 = (*ws).total_time;

        if active {
            let curr_time: s64 = bpf_ktime_get_ns();
            let prevent_time: s64 = (*ws).start_prevent_time;

            if curr_time > last_time {
                active_time = curr_time - last_time;
            }

            total_time += active_time;
            if active_time > max_time {
                max_time = active_time;
            }
            if autosleep_enable && curr_time > prevent_time {
                prevent_sleep_time += curr_time - prevent_time;
            }
        }

        (*e).active_count = (*ws).active_count;
        (*e).active_time_ns = active_time;
        (*e).event_count = (*ws).event_count;
        (*e).expire_count = (*ws).expire_count;
        (*e).last_time_ns = last_time;
        (*e).max_time_ns = max_time;
        (*e).prevent_sleep_time_ns = prevent_sleep_time;
        (*e).total_time_ns = total_time;
        (*e).wakeup_count = (*ws).wakeup_count;

        if bpf_probe_read_kernel_str(
            (*e).name.as_mut_ptr(),
            WAKEUP_NAME_LEN,
            (*ws).name as *const core::ffi::c_void,
        ) < 0
        {
            (*e).name[0] = b'\0' as core::ffi::c_char;
        }

        bpf_ringbuf_submit(e as *mut core::ffi::c_void, 0);
        i += 1;
    }

    bpf_wakeup_sources_read_unlock(lock);
    return 0;
}

#[no_mangle]
pub static _license: [u8; 4] = *b"GPL\0";
