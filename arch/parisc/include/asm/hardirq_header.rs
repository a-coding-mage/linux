/* SPDX-License-Identifier: GPL-2.0 */
/* hardirq.h: PA-RISC hard IRQ support.
 *
 * Copyright (C) 2001 Matthew Wilcox <matthew@wil.cx>
 * Copyright (C) 2013 Helge Deller <deller@gmx.de>
 */

// Dependencies supplied by the surrounding kernel translation:
// linux/cache.h, linux/threads.h, and linux/irq.h

#[repr(C)]
pub struct irq_cpustat_t {
    pub __softirq_pending: ::core::ffi::c_uint,
    pub kernel_stack_usage: ::core::ffi::c_uint,
    pub irq_stack_usage: ::core::ffi::c_uint,
    // CONFIG_SMP is a build-time condition from the original header.
    #[cfg(feature = "CONFIG_SMP")]
    pub irq_resched_count: ::core::ffi::c_uint,
    #[cfg(feature = "CONFIG_SMP")]
    pub irq_call_count: ::core::ffi::c_uint,
    pub irq_unaligned_count: ::core::ffi::c_uint,
    pub irq_fpassist_count: ::core::ffi::c_uint,
    pub irq_tlb_count: ::core::ffi::c_uint,
}

// ____cacheline_aligned is an external cache-line alignment attribute from
// the original kernel headers and is supplied by the surrounding translation.

// DECLARE_PER_CPU_SHARED_ALIGNED(irq_cpustat_t, irq_stat)
// The per-CPU declaration is supplied by the surrounding translation.

#[macro_export]
macro_rules! __ARCH_IRQ_STAT {
    () => {};
}

#[macro_export]
macro_rules! inc_irq_stat {
    ($member:ident) => {
        this_cpu_inc!(irq_stat.$member)
    };
}

#[macro_export]
macro_rules! __inc_irq_stat {
    ($member:ident) => {
        __this_cpu_inc!(irq_stat.$member)
    };
}

#[macro_export]
macro_rules! ack_bad_irq {
    ($irq:expr) => {
        WARN!(1, "unexpected IRQ trap at vector %02x\n", $irq)
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
