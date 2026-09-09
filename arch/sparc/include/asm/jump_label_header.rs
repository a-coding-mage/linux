/* SPDX-License-Identifier: GPL-2.0 */

/*
 * This header is excluded when compiling assembler (__ASSEMBLER__).
 * The Linux types dependency is supplied by the surrounding translation.
 */

pub const JUMP_LABEL_NOP_SIZE: usize = 4;

/// Architecture-specific static branch implementation.
///
/// The C implementation uses `asm goto` to emit two nops and a jump-table
/// entry, with the `l_yes` label selected by runtime static-key patching.
/// Rust has no direct equivalent of GCC's `asm goto`; the original assembly
/// and its control-flow intent are retained here for the target integration.
#[inline(always)]
pub unsafe fn arch_static_branch(
    key: *mut crate::static_key,
    branch: bool,
) -> bool {
    let _ = (key, branch);
    /*
     * asm goto("1:\n\t"
     *          "nop\n\t"
     *          "nop\n\t"
     *          ".pushsection __jump_table,  \"aw\"\n\t"
     *          ".align 4\n\t"
     *          ".word 1b, %l[l_yes], %c0\n\t"
     *          ".popsection \n\t"
     *          : :  "i" (&((char *)key)[branch]) : : l_yes);
     *
     * TODO: preserve the assembler-goto `l_yes` target when integrated with
     * the architecture's inline-assembly support.
     */
    false
}

/// Architecture-specific static branch-with-jump implementation.
///
/// The C implementation uses `asm goto` to emit a branch, delay-slot nop,
/// and jump-table entry, with `l_yes` selected by static-key patching.
#[inline(always)]
pub unsafe fn arch_static_branch_jump(
    key: *mut crate::static_key,
    branch: bool,
) -> bool {
    let _ = (key, branch);
    /*
     * asm goto("1:\n\t"
     *          "b %l[l_yes]\n\t"
     *          "nop\n\t"
     *          ".pushsection __jump_table,  \"aw\"\n\t"
     *          ".align 4\n\t"
     *          ".word 1b, %l[l_yes], %c0\n\t"
     *          ".popsection \n\t"
     *          : :  "i" (&((char *)key)[branch]) : : l_yes);
     *
     * TODO: preserve the assembler-goto `l_yes` target when integrated with
     * the architecture's inline-assembly support.
     */
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
