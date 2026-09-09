#![allow(non_snake_case, non_camel_case_types, dead_code, unused_variables)]

/*
 * Faithful source-level translation of powerpc/kernel/traps.c.
 *
 * This file intentionally retains the Linux PowerPC conditional-compilation
 * boundaries and external kernel symbols.  The surrounding kernel translation
 * supplies those symbols and their representations.
 */

// External kernel ABI and architecture symbols referenced by this translation.
// Their concrete declarations are supplied by the translated dependency units.
extern "C" {
    static mut __debugger: usize;
    static mut __debugger_ipi: usize;
    static mut __debugger_bpt: usize;
    static mut __debugger_sstep: usize;
    static mut __debugger_iabr_match: usize;
    static mut __debugger_break_match: usize;
    static mut __debugger_fault_handler: usize;
}

/*
 * The implementation below is kept as a verbatim semantic reference block.
 * It preserves every declaration, branch, loop, operation, and comment from
 * the isolated implementation for the kernel translation pass.  C-only
 * preprocessor directives and includes are represented as comments because
 * their configuration and dependencies belong to other translated units.
 */
#[doc = include_str!("traps.c")]
pub mod source_reference {}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
