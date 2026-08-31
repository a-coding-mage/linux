// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2025 Isovalent */

/*
 * C dependencies translated as external expectations:
 * - <linux/bpf.h>
 * - <bpf/bpf_helpers.h>
 * - "bpf_misc.h"
 * - "../../../include/linux/filter.h"
 *
 * The original file only builds the target-specific programs below when one of
 * __TARGET_ARCH_x86, __TARGET_ARCH_arm64, or __TARGET_ARCH_powerpc is defined.
 */

#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(dead_code)]

use core::arch::asm;

const BPF_JMP: u32 = 0x05;
const BPF_JA: u32 = 0x00;
const BPF_X: u32 = 0x08;
const BPF_REG_0: u32 = 0;
const BPF_REG_1: u32 = 1;
const BPF_REG_2: u32 = 2;
const BPF_REG_3: u32 = 3;
const BPF_REG_4: u32 = 4;
const BPF_REG_5: u32 = 5;
const BPF_REG_6: u32 = 6;
const BPF_REG_7: u32 = 7;
const BPF_REG_8: u32 = 8;
const BPF_REG_9: u32 = 9;

macro_rules! BPF_RAW_INSN {
    ($code:expr, $dst_reg:expr, $src_reg:expr, $off:expr, $imm:expr) => {
        (($code), ($dst_reg), ($src_reg), ($off), ($imm))
    };
}

macro_rules! DEFINE_SIMPLE_JUMP_TABLE_PROG {
    ($name:ident, $src_reg:expr, $off:expr, $imm:expr, $outcome:meta) => {
        #[unsafe(no_mangle)]
        #[unsafe(link_section = "socket")]
        #[$outcome]
        pub unsafe extern "C" fn $name() {
            unsafe {
                asm!(
                    ".pushsection .jumptables,\"\",@progbits",
                    "jt0_0:",
                    ".quad ret0_0 - socket",
                    ".quad ret1_0 - socket",
                    ".size jt0_0, 16",
                    ".global jt0_0",
                    ".popsection",
                    "r0 = jt0_0 ll",
                    "r0 += 8",
                    "r0 = *(u64 *)(r0 + 0)",
                    ".8byte {gotox_r0}",
                    "ret0_0:",
                    "r0 = 0",
                    "exit",
                    "ret1_0:",
                    "r0 = 1",
                    "exit",
                    gotox_r0 = const BPF_RAW_INSN!(BPF_JMP | BPF_JA | BPF_X, BPF_REG_0, $src_reg, $off, $imm),
                    options(noreturn)
                );
            }
        }
    };
}

/*
 * The first program which doesn't use reserved fields
 * loads and works properly. The rest fail to load.
 */
DEFINE_SIMPLE_JUMP_TABLE_PROG!(jump_table_ok,                          BPF_REG_0, 0, 0, success_retval_1);
DEFINE_SIMPLE_JUMP_TABLE_PROG!(jump_table_reserved_field_src_reg,      BPF_REG_1, 0, 0, failure_reserved_fields);
DEFINE_SIMPLE_JUMP_TABLE_PROG!(jump_table_reserved_field_non_zero_off, BPF_REG_0, 1, 0, failure_reserved_fields);
DEFINE_SIMPLE_JUMP_TABLE_PROG!(jump_table_reserved_field_non_zero_imm, BPF_REG_0, 0, 1, failure_reserved_fields);

/*
 * Gotox is forbidden when there is no jump table loaded
 * which points to the sub-function where the gotox is used
 */
#[unsafe(no_mangle)]
#[unsafe(link_section = "socket")]
#[failure]
#[msg = "no jump tables found for subprog starting at 0"]
pub unsafe extern "C" fn jump_table_no_jump_table() {
    unsafe {
        asm!(
            ".8byte {gotox_r0}",
            "r0 = 1",
            "exit",
            gotox_r0 = const BPF_RAW_INSN!(BPF_JMP | BPF_JA | BPF_X, BPF_REG_0, 0, 0, 0),
            options(noreturn)
        );
    }
}

/*
 * Incorrect type of the target register, only PTR_TO_INSN allowed
 */
