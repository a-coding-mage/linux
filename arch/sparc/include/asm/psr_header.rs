/* SPDX-License-Identifier: GPL-2.0 */
/*
 * psr.h: This file holds the macros for masking off various parts of
 *        the processor status register on the Sparc. This is valid
 *        for Version 8. On the V9 this is renamed to the PSTATE
 *        register and its members are accessed as fields like
 *        PSTATE.PRIV for the current CPU privilege level.
 *
 * Copyright (C) 1994 David S. Miller (davem@caip.rutgers.edu)
 */

// Dependency supplied by <uapi/asm/psr.h> in the C header is intentionally
// left to the surrounding translation unit.

#[cfg(not(target_arch = "sparc"))]
compile_error!("psr_header.rs requires the SPARC target");

/* Get the %psr register. */
#[inline]
pub unsafe fn get_psr() -> u32 {
    let psr: u32;
    core::arch::asm!(
        "rd %psr, {psr}\n\t",
        "nop\n\t",
        "nop\n\t",
        "nop\n\t",
        psr = out(reg) psr,
        options(nostack)
    );
    psr
}

#[inline]
pub unsafe fn put_psr(new_psr: u32) {
    core::arch::asm!(
        "wr {new_psr}, 0x0, %psr\n\t",
        "nop\n\t",
        "nop\n\t",
        "nop\n\t",
        new_psr = in(reg) new_psr,
        options(nostack)
    );
}

/* Get the %fsr register.  Be careful, make sure the floating point
 * enable bit is set in the %psr when you execute this or you will
 * incur a trap.
 */

extern "C" {
    pub static mut fsr_storage: u32;
}

#[inline]
pub unsafe fn get_fsr() -> u32 {
    let mut fsr: u32 = 0;

    core::arch::asm!(
        "st %fsr, [{storage}]\n\t",
        "ld [{storage}], {fsr}\n\t",
        storage = in(reg) (&raw mut fsr_storage),
        fsr = out(reg) fsr,
        options(nostack)
    );

    fsr
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
