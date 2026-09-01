// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2020 Facebook

// C dependencies translated as external requirements:
// <stdbool.h>, <linux/bpf.h>, <stdint.h>, <bpf/bpf_helpers.h>, "bpf_misc.h"

use core::ffi::c_void;

const BPF_MAP_TYPE_RINGBUF: u32 = 27;
const BPF_RB_NO_WAKEUP: i64 = 1;
const BPF_RB_FORCE_WAKEUP: i64 = 2;
const BPF_RB_AVAIL_DATA: u64 = 0;

// Original C uses SEC("license").
#[no_mangle]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[repr(C)]
pub struct RingbufMap {
    // Original C:
    // struct {
    //     __uint(type, BPF_MAP_TYPE_RINGBUF);
    // } ringbuf SEC(".maps");
    //
    // __uint is a BPF helper macro that encodes BTF map metadata.
    pub type_: *mut [u32; BPF_MAP_TYPE_RINGBUF as usize],
}

// Original C uses SEC(".maps").
#[no_mangle]
pub static mut ringbuf: RingbufMap = RingbufMap {
    type_: core::ptr::null_mut(),
};

#[no_mangle]
pub static mut batch_cnt: i32 = 0;
#[no_mangle]
pub static mut use_output: i64 = 0;
#[no_mangle]
pub static mut bench_producer: bool = false;

#[no_mangle]
pub static mut sample_val: i64 = 42;
#[repr(align(128))]
pub struct AlignedLong {
    pub value: i64,
}

#[no_mangle]
pub static mut dropped: AlignedLong = AlignedLong { value: 0 };
#[no_mangle]
pub static mut hits: AlignedLong = AlignedLong { value: 0 };

#[no_mangle]
pub static mut wakeup_data_size: i64 = 0;

extern "C" {
    fn bpf_ringbuf_query(ringbuf: *mut RingbufMap, flags: u64) -> i64;
    fn bpf_ringbuf_reserve(ringbuf: *mut RingbufMap, size: u64, flags: u64) -> *mut c_void;
    fn bpf_ringbuf_submit(data: *mut c_void, flags: i64);
    fn bpf_ringbuf_output(
        ringbuf: *mut RingbufMap,
        data: *const c_void,
        size: u64,
        flags: i64,
    ) -> i64;
}

#[inline(always)]
unsafe fn get_flags() -> i64 {
    let sz: i64;

    if core::ptr::read_volatile(core::ptr::addr_of!(bench_producer)) {
        return BPF_RB_NO_WAKEUP;
    }

    if core::ptr::read_volatile(core::ptr::addr_of!(wakeup_data_size)) == 0 {
        return 0;
    }

    sz = bpf_ringbuf_query(core::ptr::addr_of_mut!(ringbuf), BPF_RB_AVAIL_DATA);
    if sz >= core::ptr::read_volatile(core::ptr::addr_of!(wakeup_data_size)) {
        BPF_RB_FORCE_WAKEUP
    } else {
        BPF_RB_NO_WAKEUP
    }
}

// Original C uses SEC("fentry/" SYS_PREFIX "sys_getpgid").
#[no_mangle]
pub unsafe extern "C" fn bench_ringbuf(ctx: *mut c_void) -> i32 {
    let mut sample: *mut i64;
    let mut flags: i64;
    let mut i: i32;

    let _ = ctx;

    if core::ptr::read_volatile(core::ptr::addr_of!(use_output)) == 0 {
        i = 0;
        while i < core::ptr::read_volatile(core::ptr::addr_of!(batch_cnt)) {
            sample = bpf_ringbuf_reserve(
                core::ptr::addr_of_mut!(ringbuf),
                core::mem::size_of::<i64>() as u64,
                0,
            ) as *mut i64;
            if sample.is_null() {
                core::sync::atomic::AtomicI64::from_ptr(core::ptr::addr_of_mut!(dropped.value))
                    .fetch_add(1, core::sync::atomic::Ordering::SeqCst);
            } else {
                core::ptr::write(sample, core::ptr::read_volatile(core::ptr::addr_of!(sample_val)));
                flags = get_flags();
                bpf_ringbuf_submit(sample as *mut c_void, flags);
                if core::ptr::read_volatile(core::ptr::addr_of!(bench_producer)) {
                    core::sync::atomic::AtomicI64::from_ptr(core::ptr::addr_of_mut!(hits.value))
                        .fetch_add(1, core::sync::atomic::Ordering::SeqCst);
                }
            }
            i += 1;
        }
    } else {
        i = 0;
        while i < core::ptr::read_volatile(core::ptr::addr_of!(batch_cnt)) {
            flags = get_flags();
            if bpf_ringbuf_output(
                core::ptr::addr_of_mut!(ringbuf),
                core::ptr::addr_of!(sample_val) as *const c_void,
                core::mem::size_of::<i64>() as u64,
                flags,
            ) != 0
            {
                core::sync::atomic::AtomicI64::from_ptr(core::ptr::addr_of_mut!(dropped.value))
                    .fetch_add(1, core::sync::atomic::Ordering::SeqCst);
            } else if core::ptr::read_volatile(core::ptr::addr_of!(bench_producer)) {
                core::sync::atomic::AtomicI64::from_ptr(core::ptr::addr_of_mut!(hits.value))
                    .fetch_add(1, core::sync::atomic::Ordering::SeqCst);
            }

            i += 1;
        }
    }
    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
