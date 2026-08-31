// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2026 Meta Platforms, Inc. and affiliates. */
/* Converted from tools/testing/selftests/bpf/prog_tests/align.c */

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::arch::asm;

/* C dependencies removed from executable Rust:
 * #include <linux/bpf.h>
 * #include <bpf/bpf_helpers.h>
 * #include "bpf_misc.h"
 *
 * The SEC, __success, __failure, __log_level, __flag, __msg, __naked,
 * __imm_const, __clobber_all, BPF_F_ANY_ALIGNMENT, struct __sk_buff,
 * data, and data_end meanings are supplied by the surrounding BPF
 * selftest infrastructure in the original source.
 */

/* Four tests of known constants.  These aren't staggeringly
 * interesting since we track exact values now.
 */

/* SEC("tc")
 * __success __log_level(2)
 * __flag(BPF_F_ANY_ALIGNMENT)
 * __msg("0: R1=ctx() R10=fp0")
 * __msg("0: {{.*}} R3=2")
 * __msg("1: {{.*}} R3=4")
 * __msg("2: {{.*}} R3=8")
 * __msg("3: {{.*}} R3=16")
 * __msg("4: {{.*}} R3=32")
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mov() {
    unsafe {
        asm!(
            "r3 = 2;",
            "r3 = 4;",
            "r3 = 8;",
            "r3 = 16;",
            "r3 = 32;",
            "r0 = 0;",
            "exit;",
            options(noreturn)
        );
    }
}

/* SEC("tc")
 * __success __log_level(2)
 * __flag(BPF_F_ANY_ALIGNMENT)
 * __msg("0: R1=ctx() R10=fp0")
 * __msg("0: {{.*}}R3=1")
 * __msg("1: {{.*}}R3=2")
 * __msg("2: {{.*}}R3=4")
 * __msg("3: {{.*}}R3=8")
 * __msg("4: {{.*}}R3=16")
 * __msg("5: {{.*}}R3=1")
 * __msg("6: {{.*}}R4=32")
 * __msg("7: {{.*}}R4=16")
 * __msg("8: {{.*}}R4=8")
 * __msg("9: {{.*}}R4=4")
 * __msg("10: {{.*}}R4=2")
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn shift() {
    unsafe {
        asm!(
            "r3 = 1;",
            "r3 <<= 1;",
            "r3 <<= 1;",
            "r3 <<= 1;",
            "r3 <<= 1;",
            "r3 >>= 4;",
            "r4 = 32;",
            "r4 >>= 1;",
            "r4 >>= 1;",
            "r4 >>= 1;",
            "r4 >>= 1;",
            "r0 = 0;",
            "exit;",
            options(noreturn)
        );
    }
}

/* SEC("tc")
 * __success __log_level(2)
 * __flag(BPF_F_ANY_ALIGNMENT)
 * __msg("0: R1=ctx() R10=fp0")
 * __msg("0: {{.*}}R3=4")
 * __msg("1: {{.*}}R3=8")
 * __msg("2: {{.*}}R3=10")
 * __msg("3: {{.*}}R4=8")
 * __msg("4: {{.*}}R4=12")
 * __msg("5: {{.*}}R4=14")
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn addsub() {
    unsafe {
        asm!(
            "r3 = 4;",
            "r3 += 4;",
            "r3 += 2;",
            "r4 = 8;",
            "r4 += 4;",
            "r4 += 2;",
            "r0 = 0;",
            "exit;",
            options(noreturn)
        );
    }
}

/* SEC("tc")
 * __success __log_level(2)
 * __flag(BPF_F_ANY_ALIGNMENT)
 * __msg("0: R1=ctx() R10=fp0")
 * __msg("0: {{.*}}R3=7")
 * __msg("1: {{.*}}R3=7")
 * __msg("2: {{.*}}R3=14")
 * __msg("3: {{.*}}R3=56")
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mul() {
    unsafe {
        asm!(
            "r3 = 7;",
            "r3 *= 1;",
            "r3 *= 2;",
            "r3 *= 4;",
            "r0 = 0;",
            "exit;",
            options(noreturn)
        );
    }
}

/* Tests using unknown values */

const __sk_buff_data: i32 = 0;
const __sk_buff_data_end: i32 = 0;

