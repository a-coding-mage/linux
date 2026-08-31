// SPDX-License-Identifier: GPL-2.0

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_attributes)]
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::arch::asm;

// Translated from C includes: <linux/bpf.h>, <limits.h>, <bpf/bpf_helpers.h>, "bpf_misc.h".
// BPF helper symbols and verifier-test attribute macros are provided by the surrounding selftest harness.
unsafe extern "C" {
    fn bpf_get_prandom_u32() -> u32;
    fn bpf_ktime_get_ns() -> u64;
}

const INT_MIN: i32 = i32::MIN;
const LLONG_MIN: i64 = i64::MIN;

/* This file contains unit tests for signed/unsigned division and modulo
 * operations (with divisor as a constant), focusing on verifying whether
 * BPF verifier's range tracking module soundly and precisely computes
 * the results.
 */

#[unsafe(link_section = "socket")]
#[doc = "__description: UDIV32, positive divisor"]
#[doc = "__success __retval(0) __log_level(2)"]
#[doc = "__msg: w1 /= 3 {{.*}}; R1=scalar(smin=smin32=0,smax=umax=smax32=umax32=3,var_off=(0x0; 0x3))"]


#[unsafe(link_section = "socket")]
#[doc = "__description: UDIV32, zero divisor"]
#[doc = "__success __retval(0) __log_level(2)"]
#[doc = "__msg: w1 /= w2 {{.*}}; R1=0"]


#[unsafe(link_section = "socket")]
#[doc = "__description: UDIV64, positive divisor"]
#[doc = "__success __retval(0) __log_level(2)"]
#[doc = "__msg: r1 /= 3 {{.*}}; R1=scalar(smin=smin32=0,smax=umax=smax32=umax32=3,var_off=(0x0; 0x3))"]


#[unsafe(link_section = "socket")]
#[doc = "__description: UDIV64, zero divisor"]
#[doc = "__success __retval(0) __log_level(2)"]
#[doc = "__msg: r1 /= r2 {{.*}}; R1=0"]


#[unsafe(link_section = "socket")]
#[doc = "__description: SDIV32, positive divisor, positive dividend"]
#[doc = "__success __retval(0) __log_level(2)"]
#[doc = "__msg: w1 s/= 3 {{.*}}; R1=scalar(smin=umin=smin32=umin32=2,smax=umax=smax32=umax32=3,var_off=(0x2; 0x1))"]


#[unsafe(link_section = "socket")]
#[doc = "__description: SDIV32, positive divisor, negative dividend"]
#[doc = "__success __retval(0) __log_level(2)"]
#[doc = "__msg: w1 s/= 3 {{.*}}; R1=scalar(smin=umin=umin32=0xfffffffd,smax=umax=umax32=0xfffffffe,smin32=-3,smax32=-2,var_off=(0xfffffffc; 0x3))"]


#[unsafe(link_section = "socket")]
#[doc = "__description: SDIV32, positive divisor, mixed sign dividend"]
#[doc = "__success __retval(0) __log_level(2)"]
#[doc = "__msg: w1 s/= 3 {{.*}}; R1=scalar(smin=0,smax=umax=0xffffffff,smin32=-2,smax32=3,var_off=(0x0; 0xffffffff))"]


#[unsafe(link_section = "socket")]
#[doc = "__description: SDIV32, negative divisor, positive dividend"]
#[doc = "__success __retval(0) __log_level(2)"]
#[doc = "__msg: w1 s/= -3 {{.*}}; R1=scalar(smin=umin=umin32=0xfffffffd,smax=umax=umax32=0xfffffffe,smin32=-3,smax32=-2,var_off=(0xfffffffc; 0x3))"]


#[unsafe(link_section = "socket")]
#[doc = "__description: SDIV32, negative divisor, positive dividend"]
#[doc = "__success __retval(0) __log_level(2)"]
#[doc = "__msg: w1 s/= -3 {{.*}}; R1=scalar(smin=umin=smin32=umin32=2,smax=umax=smax32=umax32=3,var_off=(0x2; 0x1))"]


#[unsafe(link_section = "socket")]
#[doc = "__description: SDIV32, negative divisor, mixed sign dividend"]
#[doc = "__success __retval(0) __log_level(2)"]
#[doc = "__msg: w1 s/= -3 {{.*}}; R1=scalar(smin=0,smax=umax=0xffffffff,smin32=-3,smax32=2,var_off=(0x0; 0xffffffff))"]


