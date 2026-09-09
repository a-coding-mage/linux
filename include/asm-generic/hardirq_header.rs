/* SPDX-License-Identifier: GPL-2.0 */
// Translated from asm-generic/hardirq.h.

// `____cacheline_aligned` is a build-configuration-dependent alignment
// attribute supplied by the surrounding kernel environment.
#[repr(C)]
pub struct irq_cpustat_t {
    pub __softirq_pending: ::core::ffi::c_uint,
    // ARCH_WANTS_NMI_IRQSTAT controls whether this member is present.
    #[cfg(ARCH_WANTS_NMI_IRQSTAT)]
    pub __nmi_count: ::core::ffi::c_uint,
}

// DECLARE_PER_CPU_ALIGNED(irq_cpustat_t, irq_stat)
unsafe extern "C" {
    pub static mut irq_stat: irq_cpustat_t;
}

// The C header includes linux/irq.h; declarations supplied by that header are
// intentionally left as external dependencies.

// The C definition is included only when ack_bad_irq is not already defined.
// Rust cfg/build integration should select the equivalent configuration.
#[cfg(not(ack_bad_irq))]
pub unsafe fn ack_bad_irq(irq: ::core::ffi::c_uint) {
    unsafe {
        printk(
            b"unexpected IRQ trap at vector %02x\n\0".as_ptr(),
            irq,
        );
    }
}

unsafe extern "C" {
    // Corresponds to the kernel printk(KERN_CRIT, ...); KERN_CRIT's prefix is
    // supplied by the external kernel environment.
    fn printk(format: *const u8, ...) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
