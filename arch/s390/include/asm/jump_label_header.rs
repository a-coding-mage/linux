/* SPDX-License-Identifier: GPL-2.0 */

// HAVE_JUMP_LABEL_BATCH

// The following declarations are available from the Linux type headers.
// They are intentionally not defined here.
// use linux_types::{bool, static_key};

pub const JUMP_LABEL_NOP_SIZE: usize = 6;

// Build-time compiler condition preserved from the C header:
// #ifdef CONFIG_CC_IS_CLANG
// const JUMP_LABEL_STATIC_KEY_CONSTRAINT: &str = "i";
// #elif __GNUC__ < 9
// const JUMP_LABEL_STATIC_KEY_CONSTRAINT: &str = "X";
// #else
// const JUMP_LABEL_STATIC_KEY_CONSTRAINT: &str = "jdd";
// #endif

// The assembler strings below are used by the architecture's jump-label
// implementation.  They are retained as Rust constants in place of C macros.
pub const ARCH_JUMP_TABLE_ENTRY_TEMPLATE: &str =
    ".pushsection __jump_table,\"aw\"\n"
    ".balign\t8\n"
    ".long\t{local_label}-. ,{label}-.\n"
    ".quad\t{key}-.\n"
    ".popsection\n";

/*
 * We use a brcl 0,<offset> instruction for jump labels so it
 * can be easily distinguished from a hotpatch generated instruction.
 */
pub const ARCH_STATIC_BRANCH_ASM_TEMPLATE: &str =
    "0:\tbrcl 0,{label}\n";

pub const ARCH_STATIC_BRANCH_JUMP_ASM_TEMPLATE: &str =
    "0:\tbrcl 15,{label}\n";

// C's `asm goto` has no direct stable Rust equivalent.  These functions keep
// the externally visible interface and control-flow intent; the architecture
// assembler integration must supply the equivalent jump-table asm.
#[inline(always)]
pub unsafe fn arch_static_branch(key: *mut static_key, branch: bool) -> bool {
    let _ = (key, branch);
    // asm goto(ARCH_STATIC_BRANCH_ASM("%0+%1", "%l[label]")
    //     : : JUMP_LABEL_STATIC_KEY_CONSTRAINT (key), "i" (branch) : : label);
    // return false;
    // label:
    // return true;
    todo!("s390 asm-goto jump-label implementation")
}

#[inline(always)]
pub unsafe fn arch_static_branch_jump(key: *mut static_key, branch: bool) -> bool {
    let _ = (key, branch);
    // asm goto(ARCH_STATIC_BRANCH_JUMP_ASM("%0+%1", "%l[label]")
    //     : : JUMP_LABEL_STATIC_KEY_CONSTRAINT (key), "i" (branch) : : label);
    // return false;
    // label:
    // return true;
    todo!("s390 asm-goto jump-label implementation")
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
