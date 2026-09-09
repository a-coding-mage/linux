/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2012 ARM Ltd.
 */

// Translated from the C header. Dependencies supplied by the surrounding
// kernel translation are intentionally referenced but not implemented here.

/*
 * Aarch64 has flags for masking: Debug, Asynchronous (serror), Interrupts and
 * FIQ exceptions, in the 'daif' register. We mask and unmask them in 'daif'
 * order:
 * Masking debug exceptions causes all other exceptions to be masked too/
 * Masking SError masks IRQ/FIQ, but not debug exceptions. IRQ and FIQ are
 * always masked and unmasked together, and have no side effects for other
 * flags. Keeping to this order makes it easier for entry.S to know which
 * exceptions should be unmasked.
 */

#[inline(always)]
pub unsafe fn __daif_local_irq_enable() {
    barrier();
    core::arch::asm!("msr daifclr, #3");
    barrier();
}

#[inline(always)]
pub unsafe fn __pmr_local_irq_enable() {
    if IS_ENABLED(CONFIG_ARM64_DEBUG_PRIORITY_MASKING) {
        let pmr: u32 = read_sysreg_s(SYS_ICC_PMR_EL1);
        WARN_ON_ONCE(pmr != GIC_PRIO_IRQON && pmr != GIC_PRIO_IRQOFF);
    }

    barrier();
    write_sysreg_s(GIC_PRIO_IRQON, SYS_ICC_PMR_EL1);
    pmr_sync();
    barrier();
}

#[inline(always)]
pub unsafe fn arch_local_irq_enable() {
    if system_uses_irq_prio_masking() {
        __pmr_local_irq_enable();
    } else {
        __daif_local_irq_enable();
    }
}

#[inline(always)]
pub unsafe fn __daif_local_irq_disable() {
    barrier();
    core::arch::asm!("msr daifset, #3");
    barrier();
}

#[inline(always)]
pub unsafe fn __pmr_local_irq_disable() {
    if IS_ENABLED(CONFIG_ARM64_DEBUG_PRIORITY_MASKING) {
        let pmr: u32 = read_sysreg_s(SYS_ICC_PMR_EL1);
        WARN_ON_ONCE(pmr != GIC_PRIO_IRQON && pmr != GIC_PRIO_IRQOFF);
    }

    barrier();
    write_sysreg_s(GIC_PRIO_IRQOFF, SYS_ICC_PMR_EL1);
    barrier();
}

#[inline(always)]
pub unsafe fn arch_local_irq_disable() {
    if system_uses_irq_prio_masking() {
        __pmr_local_irq_disable();
    } else {
        __daif_local_irq_disable();
    }
}

#[inline(always)]
pub unsafe fn __daif_local_save_flags() -> u64 {
    read_sysreg(daif)
}

#[inline(always)]
pub unsafe fn __pmr_local_save_flags() -> u64 {
    read_sysreg_s(SYS_ICC_PMR_EL1)
}

/*
 * Save the current interrupt enable state.
 */
#[inline(always)]
pub unsafe fn arch_local_save_flags() -> u64 {
    if system_uses_irq_prio_masking() {
        __pmr_local_save_flags()
    } else {
        __daif_local_save_flags()
    }
}

#[inline(always)]
pub unsafe fn __daif_irqs_disabled_flags(flags: u64) -> bool {
    (flags & PSR_I_BIT) != 0
}

#[inline(always)]
pub unsafe fn __pmr_irqs_disabled_flags(flags: u64) -> bool {
    flags != GIC_PRIO_IRQON
}

#[inline(always)]
pub unsafe fn arch_irqs_disabled_flags(flags: u64) -> bool {
    if system_uses_irq_prio_masking() {
        __pmr_irqs_disabled_flags(flags)
    } else {
        __daif_irqs_disabled_flags(flags)
    }
}

#[inline(always)]
pub unsafe fn __daif_irqs_disabled() -> bool {
    __daif_irqs_disabled_flags(__daif_local_save_flags())
}

#[inline(always)]
pub unsafe fn __pmr_irqs_disabled() -> bool {
    __pmr_irqs_disabled_flags(__pmr_local_save_flags())
}

#[inline(always)]
pub unsafe fn arch_irqs_disabled() -> bool {
    if system_uses_irq_prio_masking() {
        __pmr_irqs_disabled()
    } else {
        __daif_irqs_disabled()
    }
}

#[inline(always)]
pub unsafe fn __daif_local_irq_save() -> u64 {
    let flags = __daif_local_save_flags();

    __daif_local_irq_disable();

    flags
}

#[inline(always)]
pub unsafe fn __pmr_local_irq_save() -> u64 {
    let flags = __pmr_local_save_flags();

    /*
     * There are too many states with IRQs disabled, just keep the current
     * state if interrupts are already disabled/masked.
     */
    if !__pmr_irqs_disabled_flags(flags) {
        __pmr_local_irq_disable();
    }

    flags
}

#[inline(always)]
pub unsafe fn arch_local_irq_save() -> u64 {
    if system_uses_irq_prio_masking() {
        __pmr_local_irq_save()
    } else {
        __daif_local_irq_save()
    }
}

#[inline(always)]
pub unsafe fn __daif_local_irq_restore(flags: u64) {
    barrier();
    write_sysreg(flags, daif);
    barrier();
}

#[inline(always)]
pub unsafe fn __pmr_local_irq_restore(flags: u64) {
    barrier();
    write_sysreg_s(flags, SYS_ICC_PMR_EL1);
    pmr_sync();
    barrier();
}

/*
 * restore saved IRQ state
 */
#[inline(always)]
pub unsafe fn arch_local_irq_restore(flags: u64) {
    if system_uses_irq_prio_masking() {
        __pmr_local_irq_restore(flags);
    } else {
        __daif_local_irq_restore(flags);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
