/* SPDX-License-Identifier: GPL-2.0 */
/* Rust translation of linux/include/linux/nmi.h. */

/* Dependencies supplied by the surrounding kernel translation. */

#[cfg(feature = "CONFIG_LOCKUP_DETECTOR")]
extern "C" {
    pub fn lockup_detector_init();
    pub fn lockup_detector_retry_init();
    pub fn lockup_detector_soft_poweroff();

    pub static mut watchdog_user_enabled: ::core::ffi::c_int;
    pub static mut watchdog_thresh: ::core::ffi::c_int;
    pub static mut watchdog_enabled: ::core::ffi::c_ulong;
    pub static mut watchdog_hardlockup_miss_thresh: ::core::ffi::c_int;
    pub static mut watchdog_cpumask: cpumask;
    pub static mut watchdog_cpumask_bits: *mut ::core::ffi::c_ulong;

    #[cfg(feature = "CONFIG_SMP")]
    pub static mut sysctl_softlockup_all_cpu_backtrace: ::core::ffi::c_int;
    #[cfg(feature = "CONFIG_SMP")]
    pub static mut sysctl_hardlockup_all_cpu_backtrace: ::core::ffi::c_int;
}

#[cfg(not(feature = "CONFIG_LOCKUP_DETECTOR"))]
#[inline]
pub unsafe fn lockup_detector_init() {}
#[cfg(not(feature = "CONFIG_LOCKUP_DETECTOR"))]
#[inline]
pub unsafe fn lockup_detector_retry_init() {}
#[cfg(not(feature = "CONFIG_LOCKUP_DETECTOR"))]
#[inline]
pub unsafe fn lockup_detector_soft_poweroff() {}

#[cfg(not(feature = "CONFIG_SMP"))]
pub const sysctl_softlockup_all_cpu_backtrace: ::core::ffi::c_int = 0;
#[cfg(not(feature = "CONFIG_SMP"))]
pub const sysctl_hardlockup_all_cpu_backtrace: ::core::ffi::c_int = 0;

#[cfg(feature = "CONFIG_SOFTLOCKUP_DETECTOR")]
extern "C" {
    pub fn touch_softlockup_watchdog_sched();
    pub fn touch_softlockup_watchdog();
    pub fn touch_softlockup_watchdog_sync();
    pub fn touch_all_softlockup_watchdogs();
    pub static mut softlockup_panic: ::core::ffi::c_uint;
    pub fn lockup_detector_online_cpu(cpu: ::core::ffi::c_uint) -> ::core::ffi::c_int;
    pub fn lockup_detector_offline_cpu(cpu: ::core::ffi::c_uint) -> ::core::ffi::c_int;
}

#[cfg(not(feature = "CONFIG_SOFTLOCKUP_DETECTOR"))]
#[inline]
pub unsafe fn touch_softlockup_watchdog_sched() {}
#[cfg(not(feature = "CONFIG_SOFTLOCKUP_DETECTOR"))]
#[inline]
pub unsafe fn touch_softlockup_watchdog() {}
#[cfg(not(feature = "CONFIG_SOFTLOCKUP_DETECTOR"))]
#[inline]
pub unsafe fn touch_softlockup_watchdog_sync() {}
#[cfg(not(feature = "CONFIG_SOFTLOCKUP_DETECTOR"))]
#[inline]
pub unsafe fn touch_all_softlockup_watchdogs() {}

#[cfg(feature = "CONFIG_DETECT_HUNG_TASK")]
extern "C" { pub fn reset_hung_task_detector(); }
#[cfg(not(feature = "CONFIG_DETECT_HUNG_TASK"))]
#[inline]
pub unsafe fn reset_hung_task_detector() {}

