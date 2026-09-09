// SPDX-License-Identifier: GPL-2.0-only
/*
 * cppc.c: CPPC Interface for x86
 * Copyright (c) 2016, Intel Corporation.
 */

const CPPC_HIGHEST_PERF_PERFORMANCE: u64 = 196;
const CPPC_HIGHEST_PERF_PREFCORE: u64 = 166;

#[repr(C)]
enum AmdPrefCore {
    AMD_PREF_CORE_UNKNOWN = 0,
    AMD_PREF_CORE_SUPPORTED,
    AMD_PREF_CORE_UNSUPPORTED,
}

static mut amd_pref_core_detected: AmdPrefCore = AmdPrefCore::AMD_PREF_CORE_UNKNOWN;
static mut boost_numerator: u64 = 0;

/* Refer to drivers/acpi/cppc_acpi.c for the description of functions */

pub unsafe fn cpc_supported_by_cpu() -> bool {
    match boot_cpu_data.x86_vendor {
        X86_VENDOR_AMD | X86_VENDOR_HYGON => {
            if boot_cpu_data.x86 == 0x19
                && (boot_cpu_data.x86_model <= 0x0f
                    || (boot_cpu_data.x86_model >= 0x20 && boot_cpu_data.x86_model <= 0x2f))
            {
                true
            } else if boot_cpu_data.x86 == 0x17
                && boot_cpu_data.x86_model >= 0x30
                && boot_cpu_data.x86_model <= 0x7f
            {
                true
            } else {
                boot_cpu_has(X86_FEATURE_CPPC)
            }
        }
        _ => false,
    }
}

pub fn cpc_ffh_supported() -> bool { true }

pub unsafe fn cpc_read_ffh(cpunum: i32, reg: *mut cpc_reg, val: *mut u64) -> i32 {
    let mut err = rdmsrq_safe_on_cpu(cpunum, (*reg).address, val);
    if err == 0 {
        let mask = genmask_ull((*reg).bit_offset + (*reg).bit_width - 1, (*reg).bit_offset);
        *val &= mask;
        *val >>= (*reg).bit_offset;
    }
    err
}

pub unsafe fn cpc_write_ffh(cpunum: i32, reg: *mut cpc_reg, mut val: u64) -> i32 {
    let mut rd_val = 0u64;
    let mut err = rdmsrq_safe_on_cpu(cpunum, (*reg).address, &mut rd_val);
    if err == 0 {
        let mask = genmask_ull((*reg).bit_offset + (*reg).bit_width - 1, (*reg).bit_offset);
        val <<= (*reg).bit_offset;
        val &= mask;
        rd_val &= !mask;
        rd_val |= val;
        err = wrmsrq_safe_on_cpu(cpunum, (*reg).address, rd_val);
    }
    err
}

unsafe fn amd_set_max_freq_ratio() {
    let mut perf_caps = cppc_perf_caps::default();
    let mut numerator = 0u64;
    let rc = cppc_get_perf_caps(0, &mut perf_caps);
    if rc != 0 { pr_debug!("Could not retrieve perf counters ({})\n", rc); return; }
    let rc = amd_get_boost_ratio_numerator(0, &mut numerator);
    if rc != 0 { pr_debug!("Could not retrieve highest performance ({})\n", rc); return; }
    let nominal_perf = perf_caps.nominal_perf;
    if nominal_perf == 0 { pr_debug!("Could not retrieve nominal performance\n"); return; }
    let perf_ratio = (div_u64(numerator * SCHED_CAPACITY_SCALE, nominal_perf)
        + SCHED_CAPACITY_SCALE) >> 1;
    freq_invariance_set_perf_ratio(perf_ratio, false);
}

static mut freq_invariance_lock: mutex = DEFINE_MUTEX!();

