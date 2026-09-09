/* SPDX-License-Identifier: GPL-2.0 */

// C dependency: linux/stringify.h
// C dependency: linux/types.h

pub const JUMP_LABEL_NOP_SIZE: usize = 4;

/*
 * NOTE about '.balign 4':
 *
 * To make atomic update of patched instruction available we need to guarantee
 * that this instruction doesn't cross L1 cache line boundary.
 *
 * As of today we simply align instruction which can be patched by 4 byte using
 * ".balign 4" directive. In that case patched instruction is aligned with one
 * 16-bit NOP_S if this is required.
 * However 'align by 4' directive is much stricter than it actually required.
 * It's enough that our 32-bit instruction don't cross L1 cache line boundary /
 * L1 I$ fetch block boundary which can be achieved by using
 * ".bundle_align_mode" assembler directive. That will save us from adding
 * useless NOP_S padding in most of the cases.
 *
 * TODO: switch to ".bundle_align_mode" directive using whin it will be
 * supported by ARC toolchain.
 */

// `asm goto` has no direct stable Rust equivalent.  The original ARC
// instruction sequence and jump-table operation are retained here as the
// file-local semantic contract; the external `static_key` type is supplied by
// the surrounding kernel translation.
#[allow(unused_variables)]
pub unsafe fn arch_static_branch(key: *mut static_key, branch: bool) -> bool {
    // asm goto(".balign " __stringify(JUMP_LABEL_NOP_SIZE) "\n"
    //          "1:\n"
    //          "nop\n"
    //          ".pushsection __jump_table, \"aw\"\n"
    //          ".word 1b, %l[l_yes], %c0\n"
    //          ".popsection\n"
    //          : : "i" (&((char *)key)[branch]) : : l_yes);
    false
    // l_yes: true
}

#[allow(unused_variables)]
pub unsafe fn arch_static_branch_jump(key: *mut static_key, branch: bool) -> bool {
    // asm goto(".balign " __stringify(JUMP_LABEL_NOP_SIZE) "\n"
    //          "1:\n"
    //          "b %l[l_yes]\n"
    //          ".pushsection __jump_table, \"aw\"\n"
    //          ".word 1b, %l[l_yes], %c0\n"
    //          ".popsection\n"
    //          : : "i" (&((char *)key)[branch]) : : l_yes);
    false
    // l_yes: true
}

pub type jump_label_t = u32;

#[repr(C)]
pub struct jump_entry {
    pub code: jump_label_t,
    pub target: jump_label_t,
    pub key: jump_label_t,
}

// External dependency supplied by the surrounding kernel translation.
pub enum static_key {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
