/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2025 Chen Miao
 *
 * Based on arch/arm/include/asm/jump_label.h
 */

// The C header guard and __ASSEMBLER__ conditional are intentionally omitted
// from executable Rust; this file is the non-assembler translation.

/// Marker indicating that jump-label batching is supported.
pub const HAVE_JUMP_LABEL_BATCH: bool = true;

/// `JUMP_LABEL_NOP_SIZE OPENRISC_INSN_SIZE`.
pub const JUMP_LABEL_NOP_SIZE: usize = OPENRISC_INSN_SIZE;

// `OPENRISC_INSN_SIZE` is supplied by asm/insn-def.h.

/// Create a jump-table entry in the dedicated `__jump_table` section.
#[macro_export]
macro_rules! JUMP_TABLE_ENTRY {
    ($key:expr, $label:expr) => {
        concat!(
            ".pushsection\t__jump_table, \"aw\"\n\t",
            ".align \t4 \n\t",
            ".long \t1b - ., ", $label, " - .\n\t",
            ".long \t", $key, " - . \n\t",
            ".popsection\n\t",
        )
    };
}

#[macro_export]
macro_rules! ARCH_STATIC_BRANCH_ASM {
    ($key:expr, $label:expr) => {
        concat!(
            ".align\t 4\n\t",
            "1: l.nop\n\t",
            "    l.nop\n\t",
            $crate::JUMP_TABLE_ENTRY!($key, $label),
        )
    };
}

/// Opaque dependency corresponding to C's `struct static_key`.
#[repr(C)]
pub struct static_key {
    _private: [u8; 0],
}

/// The C `asm goto` instruction sequence cannot be expressed by Rust's
/// `asm!` interface. The branch target and patchable instruction semantics
/// are retained here as the source-level contract.
#[inline(always)]
pub unsafe fn arch_static_branch(_key: *mut static_key, _branch: bool) -> bool {
    // asm goto (ARCH_STATIC_BRANCH_ASM("%0", "%l[l_yes]"));
    // return false; l_yes: return true;
    false
}

#[macro_export]
macro_rules! ARCH_STATIC_BRANCH_JUMP_ASM {
    ($key:expr, $label:expr) => {
        concat!(
            ".align\t\t4\n\t",
            "1: l.j\t", $label, "\n\t",
            "    l.nop\n\t",
            $crate::JUMP_TABLE_ENTRY!($key, $label),
        )
    };
}

#[inline(always)]
pub unsafe fn arch_static_branch_jump(_key: *mut static_key, _branch: bool) -> bool {
    // asm goto (ARCH_STATIC_BRANCH_JUMP_ASM("%0", "%l[l_yes]"));
    // return false; l_yes: return true;
    false
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