unsafe fn init_freq_invariance_cppc() {
    static mut init_done: bool = false;
    if !cpu_feature_enabled(X86_FEATURE_APERFMPERF) || boot_cpu_data.x86_vendor != X86_VENDOR_AMD { return; }
    mutex_lock(&mut freq_invariance_lock);
    if !init_done { amd_set_max_freq_ratio(); }
    init_done = true;
    mutex_unlock(&mut freq_invariance_lock);
}

pub unsafe fn acpi_processor_init_invariance_cppc() { init_freq_invariance_cppc(); }

/* Get the highest performance register value. */
pub unsafe fn amd_get_highest_perf(cpu: u32, highest_perf: *mut u32) -> i32 {
    let mut val = 0u64;
    let ret;
    if cpu_feature_enabled(X86_FEATURE_CPPC) {
        ret = rdmsrq_safe_on_cpu(cpu, MSR_AMD_CPPC_CAP1, &mut val);
        if ret != 0 { return ret; }
        val = field_get(AMD_CPPC_HIGHEST_PERF_MASK, val);
    } else {
        ret = cppc_get_highest_perf(cpu, &mut val);
        if ret != 0 { return ret; }
    }
    write_once(highest_perf, val as u32);
    ret
}

pub unsafe fn amd_detect_prefcore(detected: *mut bool) -> i32 {
    let mut count = 0usize;
    let mut highest_perf = [0u64; 2];
    if detected.is_null() { warn_on(true); return -EINVAL; }
    match amd_pref_core_detected {
        AmdPrefCore::AMD_PREF_CORE_SUPPORTED => { *detected = true; return 0; }
        AmdPrefCore::AMD_PREF_CORE_UNSUPPORTED => { *detected = false; return 0; }
        _ => {}
    }
    for_each_online_cpu!(cpu, {
        let mut tmp = 0u32;
        let ret = amd_get_highest_perf(cpu, &mut tmp);
        if ret != 0 { return ret; }
        if count == 0 || (count == 1 && tmp as u64 != highest_perf[0]) {
            highest_perf[count] = tmp as u64;
            count += 1;
        }
        if count == 2 { break; }
    });
    *detected = count == 2;
    boost_numerator = highest_perf[0];
    amd_pref_core_detected = if *detected { AmdPrefCore::AMD_PREF_CORE_SUPPORTED } else { AmdPrefCore::AMD_PREF_CORE_UNSUPPORTED };
    pr_debug!("AMD CPPC preferred core is {}supported (highest perf: 0x{:x})\n", if *detected { "" } else { "un" }, highest_perf[0]);
    0
}

pub unsafe fn amd_get_boost_ratio_numerator(cpu: u32, numerator: *mut u64) -> i32 {
    let mut prefcore = false;
    let ret = amd_detect_prefcore(&mut prefcore);
    if ret != 0 { return ret; }
    if !prefcore { *numerator = boost_numerator; return 0; }
    if cpu_feature_enabled(X86_FEATURE_ZEN4) && (0x70..=0x7f).contains(&boot_cpu_data.x86_model) {
        *numerator = CPPC_HIGHEST_PERF_PERFORMANCE; return 0;
    }
    if cpu_feature_enabled(X86_FEATURE_AMD_HTR_CORES) {
        match cpu_data(cpu).topo.cpu_type {
            TOPO_CPU_TYPE_UNKNOWN | TOPO_CPU_TYPE_ANY => pr_warn!("Undefined core type found for cpu {}\n", cpu),
            TOPO_CPU_TYPE_PERFORMANCE => { *numerator = CPPC_HIGHEST_PERF_PERFORMANCE; return 0; }
            TOPO_CPU_TYPE_LOW_POWER | TOPO_CPU_TYPE_EFFICIENCY => {
                let mut tmp = 0u32; let ret = amd_get_highest_perf(cpu, &mut tmp);
                if ret != 0 { return ret; } *numerator = tmp as u64; return 0;
            }
        }
    }
    *numerator = CPPC_HIGHEST_PERF_PREFCORE;
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
