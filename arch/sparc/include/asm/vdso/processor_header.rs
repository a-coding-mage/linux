/* SPDX-License-Identifier: GPL-2.0 */

/* Translated from asm/vdso/processor.h.  The compiler barrier is supplied by
 * the surrounding platform/compiler support. */

#[cfg(__arch64__)]
/* Please see the commentary in asm/backoff.h for a description of
 * what these instructions are doing and how they have been chosen.
 * To make a long story short, we are trying to yield the current cpu
 * strand during busy loops.
 */
#[cfg(BUILD_VDSO)]
macro_rules! cpu_relax {
    () => {{
        unsafe {
            core::arch::asm!(
                "99:",
                "rd %ccr, %g0",
                "rd %ccr, %g0",
                "rd %ccr, %g0",
                options(nostack, preserves_flags)
            );
        }
    }};
}

#[cfg(all(__arch64__, not(BUILD_VDSO)))]
macro_rules! cpu_relax {
    () => {{
        unsafe {
            core::arch::asm!(
                "99:",
                "rd %ccr, %g0",
                "rd %ccr, %g0",
                "rd %ccr, %g0",
                ".section .pause_3insn_patch,\"ax\"",
                ".word 99b",
                "wr %g0, 128, %asr27",
                "nop",
                "nop",
                ".previous",
                options(nostack, preserves_flags)
            );
        }
    }};
}

#[cfg(not(__arch64__))]
macro_rules! cpu_relax {
    () => {{
        barrier();
    }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