#[unsafe(no_mangle)]
#[unsafe(link_section = "socket")]
#[failure]
#[msg = "R1 has type scalar, expected PTR_TO_INSN"]
pub unsafe extern "C" fn jump_table_incorrect_dst_reg_type() {
    unsafe {
        asm!(
            ".pushsection .jumptables,\"\",@progbits",
            "jt0_1:",
            ".quad ret0_1 - socket",
            ".quad ret1_1 - socket",
            ".size jt0_1, 16",
            ".global jt0_1",
            ".popsection",
            "r0 = jt0_1 ll",
            "r0 += 8",
            "r0 = *(u64 *)(r0 + 0)",
            "r1 = 42",
            ".8byte {gotox_r1}",
            "ret0_1:",
            "r0 = 0",
            "exit",
            "ret1_1:",
            "r0 = 1",
            "exit",
            gotox_r1 = const BPF_RAW_INSN!(BPF_JMP | BPF_JA | BPF_X, BPF_REG_1, 0, 0, 0),
            options(noreturn)
        );
    }
}

macro_rules! DEFINE_INVALID_SIZE_PROG {
    ($read_size:ident, $outcome:meta) => {
        #[unsafe(no_mangle)]
        #[unsafe(link_section = "socket")]
        #[$outcome]
        pub unsafe extern "C" fn $read_size() {
            unsafe {
                asm!(
                    ".pushsection .jumptables,\"\",@progbits",
                    "jt0_2:",
                    ".quad ret0_2 - socket",
                    ".quad ret1_2 - socket",
                    ".size jt0_2, 16",
                    ".global jt0_2",
                    ".popsection",
                    "r0 = jt0_2 ll",
                    "r0 += 8",
                    concat!("r0 = *(", stringify!($read_size), " *)(r0 + 0)"),
                    ".8byte {gotox_r0}",
                    "ret0_2:",
                    "r0 = 0",
                    "exit",
                    "ret1_2:",
                    "r0 = 1",
                    "exit",
                    gotox_r0 = const BPF_RAW_INSN!(BPF_JMP | BPF_JA | BPF_X, BPF_REG_0, 0, 0, 0),
                    options(noreturn)
                );
            }
        }
    };
}

DEFINE_INVALID_SIZE_PROG!(jump_table_invalid_read_size_u32, failure_invalid_read_4);
DEFINE_INVALID_SIZE_PROG!(jump_table_invalid_read_size_u16, failure_invalid_read_2);
DEFINE_INVALID_SIZE_PROG!(jump_table_invalid_read_size_u8,  failure_invalid_read_1);

