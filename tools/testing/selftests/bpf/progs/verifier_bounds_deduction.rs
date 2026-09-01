// SPDX-License-Identifier: GPL-2.0
/* Converted from tools/testing/selftests/bpf/verifier/bounds_deduction.c */

#![allow(non_upper_case_globals)]

// C includes translated as dependency intent:
// #include <linux/bpf.h>
// #include <bpf/bpf_helpers.h>
// #include "bpf_misc.h"

use core::arch::global_asm;

unsafe extern "C" {
    fn bpf_get_prandom_u32() -> u32;
}

/*
SEC("socket")
__description("check deducing bounds from const, 1")
__failure __msg("R0 tried to subtract pointer from scalar")
__msg_unpriv("R1 has pointer with unsupported alu operation")
__naked void deducing_bounds_from_const_1(void)
*/
global_asm!(
    r#"
    .section socket,"ax"
    .global deducing_bounds_from_const_1
deducing_bounds_from_const_1:
    r0 = 1
    if r0 s>= 1 goto l0_1
l0_1:
    r0 -= r1
    exit
"#
);

/*
SEC("socket")
__description("check deducing bounds from const, 2")
__success __failure_unpriv
__msg_unpriv("R1 has pointer with unsupported alu operation")
__retval(1)
__naked void deducing_bounds_from_const_2(void)
*/
global_asm!(
    r#"
    .section socket,"ax"
    .global deducing_bounds_from_const_2
deducing_bounds_from_const_2:
    r0 = 1
    if r0 s>= 1 goto l0_2
    exit
l0_2:
    if r0 s<= 1 goto l1_2
    exit
l1_2:
    r1 -= r0
    exit
"#
);

/*
SEC("socket")
__description("check deducing bounds from const, 3")
__failure __msg("R0 tried to subtract pointer from scalar")
__msg_unpriv("R1 has pointer with unsupported alu operation")
__naked void deducing_bounds_from_const_3(void)
*/
global_asm!(
    r#"
    .section socket,"ax"
    .global deducing_bounds_from_const_3
deducing_bounds_from_const_3:
    r0 = 0
    if r0 s<= 0 goto l0_3
l0_3:
    r0 -= r1
    exit
"#
);

/*
SEC("socket")
__description("check deducing bounds from const, 4")
__success __failure_unpriv
__msg_unpriv("R6 has pointer with unsupported alu operation")
__retval(0)
__naked void deducing_bounds_from_const_4(void)
*/
global_asm!(
    r#"
    .section socket,"ax"
    .global deducing_bounds_from_const_4
deducing_bounds_from_const_4:
    r6 = r1
    r0 = 0
    if r0 s<= 0 goto l0_4
    exit
l0_4:
    if r0 s>= 0 goto l1_4
    exit
l1_4:
    r6 -= r0
    exit
"#
);

/*
SEC("socket")
__description("check deducing bounds from const, 5")
__failure __msg("R0 tried to subtract pointer from scalar")
__msg_unpriv("R1 has pointer with unsupported alu operation")
__naked void deducing_bounds_from_const_5(void)
*/
global_asm!(
    r#"
    .section socket,"ax"
    .global deducing_bounds_from_const_5
deducing_bounds_from_const_5:
    r0 = 0
    if r0 s>= 1 goto l0_5
    r0 -= r1
l0_5:
    exit
"#
);

/*
SEC("socket")
__description("check deducing bounds from const, 6")
__failure __msg("R0 tried to subtract pointer from scalar")
__msg_unpriv("R1 has pointer with unsupported alu operation")
__naked void deducing_bounds_from_const_6(void)
*/
global_asm!(
    r#"
    .section socket,"ax"
    .global deducing_bounds_from_const_6
deducing_bounds_from_const_6:
    r0 = 0
    if r0 s>= 0 goto l0_6
    exit
l0_6:
    r0 -= r1
    exit
"#
);

/*
SEC("socket")
__description("check deducing bounds from const, 7")
__failure __msg("dereference of modified ctx ptr")
__msg_unpriv("R1 has pointer with unsupported alu operation")
__flag(BPF_F_ANY_ALIGNMENT)
__naked void deducing_bounds_from_const_7(void)

The C inline-asm operands are:
__imm_const(__imm_0, ~0)
__imm_const(__sk_buff_mark, offsetof(struct __sk_buff, mark))
*/
global_asm!(
    r#"
    .section socket,"ax"
    .global deducing_bounds_from_const_7
deducing_bounds_from_const_7:
    r0 = -1
    if r0 s>= 0 goto l0_7
l0_7:
    r1 -= r0
    r0 = *(u32*)(r1 + __sk_buff_mark)
    exit
"#
);

/*
SEC("socket")
__description("check deducing bounds from const, 8")
__failure __msg("negative offset ctx ptr R1 off=-1 disallowed")
__msg_unpriv("R1 has pointer with unsupported alu operation")
__flag(BPF_F_ANY_ALIGNMENT)
__naked void deducing_bounds_from_const_8(void)

The C inline-asm operands are:
__imm_const(__imm_0, ~0)
__imm_const(__sk_buff_mark, offsetof(struct __sk_buff, mark))
*/
global_asm!(
    r#"
    .section socket,"ax"
    .global deducing_bounds_from_const_8
deducing_bounds_from_const_8:
    r0 = -1
    if r0 s>= 0 goto l0_8
    r1 += r0
l0_8:
    r0 = *(u32*)(r1 + __sk_buff_mark)
    exit
"#
);

/*
SEC("socket")
__description("check deducing bounds from const, 9")
__failure __msg("R0 tried to subtract pointer from scalar")
__msg_unpriv("R1 has pointer with unsupported alu operation")
__naked void deducing_bounds_from_const_9(void)
*/
global_asm!(
    r#"
    .section socket,"ax"
    .global deducing_bounds_from_const_9
deducing_bounds_from_const_9:
    r0 = 0
    if r0 s>= 0 goto l0_9
l0_9:
    r0 -= r1
    exit
"#
);

/*
SEC("socket")
__description("check deducing bounds from const, 10")
__failure
__msg("math between ctx pointer and register with unbounded min value is not allowed")
__failure_unpriv
__naked void deducing_bounds_from_const_10(void)
*/
global_asm!(
    r#"
    .section socket,"ax"
    .global deducing_bounds_from_const_10
deducing_bounds_from_const_10:
    r6 = r1
    r0 = 0
    if r0 s<= 0 goto l0_10
l0_10:
    /* Marks r0 as unknown. */
    call bpf_get_prandom_u32
    r0 -= r6
    exit
"#
);

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
