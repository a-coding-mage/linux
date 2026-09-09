/* SPDX-License-Identifier: GPL-2.0 */

pub const NR_IRQS_LEGACY: i32 = 16;

// When CONFIG_SPARSE_IRQ is not enabled, NR_IRQS is supplied by <mach/irqs.h>.
// With CONFIG_SPARSE_IRQ enabled, the C header defines NR_IRQS to NR_IRQS_LEGACY.
#[cfg(CONFIG_SPARSE_IRQ)]
pub const NR_IRQS: i32 = NR_IRQS_LEGACY;

// C macro: #define irq_canonicalize(i) (i), when no prior definition exists.
#[inline]
pub const fn irq_canonicalize<T>(i: T) -> T {
    i
}

/*
 * Use this value to indicate lack of interrupt
 * capability
 */
pub const NO_IRQ: u32 = u32::MAX;

pub struct irqaction;
pub struct pt_regs;

unsafe extern "C" {
    pub fn handle_IRQ(irq: u32, regs: *mut pt_regs);
}

// CONFIG_SMP conditional declarations are preserved below. The cpumask_t type
// and arch_trigger_cpumask_backtrace function are supplied by other headers.
#[cfg(CONFIG_SMP)]
extern "C" {
    pub fn arch_trigger_cpumask_backtrace(mask: *const cpumask_t, exclude_cpu: i32);
}

#[cfg(CONFIG_SMP)]
pub type cpumask_t = core::ffi::c_void;

#[inline]
pub const fn nr_legacy_irqs() -> i32 {
    NR_IRQS_LEGACY
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
