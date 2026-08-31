// SPDX-License-Identifier: GPL-2.0
/* Copyright (C) 2025. Huawei Technologies Co., Ltd */

// Original C dependencies:
// #include <linux/bpf.h>
// #include <bpf/bpf_helpers.h>
// #include "bpf_misc.h"

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

type c_void = core::ffi::c_void;

const BPF_MAP_TYPE_RINGBUF: u32 = 27;
const BPF_F_RB_OVERWRITE: u32 = 1 << 6;
const BPF_RB_AVAIL_DATA: u64 = 0;
const BPF_RB_RING_SIZE: u64 = 1;
const BPF_RB_CONS_POS: u64 = 2;
const BPF_RB_PROD_POS: u64 = 3;
const BPF_RB_OVERWRITE_POS: u64 = 4;

#[repr(C)]
pub struct ringbuf_map_def {
    pub type_: u32,
    pub map_flags: u32,
}

#[link_section = "license"]
#[no_mangle]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[link_section = ".maps"]
#[no_mangle]
pub static mut ringbuf: ringbuf_map_def = ringbuf_map_def {
    type_: BPF_MAP_TYPE_RINGBUF,
    map_flags: BPF_F_RB_OVERWRITE,
};

#[no_mangle]
pub static mut pid: i32 = 0;

#[no_mangle]
pub static LEN1: core::cell::UnsafeCell<u64> = core::cell::UnsafeCell::new(0);
#[no_mangle]
pub static LEN2: core::cell::UnsafeCell<u64> = core::cell::UnsafeCell::new(0);
#[no_mangle]
pub static LEN3: core::cell::UnsafeCell<u64> = core::cell::UnsafeCell::new(0);
#[no_mangle]
pub static LEN4: core::cell::UnsafeCell<u64> = core::cell::UnsafeCell::new(0);
#[no_mangle]
pub static LEN5: core::cell::UnsafeCell<u64> = core::cell::UnsafeCell::new(0);

#[no_mangle]
pub static mut reserve1_fail: i64 = 0;
#[no_mangle]
pub static mut reserve2_fail: i64 = 0;
#[no_mangle]
pub static mut reserve3_fail: i64 = 0;
#[no_mangle]
pub static mut reserve4_fail: i64 = 0;
#[no_mangle]
pub static mut reserve5_fail: i64 = 0;

#[no_mangle]
pub static mut avail_data: u64 = 0;
#[no_mangle]
pub static mut ring_size: u64 = 0;
#[no_mangle]
pub static mut cons_pos: u64 = 0;
#[no_mangle]
pub static mut prod_pos: u64 = 0;
#[no_mangle]
pub static mut over_pos: u64 = 0;

extern "C" {
    fn bpf_get_current_pid_tgid() -> u64;
    fn bpf_ringbuf_reserve(ringbuf: *mut ringbuf_map_def, size: u64, flags: u64) -> *mut i8;
    fn bpf_ringbuf_discard(data: *mut i8, flags: u64);
    fn bpf_ringbuf_submit(data: *mut i8, flags: u64);
    fn bpf_ringbuf_query(ringbuf: *mut ringbuf_map_def, flags: u64) -> u64;
}

// Original section was SEC("fentry/" SYS_PREFIX "sys_getpgid").
// SYS_PREFIX is supplied by bpf_misc.h in the C build.
#[link_section = "fentry/sys_getpgid"]
#[no_mangle]
pub unsafe extern "C" fn test_overwrite_ringbuf(ctx: *mut c_void) -> i32 {
    let mut rec1: *mut i8;
    let mut rec2: *mut i8;
    let mut rec3: *mut i8;
    let mut rec4: *mut i8;
    let mut rec5: *mut i8;
    let cur_pid: i32 = (bpf_get_current_pid_tgid() >> 32) as i32;

    let _ = ctx;

    if cur_pid != pid {
        return 0;
    }

    rec1 = bpf_ringbuf_reserve(
        core::ptr::addr_of_mut!(ringbuf),
        core::ptr::read_volatile(LEN1.get()),
        0,
    );
    if rec1.is_null() {
        reserve1_fail = 1;
        return 0;
    }

    rec2 = bpf_ringbuf_reserve(
        core::ptr::addr_of_mut!(ringbuf),
        core::ptr::read_volatile(LEN2.get()),
        0,
    );
    if rec2.is_null() {
        bpf_ringbuf_discard(rec1, 0);
        reserve2_fail = 1;
        return 0;
    }

    rec3 = bpf_ringbuf_reserve(
        core::ptr::addr_of_mut!(ringbuf),
        core::ptr::read_volatile(LEN3.get()),
        0,
    );
    /* expect failure */
    if rec3.is_null() {
        reserve3_fail = 1;
    } else {
        bpf_ringbuf_discard(rec1, 0);
        bpf_ringbuf_discard(rec2, 0);
        bpf_ringbuf_discard(rec3, 0);
        return 0;
    }

    rec4 = bpf_ringbuf_reserve(
        core::ptr::addr_of_mut!(ringbuf),
        core::ptr::read_volatile(LEN4.get()),
        0,
    );
    if rec4.is_null() {
        reserve4_fail = 1;
        bpf_ringbuf_discard(rec1, 0);
        bpf_ringbuf_discard(rec2, 0);
        return 0;
    }

    bpf_ringbuf_submit(rec1, 0);
    bpf_ringbuf_submit(rec2, 0);
    bpf_ringbuf_submit(rec4, 0);

    rec5 = bpf_ringbuf_reserve(
        core::ptr::addr_of_mut!(ringbuf),
        core::ptr::read_volatile(LEN5.get()),
        0,
    );
    if rec5.is_null() {
        reserve5_fail = 1;
        return 0;
    }

    let mut i: i32 = 0;
    while (i as u64) < core::ptr::read_volatile(LEN3.get()) {
        *rec5.offset(i as isize) = 0xdd_u8 as i8;
        i += 1;
    }

    bpf_ringbuf_submit(rec5, 0);

    ring_size = bpf_ringbuf_query(core::ptr::addr_of_mut!(ringbuf), BPF_RB_RING_SIZE);
    avail_data = bpf_ringbuf_query(core::ptr::addr_of_mut!(ringbuf), BPF_RB_AVAIL_DATA);
    cons_pos = bpf_ringbuf_query(core::ptr::addr_of_mut!(ringbuf), BPF_RB_CONS_POS);
    prod_pos = bpf_ringbuf_query(core::ptr::addr_of_mut!(ringbuf), BPF_RB_PROD_POS);
    over_pos = bpf_ringbuf_query(core::ptr::addr_of_mut!(ringbuf), BPF_RB_OVERWRITE_POS);

    0
}
