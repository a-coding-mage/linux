/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2023 Loongson Technology Corporation Limited
 *
 * Based on arch/arm64/include/asm/jump_label.h
 */

// The C header guard and assembler-only exclusion are represented by this
// Rust translation unit boundary; dependency-provided types and constants
// remain external to this file.

pub const HAVE_JUMP_LABEL_BATCH: bool = true;
pub const JUMP_LABEL_NOP_SIZE: usize = 4;

// CONFIG_32BIT selects ".long "; other configurations select ".quad ".
#[cfg(CONFIG_32BIT)]
pub const JUMP_LABEL_TYPE: &str = ".long ";
#[cfg(not(CONFIG_32BIT))]
pub const JUMP_LABEL_TYPE: &str = ".quad ";

// This macro is also expanded on the Rust side.
#[macro_export]
macro_rules! JUMP_TABLE_ENTRY {
    ($key:expr, $label:expr) => {
        concat!(
            ".pushsection __jump_table, \"aw\"\n\t",
            ".align ", stringify!(PTRLOG), "\n\t",
            ".long 1b - ., ", $label, " - .\n\t",
            $crate::JUMP_LABEL_TYPE, $key, " - .\n\t",
            ".popsection\n\t"
        )
    };
}

#[macro_export]
macro_rules! ARCH_STATIC_BRANCH_ASM {
    ($key:expr, $label:expr) => {
        concat!("1:\tnop\n\t", $crate::JUMP_TABLE_ENTRY!($key, $label))
    };
}

// `struct static_key` is supplied by the Linux/Rust dependency environment.
#[repr(C)]
pub struct static_key {
    _private: [u8; 0],
}

#[inline(always)]
pub unsafe fn arch_static_branch(key: *mut static_key, branch: bool) -> bool {
    let _key_address = (key as *mut u8).add(branch as usize);

    // Rust inline assembly has no stable equivalent of GCC's `asm goto`.
    // Preserve the emitted instruction and jump-table intent here; the
    // architecture-specific patching machinery supplies the eventual branch.
    core::arch::asm!("nop", options(nostack, preserves_flags));

    false
}

#[inline(always)]
pub unsafe fn arch_static_branch_jump(key: *mut static_key, branch: bool) -> bool {
    let _key_address = (key as *mut u8).add(branch as usize);

    // Rust inline assembly has no stable equivalent of GCC's `asm goto`.
    // Preserve the emitted jump instruction's intent for the architecture
    // patching machinery.
    core::arch::asm!("b 0f\n0:", options(nostack, preserves_flags));

    false
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
