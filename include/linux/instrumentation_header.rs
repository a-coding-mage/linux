/* SPDX-License-Identifier: GPL-2.0 */

/*
 * C header translation.  The CONFIG_NOINSTR_VALIDATION configuration is
 * represented by the Rust cfg feature of the same name.  The objtool and
 * stringify annotations are build-time external dependencies and therefore
 * remain described by the inline-assembly comments below.
 */

#[cfg(feature = "CONFIG_NOINSTR_VALIDATION")]
#[inline(always)]
pub unsafe fn __instrumentation_begin(_c: usize) {
    // C: asm volatile(__stringify(c) ": nop\n\t" ANNOTATE_INSTR_BEGIN(...));
    core::arch::asm!("nop", options(nostack, preserves_flags));
}

#[cfg(feature = "CONFIG_NOINSTR_VALIDATION")]
#[inline(always)]
pub unsafe fn __instrumentation_end(_c: usize) {
    // C: asm volatile(__stringify(c) ": nop\n\t" ANNOTATE_INSTR_END(...));
    core::arch::asm!("nop", options(nostack, preserves_flags));
}

/*
 * instrumentation_{begin,end}() may nest.  Objtool treats begin as +1 and
 * end as -1 and sums the values over instructions.  The end marker is a NOP
 * so that an end at the end of a conditional block remains in that block.
 *
 * The C __COUNTER__ operand is a compiler-generated annotation counter; Rust
 * has no direct equivalent, so the translated operations retain the same
 * ordering and side effect while passing a file-local marker value.
 */
#[cfg(feature = "CONFIG_NOINSTR_VALIDATION")]
#[macro_export]
macro_rules! instrumentation_begin {
    () => {{
        unsafe { $crate::__instrumentation_begin(0usize) }
    }};
}

#[cfg(feature = "CONFIG_NOINSTR_VALIDATION")]
#[macro_export]
macro_rules! instrumentation_end {
    () => {{
        unsafe { $crate::__instrumentation_end(0usize) }
    }};
}

#[cfg(not(feature = "CONFIG_NOINSTR_VALIDATION"))]
#[macro_export]
macro_rules! instrumentation_begin {
    () => {{
        // C: do { } while (0)
    }};
}

#[cfg(not(feature = "CONFIG_NOINSTR_VALIDATION"))]
#[macro_export]
macro_rules! instrumentation_end {
    () => {{
        // C: do { } while (0)
    }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
