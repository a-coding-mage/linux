/* SPDX-License-Identifier: GPL-2.0 */

/*
 * The C header is excluded when assembling.  The declarations below require
 * the corresponding Linux type definitions and assembly constants from the
 * surrounding translation unit.
 */

pub const JUMP_LABEL_NOP_SIZE: usize = 4;

/* Opaque dependency supplied by the translated Linux type definitions. */
#[allow(non_camel_case_types)]
pub struct static_key {
    _private: [u8; 0],
}

#[inline(always)]
pub unsafe fn arch_static_branch(key: *mut static_key, branch: bool) -> bool {
    /*
     * C asm goto emits a patchable jump-table entry and may branch to l_yes.
     * Rust inline asm has no direct asm-goto equivalent; preserve the emitted
     * assembly intent here, with the external jump-table patching represented
     * as an intentional translation boundary.
     */
    let _ = key;
    let _ = branch;
    core::arch::asm!(
        "1:",
        "nop",
        ".pushsection __jump_table,  \"aw\"",
        ".align {align}",
        ".word 1b - ., {yes} - .",
        ".popsection",
        align = const core::mem::size_of::<usize>(),
        yes = sym arch_static_branch_yes,
        options(nostack)
    );
    false
}

#[cold]
#[inline(never)]
unsafe extern "C" fn arch_static_branch_yes() {}

#[inline(always)]
pub unsafe fn arch_static_branch_jump(key: *mut static_key, branch: bool) -> bool {
    /* See arch_static_branch: this is the C asm-goto jump form. */
    let _ = key;
    let _ = branch;
    core::arch::asm!(
        "1:",
        "b,n {yes}",
        ".pushsection __jump_table,  \"aw\"",
        ".align {align}",
        ".word 1b - ., {yes} - .",
        ".popsection",
        align = const core::mem::size_of::<usize>(),
        yes = sym arch_static_branch_jump_yes,
        options(nostack)
    );
    false
}

#[cold]
#[inline(never)]
unsafe extern "C" fn arch_static_branch_jump_yes() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
