/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright 2012 Calxeda, Inc.
 */

// C dependency: <asm/insn.h>

use core::arch::asm;

// C: register unsigned long current_stack_pointer asm ("sp");
// The stack-pointer register is referenced directly by the inline assembly below.

#[cfg(CONFIG_SMP)]
#[inline]
pub unsafe fn set_my_cpu_offset(off: usize) {
    unsafe extern "C" {
        static mut smp_on_up: u32;
    }

    // C: if (IS_ENABLED(CONFIG_CPU_V6) && !smp_on_up) return;
    #[cfg(CONFIG_CPU_V6)]
    if !smp_on_up {
        return;
    }

    // Set TPIDRPRW.
    asm!("mcr p15, 0, {0}, c13, c0, 4", in(reg) off, options(nostack));
}

#[cfg(CONFIG_SMP)]
#[inline(always)]
pub unsafe fn __my_cpu_offset() -> usize {
    let mut off: usize;

    /*
     * Read TPIDRPRW.
     * We want to allow caching the value, so avoid using volatile and
     * instead use a fake stack read to hazard against barrier().
     *
     * The following ARM alternatives and relocation sequences are retained
     * as inline assembly; their availability is controlled by the same build
     * configuration as the original C header.
     */
    #[cfg(CONFIG_CPU_V6)]
    asm!(
        "0: mrc p15, 0, {off}, c13, c0, 4",
        "1:",
        ".subsection 1",
        "2: ldr {off}, 3f",
        "   ldr {off}, [{off}]",
        "   b 1b",
        "3: .long __per_cpu_offset",
        ".previous",
        ".pushsection \".alt.smp.init\", \"a\"",
        ".long 0b - .",
        "b . + (2b - 0b)",
        ".popsection",
        off = out(reg) off,
        in("sp") 0usize,
        options(nostack)
    );

    #[cfg(not(CONFIG_CPU_V6))]
    asm!(
        "mrc p15, 0, {off}, c13, c0, 4",
        off = out(reg) off,
        in("sp") 0usize,
        options(nostack)
    );

    off
}

// C macro: #define __my_cpu_offset __my_cpu_offset()

#[cfg(not(CONFIG_SMP))]
#[inline(always)]
pub unsafe fn set_my_cpu_offset(_x: usize) {}

// Dependency: <asm-generic/percpu.h>

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