pub const WATCHDOG_HARDLOCKUP_ENABLED_BIT: i32 = 0;
pub const WATCHDOG_SOFTLOCKUP_ENABLED_BIT: i32 = 1;
pub const WATCHDOG_HARDLOCKUP_ENABLED: i32 = 1 << WATCHDOG_HARDLOCKUP_ENABLED_BIT;
pub const WATCHDOG_SOFTLOCKUP_ENABLED: i32 = 1 << WATCHDOG_SOFTLOCKUP_ENABLED_BIT;

#[cfg(feature = "CONFIG_HARDLOCKUP_DETECTOR")]
extern "C" {
    pub fn hardlockup_detector_disable();
    pub static mut hardlockup_panic: ::core::ffi::c_uint;
    pub static mut hardlockup_si_mask: ::core::ffi::c_ulong;
}
#[cfg(not(feature = "CONFIG_HARDLOCKUP_DETECTOR"))]
#[inline]
pub unsafe fn hardlockup_detector_disable() {}

#[cfg(any(feature = "CONFIG_HARDLOCKUP_DETECTOR", feature = "CONFIG_HARDLOCKUP_DETECTOR_SPARC64"))]
extern "C" { pub fn arch_touch_nmi_watchdog(); }
#[cfg(not(any(feature = "CONFIG_HARDLOCKUP_DETECTOR", feature = "CONFIG_HARDLOCKUP_DETECTOR_SPARC64")))]
#[inline]
pub unsafe fn arch_touch_nmi_watchdog() {}

#[cfg(feature = "CONFIG_HARDLOCKUP_DETECTOR_COUNTS_HRTIMER")]
extern "C" {
    pub fn watchdog_hardlockup_touch_cpu(cpu: ::core::ffi::c_uint);
    pub fn watchdog_hardlockup_check(cpu: ::core::ffi::c_uint, regs: *mut pt_regs);
}

#[cfg(feature = "CONFIG_HARDLOCKUP_DETECTOR_PERF")]
extern "C" {
    pub fn hardlockup_detector_perf_stop();
    pub fn hardlockup_detector_perf_restart();
    pub fn hardlockup_config_perf_event(string: *const ::core::ffi::c_char);
    pub fn hardlockup_detector_perf_adjust_period(period: u64);
}
#[cfg(not(feature = "CONFIG_HARDLOCKUP_DETECTOR_PERF"))]
#[inline] pub unsafe fn hardlockup_detector_perf_stop() {}
#[cfg(not(feature = "CONFIG_HARDLOCKUP_DETECTOR_PERF"))]
#[inline] pub unsafe fn hardlockup_detector_perf_restart() {}
#[cfg(not(feature = "CONFIG_HARDLOCKUP_DETECTOR_PERF"))]
#[inline] pub unsafe fn hardlockup_config_perf_event(_: *const ::core::ffi::c_char) {}
#[cfg(not(feature = "CONFIG_HARDLOCKUP_DETECTOR_PERF"))]
#[inline] pub unsafe fn hardlockup_detector_perf_adjust_period(_: u64) {}

extern "C" {
    pub fn watchdog_hardlockup_stop();
    pub fn watchdog_hardlockup_start();
    pub fn watchdog_hardlockup_probe() -> ::core::ffi::c_int;
    pub fn watchdog_hardlockup_enable(cpu: ::core::ffi::c_uint);
    pub fn watchdog_hardlockup_disable(cpu: ::core::ffi::c_uint);
    pub fn lockup_detector_reconfigure();
}

#[cfg(feature = "CONFIG_HARDLOCKUP_DETECTOR_BUDDY")]
extern "C" { pub fn watchdog_buddy_check_hardlockup(hrtimer_interrupts: ::core::ffi::c_int); }
#[cfg(not(feature = "CONFIG_HARDLOCKUP_DETECTOR_BUDDY"))]
#[inline]
pub unsafe fn watchdog_buddy_check_hardlockup(_: ::core::ffi::c_int) {}

#[inline]
pub unsafe fn touch_nmi_watchdog() {
    arch_touch_nmi_watchdog();
    touch_softlockup_watchdog();
}

