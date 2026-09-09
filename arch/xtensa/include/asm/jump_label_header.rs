/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright (C) 2018 Cadence Design Systems Inc. */

// C header guard and __ASSEMBLER__ condition omitted from executable Rust.
// Dependency: `struct static_key` is supplied by the surrounding kernel code.

pub const JUMP_LABEL_NOP_SIZE: usize = 3;

/// Translation of `arch_static_branch`.
///
/// The source uses Xtensa `asm goto` to register a jump-table entry and may
/// branch to `l_yes` when patched by the kernel. Rust has no direct equivalent
/// for C's `asm goto`; the original assembly is preserved below as intent.
#[inline(always)]
pub unsafe fn arch_static_branch(key: *mut static_key, branch: bool) -> bool {
    let _ = (key, branch);
    // asm goto("1:\n\t_nop\n\t.pushsection __jump_table, \"aw\"\n\t.word 1b, %l[l_yes], %c0\n\t.popsection" : : "i" (&((char *)key)[branch]) : : l_yes);
    false
    // l_yes: return true;
}

/// Translation of `arch_static_branch_jump`.
///
/// The source uses Xtensa `asm goto` and assembler no-transform directives;
/// these are retained as comments because Rust cannot express this control
/// flow or target-specific assembly in a file-local, portable item.
#[inline(always)]
pub unsafe fn arch_static_branch_jump(key: *mut static_key, branch: bool) -> bool {
    let _ = (key, branch);
    // Xtensa assembler will mark certain points in the code as unreachable,
    // so that later assembler or linker relaxation passes could use them. A
    // spot right after the J instruction is one such point. Assembler and/or
    // linker may insert padding or literals here, breaking code flow in case
    // the J instruction is later replaced with NOP. Put a label right after
    // the J to make it reachable and wrap both into a no-transform block to
    // avoid any assembler interference with this.
    // asm goto("1:\n\t.begin no-transform\n\t_j %l[l_yes]\n\t2:\n\t.end no-transform\n\t.pushsection __jump_table, \"aw\"\n\t.word 1b, %l[l_yes], %c0\n\t.popsection" : : "i" (&((char *)key)[branch]) : : l_yes);
    false
    // l_yes: return true;
}

pub type jump_label_t = u32;

#[repr(C)]
pub struct jump_entry {
    pub code: jump_label_t,
    pub target: jump_label_t,
    pub key: jump_label_t,
}

// External dependency declaration corresponding to `struct static_key`.
pub struct static_key;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
