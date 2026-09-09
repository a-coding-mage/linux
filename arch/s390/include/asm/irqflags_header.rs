/* SPDX-License-Identifier: GPL-2.0 */
/*
 *    Copyright IBM Corp. 2006, 2010
 *    Author(s): Martin Schwidefsky <schwidefsky@de.ibm.com>
 */

// The original header includes linux/types.h for these target definitions.

pub const ARCH_IRQ_ENABLED: usize = 3usize << (usize::BITS - 8);

/* store then OR system mask. */
#[inline(always)]
pub unsafe fn __arch_local_irq_stosm(or_mask: u8) -> usize {
    let mut mask: usize;
    core::arch::asm!(
        "stosm {mask}, {or_mask}",
        mask = lateout(reg) mask,
        or_mask = const or_mask,
        options(nostack)
    );
    mask
}

/* store then AND system mask. */
#[inline(always)]
pub unsafe fn __arch_local_irq_stnsm(and_mask: u8) -> usize {
    let mut mask: usize;
    core::arch::asm!(
        "stnsm {mask}, {and_mask}",
        mask = lateout(reg) mask,
        and_mask = const and_mask,
        options(nostack)
    );
    mask
}

/* set system mask. */
#[inline(always)]
pub unsafe fn __arch_local_irq_ssm(flags: usize) {
    core::arch::asm!("ssm {flags}", flags = in(reg) flags, options(nostack));
}

// Under CONFIG_KMSAN (except in the decompressor), these are externally
// provided declarations.  Otherwise the original macros alias them to the
// architecture-local implementations below.
#[cfg(all(feature = "CONFIG_KMSAN", not(feature = "__DECOMPRESSOR")))]
unsafe extern "C" {
    pub fn arch_local_save_flags() -> usize;
    pub fn arch_local_irq_save() -> usize;
    pub fn arch_local_irq_enable_external();
    pub fn arch_local_irq_enable();
}

#[inline(always)]
pub unsafe fn __arch_local_save_flags() -> usize {
    __arch_local_irq_stnsm(0xff)
}

#[inline(always)]
pub unsafe fn __arch_local_irq_save() -> usize {
    __arch_local_irq_stnsm(0xfc)
}

#[cfg(any(not(feature = "CONFIG_KMSAN"), feature = "__DECOMPRESSOR"))]
#[inline(always)]
pub unsafe fn arch_local_save_flags() -> usize {
    __arch_local_save_flags()
}

#[cfg(any(not(feature = "CONFIG_KMSAN"), feature = "__DECOMPRESSOR"))]
#[inline(always)]
pub unsafe fn arch_local_irq_save() -> usize {
    __arch_local_irq_save()
}

#[inline(always)]
pub unsafe fn arch_local_irq_disable() {
    arch_local_irq_save();
}

#[inline(always)]
pub unsafe fn __arch_local_irq_enable_external() {
    __arch_local_irq_stosm(0x01);
}

#[inline(always)]
pub unsafe fn __arch_local_irq_enable() {
    __arch_local_irq_stosm(0x03);
}

#[cfg(any(not(feature = "CONFIG_KMSAN"), feature = "__DECOMPRESSOR"))]
#[inline(always)]
pub unsafe fn arch_local_irq_enable_external() {
    __arch_local_irq_enable_external();
}

#[cfg(any(not(feature = "CONFIG_KMSAN"), feature = "__DECOMPRESSOR"))]
#[inline(always)]
pub unsafe fn arch_local_irq_enable() {
    __arch_local_irq_enable();
}

/* This only restores external and I/O interrupt state */
#[inline(always)]
pub unsafe fn arch_local_irq_restore(flags: usize) {
    /* only disabled->disabled and disabled->enabled is valid */
    if flags & ARCH_IRQ_ENABLED != 0 {
        arch_local_irq_enable();
    }
}

#[inline(always)]
pub unsafe fn arch_irqs_disabled_flags(flags: usize) -> bool {
    flags & ARCH_IRQ_ENABLED == 0
}

#[inline(always)]
pub unsafe fn arch_irqs_disabled() -> bool {
    arch_irqs_disabled_flags(arch_local_save_flags())
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
