/* SPDX-License-Identifier: GPL-2.0 */

// Translated from parisc/include/asm/irqflags.h.
// The C header's included definitions are supplied by the surrounding build.

use core::arch::asm;

// Supplied by asm/psw.h in the original header.
// `PSW_I` is intentionally referenced as an external dependency.

#[inline]
pub fn arch_local_save_flags() -> usize {
    let flags: usize;
    unsafe {
        asm!("ssm 0, {flags}", flags = out(reg) flags, options(nostack));
    }
    flags
}

#[inline]
pub fn arch_local_irq_disable() {
    unsafe {
        asm!("rsm {psw_i}, r0", psw_i = const PSW_I, options(nostack));
    }
}

#[inline]
pub fn arch_local_irq_enable() {
    unsafe {
        asm!("ssm {psw_i}, r0", psw_i = const PSW_I, options(nostack));
    }
}

#[inline]
pub fn arch_local_irq_save() -> usize {
    let flags: usize;
    unsafe {
        asm!("rsm {psw_i}, {flags}",
            psw_i = const PSW_I,
            flags = out(reg) flags,
            options(nostack));
    }
    flags
}

#[inline]
pub fn arch_local_irq_restore(flags: usize) {
    // warn if IRQs are on although they should be off
    // CONFIG_LIGHTWEIGHT_SPINLOCK_CHECK corresponds to this conditional build option.
    #[cfg(feature = "CONFIG_LIGHTWEIGHT_SPINLOCK_CHECK")]
    if arch_local_save_flags() & PSW_I != 0 {
        unsafe {
            asm!("break 6,6", options(nostack)); // SPINLOCK_BREAK_INSN
        }
    }

    unsafe {
        asm!("mtsm {flags}", flags = in(reg) flags, options(nostack));
    }
}

#[inline]
pub fn arch_irqs_disabled_flags(flags: usize) -> bool {
    (flags & PSW_I) == 0
}

#[inline]
pub fn arch_irqs_disabled() -> bool {
    arch_irqs_disabled_flags(arch_local_save_flags())
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
