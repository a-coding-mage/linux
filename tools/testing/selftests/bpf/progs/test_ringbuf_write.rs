// SPDX-License-Identifier: GPL-2.0

// Dependencies from the original C source:
// #include <linux/bpf.h>
// #include <bpf/bpf_helpers.h>
// #include "bpf_misc.h"

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

pub const BPF_MAP_TYPE_RINGBUF: u32 = 27;

#[repr(C)]
pub struct ringbuf_map_def {
    pub type_: u32,
}

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[unsafe(link_section = ".maps")]
#[unsafe(no_mangle)]
pub static mut ringbuf: ringbuf_map_def = ringbuf_map_def {
    type_: BPF_MAP_TYPE_RINGBUF,
};

/* inputs */
#[unsafe(no_mangle)]
pub static mut pid: i32 = 0;

/* outputs */
#[unsafe(no_mangle)]
pub static mut passed: i64 = 0;
#[unsafe(no_mangle)]
pub static mut discarded: i64 = 0;

unsafe extern "C" {
    fn bpf_get_current_pid_tgid() -> u64;
    fn bpf_ringbuf_reserve(ringbuf: *mut ringbuf_map_def, size: u64, flags: u64) -> *mut core::ffi::c_void;
    fn bpf_ringbuf_discard(data: *mut core::ffi::c_void, flags: u64);
}

// Original section: SEC("fentry/" SYS_PREFIX "sys_getpgid")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_ringbuf_write(ctx: *mut core::ffi::c_void) -> i32 {
    let mut foo: *mut i32;
    let cur_pid: i32 = (unsafe { bpf_get_current_pid_tgid() } >> 32) as i32;
    let mut sample1: *mut core::ffi::c_void;
    let mut sample2: *mut core::ffi::c_void;

    let _ = ctx;

    if cur_pid != unsafe { pid } {
        return 0;
    }

    sample1 = unsafe { bpf_ringbuf_reserve(&raw mut ringbuf, 0x30000, 0) };
    if sample1.is_null() {
        return 0;
    }
    /* first one can pass */
    sample2 = unsafe { bpf_ringbuf_reserve(&raw mut ringbuf, 0x30000, 0) };
    if sample2.is_null() {
        unsafe { bpf_ringbuf_discard(sample1, 0) };
        unsafe {
            core::intrinsics::atomic_xadd_relaxed(&raw mut discarded, 1);
        }
        return 0;
    }
    /* second one must not */
    unsafe {
        core::intrinsics::atomic_xadd_relaxed(&raw mut passed, 1);
    }
    foo = unsafe { (sample2 as *mut u8).add(4084) as *mut i32 };
    unsafe {
        *foo = 256;
    }
    unsafe { bpf_ringbuf_discard(sample1, 0) };
    unsafe { bpf_ringbuf_discard(sample2, 0) };
    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
