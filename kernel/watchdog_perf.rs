// SPDX-License-Identifier: GPL-2.0
/*
 * Detect hard lockups on a system using perf
 *
 * started by Don Zickus, Copyright (C) 2010 Red Hat, Inc.
 *
 * Note: Most of this code is borrowed heavily from the original softlockup
 * detector, so thanks to Ingo for the initial implementation.
 * Some chunks also taken from the old x86-specific nmi watchdog code, thanks
 * to those contributors as well.
 */

// Kernel includes and the `pr_fmt` definition are supplied by the surrounding
// Linux kernel environment.

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct PerfEventAttr {
    pub type_: u32,
    pub size: u32,
    pub config: u64,
    pub sample_period: u64,
    pub pinned: u64,
    pub disabled: u64,
}

#[repr(C)]
pub struct PerfEventHw { pub interrupts: u64 }
#[repr(C)]
pub struct PerfEvent { pub hw: PerfEventHw, pub attr: PerfEventAttr }
#[repr(C)]
pub struct PerfSampleData;
#[repr(C)]
pub struct PtRegs;

extern "C" {
    static mut watchdog_thresh: u64;
    static mut watchdog_enabled: u32;
    fn panic_in_progress() -> bool;
    fn watchdog_hardlockup_check(cpu: c_int, regs: *mut PtRegs);
    fn smp_processor_id() -> c_int;
    fn raw_smp_processor_id() -> u32;
    fn hw_nmi_get_sample_period(threshold: u64) -> u64;
    fn perf_event_create_kernel_counter(attr: *mut PerfEventAttr, cpu: u32,
        task: *mut c_void, callback: unsafe extern "C" fn(*mut PerfEvent, *mut PerfSampleData, *mut PtRegs), context: *mut c_void) -> *mut PerfEvent;
    fn perf_event_enable(event: *mut PerfEvent);
    fn perf_event_disable(event: *mut PerfEvent);
    fn perf_event_release_kernel(event: *mut PerfEvent);
    fn perf_event_period(event: *mut PerfEvent, period: u64) -> c_int;
    fn ptr_err(event: *mut PerfEvent) -> isize;
    fn is_err(event: *mut PerfEvent) -> bool;
    fn lockdep_assert_cpus_held();
    fn arch_perf_nmi_is_available() -> bool;
    fn kstrtoull(s: *const c_char, base: u32, result: *mut u64) -> c_int;
    fn strscpy(dst: *mut c_char, src: *const c_char, count: usize) -> isize;
    fn strchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn pr_debug(fmt: *const c_char, ...);
    fn pr_info(fmt: *const c_char, ...);
    fn pr_err(fmt: *const c_char, ...);
    fn warn_on_once(condition: bool);
    fn warn_once(condition: bool, fmt: *const c_char, ...);
    fn atomic_fetch_inc(value: *mut c_int) -> c_int;
    fn atomic_dec(value: *mut c_int);
}

const PERF_TYPE_HARDWARE: u32 = 0;
const PERF_TYPE_RAW: u32 = 4;
const PERF_COUNT_HW_CPU_CYCLES: u64 = 0;
const WATCHDOG_HARDLOCKUP_ENABLED: u32 = 1;

static mut watchdog_ev: *mut PerfEvent = core::ptr::null_mut();
static mut watchdog_cpus: c_int = 0;
static mut wd_hw_attr: PerfEventAttr = PerfEventAttr { type_: PERF_TYPE_HARDWARE, config: PERF_COUNT_HW_CPU_CYCLES, size: core::mem::size_of::<PerfEventAttr>() as u32, pinned: 1, disabled: 1, sample_period: 0 };
static mut fallback_wd_hw_attr: PerfEventAttr = PerfEventAttr { type_: PERF_TYPE_HARDWARE, config: PERF_COUNT_HW_CPU_CYCLES, size: core::mem::size_of::<PerfEventAttr>() as u32, pinned: 1, disabled: 1, sample_period: 0 };

#[cfg(feature = "CONFIG_HARDLOCKUP_CHECK_TIMESTAMP")]
static mut last_timestamp: i64 = 0;
#[cfg(feature = "CONFIG_HARDLOCKUP_CHECK_TIMESTAMP")]
static mut nmi_rearmed: u32 = 0;
#[cfg(feature = "CONFIG_HARDLOCKUP_CHECK_TIMESTAMP")]
static mut watchdog_hrtimer_sample_threshold: i64 = 0;

#[cfg(feature = "CONFIG_HARDLOCKUP_CHECK_TIMESTAMP")]
extern "C" { fn ktime_get_mono_fast_ns() -> i64; }

#[cfg(feature = "CONFIG_HARDLOCKUP_CHECK_TIMESTAMP")]
#[no_mangle]
pub unsafe extern "C" fn watchdog_update_hrtimer_threshold(period: u64) {
    watchdog_hrtimer_sample_threshold = (period * 2) as i64;
}

#[cfg(feature = "CONFIG_HARDLOCKUP_CHECK_TIMESTAMP")]
unsafe fn watchdog_check_timestamp() -> bool {
    let now = ktime_get_mono_fast_ns();
    let delta = now - last_timestamp;
    if delta < watchdog_hrtimer_sample_threshold {
        nmi_rearmed = nmi_rearmed.wrapping_add(1);
        if nmi_rearmed < 10 { return false; }
    }
    nmi_rearmed = 0;
    last_timestamp = now;
    true
}

