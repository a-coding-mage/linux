/* SPDX-License-Identifier: GPL-2.0 */

/* Translated from asm/smp.h. C preprocessor configuration conditions are
 * retained as Rust cfg conditions where applicable. */

extern "C" {
    pub static mut cpu_number: i32;
    pub static mut cpu_sibling_map: cpumask_var_t;
    pub static mut cpu_core_map: cpumask_var_t;
    pub static mut cpu_die_map: cpumask_var_t;
    pub static mut cpu_llc_shared_map: cpumask_var_t;
    pub static mut cpu_l2c_shared_map: cpumask_var_t;
    pub static mut x86_cpu_to_apicid: u32;
    pub static mut x86_cpu_to_acpiid: u32;
}

/* Supplied by the kernel's cpumask definitions. */
pub type cpumask_var_t = *mut cpumask;

#[repr(C)]
pub struct cpumask {
    _private: [u8; 0],
}

#[repr(C)]
pub struct task_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}

#[repr(C)]
pub struct smp_ops {
    pub smp_prepare_boot_cpu: Option<unsafe extern "C" fn()>,
    pub smp_prepare_cpus: Option<unsafe extern "C" fn(max_cpus: u32)>,
    pub smp_cpus_done: Option<unsafe extern "C" fn(max_cpus: u32)>,
    pub stop_other_cpus: Option<unsafe extern "C" fn(wait: i32)>,
    pub crash_stop_other_cpus: Option<unsafe extern "C" fn()>,
    pub smp_send_reschedule: Option<unsafe extern "C" fn(cpu: i32)>,
    pub cleanup_dead_cpu: Option<unsafe extern "C" fn(cpu: u32)>,
    pub poll_sync_state: Option<unsafe extern "C" fn()>,
    pub kick_ap_alive: Option<unsafe extern "C" fn(cpu: u32, tidle: *mut task_struct) -> i32>,
    pub cpu_disable: Option<unsafe extern "C" fn() -> i32>,
    pub cpu_die: Option<unsafe extern "C" fn(cpu: u32)>,
    pub play_dead: Option<unsafe extern "C" fn()>,
    pub stop_this_cpu: Option<unsafe extern "C" fn()>,
    pub send_call_func_ipi: Option<unsafe extern "C" fn(mask: *const cpumask)>,
    pub send_call_func_single_ipi: Option<unsafe extern "C" fn(cpu: i32)>,
}

extern "C" {
    pub fn set_cpu_sibling_map(cpu: i32);
}

/* CONFIG_SMP */
extern "C" {
    pub static mut smp_ops: smp_ops;
    pub fn cpu_disable_common();
    pub fn native_smp_prepare_boot_cpu();
    pub fn smp_prepare_cpus_common();
    pub fn native_smp_prepare_cpus(max_cpus: u32);
    pub fn native_smp_cpus_done(max_cpus: u32);
    pub fn common_cpu_up(cpunum: u32, tidle: *mut task_struct) -> i32;
    pub fn native_kick_ap(cpu: u32, tidle: *mut task_struct) -> i32;
    pub fn native_cpu_disable() -> i32;
    pub fn hlt_play_dead() -> !;
    pub fn native_play_dead() -> !;
    pub fn play_dead_common();
    pub fn wbinvd_on_cpu(cpu: i32);
    pub fn wbinvd_on_all_cpus();
    pub fn wbinvd_on_cpus_mask(cpus: *mut cpumask);
    pub fn wbnoinvd_on_all_cpus();
    pub fn wbnoinvd_on_cpus_mask(cpus: *mut cpumask);
    pub fn smp_kick_mwait_play_dead();
    pub fn mwait_play_dead(eax_hint: u32) -> !;
    pub fn native_smp_send_reschedule(cpu: i32);
    pub fn native_send_call_func_ipi(mask: *const cpumask);
    pub fn native_send_call_func_single_ipi(cpu: i32);
    pub fn smp_reboot_interrupt();
    pub fn smp_reschedule_interrupt(regs: *mut pt_regs);
    pub fn smp_call_function_interrupt(regs: *mut pt_regs);
    pub fn smp_call_function_single_interrupt(r: *mut pt_regs);
    pub fn wbinvd();
    pub fn wbnoinvd();
}

#[inline]
pub unsafe fn smp_send_stop() { if let Some(f) = smp_ops.stop_other_cpus { f(0); } }
#[inline]
pub unsafe fn stop_other_cpus() { if let Some(f) = smp_ops.stop_other_cpus { f(1); } }
#[inline]
pub unsafe fn smp_prepare_cpus(max_cpus: u32) { if let Some(f) = smp_ops.smp_prepare_cpus { f(max_cpus); } }
#[inline]
pub unsafe fn smp_cpus_done(max_cpus: u32) { if let Some(f) = smp_ops.smp_cpus_done { f(max_cpus); } }
#[inline]
pub unsafe fn __cpu_disable() -> i32 { smp_ops.cpu_disable.unwrap()() }
#[inline]
pub unsafe fn __cpu_die(cpu: u32) { if let Some(f) = smp_ops.cpu_die { f(cpu); } }
#[inline]
pub unsafe fn play_dead() -> ! { smp_ops.play_dead.unwrap()(); panic!("BUG"); }
#[inline]
pub unsafe fn arch_smp_send_reschedule(cpu: i32) { smp_ops.smp_send_reschedule.unwrap()(cpu); }
#[inline]
pub unsafe fn arch_send_call_function_single_ipi(cpu: i32) { smp_ops.send_call_func_single_ipi.unwrap()(cpu); }
#[inline]
pub unsafe fn arch_send_call_function_ipi_mask(mask: *const cpumask) { smp_ops.send_call_func_ipi.unwrap()(mask); }

/* !CONFIG_SMP equivalents. */
#[inline]
pub unsafe fn wbinvd_on_cpu(_cpu: i32) { wbinvd(); }
#[inline]
pub unsafe fn wbinvd_on_all_cpus_nosmp() { wbinvd(); }
#[inline]
pub unsafe fn wbinvd_on_cpus_mask_nosmp(_cpus: *mut cpumask) { wbinvd(); }
#[inline]
pub unsafe fn wbnoinvd_on_all_cpus() { wbnoinvd(); }
#[inline]
pub unsafe fn wbnoinvd_on_cpus_mask(_cpus: *mut cpumask) { wbnoinvd(); }
#[inline]
pub unsafe fn mwait_play_dead_nosmp(_eax_hint: u32) -> ! { panic!("BUG"); }

/* CONFIG_DEBUG_NMI_SELFTEST */
extern "C" { pub fn nmi_selftest(); }

#[inline]
pub fn nmi_selftest_nosmp() {}

/* Per-CPU accessors and BUG/wbinvd/cpumask_of are supplied by dependencies. */
pub const STARTUP_READ_APICID: u32 = 0x80000000;
pub const STARTUP_PARALLEL_MASK: u32 = 0xFF000000;

extern "C" {
    pub static mut smpboot_control: u32;
    pub static mut apic_mmio_base: usize;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
