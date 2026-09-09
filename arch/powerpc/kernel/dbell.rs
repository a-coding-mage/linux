// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Author: Kumar Gala <galak@kernel.crashing.org>
 *
 * Copyright 2009 Freescale Semiconductor Inc.
 */

// CONFIG_SMP is a build-time configuration condition from the C source.

#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}

extern "C" {
    fn set_irq_regs(regs: *mut pt_regs) -> *mut pt_regs;
    fn trace_doorbell_entry(regs: *mut pt_regs);
    fn ppc_msgsync();
    fn should_hard_irq_enable(regs: *mut pt_regs) -> bool;
    fn do_hard_irq_enable();
    fn kvmppc_clear_host_ipi(cpu: i32);
    fn smp_processor_id() -> i32;
    fn smp_ipi_demux_relaxed();
    fn trace_doorbell_exit(regs: *mut pt_regs);
    static KERN_WARNING: *const u8;
    fn printk(fmt: *const u8, ...);
}

#[cfg(feature = "CONFIG_SMP")]
pub unsafe extern "C" fn doorbell_exception(regs: *mut pt_regs) {
    let old_regs: *mut pt_regs = set_irq_regs(regs);

    trace_doorbell_entry(regs);

    ppc_msgsync();

    if should_hard_irq_enable(regs) {
        do_hard_irq_enable();
    }

    kvmppc_clear_host_ipi(smp_processor_id());
    // Equivalent of the C per-CPU increment: __this_cpu_inc(irq_stat.doorbell_irqs).
    __this_cpu_inc!(irq_stat.doorbell_irqs);

    smp_ipi_demux_relaxed(); // already performed the barrier

    trace_doorbell_exit(regs);

    set_irq_regs(old_regs);
}

#[cfg(not(feature = "CONFIG_SMP"))]
pub unsafe extern "C" fn doorbell_exception(_regs: *mut pt_regs) {
    printk(KERN_WARNING, b"Received doorbell on non-smp system\0".as_ptr());
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
