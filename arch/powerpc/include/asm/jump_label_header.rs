/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright 2010 Michael Ellerman, IBM Corp.
 */

// C dependencies supplied by the surrounding translation unit:
// linux/types.h, asm/feature-fixups.h, and asm/asm-const.h.

pub const JUMP_ENTRY_TYPE: &str = "FTR_ENTRY_LONG";
pub const JUMP_LABEL_NOP_SIZE: usize = 4;

#[macro_export]
macro_rules! jump_table_entry {
    ($key:expr, $label:expr) => {
        concat!(
            ".pushsection __jump_table,  \"aw\"\n\t",
            ".long 1b - ., ", $label, " - .\n\t",
            $key, " - .\n\t",
            ".popsection                 \n\t"
        )
    };
}

#[macro_export]
macro_rules! arch_static_branch_asm {
    ($key:expr, $label:expr) => {
        concat!("1:\tnop\n\t", jump_table_entry!($key, $label))
    };
}

// The C asm-goto statements have no direct stable Rust equivalent.  The
// inline assembly and its local label are preserved as a source-level note;
// the external static_key type is supplied by the surrounding translation.
#[inline(always)]
pub unsafe fn arch_static_branch(
    key: *mut static_key,
    branch: bool,
) -> bool {
    let _ = (key, branch);
    // asm goto(ARCH_STATIC_BRANCH_ASM("%c0", "%l[l_yes]"));
    false
}

#[inline(always)]
pub unsafe fn arch_static_branch_jump(
    key: *mut static_key,
    branch: bool,
) -> bool {
    let _ = (key, branch);
    // asm goto("1:\n\tb %l[l_yes] # arch_static_branch_jump\n\t"
    //          JUMP_TABLE_ENTRY("%c0", "%l[l_yes]") : : "i"
    //          (&((char *)key)[branch]) : : l_yes);
    false
}

// Declaration supplied by the translated Linux static-key definitions.
pub enum static_key {}

// Assembler-only form (the C preprocessor branch under __ASSEMBLER__).
#[macro_export]
macro_rules! arch_static_branch {
    ($label:expr, $key:expr) => {
        concat!(
            "1098:\tnop;\n",
            ".pushsection __jump_table, \"aw\";\n",
            ".long 1098b - ., ", $label, " - .;\n",
            "FTR_ENTRY_LONG ", $key, " - .;\n",
            ".popsection"
        )
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