#[unsafe(link_section = "socket")]
#[doc = "__description: SDIV32, zero divisor"]
#[doc = "__success __retval(0) __log_level(2)"]
#[doc = "__msg: w1 s/= w2 {{.*}}; R1=0"]


#[unsafe(link_section = "socket")]
#[doc = "__description: SDIV32, overflow (S32_MIN/-1)"]
#[doc = "__success __retval(0) __log_level(2)"]
#[doc = "__msg: w1 s/= -1 {{.*}}; R1=scalar(smin=0,smax=umax=0xffffffff,var_off=(0x0; 0xffffffff))"]


#[unsafe(link_section = "socket")]
#[doc = "__description: SDIV32, overflow (S32_MIN/-1), constant dividend"]
#[doc = "__success __retval(0) __log_level(2)"]
#[doc = "__msg: w1 s/= -1 {{.*}}; R1=0x80000000"]


#[unsafe(link_section = "socket")]
#[doc = "__description: SDIV64, positive divisor, positive dividend"]
#[doc = "__success __retval(0) __log_level(2)"]
#[doc = "__msg: r1 s/= 3 {{.*}}; R1=scalar(smin=umin=smin32=umin32=2,smax=umax=smax32=umax32=3,var_off=(0x2; 0x1))"]


#[unsafe(link_section = "socket")]
#[doc = "__description: SDIV64, positive divisor, negative dividend"]
#[doc = "__success __retval(0) __log_level(2)"]
#[doc = "__msg: r1 s/= 3 {{.*}}; R1=scalar(smin=smin32=-3,smax=smax32=-2,umin=0xfffffffffffffffd,umax=0xfffffffffffffffe,umin32=0xfffffffd,umax32=0xfffffffe,var_off=(0xfffffffffffffffc; 0x3))"]


#[unsafe(link_section = "socket")]
#[doc = "__description: SDIV64, positive divisor, mixed sign dividend"]
#[doc = "__success __retval(0) __log_level(2)"]
#[doc = "__msg: r1 s/= 3 {{.*}}; R1=scalar(smin=smin32=-2,smax=smax32=3)"]


#[unsafe(link_section = "socket")]
#[doc = "__description: SDIV64, negative divisor, positive dividend"]
#[doc = "__success __retval(0) __log_level(2)"]
#[doc = "__msg: r1 s/= -3 {{.*}}; R1=scalar(smin=smin32=-3,smax=smax32=-2,umin=0xfffffffffffffffd,umax=0xfffffffffffffffe,umin32=0xfffffffd,umax32=0xfffffffe,var_off=(0xfffffffffffffffc; 0x3))"]


#[unsafe(link_section = "socket")]
#[doc = "__description: SDIV64, negative divisor, positive dividend"]
#[doc = "__success __retval(0) __log_level(2)"]
#[doc = "__msg: r1 s/= -3 {{.*}}; R1=scalar(smin=umin=smin32=umin32=2,smax=umax=smax32=umax32=3,var_off=(0x2; 0x1))"]


#[unsafe(link_section = "socket")]
#[doc = "__description: SDIV64, negative divisor, mixed sign dividend"]
#[doc = "__success __retval(0) __log_level(2)"]
#[doc = "__msg: r1 s/= -3 {{.*}}; R1=scalar(smin=smin32=-3,smax=smax32=2)"]


#[unsafe(link_section = "socket")]
#[doc = "__description: SDIV64, zero divisor"]
#[doc = "__success __retval(0) __log_level(2)"]
#[doc = "__msg: r1 s/= r2 {{.*}}; R1=0"]


#[unsafe(link_section = "socket")]
#[doc = "__description: SDIV64, overflow (S64_MIN/-1)"]
#[doc = "__success __retval(0) __log_level(2)"]
#[doc = "__msg: r1 s/= -1 {{.*}}; R1=scalar()"]


#[unsafe(link_section = "socket")]
#[doc = "__description: SDIV64, overflow (S64_MIN/-1), constant dividend"]
#[doc = "__success __retval(0) __log_level(2)"]
#[doc = "__msg: r1 s/= -1 {{.*}}; R1=0x8000000000000000"]


#[unsafe(link_section = "socket")]
#[doc = "__description: UMOD32, positive divisor"]
#[doc = "__success __retval(0) __log_level(2)"]
#[doc = "__msg: w1 %= 3 {{.*}}; R1=scalar(smin=smin32=0,smax=umax=smax32=umax32=2,var_off=(0x0; 0x3))"]


