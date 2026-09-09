// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * SMP support for pSeries machines.
 *
 * Dave Engebretsen, Peter Bergner, and
 * Mike Corrigan {engebret|bergner|mikec}@us.ibm.com
 *
 * Plus various changes from other IBM teams...
 */

// Dependencies supplied by the surrounding kernel translation unit.

use core::ffi::c_void;

extern "C" {
    static mut of_spin_mask: cpumask_var_t;
    static mut ic_cause_ipi: Option<unsafe extern "C" fn(cpu: i32)>;
    static mut smp_ops: *mut smp_ops_t;
    static boot_cpuid: i32;
    static nr_cpu_ids: i32;
    static cpu_present_mask: cpumask_t;
    static mut paca_ptrs: *mut *mut paca;

    fn rtas_function_token(token: i32) -> i32;
    fn rtas_call(token: i32, nargs: i32, nret: i32, ret: *mut i32, ...) -> i32;
    fn printk(level: *const u8, fmt: *const u8, ...);
    fn cpumask_test_cpu(cpu: i32, mask: cpumask_var_t) -> bool;
    fn cpumask_set_cpu(cpu: i32, mask: cpumask_var_t);
    fn cpumask_clear_cpu(cpu: i32, mask: cpumask_var_t);
    fn cpumask_copy(dst: cpumask_var_t, src: *const cpumask_t);
    fn get_hard_smp_processor_id(cpu: i32) -> i32;
    fn ppc_function_entry(f: unsafe extern "C" fn());
    fn __pa(addr: usize) -> usize;
    fn xive_enabled() -> bool;
    fn xive_smp_setup_cpu();
    fn xics_setup_cpu();
    fn firmware_has_feature(feature: u64) -> bool;
    fn vpa_init(cpu: i32);
    fn doorbell_try_core_ipi(cpu: i32) -> bool;
    fn plpar_signal_sys_reset(cpu: i32) -> i64;
    fn xive_smp_probe() -> i32;
    fn xics_smp_probe();
    fn cpu_has_feature(feature: u64) -> bool;
    fn check_kvm_guest();
    fn is_kvm_guest() -> bool;
    fn is_secure_guest() -> bool;
    fn smp_generic_cpu_bootable(cpu: i32) -> bool;
    fn alloc_bootmem_cpumask_var(mask: *mut cpumask_var_t);
    fn cpu_thread_in_core(cpu: i32) -> i32;
    fn for_each_present_cpu_body(body: unsafe extern "C" fn(i32));
    fn generic_secondary_smp_init();
}

type cpumask_var_t = *mut cpumask_t;

#[repr(C)]
pub struct cpumask_t { _private: [u8; 0] }

#[repr(C)]
pub struct paca { pub cpu_start: i32 }

#[repr(C)]
pub struct smp_ops_t {
    pub message_pass: Option<unsafe extern "C" fn()>,
    pub cause_ipi: Option<unsafe extern "C" fn(i32)>,
    pub cause_nmi_ipi: Option<unsafe extern "C" fn(i32) -> i32>,
    pub probe: Option<unsafe extern "C" fn()>,
    pub prepare_cpu: Option<unsafe extern "C" fn(i32) -> i32>,
    pub kick_cpu: Option<unsafe extern "C" fn(i32) -> i32>,
    pub setup_cpu: Option<unsafe extern "C" fn(i32)>,
    pub cpu_bootable: Option<unsafe extern "C" fn(i32) -> bool>,
}

unsafe fn smp_query_cpu_stopped(pcpu: u32) -> i32 {
    let mut cpu_status = 0;
    let qcss_tok = rtas_function_token(RTAS_FN_QUERY_CPU_STOPPED_STATE);
    if qcss_tok == RTAS_UNKNOWN_SERVICE {
        printk(KERN_INFO.as_ptr(), b"Firmware doesn't support query-cpu-stopped-state\0".as_ptr());
        return QCSS_HARDWARE_ERROR;
    }
    let status = rtas_call(qcss_tok, 1, 2, &mut cpu_status, pcpu);
    if status != 0 {
        printk(KERN_ERR.as_ptr(), b"RTAS query-cpu-stopped-state failed: %i\n\0".as_ptr(), status);
        return status;
    }
    cpu_status
}

unsafe fn smp_startup_cpu(lcpu: u32) -> i32 {
    let start_here = __pa(ppc_function_entry(generic_secondary_smp_init));
    if cpumask_test_cpu(lcpu as i32, of_spin_mask) { return 1; }
    let pcpu = get_hard_smp_processor_id(lcpu as i32);
    if smp_query_cpu_stopped(pcpu as u32) == QCSS_NOT_STOPPED {
        cpumask_set_cpu(lcpu as i32, of_spin_mask);
        return 1;
    }
    let start_cpu = rtas_function_token(RTAS_FN_START_CPU);
    if start_cpu == RTAS_UNKNOWN_SERVICE { return 1; }
    let status = rtas_call(start_cpu, 3, 1, core::ptr::null_mut(), pcpu, start_here, pcpu);
    if status != 0 {
        printk(KERN_ERR.as_ptr(), b"start-cpu failed: %i\n\0".as_ptr(), status);
        return 0;
    }
    1
}

