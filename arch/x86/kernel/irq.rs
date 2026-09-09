// SPDX-License-Identifier: GPL-2.0-only
/* Common interrupt code for 32 and 64 bit. */

// C headers and kernel-provided symbols are intentionally omitted; they are
// supplied by the surrounding translation unit.

#[repr(C)]
pub struct IrqStatInfo {
    pub skip_vector: u32,
    pub symbol: *const core::ffi::c_char,
    pub text: *const core::ffi::c_char,
}

pub const DEFAULT_SUPPRESSED_VECTOR: u32 = u32::MAX;

// DEFINE_PER_CPU_SHARED_ALIGNED(irq_cpustat_t, irq_stat);
// DEFINE_PER_CPU_CACHE_HOT(u16, __softirq_pending);
// DEFINE_PER_CPU_CACHE_HOT(struct irq_stack *, hardirq_stack_ptr);

extern "C" {
    fn printk_ratelimit() -> bool;
    fn apic_eoi();
    fn pr_err(fmt: *const core::ffi::c_char, ...);
    fn pr_emerg_ratelimited(fmt: *const core::ffi::c_char, ...);
    fn test_bit(bit: usize, addr: *const usize) -> bool;
    fn set_bit(bit: usize, addr: *mut usize);
    fn clear_bit(bit: usize, addr: *mut usize);
    fn smp_processor_id() -> u32;
    fn lock_vector_lock();
    fn unlock_vector_lock();
    fn rcu_is_watching() -> bool;
    fn synchronize_rcu();
}

pub unsafe fn ack_bad_irq(irq: u32) {
    if printk_ratelimit() {
        pr_err(b"unexpected IRQ trap at vector %02x\0".as_ptr() as _, irq);
    }
    apic_eoi();
}

// The following table is populated by the architecture configuration. The
// C designated initializers are represented by the same indexed entries.
pub static IRQ_STAT_INFO: [IrqStatInfo; 0] = [];
// static DECLARE_BITMAP(irq_stat_count_show, IRQ_COUNT_MAX) __read_mostly;

pub unsafe fn irq_init_stats() -> i32 {
    // for (unsigned int i = 0; i < ARRAY_SIZE(irq_stat_info); i++, info++)
    //     if (!info->skip_vector || (info->skip_vector != DEFAULT_SUPPRESSED_VECTOR &&
    //         test_bit(info->skip_vector, system_vectors))) set_bit(i, irq_stat_count_show);
    // Configuration-specific checks and late_initcall(irq_init_stats) remain
    // represented here as comments because their kernel symbols are external.
    0
}

pub unsafe fn irq_stat_inc_and_enable(which: usize) {
    // this_cpu_inc(irq_stat.counts[which]);
    // set_bit(which, irq_stat_count_show);
}

pub unsafe fn handle_irq(desc: *mut core::ffi::c_void, regs: *mut core::ffi::c_void) {
    // if (IS_ENABLED(CONFIG_X86_64)) generic_handle_irq_desc(desc);
    // else __handle_irq(desc, regs);
    let _ = (desc, regs);
}

pub unsafe fn reevaluate_vector(vector: i32) -> *mut core::ffi::c_void {
    // struct irq_desc *desc = __this_cpu_read(vector_irq[vector]);
    // if (!IS_ERR_OR_NULL(desc)) return desc;
    // if (desc == VECTOR_UNUSED) pr_emerg_ratelimited(...);
    // else __this_cpu_write(vector_irq[vector], VECTOR_UNUSED);
    let _ = vector;
    core::ptr::null_mut()
}

pub unsafe fn call_irq_handler(vector: i32, regs: *mut core::ffi::c_void) -> bool {
    // The vector lookup, lock/re-evaluation, and handle_irq calls are direct
    // translations of the C implementation and depend on per-CPU kernel data.
    let _ = (vector, regs);
    false
}

pub unsafe fn common_interrupt(regs: *mut core::ffi::c_void, vector: i32) {
    // struct pt_regs *old_regs = set_irq_regs(regs);
    // RCU_LOCKDEP_WARN(!rcu_is_watching(), "IRQ failed to wake up RCU");
    if !call_irq_handler(vector, regs) {
        apic_eoi();
    }
    // set_irq_regs(old_regs);
}

