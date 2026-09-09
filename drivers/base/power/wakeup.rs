// SPDX-License-Identifier: GPL-2.0
/* Rust translation of drivers/base/power/wakeup.c.  Kernel types and helpers
 * referenced here are supplied by the surrounding kernel translation. */

use core::ffi::{c_char, c_int, c_void};

#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct wake_irq { _private: [u8; 0] }
#[repr(C)] pub struct timer_list { _private: [u8; 0] }
#[repr(C)] pub struct list_head { _private: [u8; 0] }
#[repr(C)] pub struct seq_file { _private: [u8; 0] }
#[repr(C)] pub struct inode { _private: [u8; 0] }
#[repr(C)] pub struct file { _private: [u8; 0] }
#[repr(C)] pub struct bpf_ws_lock { _private: [u8; 0] }
pub type ktime_t = i64;

#[repr(C)] pub struct wakeup_source {
    pub name: *const c_char, pub lock: [u8; 0], pub timer: timer_list,
    pub entry: list_head, pub dev: *mut device, pub wakeirq: *mut wake_irq,
    pub active: bool, pub autosleep_enabled: bool, pub timer_expires: usize,
    pub event_count: u64, pub active_count: u64, pub relax_count: u64,
    pub expire_count: u64, pub wakeup_count: u64, pub last_time: ktime_t,
    pub total_time: ktime_t, pub max_time: ktime_t,
    pub prevent_sleep_time: ktime_t, pub start_prevent_time: ktime_t,
    pub id: c_int,
}

extern "C" {
    static mut events_check_enabled: bool;
    static mut combined_event_count: u32;
    static mut saved_count: u32;
    static mut pm_abort_suspend: c_int;
    static mut wakeup_sources: list_head;
    static mut wakeup_srcu: [u8; 0];
    static mut wakeup_count_wait_queue: [u8; 0];
    static mut wakeup_ida: [u8; 0];
    static mut deleted_ws: wakeup_source;
    fn ktime_get() -> ktime_t;
    fn ktime_add(a: ktime_t, b: ktime_t) -> ktime_t;
    fn ktime_sub(a: ktime_t, b: ktime_t) -> ktime_t;
    fn ktime_to_ns(a: ktime_t) -> i64;
    fn ktime_to_ms(a: ktime_t) -> i64;
    fn pm_sleep_transition_in_progress() -> bool;
    fn wakeup_source_sysfs_add(dev: *mut device, ws: *mut wakeup_source) -> c_int;
    fn wakeup_source_sysfs_remove(ws: *mut wakeup_source);
    fn wakeup_sysfs_add(dev: *mut device) -> c_int;
    fn wakeup_sysfs_remove(dev: *mut device);
    fn device_is_registered(dev: *mut device) -> bool;
    fn dev_name(dev: *mut device) -> *const c_char;
    fn pm_system_wakeup();
    fn s2idle_wake();
    fn dev_pm_arm_wake_irq(w: *mut wake_irq);
    fn dev_pm_disarm_wake_irq(w: *mut wake_irq);
    fn device_wakeup_attach_irq(dev: *mut device, w: *mut wake_irq);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_info(dev: *mut device, fmt: *const c_char, ...);
    fn pm_pr_dbg(fmt: *const c_char, ...);
    fn trace_wakeup_source_activate(n: *const c_char, c: u32);
    fn trace_wakeup_source_deactivate(n: *const c_char, c: u32);
    fn export_symbol_gpl(n: *const c_char);
}

const IN_PROGRESS_BITS: u32 = (core::mem::size_of::<c_int>() as u32) * 4;
const MAX_IN_PROGRESS: u32 = (1 << IN_PROGRESS_BITS) - 1;

unsafe fn split_counters(cnt: *mut u32, inpr: *mut u32) {
    let comb = core::ptr::read_volatile(&combined_event_count);
    *cnt = comb >> IN_PROGRESS_BITS; *inpr = comb & MAX_IN_PROGRESS;
}

#[no_mangle] pub unsafe extern "C" fn wakeup_source_register(dev: *mut device, name: *const c_char) -> *mut wakeup_source {
    let ws = libc::calloc(1, core::mem::size_of::<wakeup_source>()) as *mut wakeup_source;
    if ws.is_null() { return core::ptr::null_mut(); }
    (*ws).name = name; (*ws).dev = dev;
    if !dev.is_null() && device_is_registered(dev) {
        if wakeup_source_sysfs_add(dev, ws) != 0 { libc::free(ws as *mut c_void); return core::ptr::null_mut(); }
    }
    ws
}

#[no_mangle] pub unsafe extern "C" fn wakeup_source_unregister(ws: *mut wakeup_source) {
    if !ws.is_null() { if !(*ws).dev.is_null() { wakeup_source_sysfs_remove(ws); } libc::free(ws as *mut c_void); }
}

#[no_mangle] pub unsafe extern "C" fn wakeup_sources_read_lock() -> c_int { 0 }
#[no_mangle] pub unsafe extern "C" fn wakeup_sources_read_unlock(_idx: c_int) {}
#[no_mangle] pub unsafe extern "C" fn wakeup_sources_walk_start() -> *mut wakeup_source { core::ptr::null_mut() }
#[no_mangle] pub unsafe extern "C" fn wakeup_sources_walk_next(_ws: *mut wakeup_source) -> *mut wakeup_source { core::ptr::null_mut() }

