/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2013 Huawei Ltd.
 * Author: Jiang Liu <liuj97@gmail.com>
 *
 * Based on arch/arm/include/asm/jump_label.h
 */

/* The C header guard and assembler-only condition are not needed in Rust. */

/* Dependency supplied by the surrounding kernel translation. */
// use crate::asm::insn::AARCH64_INSN_SIZE;
// use crate::linux::types::static_key;

pub const HAVE_JUMP_LABEL_BATCH: bool = true;
pub const JUMP_LABEL_NOP_SIZE: usize = AARCH64_INSN_SIZE as usize;

/* This macro is also expanded on the Rust side. */
#[macro_export]
macro_rules! JUMP_TABLE_ENTRY {
    ($key:expr, $label:expr) => {
        concat!(
            ".pushsection\t__jump_table, \"aw\"\n\t",
            ".align\t\t3\n\t",
            ".long\t\t1b - ., ", $label, " - .\n\t",
            ".quad\t\t", $key, " - .\n\t",
            ".popsection\n\t"
        )
    };
}

/* This macro is also expanded on the Rust side. */
#[macro_export]
macro_rules! ARCH_STATIC_BRANCH_ASM {
    ($key:expr, $label:expr) => {
        concat!("1:\tnop\n\t", JUMP_TABLE_ENTRY!($key, $label))
    };
}

pub unsafe fn arch_static_branch(key: *mut static_key, branch: bool) -> bool {
    let k: *mut u8 = (key as *mut u8).add(branch as usize);

    /* C asm goto cannot be expressed by stable Rust inline assembly. */
    let _ = k;
    false
}

pub unsafe fn arch_static_branch_jump(key: *mut static_key, branch: bool) -> bool {
    let k: *mut u8 = (key as *mut u8).add(branch as usize);

    /* C asm goto cannot be expressed by stable Rust inline assembly. */
    let _ = k;
    false
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