#[cfg(feature = "CONFIG_HARDLOCKUP_CHECK_TIMESTAMP")]
unsafe fn watchdog_init_timestamp() { nmi_rearmed = 0; last_timestamp = ktime_get_mono_fast_ns(); }
#[cfg(not(feature = "CONFIG_HARDLOCKUP_CHECK_TIMESTAMP"))]
unsafe fn watchdog_check_timestamp() -> bool { true }
#[cfg(not(feature = "CONFIG_HARDLOCKUP_CHECK_TIMESTAMP"))]
unsafe fn watchdog_init_timestamp() {}

unsafe extern "C" fn watchdog_overflow_callback(event: *mut PerfEvent, _data: *mut PerfSampleData, regs: *mut PtRegs) {
    (*event).hw.interrupts = 0;
    if panic_in_progress() || !watchdog_check_timestamp() { return; }
    watchdog_hardlockup_check(smp_processor_id(), regs);
}

unsafe fn hardlockup_detector_event_create(cpu: u32) -> *mut PerfEvent {
    wd_hw_attr.sample_period = hw_nmi_get_sample_period(watchdog_thresh);
    let mut evt = perf_event_create_kernel_counter(&mut wd_hw_attr, cpu, core::ptr::null_mut(), watchdog_overflow_callback, core::ptr::null_mut());
    if is_err(evt) {
        fallback_wd_hw_attr.sample_period = hw_nmi_get_sample_period(watchdog_thresh);
        evt = perf_event_create_kernel_counter(&mut fallback_wd_hw_attr, cpu, core::ptr::null_mut(), watchdog_overflow_callback, core::ptr::null_mut());
    }
    evt
}

#[no_mangle]
pub unsafe extern "C" fn watchdog_hardlockup_enable(cpu: u32) {
    warn_on_once(cpu as c_int != smp_processor_id());
    let evt = hardlockup_detector_event_create(cpu);
    if is_err(evt) { pr_debug(b"Perf event create on CPU %d failed with %ld\0".as_ptr() as _, cpu, ptr_err(evt)); return; }
    if atomic_fetch_inc(&mut watchdog_cpus) == 0 { pr_info(b"Enabled. Permanently consumes one hw-PMU counter.\n\0".as_ptr() as _); }
    warn_once(!watchdog_ev.is_null(), b"unexpected watchdog_ev leak\0".as_ptr() as _);
    watchdog_ev = evt;
    watchdog_init_timestamp();
    perf_event_enable(evt);
}

#[no_mangle]
pub unsafe extern "C" fn watchdog_hardlockup_disable(cpu: u32) {
    let event = watchdog_ev;
    warn_on_once(cpu as c_int != smp_processor_id());
    if !event.is_null() { perf_event_disable(event); perf_event_release_kernel(event); watchdog_ev = core::ptr::null_mut(); atomic_dec(&mut watchdog_cpus); }
}

#[no_mangle]
pub unsafe extern "C" fn hardlockup_detector_perf_adjust_period(period: u64) {
    let event = watchdog_ev;
    if watchdog_enabled & WATCHDOG_HARDLOCKUP_ENABLED == 0 || event.is_null() || (*event).attr.sample_period == period { return; }
    if perf_event_period(event, period) != 0 { pr_err(b"failed to change period to %llu\n\0".as_ptr() as _, period); }
}

#[no_mangle]
pub unsafe extern "C" fn hardlockup_detector_perf_stop() {
    lockdep_assert_cpus_held();
    // for_each_online_cpu(cpu)
    for cpu in 0..0 { let _ = cpu; let event = watchdog_ev; if !event.is_null() { perf_event_disable(event); } }
}

#[no_mangle]
pub unsafe extern "C" fn hardlockup_detector_perf_restart() {
    lockdep_assert_cpus_held();
    if watchdog_enabled & WATCHDOG_HARDLOCKUP_ENABLED == 0 { return; }
    // for_each_online_cpu(cpu)
    for cpu in 0..0 { let _ = cpu; let event = watchdog_ev; if !event.is_null() { perf_event_enable(event); } }
}

#[no_mangle]
pub unsafe extern "C" fn watchdog_hardlockup_probe() -> c_int {
    if !arch_perf_nmi_is_available() { return -19; }
    if hw_nmi_get_sample_period(watchdog_thresh) == 0 { return -22; }
    let evt = hardlockup_detector_event_create(raw_smp_processor_id());
    if is_err(evt) { pr_info(b"Perf NMI watchdog permanently disabled\0".as_ptr() as _); ptr_err(evt) as c_int } else { perf_event_release_kernel(evt); 0 }
}

#[no_mangle]
pub unsafe extern "C" fn hardlockup_config_perf_event(str_: *const c_char) {
    let mut config = 0u64;
    let mut buf = [0i8; 24];
    let comma = strchr(str_, ',' as c_int);
    if comma.is_null() {
        if kstrtoull(str_, 16, &mut config) != 0 { return; }
    } else {
        let len = comma.offset_from(str_) as usize;
        if len > core::mem::size_of_val(&buf) { return; }
        strscpy(buf.as_mut_ptr(), str_, len);
        if kstrtoull(buf.as_ptr(), 16, &mut config) != 0 { return; }
    }
    wd_hw_attr.type_ = PERF_TYPE_RAW;
    wd_hw_attr.config = config;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
