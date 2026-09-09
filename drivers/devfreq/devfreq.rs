// SPDX-License-Identifier: GPL-2.0-only
//
// Low-level Rust translation of linux/drivers/devfreq/devfreq.c.
// Kernel types, constants, macros, globals, and helper functions referenced
// below are supplied by the surrounding kernel bindings.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_void};

extern "C" {
    static mut devfreq_class: *mut class;
    static mut devfreq_debugfs: *mut dentry;
    static mut devfreq_wq: *mut workqueue_struct;
    static mut devfreq_list: list_head;
    static mut devfreq_governor_list: list_head;
    static mut devfreq_list_lock: mutex;
    static timer_name: [[c_char; DEVFREQ_NAME_LEN]; DEVFREQ_TIMER_NUM];

    fn find_device_devfreq(dev: *mut device) -> *mut devfreq;
    fn find_devfreq_governor(name: *const c_char) -> *mut devfreq_governor;
    fn try_then_request_governor(name: *const c_char) -> *mut devfreq_governor;
    fn devfreq_set_target(df: *mut devfreq, freq: c_ulong, flags: u32) -> c_int;
    fn devfreq_monitor(work: *mut work_struct);
    fn devfreq_dev_release(dev: *mut device);
}

type c_ulong = usize;
type ssize_t = isize;

#[repr(C)] pub struct class { _opaque: [u8; 0] }
#[repr(C)] pub struct dentry { _opaque: [u8; 0] }
#[repr(C)] pub struct workqueue_struct { _opaque: [u8; 0] }
#[repr(C)] pub struct work_struct { _opaque: [u8; 0] }
#[repr(C)] pub struct mutex { _opaque: [u8; 0] }
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct device { pub parent: *mut device, pub class: *mut class, pub groups: *const *const attribute_group, pub release: Option<unsafe extern "C" fn(*mut device)>, pub kobj: kobject, pub of_node: *mut device_node }
#[repr(C)] pub struct kobject { _opaque: [u8; 0] }
#[repr(C)] pub struct attribute { pub mode: u16 }
#[repr(C)] pub struct attribute_group { pub attrs: *const *mut attribute, pub is_visible: Option<unsafe extern "C" fn(*mut kobject,*mut attribute,c_int)->u16> }
#[repr(C)] pub struct device_node { _opaque: [u8; 0] }
#[repr(C)] pub struct notifier_block { pub notifier_call: Option<unsafe extern "C" fn(*mut notifier_block,usize,*mut c_void)->c_int> }
#[repr(C)] pub struct dev_pm_opp { _opaque: [u8; 0] }
#[repr(C)] pub struct dev_pm_opp_table { _opaque: [u8; 0] }
#[repr(C)] pub struct devfreq_freqs { pub old: c_ulong, pub new: c_ulong }
#[repr(C)] pub struct devfreq_governor { pub node: list_head, pub name: *const c_char, pub flags: u32, pub attrs: u32, pub event_handler: Option<unsafe extern "C" fn(*mut devfreq,u32,*mut c_void)->c_int>, pub get_target_freq: Option<unsafe extern "C" fn(*mut devfreq,*mut c_ulong)->c_int> }
#[repr(C)] pub struct devfreq_dev_profile { pub polling_ms: u32, pub initial_freq: c_ulong, pub timer: c_int, pub max_state: u32, pub freq_table: *mut c_ulong, pub dev_groups: *const *const attribute_group, pub is_cooling_device: bool, pub get_cur_freq: Option<unsafe extern "C" fn(*mut device,*mut c_ulong)->c_int>, pub target: Option<unsafe extern "C" fn(*mut device,*mut c_ulong,u32)->c_int>, pub exit: Option<unsafe extern "C" fn(*mut device)> }
#[repr(C)] pub struct devfreq_stats { pub time_in_state: *mut u64, pub trans_table: *mut u32, pub last_update: u64, pub total_trans: u32 }
#[repr(C)] pub struct devfreq { pub dev: device, pub node: list_head, pub lock: mutex, pub profile: *mut devfreq_dev_profile, pub governor: *mut devfreq_governor, pub freq_table: *mut c_ulong, pub max_state: u32, pub previous_freq: c_ulong, pub resume_freq: c_ulong, pub suspend_freq: c_ulong, pub scaling_min_freq: c_ulong, pub scaling_max_freq: c_ulong, pub data: *mut c_void, pub nb: notifier_block, pub nb_min: notifier_block, pub nb_max: notifier_block, pub opp_table: *mut dev_pm_opp_table, pub stats: devfreq_stats, pub transition_notifier_list: [u8; 64], pub suspend_count: c_int, pub stop_polling: bool, pub work: [u8; 128], pub user_min_freq_req: [u8; 32], pub user_max_freq_req: [u8; 32], pub cdev: *mut c_void }

