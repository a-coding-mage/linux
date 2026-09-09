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

// Dependency supplied by the corresponding low-level header.
use crate::PIL_NORMAL_MAX;

#[inline(always)]
pub unsafe fn arch_local_save_flags() -> u64 {
    let flags: u64;

    core::arch::asm!(
        "rdpr %pil, {flags}",
        flags = out(reg) flags,
        options(nostack, preserves_flags),
    );

    flags
}

#[inline(always)]
pub unsafe fn arch_local_irq_restore(flags: u64) {
    core::arch::asm!(
        "wrpr {flags}, %pil",
        flags = in(reg) flags,
        options(nostack),
    );
}

#[inline(always)]
pub unsafe fn arch_local_irq_disable() {
    core::arch::asm!(
        "wrpr {max}, %pil",
        max = const PIL_NORMAL_MAX,
        options(nostack),
    );
}

#[inline(always)]
pub unsafe fn arch_local_irq_enable() {
    core::arch::asm!(
        "wrpr 0, %pil",
        options(nostack),
    );
}

#[inline(always)]
pub fn arch_irqs_disabled_flags(flags: u64) -> i32 {
    (flags > 0) as i32
}

#[inline(always)]
pub unsafe fn arch_irqs_disabled() -> i32 {
    arch_irqs_disabled_flags(arch_local_save_flags())
}

#[inline(always)]
pub unsafe fn arch_local_irq_save() -> u64 {
    let flags: u64;
    let tmp: u64;

    /* Disable interrupts to PIL_NORMAL_MAX unless we already
     * are using PIL_NMI, in which case PIL_NMI is retained.
     *
     * The only values we ever program into the %pil are 0,
     * PIL_NORMAL_MAX and PIL_NMI.
     *
     * Since PIL_NMI is the largest %pil value and all bits are
     * set in it (0xf), it doesn't matter what PIL_NORMAL_MAX
     * actually is.
     */
    core::arch::asm!(
        "rdpr %pil, {flags}",
        "or {flags}, {max}, {tmp}",
        "wrpr {tmp}, 0x0, %pil",
        flags = out(reg) flags,
        tmp = out(reg) tmp,
        max = const PIL_NORMAL_MAX,
        options(nostack),
    );

    flags
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
