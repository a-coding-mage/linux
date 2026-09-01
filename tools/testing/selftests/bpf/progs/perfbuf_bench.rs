// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2020 Facebook

// C dependencies translated as external symbols/constants expected from the
// surrounding BPF build environment:
//   <linux/bpf.h>, <stdint.h>, <bpf/bpf_helpers.h>, "bpf_misc.h"

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]

type c_void = core::ffi::c_void;

extern "C" {
    static BPF_MAP_TYPE_PERF_EVENT_ARRAY: u32;
    static BPF_F_CURRENT_CPU: u64;

    fn bpf_perf_event_output(
        ctx: *mut c_void,
        map: *const PerfbufMap,
        flags: u64,
        data: *const c_void,
        size: u64,
    ) -> i64;
}

#[link_section = "license"]
#[no_mangle]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[repr(C)]
pub struct PerfbufMap {
    type_: *const u32,
    value_size: *const [i32; core::mem::size_of::<i32>()],
    key_size: *const [i32; core::mem::size_of::<i32>()],
}

#[link_section = ".maps"]
#[no_mangle]
pub static mut perfbuf: PerfbufMap = PerfbufMap {
    type_: unsafe { &BPF_MAP_TYPE_PERF_EVENT_ARRAY as *const u32 },
    value_size: core::ptr::null(),
    key_size: core::ptr::null(),
};

#[no_mangle]
pub static batch_cnt: i32 = 0;

#[no_mangle]
pub static mut sample_val: i64 = 42;

#[repr(align(128))]
pub struct AlignedLong(pub i64);

#[no_mangle]
pub static mut dropped: AlignedLong = AlignedLong(0);

// Original C section is SEC("fentry/" SYS_PREFIX "sys_getpgid").
#[link_section = "fentry/sys_getpgid"]
#[no_mangle]
pub unsafe extern "C" fn bench_perfbuf(ctx: *mut c_void) -> i32 {
    let mut i: i32;

    i = 0;
    while i < core::ptr::read_volatile(&batch_cnt) {
        if bpf_perf_event_output(
            ctx,
            &perfbuf as *const PerfbufMap,
            BPF_F_CURRENT_CPU,
            &sample_val as *const i64 as *const c_void,
            core::mem::size_of_val(&sample_val) as u64,
        ) != 0
        {
            core::intrinsics::atomic_xadd_seqcst(&mut dropped.0, 1);
        }
        i += 1;
    }
    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
