/* SPDX-License-Identifier: GPL-2.0 */
/*
 * include/asm-parisc/irq.h
 *
 * Copyright 2005 Matthew Wilcox <matthew@wil.cx>
 */

// C dependencies: linux/cpumask.h and asm/types.h.

pub const NO_IRQ: i32 = -1;

#[cfg(feature = "CONFIG_GSC")]
pub const GSC_IRQ_BASE: i32 = 16;
#[cfg(feature = "CONFIG_GSC")]
pub const GSC_IRQ_MAX: i32 = 63;
#[cfg(feature = "CONFIG_GSC")]
pub const CPU_IRQ_BASE: i32 = 64;
#[cfg(not(feature = "CONFIG_GSC"))]
pub const CPU_IRQ_BASE: i32 = 16;

pub const TIMER_IRQ: i32 = CPU_IRQ_BASE + 0;
pub const IPI_IRQ: i32 = CPU_IRQ_BASE + 1;
pub const CPU_IRQ_MAX: i32 = CPU_IRQ_BASE + (usize::BITS as i32 - 1);

pub const NR_IRQS: i32 = CPU_IRQ_MAX + 1;

#[inline]
pub fn irq_canonicalize(irq: i32) -> i32 {
    if irq == 2 { 9 } else { irq }
}

#[repr(C)]
pub struct irq_chip {
    _private: [u8; 0],
}

#[repr(C)]
pub struct irq_data {
    _private: [u8; 0],
}

#[repr(C)]
pub struct cpumask {
    _private: [u8; 0],
}

extern "C" {
    pub fn cpu_ack_irq(d: *mut irq_data);
    pub fn cpu_eoi_irq(d: *mut irq_data);

    pub fn txn_alloc_irq(nbits: u32) -> i32;
    pub fn txn_claim_irq(arg: i32) -> i32;
    pub fn txn_alloc_data(arg: u32) -> u32;
    pub fn txn_alloc_addr(arg: u32) -> usize;
    pub fn txn_affinity_addr(irq: u32, cpu: i32) -> usize;

    pub fn cpu_claim_irq(irq: u32, chip: *mut irq_chip, arg: *mut core::ffi::c_void) -> i32;
    pub fn cpu_check_affinity(d: *mut irq_data, dest: *const cpumask) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
