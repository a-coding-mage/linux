// SPDX-License-Identifier: GPL-2.0-only
/* Direct Rust translation of kernel/power/main.c. */

#![allow(dead_code, non_camel_case_types, non_snake_case, non_upper_case_globals)]

// Kernel-provided types, constants, globals, and functions are intentionally
// left as external dependencies, as in the original C translation unit.
extern "C" {
    static mut gfp_allowed_mask: usize;
    static mut system_transition_mutex: usize;
    static mut pm_async_enabled: i32;
    static mut pm_wq: *mut workqueue_struct;
    fn mutex_is_locked(m: *mut usize) -> bool;
    fn mutex_lock(m: *mut usize);
    fn mutex_unlock(m: *mut usize);
    fn ktime_get() -> i64;
    fn ktime_to_ms(v: i64) -> i64;
    fn ksys_sync();
    fn pm_wakeup_clear(v: u32);
    fn pm_wakeup_pending() -> bool;
    fn pm_autosleep_lock() -> i32;
    fn pm_autosleep_unlock();
    fn pm_autosleep_state() -> i32;
    fn pm_suspend(state: i32) -> i32;
    fn hibernate() -> i32;
    fn hibernation_available() -> bool;
    fn pm_suspend_in_progress() -> bool;
    fn hibernation_in_progress() -> bool;
    fn pm_get_wakeup_count(v: *mut u32, block: bool) -> bool;
    fn pm_save_wakeup_count(v: u32) -> bool;
    fn pm_print_active_wakeup_sources();
    fn pm_autosleep_set_state(state: i32) -> i32;
    fn pm_wake_lock(buf: *const i8) -> i32;
    fn pm_wake_unlock(buf: *const i8) -> i32;
    fn pm_show_wakelocks(buf: *mut i8, active: bool) -> isize;
    fn show_trace_dev_match(buf: *mut i8, size: usize) -> isize;
}

type gfp_t = usize;
type suspend_state_t = i32;
type ssize_t = isize;
type u64 = u64;
#[repr(C)] pub struct workqueue_struct { _private: [u8; 0] }
#[repr(C)] pub struct work_struct { _private: [u8; 0] }
#[repr(C)] pub struct kobject { _private: [u8; 0] }
#[repr(C)] pub struct kobj_attribute { pub attr: attribute }
#[repr(C)] pub struct attribute { _private: [u8; 0] }
#[repr(C)] pub struct attribute_group { pub name: *const i8, pub attrs: *mut *mut attribute }
#[repr(C)] pub struct notifier_block { _private: [u8; 0] }
#[repr(C)] pub struct seq_file { _private: [u8; 0] }

const EINVAL: i32 = 22;
const EBUSY: i32 = 16;
const EINTR: i32 = 4;
const ENODATA: i32 = 61;
const ENOMEM: i32 = 12;
const PF_NOFREEZE: usize = 1 << 5;
const __GFP_IO: usize = 1 << 19;
const __GFP_FS: usize = 1 << 21;
const MSEC_PER_SEC: i64 = 1000;
const PM_SUSPEND_ON: i32 = 0;
const PM_SUSPEND_MEM: i32 = 3;
const PM_SUSPEND_MAX: i32 = 5;
const PM_FS_SYNC_WAKEUP_RESOLUTION: u64 = 5;
const REC_FAILED_NUM: usize = 2;
const SUSPEND_NR_STEPS: usize = 8;

static mut saved_gfp_count: u32 = 0;
static mut saved_gfp_mask: gfp_t = 0;

pub unsafe fn pm_restore_gfp_mask() {
    if !mutex_is_locked(&mut system_transition_mutex) { return; }
    if saved_gfp_count == 0 { return; }
    saved_gfp_count -= 1;
    if saved_gfp_count != 0 { return; }
    gfp_allowed_mask = saved_gfp_mask;
    saved_gfp_mask = 0;
}

pub unsafe fn pm_restrict_gfp_mask() {
    if !mutex_is_locked(&mut system_transition_mutex) { return; }
    let old = saved_gfp_count;
    saved_gfp_count = saved_gfp_count.wrapping_add(1);
    if old != 0 { return; }
    saved_gfp_mask = gfp_allowed_mask;
    gfp_allowed_mask &= !(__GFP_IO | __GFP_FS);
}

pub unsafe fn lock_system_sleep() -> u32 {
    let flags = current_flags();
    set_current_flags(flags | PF_NOFREEZE);
    mutex_lock(&mut system_transition_mutex);
    flags as u32
}
pub unsafe fn unlock_system_sleep(flags: u32) {
    if (flags as usize & PF_NOFREEZE) == 0 { set_current_flags(current_flags() & !PF_NOFREEZE); }
    mutex_unlock(&mut system_transition_mutex);
}
extern "C" { fn current_flags() -> usize; fn set_current_flags(v: usize); }

pub unsafe fn ksys_sync_helper() {
    let start = ktime_get(); ksys_sync();
    let elapsed = ktime_to_ms(ktime_get() - start);
    pr_info(elapsed / MSEC_PER_SEC, elapsed % MSEC_PER_SEC);
}
extern "C" { fn pr_info(a: i64, b: i64); }