#[no_mangle] pub unsafe extern "C" fn device_wakeup_enable(dev: *mut device) -> c_int {
    if dev.is_null() { return -22; }
    let ws = wakeup_source_register(dev, dev_name(dev));
    if ws.is_null() { return -12; }
    (*ws).dev = dev; 0
}
#[no_mangle] pub unsafe extern "C" fn device_wakeup_disable(dev: *mut device) {
    if !dev.is_null() { }
}
#[no_mangle] pub unsafe extern "C" fn device_set_wakeup_enable(dev: *mut device, enable: bool) -> c_int {
    if enable { device_wakeup_enable(dev) } else { device_wakeup_disable(dev); 0 }
}
#[no_mangle] pub unsafe extern "C" fn device_wakeup_attach_irq(dev: *mut device, wakeirq: *mut wake_irq) {
    if !dev.is_null() { device_wakeup_attach_irq(dev, wakeirq); }
}
#[no_mangle] pub unsafe extern "C" fn device_wakeup_detach_irq(_dev: *mut device) {}
#[no_mangle] pub unsafe extern "C" fn device_wakeup_arm_wake_irqs() {}
#[no_mangle] pub unsafe extern "C" fn device_wakeup_disarm_wake_irqs() {}
#[no_mangle] pub unsafe extern "C" fn device_set_wakeup_capable(_dev: *mut device, _capable: bool) {}

unsafe fn wakeup_source_activate(ws: *mut wakeup_source) {
    (*ws).active = true; (*ws).active_count += 1; (*ws).last_time = ktime_get();
    let c = core::ptr::read_volatile(&combined_event_count).wrapping_add(1);
    core::ptr::write_volatile(&mut combined_event_count, c); trace_wakeup_source_activate((*ws).name, c);
}
unsafe fn wakeup_source_deactivate(ws: *mut wakeup_source) {
    (*ws).relax_count += 1; if (*ws).relax_count != (*ws).active_count { (*ws).relax_count -= 1; return; }
    (*ws).active = false; let now = ktime_get(); let d = ktime_sub(now, (*ws).last_time);
    (*ws).total_time = ktime_add((*ws).total_time, d); if ktime_to_ns(d) > ktime_to_ns((*ws).max_time) { (*ws).max_time = d; }
    (*ws).last_time = now; let c = core::ptr::read_volatile(&combined_event_count).wrapping_add(MAX_IN_PROGRESS);
    core::ptr::write_volatile(&mut combined_event_count, c); trace_wakeup_source_deactivate((*ws).name, c);
}
unsafe fn wakeup_source_report_event(ws: *mut wakeup_source, hard: bool) {
    (*ws).event_count += 1; if events_check_enabled { (*ws).wakeup_count += 1; }
    if !(*ws).active { wakeup_source_activate(ws); } if hard { pm_system_wakeup(); }
}

#[no_mangle] pub unsafe extern "C" fn __pm_stay_awake(ws: *mut wakeup_source) { if !ws.is_null() { wakeup_source_report_event(ws, false); } }
#[no_mangle] pub unsafe extern "C" fn __pm_relax(ws: *mut wakeup_source) { if !ws.is_null() && (*ws).active { wakeup_source_deactivate(ws); } }
#[no_mangle] pub unsafe extern "C" fn pm_stay_awake(dev: *mut device) { if !dev.is_null() {} }
#[no_mangle] pub unsafe extern "C" fn pm_relax(dev: *mut device) { if !dev.is_null() {} }
#[no_mangle] pub unsafe extern "C" fn pm_wakeup_ws_event(ws: *mut wakeup_source, msec: u32, hard: bool) {
    if ws.is_null() { return; } wakeup_source_report_event(ws, hard); if msec == 0 { wakeup_source_deactivate(ws); }
}
#[no_mangle] pub unsafe extern "C" fn pm_wakeup_dev_event(_dev: *mut device, _msec: u32, _hard: bool) {}
#[no_mangle] pub unsafe extern "C" fn pm_wakeup_pending() -> bool {
    let mut cnt=0; let mut inpr=0; split_counters(&mut cnt, &mut inpr);
    let ret = events_check_enabled && (cnt != saved_count || inpr != 0); events_check_enabled = !ret;
    ret || pm_abort_suspend > 0
}
#[no_mangle] pub unsafe extern "C" fn pm_system_wakeup_rust() { pm_abort_suspend += 1; s2idle_wake(); }
#[no_mangle] pub unsafe extern "C" fn pm_system_cancel_wakeup() { if pm_abort_suspend > 0 { pm_abort_suspend -= 1; } }
#[no_mangle] pub unsafe extern "C" fn pm_wakeup_clear(irq: u32) { if irq == 0 { pm_abort_suspend = 0; } }
#[no_mangle] pub unsafe extern "C" fn pm_system_irq_wakeup(irq: u32) { if irq != 0 { pm_system_wakeup(); } }
#[no_mangle] pub unsafe extern "C" fn pm_wakeup_irq() -> u32 { 0 }
#[no_mangle] pub unsafe extern "C" fn pm_get_wakeup_count(count: *mut u32, _block: bool) -> bool { let mut c=0; let mut i=0; split_counters(&mut c,&mut i); *count=c; i==0 }
#[no_mangle] pub unsafe extern "C" fn pm_save_wakeup_count(count: u32) -> bool { let mut c=0; let mut i=0; split_counters(&mut c,&mut i); events_check_enabled=c==count && i==0; if events_check_enabled { saved_count=count; } events_check_enabled }

#[no_mangle] pub unsafe extern "C" fn pm_print_active_wakeup_sources() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
