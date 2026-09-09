/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2020-2022 Loongson Technology Corporation Limited
 */

// Dependencies supplied by the surrounding kernel translation:
// linux/cache.h, linux/threads.h, and linux/irq.h.

unsafe extern "C" {
    pub fn ack_bad_irq(irq: core::ffi::c_uint);
}

// #define ack_bad_irq ack_bad_irq

pub const NR_IPI: usize = 4;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ipi_msg_type {
    IPI_RESCHEDULE,
    IPI_CALL_FUNCTION,
    IPI_IRQ_WORK,
    IPI_CLEAR_VECTOR,
}

#[repr(C)]
pub struct irq_cpustat_t {
    pub ipi_irqs: [core::ffi::c_uint; NR_IPI],
    pub __softirq_pending: core::ffi::c_uint,
    // atomic_t message ____cacheline_aligned_in_smp;
    pub message: atomic_t,
}

// ____cacheline_aligned

// DECLARE_PER_CPU_SHARED_ALIGNED(irq_cpustat_t, irq_stat)
// The per-CPU storage declaration is supplied by the surrounding kernel translation.

// #define __ARCH_IRQ_STAT


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
