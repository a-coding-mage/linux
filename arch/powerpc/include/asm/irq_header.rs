/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Translated from the __KERNEL__ portion of the C header. */

/* The original header depends on linux and asm declarations supplied elsewhere. */

extern "C" {
    pub static mut ppc_n_lost_interrupts: atomic_t;
}

/* Total number of virq in the platform */
pub const NR_IRQS: usize = CONFIG_NR_IRQS;

/* Number of irqs reserved for a legacy isa controller */
pub const NR_IRQS_LEGACY: usize = 16;

extern "C" {
    pub fn virq_to_hw(virq: ::core::ffi::c_uint) -> irq_hw_number_t;
}

#[inline]
pub unsafe fn irq_canonicalize(irq: ::core::ffi::c_int) -> ::core::ffi::c_int {
    irq
}

extern "C" {
    pub static mut distribute_irqs: ::core::ffi::c_int;
}

#[cfg(CONFIG_BOOKE)]
extern "C" {
    /*
     * Per-cpu stacks for handling critical, debug and machine check
     * level interrupts.
     */
    pub static mut critirq_ctx: [*mut ::core::ffi::c_void; NR_CPUS];
    pub static mut dbgirq_ctx: [*mut ::core::ffi::c_void; NR_CPUS];
    pub static mut mcheckirq_ctx: [*mut ::core::ffi::c_void; NR_CPUS];
}

/*
 * Per-cpu stacks for handling hard and soft interrupts.
 */
extern "C" {
    pub static mut hardirq_ctx: [*mut ::core::ffi::c_void; NR_CPUS];
    pub static mut softirq_ctx: [*mut ::core::ffi::c_void; NR_CPUS];
}

extern "C" {
    pub fn __do_IRQ(regs: *mut pt_regs);
    pub fn irq_choose_cpu(mask: *const cpumask) -> ::core::ffi::c_int;
}

#[cfg(all(CONFIG_PPC_BOOK3S_64, CONFIG_NMI_IPI))]
extern "C" {
    pub fn arch_trigger_cpumask_backtrace(
        mask: *const cpumask_t,
        exclude_cpu: ::core::ffi::c_int,
    );
}

/* The C macro re-declares arch_trigger_cpumask_backtrace with the same name. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
