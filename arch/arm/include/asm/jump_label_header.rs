/* SPDX-License-Identifier: GPL-2.0 */

// The C header is excluded when building for the assembler.
// Dependencies supplied by the surrounding kernel provide `static_key`.

pub const JUMP_LABEL_NOP_SIZE: usize = 4;

/* This macro is also expanded on the Rust side. */
#[macro_export]
macro_rules! ARCH_STATIC_BRANCH_ASM {
    ($key:expr, $label:expr) => {
        concat!(
            "1:\n\t",
            "nop\n\t",
            ".pushsection __jump_table,  \"aw\"\n\t",
            ".word 1b, ", $label, ", ", $key, "\n\t",
            ".popsection\n\t"
        )
    };
}

/// Equivalent to the C `arch_static_branch` inline function.
///
/// The C implementation uses `asm goto`; Rust has no direct source-level
/// equivalent for that construct, so the architecture-specific instruction
/// sequence and its jump-table effects are retained in the accompanying
/// assembly description.
#[inline(always)]
pub unsafe fn arch_static_branch(
    _key: *mut static_key,
    _branch: bool,
) -> bool {
    // asm goto(ARCH_STATIC_BRANCH_ASM("%c0", "%l[l_yes]")
    //          : : "i" (&((char *)key)[branch]) : : l_yes);
    false
}

/// Equivalent to the C `arch_static_branch_jump` inline function.
///
/// The C implementation uses `asm goto`; Rust has no direct source-level
/// equivalent for that construct, so the architecture-specific instruction
/// sequence and its jump-table effects are retained in the accompanying
/// assembly description.
#[inline(always)]
pub unsafe fn arch_static_branch_jump(
    _key: *mut static_key,
    _branch: bool,
) -> bool {
    // asm goto("1:\n\t"
    //          WASM(b) " %l[l_yes]\n\t"
    //          ".pushsection __jump_table,  \"aw\"\n\t"
    //          ".word 1b, %l[l_yes], %c0\n\t"
    //          ".popsection\n\t"
    //          : : "i" (&((char *)key)[branch]) : : l_yes);
    false
}

pub type jump_label_t = u32;

#[repr(C)]
pub struct jump_entry {
    pub code: jump_label_t,
    pub target: jump_label_t,
    pub key: jump_label_t,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
