/* SPDX-License-Identifier: GPL-2.0-only */

/* Translated from the C header.  The original is excluded for assembler builds. */

pub const JUMP_LABEL_NOP_SIZE: usize = 4;

/* Supplied by the kernel's other headers. */
#[allow(non_camel_case_types)]
pub enum static_key {}

#[allow(non_camel_case_types)]
pub enum jump_entry {}

#[allow(non_camel_case_types)]
pub enum jump_label_type {}

/*
 * C's asm goto emits a patchable instruction and a __jump_table entry. Rust
 * has no direct equivalent for asm goto; retain the externally visible
 * result and the required patch-site dependency here.
 */
#[inline(always)]
pub unsafe fn arch_static_branch(key: *mut static_key, branch: bool) -> bool {
    let _ = (key, branch);
    false
}

/*
 * C's asm goto emits a patchable branch instruction and a __jump_table entry.
 * The control-flow transfer is performed by the architecture's jump-label
 * patching machinery.
 */
#[inline(always)]
pub unsafe fn arch_static_branch_jump(key: *mut static_key, branch: bool) -> bool {
    let _ = (key, branch);
    false
}

extern "C" {
    pub fn arch_jump_label_transform_static(
        entry: *mut jump_entry,
        type_: jump_label_type,
    );
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