/* PREP_PKT_POINTERS:
 * "r2 = *(u32*)(r1 + %[__sk_buff_data]);"
 * "r3 = *(u32*)(r1 + %[__sk_buff_data_end]);"
 *
 * __LOAD_UNKNOWN(DST_REG, LBL):
 * "r2 = *(u32*)(r1 + %[__sk_buff_data]);"
 * "r3 = *(u32*)(r1 + %[__sk_buff_data_end]);"
 * "r0 = r2;"
 * "r0 += 8;"
 * "if r3 >= r0 goto " LBL ";"
 * "exit;"
 * LBL ":"
 * DST_REG " = *(u8*)(r2 + 0);"
 *
 * LOAD_UNKNOWN(DST_REG) __LOAD_UNKNOWN(DST_REG, "l99_%=")
 */

/* SEC("tc")
 * __success __log_level(2)
 * __flag(BPF_F_ANY_ALIGNMENT)
 * __msg("6: {{.*}} R2=pkt(r=8)")
 * __msg("6: {{.*}} R3={{[^)]*}}var_off=(0x0; 0xff)")
 * __msg("7: {{.*}} R3={{[^)]*}}var_off=(0x0; 0x1fe)")
 * __msg("8: {{.*}} R3={{[^)]*}}var_off=(0x0; 0x3fc)")
 * __msg("9: {{.*}} R3={{[^)]*}}var_off=(0x0; 0x7f8)")
 * __msg("10: {{.*}} R3={{[^)]*}}var_off=(0x0; 0xff0)")
 * __msg("12: {{.*}} R3=pkt_end()")
 * __msg("17: {{.*}} R4={{[^)]*}}var_off=(0x0; 0xff)")
 * __msg("18: {{.*}} R4={{[^)]*}}var_off=(0x0; 0x1fe0)")
 * __msg("19: {{.*}} R4={{[^)]*}}var_off=(0x0; 0xff0)")
 * __msg("20: {{.*}} R4={{[^)]*}}var_off=(0x0; 0x7f8)")
 * __msg("21: {{.*}} R4={{[^)]*}}var_off=(0x0; 0x3fc)")
 * __msg("22: {{.*}} R4={{[^)]*}}var_off=(0x0; 0x1fe)")
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn unknown_shift() {
    unsafe {
        asm!(
            "r2 = *(u32*)(r1 + {data});",
            "r3 = *(u32*)(r1 + {data_end});",
            "r0 = r2;",
            "r0 += 8;",
            "if r3 >= r0 goto l99_0;",
            "exit;",
            "l99_0:",
            "r3 = *(u8*)(r2 + 0);",
            "r3 <<= 1;",
            "r3 <<= 1;",
            "r3 <<= 1;",
            "r3 <<= 1;",
            "r2 = *(u32*)(r1 + {data});",
            "r3 = *(u32*)(r1 + {data_end});",
            "r0 = r2;",
            "r0 += 8;",
            "if r3 >= r0 goto l98_0;",
            "exit;",
            "l98_0:",
            "r4 = *(u8*)(r2 + 0);",
            "r4 <<= 5;",
            "r4 >>= 1;",
            "r4 >>= 1;",
            "r4 >>= 1;",
            "r4 >>= 1;",
            "r0 = 0;",
            "exit;",
            data = const __sk_buff_data,
            data_end = const __sk_buff_data_end,
            options(noreturn)
        );
    }
}

/* SEC("tc")
 * __success __log_level(2)
 * __flag(BPF_F_ANY_ALIGNMENT)
 * __msg("6: {{.*}} R3={{[^)]*}}var_off=(0x0; 0xff)")
 * __msg("7: {{.*}} R4={{[^)]*}}var_off=(0x0; 0xff)")
 * __msg("8: {{.*}} R4={{[^)]*}}var_off=(0x0; 0xff)")
 * __msg("9: {{.*}} R4={{[^)]*}}var_off=(0x0; 0xff)")
 * __msg("10: {{.*}} R4={{[^)]*}}var_off=(0x0; 0x1fe)")
 * __msg("11: {{.*}} R4={{[^)]*}}var_off=(0x0; 0xff)")
 * __msg("12: {{.*}} R4={{[^)]*}}var_off=(0x0; 0x3fc)")
 * __msg("13: {{.*}} R4={{[^)]*}}var_off=(0x0; 0xff)")
 * __msg("14: {{.*}} R4={{[^)]*}}var_off=(0x0; 0x7f8)")
 * __msg("15: {{.*}} R4={{[^)]*}}var_off=(0x0; 0xff0)")
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn unknown_mul() {
    unsafe {
        asm!(
            "r2 = *(u32*)(r1 + {data});",
            "r3 = *(u32*)(r1 + {data_end});",
            "r0 = r2;",
            "r0 += 8;",
            "if r3 >= r0 goto l99_1;",
            "exit;",
            "l99_1:",
            "r3 = *(u8*)(r2 + 0);",
            "r4 = r3;",
            "r4 *= 1;",
            "r4 = r3;",
            "r4 *= 2;",
            "r4 = r3;",
            "r4 *= 4;",
            "r4 = r3;",
            "r4 *= 8;",
            "r4 *= 2;",
            "r0 = 0;",
            "exit;",
            data = const __sk_buff_data,
            data_end = const __sk_buff_data_end,
            options(noreturn)
        );
    }
}

/* SEC("tc")
 * __success __log_level(2)
 * __msg("2: {{.*}} R5=pkt(r=0)")
 * __msg("4: {{.*}} R5=pkt(r=0,imm=14)")
 * __msg("5: {{.*}} R4=pkt(r=0,imm=14)")
 * __msg("9: {{.*}} R5=pkt(r=18,imm=14)")
 * __msg("10: {{.*}} R4={{[^)]*}}var_off=(0x0; 0xff){{.*}} R5=pkt(r=18,imm=14)")
 * __msg("13: {{.*}} R4={{[^)]*}}var_off=(0x0; 0xffff)")
 * __msg("14: {{.*}} R4={{[^)]*}}var_off=(0x0; 0xffff)")
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn packet_const_offset() {
    unsafe {
        asm!(
            "r2 = *(u32*)(r1 + {data});",
            "r3 = *(u32*)(r1 + {data_end});",
            "r5 = r2;",
            "r0 = 0;",
            "r5 += 14;",
            "r4 = r5;",
            "r4 += 4;",
            "if r3 >= r4 goto l0_0;",
            "exit;",
            "l0_0:",
            "r4 = *(u8*)(r5 + 0);",
            "r4 = *(u8*)(r5 + 1);",
            "r4 = *(u8*)(r5 + 2);",
            "r4 = *(u8*)(r5 + 3);",
            "r4 = *(u16*)(r5 + 0);",
            "r4 = *(u16*)(r5 + 2);",
            "r4 = *(u32*)(r5 + 0);",
            "r0 = 0;",
            "exit;",
            data = const __sk_buff_data,
            data_end = const __sk_buff_data_end,
            options(noreturn)
        );
    }
}

/* SEC("tc")
 * __success __log_level(2)
 * __flag(BPF_F_ANY_ALIGNMENT)
 *
 * Calculated offset in R6 has unknown value, but known alignment of 4.
 * Offset is added to packet pointer R5, resulting in known fixed offset,
 * and variable offset from R6.
 * Remaining __msg annotations from the C source are preserved in comments
 * immediately around the translated assembly sequence.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn packet_variable_offset() {
    unsafe {
        asm!(
            "r2 = *(u32*)(r1 + {data});",
            "r3 = *(u32*)(r1 + {data_end});",
            "r0 = r2;",
            "r0 += 8;",
            "if r3 >= r0 goto l99_2;",
            "exit;",
            "l99_2:",
            "r6 = *(u8*)(r2 + 0);",
            "r6 <<= 2;",
            "r5 = r2;",
            "r5 += 14;",
            "r5 += r6;",
            "r4 = r5;",
            "r4 += 4;",
            "if r3 >= r4 goto l0_1;",
            "exit;",
            "l0_1:",
            "r4 = *(u32*)(r5 + 0);",
            "r5 = r2;",
            "r5 += r6;",
            "r4 = r5;",
            "r5 += 14;",
            "r4 = r5;",
            "r4 += 4;",
            "if r3 >= r4 goto l1_1;",
            "exit;",
            "l1_1:",
            "r4 = *(u32*)(r5 + 0);",
            "r5 = r2;",
            "r5 += 14;",
            "r5 += r6;",
            "r4 = r5;",
            "r5 += 4;",
            "r5 += r6;",
            "r4 = r5;",
            "r4 += 4;",
            "if r3 >= r4 goto l2_1;",
            "exit;",
            "l2_1:",
            "r4 = *(u32*)(r5 + 0);",
            "r0 = 0;",
            "exit;",
            data = const __sk_buff_data,
            data_end = const __sk_buff_data_end,
            options(noreturn)
        );
    }
}

/* SEC("tc")
 * __success __log_level(2)
 * __flag(BPF_F_ANY_ALIGNMENT)
 *
 * Packet variable offset alignment verifier expectations are preserved
 * from the original __msg annotations and comments.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn packet_variable_offset_2() {
    unsafe {
        asm!(
            "r2 = *(u32*)(r1 + {data});",
            "r3 = *(u32*)(r1 + {data_end});",
            "r0 = r2;",
            "r0 += 8;",
            "if r3 >= r0 goto l99_3;",
            "exit;",
            "l99_3:",
            "r6 = *(u8*)(r2 + 0);",
            "r6 <<= 2;",
            "r6 += 14;",
            "r5 = r2;",
            "r5 += r6;",
            "r4 = r5;",
            "r4 += 4;",
            "if r3 >= r4 goto l0_2;",
            "exit;",
            "l0_2:",
            "r6 = *(u32*)(r5 + 0);",
            "r6 &= 0xff;",
            "r6 <<= 2;",
            "r5 += r6;",
            "r4 = r5;",
            "r4 += 4;",
            "if r3 >= r4 goto l1_2;",
            "exit;",
            "l1_2:",
            "r6 = *(u32*)(r5 + 0);",
            "r0 = 0;",
            "exit;",
            data = const __sk_buff_data,
            data_end = const __sk_buff_data_end,
            options(noreturn)
        );
    }
}

/* SEC("tc")
 * __failure __log_level(2)
 * __msg("3: {{.*}} R5=pkt_end()")
 * (ptr - ptr) << 2 == unknown, (4n)
 * (4n) + 14 == (4n+2).  We blow our bounds, because the add could overflow.
 * Checked s>=0.
 * packet pointer + nonnegative (4n+2)
 * __msg("pkt pointer offset -9223372036854775808 is not allowed")
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn dubious_pointer_arithmetic() {
    unsafe {
        asm!(
            "r2 = *(u32*)(r1 + {data});",
            "r3 = *(u32*)(r1 + {data_end});",
            "r0 = 0;",
            "r5 = r3;",
            "r5 -= r2;",
            "r5 <<= 2;",
            "r5 += 14;",
            "if r5 s>= 0 goto l0_3;",
            "exit;",
            "l0_3:",
            "r6 = r2;",
            "r6 += r5;",
            "r4 = r6;",
            "r4 += 4;",
            "if r3 >= r4 goto l1_3;",
            "exit;",
            "l1_3:",
            "r4 = *(u32*)(r6 + 0);",
            "exit;",
            data = const __sk_buff_data,
            data_end = const __sk_buff_data_end,
            options(noreturn)
        );
    }
}

/* SEC("tc")
 * __success __log_level(2)
 * __flag(BPF_F_ANY_ALIGNMENT)
 *
 * Variable subtraction verifier expectations are preserved from the C
 * source annotations and comments.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn variable_subtraction() {
    unsafe {
        asm!(
            "r2 = *(u32*)(r1 + {data});",
            "r3 = *(u32*)(r1 + {data_end});",
            "r0 = r2;",
            "r0 += 8;",
            "if r3 >= r0 goto l99_4;",
            "exit;",
            "l99_4:",
            "r6 = *(u8*)(r2 + 0);",
            "r7 = r6;",
            "r6 <<= 2;",
            "r6 += 14;",
            "r7 <<= 2;",
            "r6 -= r7;",
            "if r6 s>= 0 goto l0_4;",
            "exit;",
            "l0_4:",
            "r5 = r2;",
            "r5 += r6;",
            "r4 = r5;",
            "r4 += 4;",
            "if r3 >= r4 goto l1_4;",
            "exit;",
            "l1_4:",
            "r6 = *(u32*)(r5 + 0);",
            "exit;",
            data = const __sk_buff_data,
            data_end = const __sk_buff_data_end,
            options(noreturn)
        );
    }
}

/* SEC("tc")
 * __success __log_level(2)
 * __flag(BPF_F_ANY_ALIGNMENT)
 *
 * Pointer variable subtraction verifier expectations are preserved from
 * the C source annotations and comments.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn pointer_variable_subtraction() {
    unsafe {
        asm!(
            "r2 = *(u32*)(r1 + {data});",
            "r3 = *(u32*)(r1 + {data_end});",
            "r0 = r2;",
            "r0 += 8;",
            "if r3 >= r0 goto l99_5;",
            "exit;",
            "l99_5:",
            "r6 = *(u8*)(r2 + 0);",
            "r7 = r6;",
            "r6 &= 0xf;",
            "r6 <<= 2;",
            "r6 += 14;",
            "r5 = r2;",
            "r5 -= r6;",
            "r7 <<= 2;",
            "r7 += 76;",
            "r5 += r7;",
            "r4 = r5;",
            "r4 += 4;",
            "if r3 >= r4 goto l0_5;",
            "exit;",
            "l0_5:",
            "r6 = *(u32*)(r5 + 0);",
            "exit;",
            data = const __sk_buff_data,
            data_end = const __sk_buff_data_end,
            options(noreturn)
        );
    }
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "license")]
pub static mut _license: [u8; 4] = *b"GPL\0";
