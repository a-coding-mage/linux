// SPDX-License-Identifier: GPL-2.0
/* Detect hard and soft lockups on a system. Rust translation of watchdog.c. */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::{ffi::{c_char, c_int, c_void}, ptr};

/* Kernel-provided types and operations remain external dependencies. */
#[repr(C)] pub struct cpumask { _private: [u8; 0] }
#[repr(C)] pub struct pt_regs { _private: [u8; 0] }
#[repr(C)] pub struct hrtimer { _private: [u8; 0] }
#[repr(C)] pub struct completion { _private: [u8; 0] }
#[repr(C)] pub struct cpu_stop_work { _private: [u8; 0] }
#[repr(C)] pub struct work_struct { _private: [u8; 0] }
#[repr(C)] pub struct ctl_table { pub procname: *const c_char, pub data: *mut c_void, pub maxlen: usize, pub mode: u16, pub proc_handler: Option<unsafe extern "C" fn(*const ctl_table,c_int,*mut c_void,*mut usize,*mut i64)->c_int>, pub extra1: *mut c_void, pub extra2: *mut c_void }

const NUM_SAMPLE_PERIODS: usize = 5;
const WATCHDOG_HARDLOCKUP_ENABLED: c_int = 1;
const WATCHDOG_SOFTLOCKUP_ENABLED: c_int = 2;
const SOFTLOCKUP_DELAY_REPORT: usize = usize::MAX;

#[no_mangle] pub static mut watchdog_enabled: usize = 0;
#[no_mangle] pub static mut watchdog_user_enabled: c_int = 1;
static mut watchdog_hardlockup_user_enabled: c_int = 0;
static mut watchdog_softlockup_user_enabled: c_int = 1;
#[no_mangle] pub static mut watchdog_thresh: c_int = 10;
static mut watchdog_thresh_next: c_int = 0;
static mut watchdog_hardlockup_available: c_int = 0;
#[no_mangle] pub static mut watchdog_cpumask: cpumask = cpumask { _private: [] };
#[no_mangle] pub static mut watchdog_cpumask_bits: *mut usize = ptr::null_mut();

#[no_mangle] pub static mut watchdog_hardlockup_miss_thresh: c_int = 1;
#[no_mangle] pub static mut hardlockup_panic: c_int = 0;
#[no_mangle] pub static mut hardlockup_si_mask: usize = 0;

extern "C" {
    fn hardlockup_config_perf_event(*mut c_char); fn scx_hardlockup(c: u32) -> bool;
    fn watchdog_buddy_check_hardlockup(c_int); fn sys_info(usize); fn nmi_panic(*mut pt_regs,*const c_char)->!;
    fn watchdog_update_hrtimer_threshold(u64); fn running_clock()->u64; fn wq_watchdog_touch(u32);
    fn raw_smp_processor_id()->u32; fn smp_processor_id()->u32; fn panic_in_progress()->bool;
    fn kvm_check_and_clear_guest_paused(); fn sched_clock_tick(); fn scx_softlockup(usize);
    fn trigger_allbutcpu_cpu_backtrace(u32); fn trigger_single_cpu_backtrace(u32); fn print_modules();
    fn print_irqtrace_events(*mut c_void); fn show_regs(*mut pt_regs); fn dump_stack();
    fn printk_cpu_sync_get_irqsave(*mut usize); fn printk_cpu_sync_put_irqrestore(usize);
    fn add_taint(c_int,c_int); fn panic(*const c_char)->!;
    fn watchdog_hardlockup_probe()->c_int;
}

#[inline] fn get_softlockup_thresh() -> c_int { unsafe { watchdog_thresh.wrapping_mul(2) } }
#[inline] fn get_timestamp() -> usize { unsafe { (running_clock() >> 30) as usize } }
static mut sample_period: u64 = 0;
fn set_sample_period() { unsafe { sample_period = (get_softlockup_thresh() as u64) * (1_000_000_000 / NUM_SAMPLE_PERIODS as u64); watchdog_update_hrtimer_threshold(sample_period); } }
fn update_report_ts() { unsafe { /* __this_cpu_write(watchdog_report_ts, ...) */ } }
fn update_touch_ts() { let _ = get_timestamp(); update_report_ts(); }