#[cfg(feature = "arch_trigger_cpumask_backtrace")]
extern "C" {
    pub fn cpumask_backtrace(mask: *const cpumask_t, exclude_cpu: ::core::ffi::c_int);
    pub fn nmi_trigger_cpumask_backtrace(mask: *const cpumask_t, exclude_cpu: ::core::ffi::c_int, raise: unsafe extern "C" fn(*mut cpumask_t));
    pub fn nmi_cpu_backtrace(regs: *mut pt_regs) -> bool;
}

#[cfg(feature = "arch_trigger_cpumask_backtrace")]
#[inline]
pub unsafe fn trigger_all_cpu_backtrace() -> bool {
    cpumask_backtrace(cpu_online_mask, -1); true
}
#[cfg(feature = "arch_trigger_cpumask_backtrace")]
#[inline]
pub unsafe fn trigger_allbutcpu_cpu_backtrace(exclude_cpu: ::core::ffi::c_int) -> bool {
    cpumask_backtrace(cpu_online_mask, exclude_cpu); true
}
#[cfg(feature = "arch_trigger_cpumask_backtrace")]
#[inline]
pub unsafe fn trigger_cpumask_backtrace(mask: *mut cpumask) -> bool {
    cpumask_backtrace(mask, -1); true
}
#[cfg(feature = "arch_trigger_cpumask_backtrace")]
#[inline]
pub unsafe fn trigger_single_cpu_backtrace(cpu: ::core::ffi::c_int) -> bool {
    cpumask_backtrace(cpumask_of(cpu), -1); true
}

#[cfg(not(feature = "arch_trigger_cpumask_backtrace"))]
#[inline] pub unsafe fn trigger_all_cpu_backtrace() -> bool { false }
#[cfg(not(feature = "arch_trigger_cpumask_backtrace"))]
#[inline] pub unsafe fn trigger_allbutcpu_cpu_backtrace(_: ::core::ffi::c_int) -> bool { false }
#[cfg(not(feature = "arch_trigger_cpumask_backtrace"))]
#[inline] pub unsafe fn trigger_cpumask_backtrace(_: *mut cpumask) -> bool { false }
#[cfg(not(feature = "arch_trigger_cpumask_backtrace"))]
#[inline] pub unsafe fn trigger_single_cpu_backtrace(_: ::core::ffi::c_int) -> bool { false }

#[cfg(feature = "CONFIG_HARDLOCKUP_DETECTOR_PERF")]
extern "C" {
    pub fn hw_nmi_get_sample_period(watchdog_thresh: ::core::ffi::c_int) -> u64;
    pub fn arch_perf_nmi_is_available() -> bool;
}

#[cfg(all(feature = "CONFIG_HARDLOCKUP_CHECK_TIMESTAMP", feature = "CONFIG_HARDLOCKUP_DETECTOR_PERF"))]
extern "C" { pub fn watchdog_update_hrtimer_threshold(period: u64); }
#[cfg(not(all(feature = "CONFIG_HARDLOCKUP_CHECK_TIMESTAMP", feature = "CONFIG_HARDLOCKUP_DETECTOR_PERF")))]
#[inline]
pub unsafe fn watchdog_update_hrtimer_threshold(_: u64) {}

#[cfg(feature = "CONFIG_NMI_CHECK_CPU")]
extern "C" {
    pub fn nmi_backtrace_stall_snap(btp: *const cpumask);
    pub fn nmi_backtrace_stall_check(btp: *const cpumask);
}
#[cfg(not(feature = "CONFIG_NMI_CHECK_CPU"))]
#[inline] pub unsafe fn nmi_backtrace_stall_snap(_: *const cpumask) {}
#[cfg(not(feature = "CONFIG_NMI_CHECK_CPU"))]
#[inline] pub unsafe fn nmi_backtrace_stall_check(_: *const cpumask) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
