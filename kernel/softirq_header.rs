// SPDX-License-Identifier: GPL-2.0-only
// Tracing-aware local IRQ policy from irqflags.h. All functions are inline so
// trace_hardirqs_on/off observe the Rust owner callsite, not a C wrapper frame.
// The native boundary below these calls is architecture/PV IRQ operations only.

#[inline(always)]
unsafe fn lupos_sirq_irq_save() -> c_ulong {
    let flags = lupos_sirq_raw_irq_save();
    #[cfg(CONFIG_TRACE_IRQFLAGS)]
    if !lupos_sirq_arch_irqs_disabled_flags(flags) {
        trace_hardirqs_off();
    }
    flags
}
#[inline(always)]
unsafe fn lupos_sirq_raw_irq_restore(flags: c_ulong) {
    #[cfg(CONFIG_DEBUG_IRQFLAGS)]
    if !lupos_sirq_arch_irqs_disabled() {
        warn_bogus_irq_restore();
    }
    lupos_sirq_arch_irq_restore(flags);
}
#[inline(always)]
unsafe fn lupos_sirq_irq_restore(flags: c_ulong) {
    #[cfg(CONFIG_TRACE_IRQFLAGS)]
    if !lupos_sirq_arch_irqs_disabled_flags(flags) {
        trace_hardirqs_on();
    }
    lupos_sirq_raw_irq_restore(flags);
}
#[inline(always)]
unsafe fn lupos_sirq_irq_disable() {
    #[cfg(CONFIG_TRACE_IRQFLAGS)]
    let was_disabled = lupos_sirq_arch_irqs_disabled();
    lupos_sirq_arch_irq_disable();
    #[cfg(CONFIG_TRACE_IRQFLAGS)]
    if !was_disabled {
        trace_hardirqs_off();
    }
}
#[inline(always)]
unsafe fn lupos_sirq_irq_enable() {
    #[cfg(CONFIG_TRACE_IRQFLAGS)]
    trace_hardirqs_on();
    lupos_sirq_arch_irq_enable();
}

// Preserve the caller visible to the original scheduler's debug/preemption
// tracing functions. An extra C wrapper would change CALLER_ADDR0 there.
#[inline(always)]
unsafe fn lupos_sirq_preempt_add(value: c_uint) {
    #[cfg(any(CONFIG_DEBUG_PREEMPT, CONFIG_TRACE_PREEMPT_TOGGLE))]
    preempt_count_add(value as c_int);
    #[cfg(not(any(CONFIG_DEBUG_PREEMPT, CONFIG_TRACE_PREEMPT_TOGGLE)))]
    lupos_sirq_preempt_add_raw(value);
}
#[inline(always)]
unsafe fn lupos_sirq_preempt_sub(value: c_uint) {
    #[cfg(any(CONFIG_DEBUG_PREEMPT, CONFIG_TRACE_PREEMPT_TOGGLE))]
    preempt_count_sub(value as c_int);
    #[cfg(not(any(CONFIG_DEBUG_PREEMPT, CONFIG_TRACE_PREEMPT_TOGGLE)))]
    lupos_sirq_preempt_sub_raw(value);
}
#[inline(always)]
unsafe fn lupos_sirq_preempt_dec() {
    lupos_sirq_preempt_sub(1);
}
