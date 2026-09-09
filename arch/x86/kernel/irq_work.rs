// SPDX-License-Identifier: GPL-2.0
/*
 * x86 specific code for irq_work
 *
 * Copyright (C) 2010 Red Hat, Inc., Peter Zijlstra
 */

// Dependencies supplied by the surrounding kernel translation.
extern "C" {
    fn apic_eoi();
    fn trace_irq_work_entry(vector: u32);
    fn inc_irq_stat(stat: u32);
    fn irq_work_run();
    fn trace_irq_work_exit(vector: u32);
    fn arch_irq_work_has_interrupt() -> bool;
    fn __apic_send_IPI_self(vector: u32);
    fn apic_wait_icr_idle();

    static IRQ_WORK_VECTOR: u32;
    static IRQ_WORK: u32;
}

// Corresponds to CONFIG_X86_LOCAL_APIC.
#[cfg(CONFIG_X86_LOCAL_APIC)]
pub unsafe extern "C" fn sysvec_irq_work() {
    apic_eoi();
    trace_irq_work_entry(IRQ_WORK_VECTOR);
    inc_irq_stat(IRQ_WORK);
    irq_work_run();
    trace_irq_work_exit(IRQ_WORK_VECTOR);
}

#[cfg(CONFIG_X86_LOCAL_APIC)]
pub unsafe extern "C" fn arch_irq_work_raise() {
    if !arch_irq_work_has_interrupt() {
        return;
    }

    __apic_send_IPI_self(IRQ_WORK_VECTOR);
    apic_wait_icr_idle();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