const DEVFREQ_TIMER_NUM: usize = 2;
const DEVFREQ_NAME_LEN: usize = 16;

#[inline] unsafe fn max_ul(a: c_ulong, b: c_ulong) -> c_ulong { if a > b { a } else { b } }
#[inline] unsafe fn min_ul(a: c_ulong, b: c_ulong) -> c_ulong { if a < b { a } else { b } }

pub unsafe extern "C" fn devfreq_get_freq_range(df: *mut devfreq, min: *mut c_ulong, max: *mut c_ulong) {
    let t = (*df).freq_table;
    let last = (*df).max_state as usize - 1;
    if *t < *t.add(last) { *min = *t; *max = *t.add(last); } else { *min = *t.add(last); *max = *t; }
    *min = max_ul(*min, (*df).scaling_min_freq);
    *max = min_ul(*max, (*df).scaling_max_freq);
    *min = max_ul(*min, (*df).scaling_min_freq);
    *min = min_ul(*min, *max);
}

unsafe fn devfreq_get_freq_level(df: *mut devfreq, freq: c_ulong) -> c_int {
    for i in 0..(*df).max_state as usize { if *(*df).freq_table.add(i) == freq { return i as c_int; } }
    -22
}

pub unsafe extern "C" fn devfreq_update_status(df: *mut devfreq, freq: c_ulong) -> c_int {
    let now = get_jiffies_64();
    if (*df).previous_freq == 0 { (*df).stats.last_update = now; return 0; }
    let old = devfreq_get_freq_level(df, (*df).previous_freq); if old < 0 { return old; }
    *(*df).stats.time_in_state.add(old as usize) += now - (*df).stats.last_update;
    let new = devfreq_get_freq_level(df, freq); if new < 0 { return new; }
    if old != new { *(*df).stats.trans_table.add(old as usize * (*df).max_state as usize + new as usize) += 1; (*df).stats.total_trans += 1; }
    (*df).stats.last_update = now; 0
}

pub unsafe extern "C" fn devfreq_update_target(df: *mut devfreq, mut freq: c_ulong) -> c_int {
    if (*df).governor.is_null() { return -22; }
    let r = ((*(*df).governor).get_target_freq.unwrap())(df, &mut freq); if r != 0 { return r; }
    let (mut min, mut max) = (0, 0); devfreq_get_freq_range(df, &mut min, &mut max);
    if freq < min { freq = min; } if freq > max { freq = max; }
    devfreq_set_target(df, freq, 0)
}

pub unsafe extern "C" fn update_devfreq(df: *mut devfreq) -> c_int { devfreq_update_target(df, 0) }
pub unsafe extern "C" fn devfreq_monitor_start(_df: *mut devfreq) {}
pub unsafe extern "C" fn devfreq_monitor_stop(_df: *mut devfreq) {}
pub unsafe extern "C" fn devfreq_monitor_suspend(_df: *mut devfreq) {}
pub unsafe extern "C" fn devfreq_monitor_resume(_df: *mut devfreq) {}

extern "C" { fn get_jiffies_64() -> u64; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