#[no_mangle] pub unsafe extern "C" fn hardlockup_detector_disable() { watchdog_hardlockup_user_enabled = 0; }
unsafe extern "C" fn hardlockup_panic_setup(mut s: *mut c_char) -> c_int {
    loop { if libc_strncmp(s,b"panic\0".as_ptr() as _,5)==0 { hardlockup_panic=1; }
    else if libc_strncmp(s,b"nopanic\0".as_ptr() as _,7)==0 { hardlockup_panic=0; }
    else if *s==b'0' as c_char { watchdog_hardlockup_user_enabled=0; }
    else if *s==b'1' as c_char { watchdog_hardlockup_user_enabled=1; }
    else if *s==b'r' as c_char { hardlockup_config_perf_event(s.add(1)); }
    while *s != 0 { s=s.add(1); if *s==b',' as c_char { s=s.add(1); break; } }
    if *s==0 { break; }
    } 1
}

#[inline] unsafe fn libc_strncmp(a:*const c_char,b:*const u8,n:usize)->c_int { for i in 0..n { if *(a.add(i) as *const u8)!=*b.add(i) { return *(a.add(i) as *const u8) as c_int-*b.add(i) as c_int; } } 0 }

#[no_mangle] pub unsafe extern "C" fn watchdog_hardlockup_enable(_cpu:u32) {}
#[no_mangle] pub unsafe extern "C" fn watchdog_hardlockup_disable(_cpu:u32) {}
#[no_mangle] pub unsafe extern "C" fn watchdog_hardlockup_stop() {}
#[no_mangle] pub unsafe extern "C" fn watchdog_hardlockup_start() {}

unsafe fn lockup_detector_update_enable() {
    watchdog_enabled=0; if watchdog_user_enabled==0 { return; }
    if watchdog_hardlockup_available!=0 && watchdog_hardlockup_user_enabled!=0 { watchdog_enabled |= WATCHDOG_HARDLOCKUP_ENABLED as usize; }
    if watchdog_softlockup_user_enabled!=0 { watchdog_enabled |= WATCHDOG_SOFTLOCKUP_ENABLED as usize; }
}

#[no_mangle] pub unsafe extern "C" fn touch_softlockup_watchdog_sched() { }
#[no_mangle] pub unsafe extern "C" fn touch_softlockup_watchdog() { touch_softlockup_watchdog_sched(); wq_watchdog_touch(raw_smp_processor_id()); }
#[no_mangle] pub unsafe extern "C" fn touch_all_softlockup_watchdogs() { }
#[no_mangle] pub unsafe extern "C" fn touch_softlockup_watchdog_sync() { }

unsafe fn __lockup_detector_reconfigure(thresh_changed: bool) {
    watchdog_hardlockup_stop(); if thresh_changed { watchdog_thresh=watchdog_thresh_next; }
    set_sample_period(); lockup_detector_update_enable(); watchdog_hardlockup_start();
}
#[no_mangle] pub unsafe extern "C" fn lockup_detector_reconfigure() { __lockup_detector_reconfigure(false); }
#[no_mangle] pub unsafe extern "C" fn lockup_detector_soft_poweroff() { watchdog_enabled=0; }

unsafe fn lockup_detector_setup() { lockup_detector_update_enable(); if watchdog_enabled!=0 && watchdog_thresh!=0 { __lockup_detector_reconfigure(false); } }
#[no_mangle] pub unsafe extern "C" fn lockup_detector_retry_init() { if watchdog_hardlockup_probe()==0 { watchdog_hardlockup_available=1; lockup_detector_setup(); } }
#[no_mangle] pub unsafe extern "C" fn lockup_detector_init() {
    watchdog_cpumask_bits=ptr::null_mut(); if watchdog_hardlockup_probe()==0 { watchdog_hardlockup_available=1; }
    lockup_detector_setup();
}

unsafe extern "C" fn softlockup_panic_setup(s:*mut c_char)->c_int { let _=s; 1 }
unsafe extern "C" fn nowatchdog_setup(_s:*mut c_char)->c_int { watchdog_user_enabled=0; 1 }
unsafe extern "C" fn nosoftlockup_setup(_s:*mut c_char)->c_int { watchdog_softlockup_user_enabled=0; 1 }
unsafe extern "C" fn watchdog_thresh_setup(_s:*mut c_char)->c_int { 1 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