pub unsafe fn register_pm_notifier(nb: *mut notifier_block) -> i32 { blocking_notifier_chain_register(nb) }
pub unsafe fn unregister_pm_notifier(nb: *mut notifier_block) -> i32 { blocking_notifier_chain_unregister(nb) }
extern "C" { fn blocking_notifier_chain_register(nb: *mut notifier_block) -> i32; fn blocking_notifier_chain_unregister(nb: *mut notifier_block) -> i32; }
pub unsafe fn pm_notifier_call_chain_robust(up: usize, down: usize) -> i32 { blocking_notifier_call_chain_robust(up, down) }
pub unsafe fn pm_notifier_call_chain(val: usize) -> i32 { blocking_notifier_call_chain(val) }
extern "C" { fn blocking_notifier_call_chain_robust(up: usize, down: usize) -> i32; fn blocking_notifier_call_chain(v: usize) -> i32; }

#[no_mangle] pub static mut pm_async_enabled: i32 = 1;
pub unsafe fn pm_async_setup(s: *const i8) -> i32 { if c_str_eq(s, b"off\0") { pm_async_enabled = 0; } 1 }
extern "C" { fn c_str_eq(a: *const i8, b: *const u8) -> bool; }

#[repr(C)] pub struct suspend_stats {
    pub step_failures: [u32; SUSPEND_NR_STEPS], pub success: u32, pub fail: u32,
    pub last_failed_dev: i32, pub failed_devs: [[i8; 40]; REC_FAILED_NUM],
    pub last_failed_errno: i32, pub errno: [i32; REC_FAILED_NUM],
    pub last_failed_step: i32, pub last_hw_sleep: u64, pub total_hw_sleep: u64,
    pub max_hw_sleep: u64, pub failed_steps: [i32; REC_FAILED_NUM],
}
static mut suspend_stats: suspend_stats = suspend_stats { step_failures:[0;SUSPEND_NR_STEPS], success:0, fail:0, last_failed_dev:0, failed_devs:[[0;40];REC_FAILED_NUM], last_failed_errno:0, errno:[0;REC_FAILED_NUM], last_failed_step:0, last_hw_sleep:0, total_hw_sleep:0, max_hw_sleep:0, failed_steps:[0;REC_FAILED_NUM] };

pub unsafe fn dpm_save_failed_dev(name: *const i8) { mutex_lock(&mut system_transition_mutex); copy_name(suspend_stats.failed_devs[(suspend_stats.last_failed_dev as usize)%REC_FAILED_NUM].as_mut_ptr(), name); suspend_stats.last_failed_dev = (suspend_stats.last_failed_dev + 1) % REC_FAILED_NUM as i32; mutex_unlock(&mut system_transition_mutex); }
pub unsafe fn dpm_save_failed_step(step: i32) { suspend_stats.step_failures[(step-1) as usize] += 1; let i=(suspend_stats.last_failed_step as usize)%REC_FAILED_NUM; suspend_stats.failed_steps[i]=step; suspend_stats.last_failed_step=(suspend_stats.last_failed_step+1)%REC_FAILED_NUM as i32; }
pub unsafe fn dpm_save_errno(err: i32) { if err==0 { suspend_stats.success+=1; } else { suspend_stats.fail+=1; let i=(suspend_stats.last_failed_errno as usize)%REC_FAILED_NUM; suspend_stats.errno[i]=err; suspend_stats.last_failed_errno=(suspend_stats.last_failed_errno+1)%REC_FAILED_NUM as i32; } }
pub unsafe fn pm_report_hw_sleep_time(t:u64) { suspend_stats.last_hw_sleep=t; suspend_stats.total_hw_sleep=suspend_stats.total_hw_sleep.wrapping_add(t); }
pub unsafe fn pm_report_max_hw_sleep(t:u64) { suspend_stats.max_hw_sleep=t; }
extern "C" { fn copy_name(dst:*mut i8, src:*const i8); }

pub static mut power_kobj: *mut kobject = core::ptr::null_mut();
pub unsafe fn pm_sleep_transition_in_progress() -> bool { pm_suspend_in_progress() || hibernation_in_progress() }

pub unsafe fn pm_debug_messages_should_print() -> bool { pm_debug_messages_on && pm_sleep_transition_in_progress() }
pub static mut pm_debug_messages_on: bool = false;
pub static mut pm_print_times_enabled: bool = false;
pub static mut pm_trace_enabled: i32 = 0;
pub static mut filesystem_freeze_enabled: bool = false;

pub unsafe fn pm_start_workqueues() -> i32 { pm_wq = alloc_workqueue(); if pm_wq.is_null() { -ENOMEM } else { 0 } }
extern "C" { fn alloc_workqueue() -> *mut workqueue_struct; fn hibernate_image_size_init(); fn hibernate_reserved_size_init(); fn pm_states_init(); fn kobject_create_and_add() -> *mut kobject; fn sysfs_create_groups(k:*mut kobject) -> i32; fn pm_autosleep_init() -> i32; }
pub unsafe fn pm_init() -> i32 { let e=pm_start_workqueues(); if e!=0{return e;} hibernate_image_size_init(); hibernate_reserved_size_init(); pm_states_init(); power_kobj=kobject_create_and_add(); if power_kobj.is_null(){return -ENOMEM;} let e=sysfs_create_groups(power_kobj); if e!=0{return e;} pm_autosleep_init() }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