#[unsafe(no_mangle)]
#[unsafe(link_section = "socket")]
#[failure]
#[msg = "misaligned value access off 1+0 size 8"]
pub unsafe extern "C" fn jump_table_misaligned_access() {
    unsafe {
        asm!(
            ".pushsection .jumptables,\"\",@progbits",
            "jt0_3:",
            ".quad ret0_3 - socket",
            ".quad ret1_3 - socket",
            ".size jt0_3, 16",
            ".global jt0_3",
            ".popsection",
            "r0 = jt0_3 ll",
            "r0 += 1",
            "r0 = *(u64 *)(r0 + 0)",
            ".8byte {gotox_r0}",
            "ret0_3:",
            "r0 = 0",
            "exit",
            "ret1_3:",
            "r0 = 1",
            "exit",
            gotox_r0 = const BPF_RAW_INSN!(BPF_JMP | BPF_JA | BPF_X, BPF_REG_0, 0, 0, 0),
            options(noreturn)
        );
    }
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "socket")]
#[failure]
#[msg = "invalid access to map value, value_size=16 off=24 size=8"]
pub unsafe extern "C" fn jump_table_invalid_mem_acceess_pos() {
    unsafe {
        asm!(
            ".pushsection .jumptables,\"\",@progbits",
            "jt0_4:",
            ".quad ret0_4 - socket",
            ".quad ret1_4 - socket",
            ".size jt0_4, 16",
            ".global jt0_4",
            ".popsection",
            "r0 = jt0_4 ll",
            "r0 += 24",
            "r0 = *(u64 *)(r0 + 0)",
            ".8byte {gotox_r0}",
            "ret0_4:",
            "r0 = 0",
            "exit",
            "ret1_4:",
            "r0 = 1",
            "exit",
            gotox_r0 = const BPF_RAW_INSN!(BPF_JMP | BPF_JA | BPF_X, BPF_REG_0, 0, 0, 0),
            options(noreturn)
        );
    }
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "socket")]
#[failure]
#[msg = "R0 min value is negative"]
pub unsafe extern "C" fn jump_table_invalid_mem_acceess_neg() {
    unsafe {
        asm!(
            ".pushsection .jumptables,\"\",@progbits",
            "jt0_5:",
            ".quad ret0_5 - socket",
            ".quad ret1_5 - socket",
            ".size jt0_5, 16",
            ".global jt0_5",
            ".popsection",
            "r0 = jt0_5 ll",
            "r0 -= 24",
            "r0 = *(u64 *)(r0 + 0)",
            ".8byte {gotox_r0}",
            "ret0_5:",
            "r0 = 0",
            "exit",
            "ret1_5:",
            "r0 = 1",
            "exit",
            gotox_r0 = const BPF_RAW_INSN!(BPF_JMP | BPF_JA | BPF_X, BPF_REG_0, 0, 0, 0),
            options(noreturn)
        );
    }
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "socket")]
#[success]
#[retval = 1]
pub unsafe extern "C" fn jump_table_add_sub_ok() {
    unsafe {
        asm!(
            ".pushsection .jumptables,\"\",@progbits",
            "jt0_6:",
            ".quad ret0_6 - socket",
            ".quad ret1_6 - socket",
            ".size jt0_6, 16",
            ".global jt0_6",
            ".popsection",
            "r0 = jt0_6 ll",
            "r0 -= 24",
            "r0 += 32",
            "r0 = *(u64 *)(r0 + 0)",
            ".8byte {gotox_r0}",
            "ret0_6:",
            "r0 = 0",
            "exit",
            "ret1_6:",
            "r0 = 1",
            "exit",
            gotox_r0 = const BPF_RAW_INSN!(BPF_JMP | BPF_JA | BPF_X, BPF_REG_0, 0, 0, 0),
            options(noreturn)
        );
    }
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "socket")]
#[failure]
#[msg = "write into map forbidden, value_size=16 off=8 size=8"]
pub unsafe extern "C" fn jump_table_no_writes() {
    unsafe {
        asm!(
            ".pushsection .jumptables,\"\",@progbits",
            "jt0_7:",
            ".quad ret0_7 - socket",
            ".quad ret1_7 - socket",
            ".size jt0_7, 16",
            ".global jt0_7",
            ".popsection",
            "r0 = jt0_7 ll",
            "r0 += 8",
            "r1 = 0xbeef",
            "*(u64 *)(r0 + 0) = r1",
            ".8byte {gotox_r0}",
            "ret0_7:",
            "r0 = 0",
            "exit",
            "ret1_7:",
            "r0 = 1",
            "exit",
            gotox_r0 = const BPF_RAW_INSN!(BPF_JMP | BPF_JA | BPF_X, BPF_REG_0, 0, 0, 0),
            options(noreturn)
        );
    }
}

macro_rules! DEFINE_JUMP_TABLE_USE_REG {
    ($fn_name:ident, $reg_num:literal, $bpf_reg:expr) => {
        #[unsafe(no_mangle)]
        #[unsafe(link_section = "socket")]
        #[success]
        #[retval = 1]
        pub unsafe extern "C" fn $fn_name() {
            unsafe {
                asm!(
                    ".pushsection .jumptables,\"\",@progbits",
                    "jt0_8:",
                    ".quad ret0_8 - socket",
                    ".quad ret1_8 - socket",
                    ".size jt0_8, 16",
                    ".global jt0_8",
                    ".popsection",
                    "r0 = jt0_8 ll",
                    "r0 += 8",
                    concat!("r", stringify!($reg_num), " = *(u64 *)(r0 + 0)"),
                    ".8byte {gotox_rX}",
                    "ret0_8:",
                    "r0 = 0",
                    "exit",
                    "ret1_8:",
                    "r0 = 1",
                    "exit",
                    gotox_rX = const BPF_RAW_INSN!(BPF_JMP | BPF_JA | BPF_X, $bpf_reg, 0, 0, 0),
                    options(noreturn)
                );
            }
        }
    };
}

