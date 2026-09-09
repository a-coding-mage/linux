/* SPDX-License-Identifier: GPL-2.0 */

/*
 * The exception table consists of pairs of relative addresses: the first is
 * the address of an instruction that is allowed to fault, and the second is
 * the address at which the program should continue.  No registers are
 * modified, so it is entirely up to the continuation code to figure out what
 * to do.
 *
 * All the routines below use bits of fixup code that are out of line with the
 * main instruction path.  This means when everything is well, we don't even
 * have to jump over them.  Further, they do not intrude on our cache or tlb
 * entries.
 */

/* C preprocessor marker: ARCH_HAS_RELATIVE_EXTABLE. */
pub const ARCH_HAS_RELATIVE_EXTABLE: bool = true;

#[repr(C)]
pub struct exception_table_entry {
    pub insn: i32,
    pub fixup: i32,
}

pub unsafe fn extable_fixup(x: *const exception_table_entry) -> usize {
    // C semantics: (unsigned long)&x->fixup + x->fixup.
    (core::ptr::addr_of!((*x).fixup) as usize)
        .wrapping_add((*x).fixup as usize)
}

/*
 * Helper macro for exception table entries.
 *
 * The C macro emits assembler directives in the __ex_table section:
 *   .section __ex_table,"a"; .balign 4;
 *   .long (_fault) - .; .long (_target) - .; .previous
 *
 * Rust's inline-assembly interface cannot express the C stringification and
 * assembler-label substitution portably, so the assembler intent is retained
 * here for the target-specific assembly layer.
 */
#[macro_export]
macro_rules! EX_TABLE {
    ($fault:expr, $target:expr) => {
        /* __ex_table: relative addresses of $fault and $target */
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
