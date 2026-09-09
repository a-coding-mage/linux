/* SPDX-License-Identifier: GPL-2.0 */
/*
 * include/asm/irqflags.h
 *
 * IRQ flags handling
 *
 * This file gets included from lowlevel asm headers too, to provide
 * wrapped versions of the local_irq_*() APIs, based on the
 * arch_local_irq_*() functions from the lowlevel headers.
 */

// The C header guard and __ASSEMBLER__ conditional are represented by this
// Rust source file being compiled only for the non-assembler interface.

use core::ffi::c_ulong;

extern "C" {
    pub fn arch_local_irq_restore(flags: c_ulong);
    pub fn arch_local_irq_save() -> c_ulong;
    pub fn arch_local_irq_enable();
}

#[inline]
pub unsafe fn arch_local_save_flags() -> c_ulong {
    let flags: c_ulong;

    core::arch::asm!("rd %psr, {0}", out(reg) flags);
    flags
}

#[inline]
pub unsafe fn arch_local_irq_disable() {
    arch_local_irq_save();
}

#[inline]
pub unsafe fn arch_irqs_disabled_flags(flags: c_ulong) -> bool {
    (flags & PSR_PIL) != 0
}

#[inline]
pub unsafe fn arch_irqs_disabled() -> bool {
    arch_irqs_disabled_flags(arch_local_save_flags())
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
