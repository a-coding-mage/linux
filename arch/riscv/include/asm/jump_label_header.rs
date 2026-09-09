/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2020 Emil Renner Berthing
 *
 * Based on arch/arm64/include/asm/jump_label.h
 */

// The original header is excluded for assembler translation units.

/// The architecture supports batching jump-label updates.
pub const HAVE_JUMP_LABEL_BATCH: bool = true;

pub const JUMP_LABEL_NOP_SIZE: usize = 4;

#[macro_export]
macro_rules! JUMP_TABLE_ENTRY {
    ($key:expr, $label:expr) => {
        concat!(
            ".pushsection\t__jump_table, \"aw\"\n\t",
            ".align\t\t", RISCV_LGPTR, "\n\t",
            ".long\t\t1b - ., ", $label, " - .\n\t",
            RISCV_PTR, "\t", $key, " - .\n\t",
            ".popsection\n\t"
        )
    };
}

/* This macro is also expanded on the Rust side. */
#[macro_export]
macro_rules! ARCH_STATIC_BRANCH_ASM {
    ($key:expr, $label:expr) => {
        concat!(
            "\t.align\t\t2\n\t",
            "\t.option push\n\t",
            "\t.option norelax\n\t",
            "\t.option norvc\n\t",
            "1:\tnop\n\t",
            "\t.option pop\n\t",
            JUMP_TABLE_ENTRY!($key, $label)
        )
    };
}

/// Corresponds to the C `asm goto` implementation.
///
/// Rust has no direct stable equivalent of GCC's `asm goto`; the assembly
/// branch and its local label remain represented by `ARCH_STATIC_BRANCH_ASM!`.
#[inline(always)]
pub unsafe fn arch_static_branch(
    key: *mut static_key,
    branch: bool,
) -> bool {
    let _ = (key, branch);
    // TODO: translate the target-specific `asm goto` when the Rust backend
    // provides an equivalent primitive.
    false
}

#[macro_export]
macro_rules! ARCH_STATIC_BRANCH_JUMP_ASM {
    ($key:expr, $label:expr) => {
        concat!(
            "\t.align\t\t2\n\t",
            "\t.option push\n\t",
            "\t.option norelax\n\t",
            "\t.option norvc\n\t",
            "1:\tj\t", $label, "\n\t",
            "\t.option pop\n\t",
            JUMP_TABLE_ENTRY!($key, $label)
        )
    };
}

#[inline(always)]
pub unsafe fn arch_static_branch_jump(
    key: *mut static_key,
    branch: bool,
) -> bool {
    let _ = (key, branch);
    // TODO: translate the target-specific `asm goto` when the Rust backend
    // provides an equivalent primitive.
    false
}

// Supplied by the Linux static-key headers.
#[allow(non_camel_case_types)]
pub struct static_key {
    _private: [u8; 0],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
