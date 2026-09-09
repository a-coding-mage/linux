/* SPDX-License-Identifier: GPL-2.0 */

// C dependency: <linux/threads.h>

#[repr(C)]
pub enum irq_stat_counts {
    IRQ_COUNT_NMI,
    #[cfg(feature = "CONFIG_X86_LOCAL_APIC")]
    IRQ_COUNT_APIC_TIMER,
    #[cfg(feature = "CONFIG_X86_LOCAL_APIC")]
    IRQ_COUNT_SPURIOUS,
    #[cfg(feature = "CONFIG_X86_LOCAL_APIC")]
    IRQ_COUNT_APIC_PERF,
    #[cfg(feature = "CONFIG_X86_LOCAL_APIC")]
    IRQ_COUNT_IRQ_WORK,
    #[cfg(feature = "CONFIG_X86_LOCAL_APIC")]
    IRQ_COUNT_ICR_READ_RETRY,
    #[cfg(feature = "CONFIG_X86_LOCAL_APIC")]
    IRQ_COUNT_X86_PLATFORM_IPI,
    #[cfg(feature = "CONFIG_SMP")]
    IRQ_COUNT_RESCHEDULE,
    #[cfg(feature = "CONFIG_SMP")]
    IRQ_COUNT_CALL_FUNCTION,
    IRQ_COUNT_TLB,
    #[cfg(feature = "CONFIG_X86_THERMAL_VECTOR")]
    IRQ_COUNT_THERMAL_APIC,
    #[cfg(feature = "CONFIG_X86_MCE_THRESHOLD")]
    IRQ_COUNT_THRESHOLD_APIC,
    #[cfg(feature = "CONFIG_X86_MCE_AMD")]
    IRQ_COUNT_DEFERRED_ERROR,
    #[cfg(feature = "CONFIG_X86_MCE")]
    IRQ_COUNT_MCE_EXCEPTION,
    #[cfg(feature = "CONFIG_X86_MCE")]
    IRQ_COUNT_MCE_POLL,
    #[cfg(feature = "CONFIG_X86_HV_CALLBACK_VECTOR")]
    IRQ_COUNT_HYPERVISOR_CALLBACK,
    #[cfg(feature = "CONFIG_HYPERV")]
    IRQ_COUNT_HYPERV_REENLIGHTENMENT,
    #[cfg(feature = "CONFIG_HYPERV")]
    IRQ_COUNT_HYPERV_STIMER0,
    #[cfg(feature = "CONFIG_KVM")]
    IRQ_COUNT_POSTED_INTR,
    #[cfg(feature = "CONFIG_KVM")]
    IRQ_COUNT_POSTED_INTR_NESTED,
    #[cfg(feature = "CONFIG_KVM")]
    IRQ_COUNT_POSTED_INTR_WAKEUP,
    #[cfg(feature = "CONFIG_GUEST_PERF_EVENTS")]
    IRQ_COUNT_PERF_GUEST_MEDIATED_PMI,
    #[cfg(feature = "CONFIG_X86_POSTED_MSI")]
    IRQ_COUNT_POSTED_MSI_NOTIFICATION,
    IRQ_COUNT_PIC_APIC_ERROR,
    #[cfg(feature = "CONFIG_X86_IO_APIC")]
    IRQ_COUNT_IOAPIC_MISROUTED,
    IRQ_COUNT_MAX,
}

#[repr(C)]
pub struct irq_cpustat_t {
    #[cfg(all(feature = "CONFIG_CPU_MITIGATIONS", feature = "CONFIG_KVM_INTEL"))]
    pub kvm_cpu_l1tf_flush_l1d: u8,
    pub counts: [core::ffi::c_uint; IRQ_COUNT_MAX as usize],
}

// C dependency: DECLARE_PER_CPU_SHARED_ALIGNED(irq_cpustat_t, irq_stat)
extern "C" {
    pub static mut irq_stat: irq_cpustat_t;
}

#[cfg(feature = "CONFIG_X86_POSTED_MSI")]
// C dependency: struct pi_desc is supplied by another header.
extern "C" {
    pub static mut posted_msi_pi_desc: pi_desc;
}

// #define __ARCH_IRQ_STAT
// #define inc_irq_stat(index) this_cpu_inc(irq_stat.counts[IRQ_COUNT_##index])
#[macro_export]
macro_rules! inc_irq_stat {
    ($index:ident) => {
        this_cpu_inc!(irq_stat.counts[irq_stat_counts::$index])
    };
}

extern "C" {
    pub fn irq_stat_inc_and_enable(which: irq_stat_counts);
    pub fn ack_bad_irq(irq: core::ffi::c_uint);
}

#[cfg(feature = "CONFIG_X86_LOCAL_APIC")]
#[macro_export]
macro_rules! inc_perf_irq_stat {
    () => {
        inc_irq_stat!(APIC_PERF)
    };
}
#[cfg(not(feature = "CONFIG_X86_LOCAL_APIC"))]
#[macro_export]
macro_rules! inc_perf_irq_stat {
    () => {};
}

#[cfg(feature = "CONFIG_PROC_FS")]
extern "C" {
    pub fn arch_irq_stat_cpu(cpu: core::ffi::c_uint) -> u64;
}

// C dependency: DECLARE_PER_CPU_CACHE_HOT(u16, __softirq_pending)
extern "C" {
    pub static mut __softirq_pending: u16;
}

// #define local_softirq_pending_ref __softirq_pending

#[cfg(all(feature = "CONFIG_CPU_MITIGATIONS", feature = "CONFIG_KVM_INTEL"))]
#[inline(always)]
pub unsafe fn kvm_set_cpu_l1tf_flush_l1d() {
    __this_cpu_write!(irq_stat.kvm_cpu_l1tf_flush_l1d, 1);
}

#[cfg(all(feature = "CONFIG_CPU_MITIGATIONS", feature = "CONFIG_KVM_INTEL"))]
#[inline(always)]
pub unsafe fn kvm_clear_cpu_l1tf_flush_l1d() {
    __this_cpu_write!(irq_stat.kvm_cpu_l1tf_flush_l1d, 0);
}

#[cfg(all(feature = "CONFIG_CPU_MITIGATIONS", feature = "CONFIG_KVM_INTEL"))]
#[inline(always)]
pub unsafe fn kvm_get_cpu_l1tf_flush_l1d() -> bool {
    __this_cpu_read!(irq_stat.kvm_cpu_l1tf_flush_l1d)
}

#[cfg(not(all(feature = "CONFIG_CPU_MITIGATIONS", feature = "CONFIG_KVM_INTEL")))]
#[inline(always)]
pub unsafe fn kvm_set_cpu_l1tf_flush_l1d() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
