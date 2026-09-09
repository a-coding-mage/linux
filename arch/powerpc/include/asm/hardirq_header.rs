/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the corresponding Linux headers are intentionally
// left external; build-time configuration selects the conditional fields.

#[repr(C)]
pub struct irq_cpustat_t {
    pub __softirq_pending: u32,
    pub timer_irqs_event: u32,
    pub broadcast_irqs_event: u32,
    pub timer_irqs_others: u32,
    pub pmu_irqs: u32,
    pub mce_exceptions: u32,
    pub spurious_irqs: u32,
    pub sreset_irqs: u32,
    #[cfg(CONFIG_PPC_WATCHDOG)]
    pub soft_nmi_irqs: u32,
    #[cfg(CONFIG_PPC_DOORBELL)]
    pub doorbell_irqs: u32,
}

// ____cacheline_aligned irq_cpustat_t

// DECLARE_PER_CPU_SHARED_ALIGNED(irq_cpustat_t, irq_stat)
unsafe extern "C" {
    pub static mut irq_stat: irq_cpustat_t;
    pub fn printk(fmt: *const core::ffi::c_char, ...) -> core::ffi::c_int;
    pub fn arch_irq_stat_cpu(cpu: u32) -> u64;
}

pub const __ARCH_IRQ_STAT: bool = true;
pub const __ARCH_IRQ_EXIT_IRQS_DISABLED: bool = true;

#[inline]
pub unsafe fn ack_bad_irq(irq: u32) {
    // KERN_CRIT is the Linux logging-prefix macro.
    let message = b"\x012unexpected IRQ trap at vector %02x\n\0";
    printk(message.as_ptr() as *const core::ffi::c_char, irq);
}

// #define arch_irq_stat_cpu arch_irq_stat_cpu

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