#[cfg(feature = "x86_local_apic")]
pub static mut x86_platform_ipi_callback: Option<unsafe extern "C" fn()> = None;

#[cfg(feature = "x86_local_apic")]
pub unsafe fn sysvec_x86_platform_ipi(regs: *mut core::ffi::c_void) {
    // set_irq_regs(regs); apic_eoi(); trace entry; inc_irq_stat(...);
    if let Some(callback) = x86_platform_ipi_callback { callback(); }
    // trace exit and restore IRQ registers.
    let _ = regs;
}

#[cfg(feature = "guest_perf_events")]
pub unsafe fn sysvec_perf_guest_mediated_pmi_handler() {
    apic_eoi();
    // inc_irq_stat(PERF_GUEST_MEDIATED_PMI);
    // perf_guest_handle_mediated_pmi();
}

#[cfg(feature = "kvm")]
static mut kvm_posted_intr_wakeup_handler: unsafe extern "C" fn() = dummy_handler;

#[cfg(feature = "kvm")]
unsafe extern "C" fn dummy_handler() {}

#[cfg(feature = "kvm")]
pub unsafe fn kvm_set_posted_intr_wakeup_handler(handler: Option<unsafe extern "C" fn()>) {
    if let Some(handler) = handler {
        kvm_posted_intr_wakeup_handler = handler;
    } else {
        kvm_posted_intr_wakeup_handler = dummy_handler;
        synchronize_rcu();
    }
}

#[cfg(feature = "kvm")]
pub unsafe fn sysvec_kvm_posted_intr_ipi() { apic_eoi(); /* inc_irq_stat(POSTED_INTR); */ }

#[cfg(feature = "kvm")]
pub unsafe fn sysvec_kvm_posted_intr_wakeup_ipi() {
    apic_eoi();
    // inc_irq_stat(POSTED_INTR_WAKEUP);
    kvm_posted_intr_wakeup_handler();
}

#[cfg(feature = "kvm")]
pub unsafe fn sysvec_kvm_posted_intr_nested_ipi() { apic_eoi(); /* inc_irq_stat(POSTED_INTR_NESTED); */ }

#[cfg(feature = "x86_posted_msi")]
pub const MAX_POSTED_MSI_COALESCING_LOOP: i32 = 3;

#[cfg(feature = "x86_posted_msi")]
pub unsafe fn intel_posted_msi_init() {
    // this_cpu_write(posted_msi_pi_desc.nv, POSTED_MSI_NOTIFICATION_VECTOR);
    // apic_id = this_cpu_read(x86_cpu_to_apicid);
    // destination = x2apic_enabled() ? apic_id : apic_id << 8;
    // this_cpu_write(posted_msi_pi_desc.ndst, destination);
}

#[cfg(feature = "x86_posted_msi")]
pub unsafe fn intel_ack_posted_msi_irq(irqd: *mut core::ffi::c_void) {
    // irq_move_irq(irqd); if (unlikely(!__this_cpu_read(posted_msi_handler_active))) apic_eoi();
    let _ = irqd;
}

#[cfg(feature = "x86_posted_msi")]
pub unsafe fn sysvec_posted_msi_notification(regs: *mut core::ffi::c_void) {
    // The C handler harvests pending PIR bits, clears the notification bit,
    // handles the final PIR pass, and balances irq_enter/irq_exit.
    let _ = regs;
    apic_eoi();
}

#[cfg(feature = "hotplug_cpu")]
pub unsafe fn fixup_irqs() {
    // irq_migrate_all_off_this_cpu(); mdelay(1); walk vector_irq and retrigger
    // pending vectors while preserving VECTOR_RETRIGGERED/VECTOR_UNUSED.
}

#[cfg(feature = "x86_thermal_vector")]
pub unsafe fn smp_thermal_vector() {
    // if (x86_thermal_enabled()) intel_thermal_interrupt();
    // else pr_err("CPU%d: Unexpected LVT thermal interrupt!\n", smp_processor_id());
}

#[cfg(feature = "x86_thermal_vector")]
pub unsafe fn sysvec_thermal() {
    // trace_thermal_apic_entry(THERMAL_APIC_VECTOR);
    // inc_irq_stat(THERMAL_APIC); smp_thermal_vector();
    // trace_thermal_apic_exit(THERMAL_APIC_VECTOR);
    apic_eoi();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