#[unsafe(link_section = "socket")]
#[doc = "__description: UMOD32, positive divisor, small dividend"]
#[doc = "__success __retval(0) __log_level(2)"]
#[doc = "__msg: w1 %= 10 {{.*}}; R1=scalar(smin=umin=smin32=umin32=1,smax=umax=smax32=umax32=9,var_off=(0x1; 0x8))"]


#[unsafe(link_section = "socket")]
#[doc = "__description: UMOD32, zero divisor"]
#[doc = "__success __retval(0) __log_level(2)"]
#[doc = "__msg: w1 %= w2 {{.*}}; R1=scalar(smin=umin=smin32=umin32=1,smax=umax=smax32=umax32=9,var_off=(0x1; 0x8))"]


#[unsafe(link_section = "socket")]
#[doc = "__description: UMOD64, positive divisor"]
#[doc = "__success __retval(0) __log_level(2)"]
#[doc = "__msg: r1 %= 3 {{.*}}; R1=scalar(smin=smin32=0,smax=umax=smax32=umax32=2,var_off=(0x0; 0x3))"]


#[unsafe(link_section = "socket")]
#[doc = "__description: UMOD64, positive divisor, small dividend"]
#[doc = "__success __retval(0) __log_level(2)"]
#[doc = "__msg: r1 %= 10 {{.*}}; R1=scalar(smin=umin=smin32=umin32=1,smax=umax=smax32=umax32=9,var_off=(0x1; 0x8))"]


#[unsafe(link_section = "socket")]
#[doc = "__description: UMOD64, zero divisor"]
#[doc = "__success __retval(0) __log_level(2)"]
#[doc = "__msg: r1 %= r2 {{.*}}; R1=scalar(smin=umin=smin32=umin32=1,smax=umax=smax32=umax32=9,var_off=(0x1; 0x8))"]


#[unsafe(link_section = "socket")]
#[doc = "__description: SMOD32, positive divisor, positive dividend"]
#[doc = "__success __retval(0) __log_level(2)"]
#[doc = "__msg: w1 s%= 3 {{.*}}; R1=scalar(smin=smin32=0,smax=umax=smax32=umax32=2,var_off=(0x0; 0x3))"]


#[unsafe(link_section = "socket")]
#[doc = "__description: SMOD32, positive divisor, negative dividend"]
#[doc = "__success __retval(0) __log_level(2)"]
#[doc = "__msg: w1 s%= 3 {{.*}}; R1=scalar(smin=0,smax=umax=0xffffffff,smin32=-2,smax32=0,var_off=(0x0; 0xffffffff))"]


#[unsafe(link_section = "socket")]
#[doc = "__description: SMOD32, positive divisor, mixed sign dividend"]
#[doc = "__success __retval(0) __log_level(2)"]
#[doc = "__msg: w1 s%= 3 {{.*}}; R1=scalar(smin=0,smax=umax=0xffffffff,smin32=-2,smax32=2,var_off=(0x0; 0xffffffff))"]


#[unsafe(link_section = "socket")]
#[doc = "__description: SMOD32, positive divisor, small dividend"]
#[doc = "__success __retval(0) __log_level(2)"]
#[doc = "__msg: w1 s%= 11 {{.*}}; R1=scalar(smin=0,smax=umax=0xffffffff,smin32=-8,smax32=10,var_off=(0x0; 0xffffffff))"]


#[unsafe(link_section = "socket")]
#[doc = "__description: SMOD32, negative divisor, positive dividend"]
#[doc = "__success __retval(0) __log_level(2)"]
#[doc = "__msg: w1 s%= -3 {{.*}}; R1=scalar(smin=smin32=0,smax=umax=smax32=umax32=2,var_off=(0x0; 0x3))"]


#[unsafe(link_section = "socket")]
#[doc = "__description: SMOD32, negative divisor, negative dividend"]
#[doc = "__success __retval(0) __log_level(2)"]
#[doc = "__msg: w1 s%= -3 {{.*}}; R1=scalar(smin=0,smax=umax=0xffffffff,smin32=-2,smax32=0,var_off=(0x0; 0xffffffff))"]


#[unsafe(link_section = "socket")]
#[doc = "__description: SMOD32, negative divisor, mixed sign dividend"]
#[doc = "__success __retval(0) __log_level(2)"]
#[doc = "__msg: w1 s%= -3 {{.*}}; R1=scalar(smin=0,smax=umax=0xffffffff,smin32=-2,smax32=2,var_off=(0x0; 0xffffffff))"]


#[unsafe(link_section = "socket")]
#[doc = "__description: SMOD32, negative divisor, small dividend"]
#[doc = "__success __retval(0) __log_level(2)"]
#[doc = "__msg: w1 s%= -11 {{.*}}; R1=scalar(smin=0,smax=umax=0xffffffff,smin32=-8,smax32=10,var_off=(0x0; 0xffffffff))"]