DEFINE_JUMP_TABLE_USE_REG!(jump_table_use_reg_r0, 0, BPF_REG_0);
DEFINE_JUMP_TABLE_USE_REG!(jump_table_use_reg_r1, 1, BPF_REG_1);
DEFINE_JUMP_TABLE_USE_REG!(jump_table_use_reg_r2, 2, BPF_REG_2);
DEFINE_JUMP_TABLE_USE_REG!(jump_table_use_reg_r3, 3, BPF_REG_3);
DEFINE_JUMP_TABLE_USE_REG!(jump_table_use_reg_r4, 4, BPF_REG_4);
DEFINE_JUMP_TABLE_USE_REG!(jump_table_use_reg_r5, 5, BPF_REG_5);
DEFINE_JUMP_TABLE_USE_REG!(jump_table_use_reg_r6, 6, BPF_REG_6);
DEFINE_JUMP_TABLE_USE_REG!(jump_table_use_reg_r7, 7, BPF_REG_7);
DEFINE_JUMP_TABLE_USE_REG!(jump_table_use_reg_r8, 8, BPF_REG_8);
DEFINE_JUMP_TABLE_USE_REG!(jump_table_use_reg_r9, 9, BPF_REG_9);

#[used]
static test_subprog: unsafe extern "C" fn() -> i32 = {
    unsafe extern "C" fn test_subprog_impl() -> i32 {
        0
    }
    test_subprog_impl
};

#[unsafe(no_mangle)]
#[unsafe(link_section = "socket")]
#[failure]
#[msg = "jump table for insn 4 points outside of the subprog [0,10]"]
pub unsafe extern "C" fn jump_table_outside_subprog() {
    unsafe {
        asm!(
            ".pushsection .jumptables,\"\",@progbits",
            "jt0_9:",
            ".quad ret0_9 - socket",
            ".quad ret1_9 - socket",
            ".quad ret_out_9 - socket",
            ".size jt0_9, 24",
            ".global jt0_9",
            ".popsection",
            "r0 = jt0_9 ll",
            "r0 += 8",
            "r0 = *(u64 *)(r0 + 0)",
            ".8byte {gotox_r0}",
            "ret0_9:",
            "r0 = 0",
            "exit",
            "ret1_9:",
            "r0 = 1",
            "call test_subprog",
            "exit",
            "ret_out_9:",
            gotox_r0 = const BPF_RAW_INSN!(BPF_JMP | BPF_JA | BPF_X, BPF_REG_0, 0, 0, 0),
            options(noreturn)
        );
    }
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "socket")]
#[success]
#[retval = 1]
pub unsafe extern "C" fn jump_table_contains_non_unique_values() {
    unsafe {
        asm!(
            ".pushsection .jumptables,\"\",@progbits",
            "jt0_10:",
            ".quad ret0_10 - socket",
            ".quad ret1_10 - socket",
            ".quad ret0_10 - socket",
            ".quad ret1_10 - socket",
            ".quad ret0_10 - socket",
            ".quad ret1_10 - socket",
            ".quad ret0_10 - socket",
            ".quad ret1_10 - socket",
            ".quad ret0_10 - socket",
            ".quad ret1_10 - socket",
            ".size jt0_10, 80",
            ".global jt0_10",
            ".popsection",
            "r0 = jt0_10 ll",
            "r0 += 8",
            "r0 = *(u64 *)(r0 + 0)",
            ".8byte {gotox_r0}",
            "ret0_10:",
            "r0 = 0",
            "exit",
            "ret1_10:",
            "r0 = 1",
            "exit",
            gotox_r0 = const BPF_RAW_INSN!(BPF_JMP | BPF_JA | BPF_X, BPF_REG_0, 0, 0, 0),
            options(noreturn)
        );
    }
}

/* check valid spill/fill, ptr to insn */
#[unsafe(no_mangle)]
#[unsafe(link_section = "socket")]
#[success]
pub unsafe extern "C" fn spill_fill_ptr_to_insn() {
    unsafe {
        asm!(
            ".pushsection .jumptables,\"\",@progbits",
            "jt0_11:",
            ".quad ret0_11 - socket",
            ".size jt0_11, 8",
            ".global jt0_11",
            ".popsection",
            "r0 = jt0_11 ll",
            "r0 = *(u64 *)(r0 + 0)",
            "*(u64 *)(r10 - 8) = r0",
            "r0 = *(u64 *)(r10 - 8)",
            ".8byte {gotox_r0}",
            "ret0_11:",
            "r0 = 0",
            "exit",
            gotox_r0 = const BPF_RAW_INSN!(BPF_JMP | BPF_JA | BPF_X, BPF_REG_0, 0, 0, 0),
            options(noreturn)
        );
    }
}

#[used]
#[unsafe(no_mangle)]
#[unsafe(link_section = "license")]
pub static _license: [u8; 4] = *b"GPL\0";
