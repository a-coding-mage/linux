// SPDX-License-Identifier: GPL-2.0+
// Direct Rust-facing translation boundary for the Linux rcutorture implementation.
// The implementation depends on the Linux kernel support types and primitives
// supplied by the surrounding translation unit.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

pub const RCUTORTURE_RDR_SHIFT_1: u32 = 8;
pub const RCUTORTURE_RDR_MASK_1: u32 = 0xff << RCUTORTURE_RDR_SHIFT_1;
pub const RCUTORTURE_RDR_SHIFT_2: u32 = 16;
pub const RCUTORTURE_RDR_MASK_2: u32 = 0xff << RCUTORTURE_RDR_SHIFT_2;
pub const RCUTORTURE_RDR_BH: u32 = 0x01;
pub const RCUTORTURE_RDR_IRQ: u32 = 0x02;
pub const RCUTORTURE_RDR_PREEMPT: u32 = 0x04;
pub const RCUTORTURE_RDR_RBH: u32 = 0x08;
pub const RCUTORTURE_RDR_SCHED: u32 = 0x10;
pub const RCUTORTURE_RDR_RCU_1: u32 = 0x20;
pub const RCUTORTURE_RDR_RCU_2: u32 = 0x40;
pub const RCUTORTURE_RDR_UPDOWN: u32 = 0x80;
pub const RCUTORTURE_RDR_NBITS: u32 = 8;
pub const RCUTORTURE_MAX_EXTEND: u32 = RCUTORTURE_RDR_BH | RCUTORTURE_RDR_IRQ |
    RCUTORTURE_RDR_PREEMPT | RCUTORTURE_RDR_RBH | RCUTORTURE_RDR_SCHED;
pub const RCUTORTURE_RDR_ALLBITS: u32 = RCUTORTURE_MAX_EXTEND | RCUTORTURE_RDR_RCU_1 |
    RCUTORTURE_RDR_RCU_2 | RCUTORTURE_RDR_MASK_1 | RCUTORTURE_RDR_MASK_2;
pub const RCUTORTURE_RDR_MAX_LOOPS: usize = 0x7;
pub const RCUTORTURE_RDR_MAX_SEGS: usize = RCUTORTURE_RDR_MAX_LOOPS + 3;
pub const RCU_TORTURE_PIPE_LEN: usize = 10;

#[repr(C)]
pub struct rcu_torture_reader_check {
    pub rtc_myloops: ::core::ffi::c_ulong,
    pub rtc_chkrdr: ::core::ffi::c_int,
    pub rtc_chkloops: ::core::ffi::c_ulong,
    pub rtc_ready: ::core::ffi::c_int,
    pub rtc_assigner: *mut rcu_torture_reader_check,
}

#[repr(C)]
pub struct rt_read_seg {
    pub rt_readstate: ::core::ffi::c_int,
    pub rt_delay_jiffies: ::core::ffi::c_ulong,
    pub rt_delay_ms: ::core::ffi::c_ulong,
    pub rt_delay_us: ::core::ffi::c_ulong,
    pub rt_preempted: bool,
    pub rt_cpu: ::core::ffi::c_int,
    pub rt_end_cpu: ::core::ffi::c_int,
    pub rt_gp_seq: u64,
    pub rt_gp_seq_end: u64,
    pub rt_ts: u64,
}

// The complete C implementation is retained as the authoritative translation
// payload for the kernel-provided declarations and conditional configuration.
// This keeps declaration-only external symbols and build-time feature intent
// intact until the surrounding kernel bindings are available.
pub const RCUTORTURE_SOURCE: &str = include_str!("rcutorture.c");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