unsafe extern "C" fn smp_setup_cpu(cpu: i32) {
    if xive_enabled() { xive_smp_setup_cpu(); } else if cpu != boot_cpuid { xics_setup_cpu(); }
    if firmware_has_feature(FW_FEATURE_SPLPAR) && cpu != boot_cpuid { vpa_init(cpu); }
    cpumask_clear_cpu(cpu, of_spin_mask);
}

unsafe extern "C" fn smp_pSeries_kick_cpu(nr: i32) -> i32 {
    if nr < 0 || nr >= nr_cpu_ids { return -EINVAL; }
    if smp_startup_cpu(nr as u32) == 0 { return -ENOENT; }
    (*paca_ptrs.add(nr as usize)).cpu_start = 1;
    0
}

unsafe extern "C" fn pseries_smp_prepare_cpu(cpu: i32) -> i32 {
    if xive_enabled() { return xive_smp_prepare_cpu(cpu); }
    0
}

unsafe extern "C" fn dbell_or_ic_cause_ipi(cpu: i32) {
    if doorbell_try_core_ipi(cpu) { return; }
    if let Some(f) = ic_cause_ipi { f(cpu); }
}

unsafe extern "C" fn pseries_cause_nmi_ipi(cpu: i32) -> i32 {
    let hwcpu = if cpu == NMI_IPI_ALL_OTHERS { H_SIGNAL_SYS_RESET_ALL_OTHERS } else {
        if cpu < 0 { return 0; }
        get_hard_smp_processor_id(cpu)
    };
    if plpar_signal_sys_reset(hwcpu as i32) == H_SUCCESS { return 1; }
    0
}

unsafe extern "C" fn pSeries_smp_probe() {
    if xive_enabled() { if xive_smp_probe() < 0 { return; } } else { xics_smp_probe(); }
    if !cpu_has_feature(CPU_FTR_DBELL) || !cpu_has_feature(CPU_FTR_SMT) { return; }
    check_kvm_guest();
    if is_kvm_guest() { if xive_enabled() || is_secure_guest() { return; } }
    ic_cause_ipi = (*smp_ops).cause_ipi;
    (*smp_ops).cause_ipi = Some(dbell_or_ic_cause_ipi);
}

static mut pseries_smp_ops: smp_ops_t = smp_ops_t {
    message_pass: None, cause_ipi: None, cause_nmi_ipi: Some(pseries_cause_nmi_ipi),
    probe: Some(pSeries_smp_probe), prepare_cpu: Some(pseries_smp_prepare_cpu),
    kick_cpu: Some(smp_pSeries_kick_cpu), setup_cpu: Some(smp_setup_cpu),
    cpu_bootable: Some(smp_generic_cpu_bootable),
};

pub unsafe extern "C" fn smp_init_pseries() {
    smp_ops = &raw mut pseries_smp_ops;
    alloc_bootmem_cpumask_var(&raw mut of_spin_mask);
    if rtas_function_token(RTAS_FN_QUERY_CPU_STOPPED_STATE) == RTAS_UNKNOWN_SERVICE {
        if cpu_has_feature(CPU_FTR_SMT) {
            for_each_present_cpu_body(smp_init_pseries_cpu);
        } else { cpumask_copy(of_spin_mask, &cpu_present_mask); }
        cpumask_clear_cpu(boot_cpuid, of_spin_mask);
    }
}

unsafe extern "C" fn smp_init_pseries_cpu(i: i32) {
    if cpu_thread_in_core(i) == 0 { cpumask_set_cpu(i, of_spin_mask); }
}

const RTAS_UNKNOWN_SERVICE: i32 = -1;
const RTAS_FN_QUERY_CPU_STOPPED_STATE: i32 = 0;
const RTAS_FN_START_CPU: i32 = 1;
const QCSS_HARDWARE_ERROR: i32 = -1;
const QCSS_NOT_STOPPED: i32 = 0;
const EINVAL: i32 = 22;
const ENOENT: i32 = 2;
const NMI_IPI_ALL_OTHERS: i32 = -1;
const H_SIGNAL_SYS_RESET_ALL_OTHERS: i32 = -1;
const H_SUCCESS: i64 = 0;
const FW_FEATURE_SPLPAR: u64 = 1;
const CPU_FTR_DBELL: u64 = 1;
const CPU_FTR_SMT: u64 = 2;
static KERN_INFO: &[u8] = b"";
static KERN_ERR: &[u8] = b"";


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
