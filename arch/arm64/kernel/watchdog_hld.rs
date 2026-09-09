// SPDX-License-Identifier: GPL-2.0
//
// Dependencies supplied by the surrounding kernel translation are intentionally
// referenced here rather than implemented in this file.

const SAFE_MAX_CPU_FREQ: c_ulong = 5_000_000_000;

extern "C" {
    static mut watchdog_thresh: c_int;

    fn smp_processor_id() -> c_int;
    fn cpufreq_get_hw_max_freq(cpu: c_uint) -> c_ulong;
    fn arm_pmu_irq_is_nmi() -> bool;
    fn hardlockup_detector_perf_adjust_period(new_period: u64);
    fn smp_call_on_cpu(
        cpu: c_int,
        func: Option<unsafe extern "C" fn(data: *mut c_void) -> c_int>,
        data: *mut c_void,
        wait: bool,
    ) -> c_int;
    fn cpufreq_register_notifier(nb: *mut notifier_block, list: c_uint) -> c_int;
}

type c_int = i32;
type c_uint = u32;
type c_ulong = usize;
type c_void = core::ffi::c_void;

#[repr(C)]
pub struct cpumask {
    _private: [u8; 0],
}

#[repr(C)]
pub struct cpufreq_policy {
    pub cpus: *mut cpumask,
}

#[repr(C)]
pub struct notifier_block {
    pub notifier_call:
        Option<unsafe extern "C" fn(nb: *mut notifier_block, val: c_ulong, data: *mut c_void) -> c_int>,
}

const CPUFREQ_CREATE_POLICY: c_ulong = 0;
const CPUFREQ_POLICY_NOTIFIER: c_uint = 0;
const NOTIFY_DONE: c_int = 0;

// The kernel's for_each_cpu macro iterates the CPUs in policy->cpus.
extern "C" {
    fn for_each_cpu_compat(mask: *mut cpumask, callback: unsafe extern "C" fn(c_int));
}

pub unsafe extern "C" fn hw_nmi_get_sample_period(watchdog_thresh_arg: c_int) -> u64 {
    let cpu: c_uint = smp_processor_id() as c_uint;
    let mut max_cpu_freq: c_ulong = cpufreq_get_hw_max_freq(cpu).wrapping_mul(1000usize);

    if max_cpu_freq == 0 {
        max_cpu_freq = SAFE_MAX_CPU_FREQ;
    }

    (max_cpu_freq as u64).wrapping_mul(watchdog_thresh_arg as u64)
}

pub unsafe extern "C" fn arch_perf_nmi_is_available() -> bool {
    // hardlockup_detector_perf_init() will success even if Pseudo-NMI turns off,
    // however, the pmu interrupts will act like a normal interrupt instead of
    // NMI and the hardlockup detector would be broken.
    arm_pmu_irq_is_nmi()
}

unsafe extern "C" fn watchdog_perf_update_period(_data: *mut c_void) -> c_int {
    let cpu: c_uint = smp_processor_id() as c_uint;
    let max_cpu_freq = cpufreq_get_hw_max_freq(cpu).wrapping_mul(1000usize);

    if max_cpu_freq == 0 {
        return 0;
    }

    let new_period = (watchdog_thresh as u64).wrapping_mul(max_cpu_freq as u64);
    hardlockup_detector_perf_adjust_period(new_period);

    0
}

unsafe extern "C" fn watchdog_freq_notifier_callback(
    _nb: *mut notifier_block,
    val: c_ulong,
    data: *mut c_void,
) -> c_int {
    let policy = data as *mut cpufreq_policy;

    if val != CPUFREQ_CREATE_POLICY {
        return NOTIFY_DONE;
    }

    // Let each online CPU related to the policy update the period by their
    // own. This will serialize with the framework on start/stop the lockup
    // detector (softlockup_{start,stop}_all) and avoid potential race
    // condition. Otherwise we may have below theoretical race condition:
    // (core 0/1 share the same policy)
    // [core 0]                      [core 1]
    //                               hardlockup_detector_event_create()
    //                                 hw_nmi_get_sample_period()
    // (cpufreq registered, notifier callback invoked)
    // watchdog_freq_notifier_callback()
    //   watchdog_perf_update_period()
    //   (since core 1's event's not yet created,
    //    the period is not set)
    //                                 perf_event_create_kernel_counter()
    //                                 (event's period is SAFE_MAX_CPU_FREQ)
    for_each_cpu_compat((*policy).cpus, watchdog_update_period_on_cpu);

    NOTIFY_DONE
}

unsafe extern "C" fn watchdog_update_period_on_cpu(cpu: c_int) {
    smp_call_on_cpu(cpu, Some(watchdog_perf_update_period), core::ptr::null_mut(), false);
}

static mut watchdog_freq_notifier: notifier_block = notifier_block {
    notifier_call: Some(watchdog_freq_notifier_callback),
};

unsafe extern "C" fn init_watchdog_freq_notifier() -> c_int {
    cpufreq_register_notifier(&raw mut watchdog_freq_notifier, CPUFREQ_POLICY_NOTIFIER)
}

// core_initcall(init_watchdog_freq_notifier);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