#[unsafe(link_section = "socket")]
#[doc = "__description: SMOD32, zero divisor"]
#[doc = "__success __retval(0) __log_level(2)"]
#[doc = "__msg: w1 s%= w2 {{.*}}; R1=scalar(smin=0,smax=umax=0xffffffff,smin32=-8,smax32=10,var_off=(0x0; 0xffffffff))"]


#[unsafe(link_section = "socket")]
#[doc = "__description: SMOD32, overflow (S32_MIN%-1)"]
#[doc = "__success __retval(0) __log_level(2)"]
#[doc = "__msg: w1 s%= -1 {{.*}}; R1=0"]


#[unsafe(link_section = "socket")]
#[doc = "__description: SMOD32, overflow (S32_MIN%-1), constant dividend"]
#[doc = "__success __retval(0) __log_level(2)"]
#[doc = "__msg: w1 s%= -1 {{.*}}; R1=0"]


#[unsafe(link_section = "socket")]
#[doc = "__description: SMOD64, positive divisor, positive dividend"]
#[doc = "__success __retval(0) __log_level(2)"]
#[doc = "__msg: r1 s%= 3 {{.*}}; R1=scalar(smin=smin32=0,smax=umax=smax32=umax32=2,var_off=(0x0; 0x3))"]


#[unsafe(link_section = "socket")]
#[doc = "__description: SMOD64, positive divisor, negative dividend"]
#[doc = "__success __retval(0) __log_level(2)"]
#[doc = "__msg: r1 s%= 3 {{.*}}; R1=scalar(smin=smin32=-2,smax=smax32=0)"]


#[unsafe(link_section = "socket")]
#[doc = "__description: SMOD64, positive divisor, mixed sign dividend"]
#[doc = "__success __retval(0) __log_level(2)"]
#[doc = "__msg: r1 s%= 3 {{.*}}; R1=scalar(smin=smin32=-2,smax=smax32=2)"]


#[unsafe(link_section = "socket")]
#[doc = "__description: SMOD64, positive divisor, small dividend"]
#[doc = "__success __retval(0) __log_level(2)"]
#[doc = "__msg: r1 s%= 11 {{.*}}; R1=scalar(smin=smin32=-8,smax=smax32=10)"]


#[unsafe(link_section = "socket")]
#[doc = "__description: SMOD64, negative divisor, positive dividend"]
#[doc = "__success __retval(0) __log_level(2)"]
#[doc = "__msg: r1 s%= -3 {{.*}}; R1=scalar(smin=smin32=0,smax=umax=smax32=umax32=2,var_off=(0x0; 0x3))"]


#[unsafe(link_section = "socket")]
#[doc = "__description: SMOD64, negative divisor, negative dividend"]
#[doc = "__success __retval(0) __log_level(2)"]
#[doc = "__msg: r1 s%= -3 {{.*}}; R1=scalar(smin=smin32=-2,smax=smax32=0)"]


#[unsafe(link_section = "socket")]
#[doc = "__description: SMOD64, negative divisor, mixed sign dividend"]
#[doc = "__success __retval(0) __log_level(2)"]
#[doc = "__msg: r1 s%= -3 {{.*}}; R1=scalar(smin=smin32=-2,smax=smax32=2)"]


#[unsafe(link_section = "socket")]
#[doc = "__description: SMOD64, negative divisor, small dividend"]
#[doc = "__success __retval(0) __log_level(2)"]
#[doc = "__msg: r1 s%= -11 {{.*}}; R1=scalar(smin=smin32=-8,smax=smax32=10)"]


#[unsafe(link_section = "socket")]
#[doc = "__description: SMOD64, zero divisor"]
#[doc = "__success __retval(0) __log_level(2)"]
#[doc = "__msg: r1 s%= r2 {{.*}}; R1=scalar(smin=smin32=-8,smax=smax32=10)"]


#[unsafe(link_section = "socket")]
#[doc = "__description: SMOD64, overflow (S64_MIN%-1)"]
#[doc = "__success __retval(0) __log_level(2)"]
#[doc = "__msg: r1 s%= -1 {{.*}}; R1=0"]


#[unsafe(link_section = "socket")]
#[doc = "__description: SMOD64, overflow (S64_MIN%-1), constant dividend"]
#[doc = "__success __retval(0) __log_level(2)"]
#[doc = "__msg: r1 s%= -1 {{.*}}; R1=0"]

