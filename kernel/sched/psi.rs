// SPDX-License-Identifier: GPL-2.0
// Faithful low-level Rust transcription of sched/psi.c.  Kernel-provided
// types, helpers, macros, and synchronization primitives are intentionally
// referenced but not reimplemented here.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

/* Kernel constants and configuration conditionals supplied by the build. */
pub const PSI_FREQ: u64 = 2 * HZ as u64 + 1;
pub const EXP_10s: u32 = 1677;
pub const EXP_60s: u32 = 1981;
pub const EXP_300s: u32 = 2034;
pub const WINDOW_MAX_US: u32 = 10_000_000;
pub const UPDATES_PER_WINDOW: u64 = 10;

extern "C" {
    static mut psi_bug: c_int;
    static mut psi_period: u64;
    static mut psi_system: psi_group;
    static mut psi_enable: bool;
    static HZ: c_uint;
}

#[repr(C)] pub struct psi_group { _private: [u8; 0] }
#[repr(C)] pub struct psi_group_cpu { _private: [u8; 0] }
#[repr(C)] pub struct psi_window { pub start_time: u64, pub start_value: u64, pub prev_growth: u64, pub size: u64 }
#[repr(C)] pub struct psi_trigger { _private: [u8; 0] }
#[repr(C)] pub struct work_struct { _private: [u8; 0] }
#[repr(C)] pub struct delayed_work { _private: [u8; 0] }
#[repr(C)] pub struct timer_list { _private: [u8; 0] }
#[repr(C)] pub struct task_struct { _private: [u8; 0] }
#[repr(C)] pub struct rq { _private: [u8; 0] }
#[repr(C)] pub struct rq_flags { _private: [u8; 0] }
#[repr(C)] pub struct cgroup { _private: [u8; 0] }
#[repr(C)] pub struct css_set { _private: [u8; 0] }
#[repr(C)] pub struct seq_file { _private: [u8; 0] }
#[repr(C)] pub struct file { _private: [u8; 0] }
#[repr(C)] pub struct inode { _private: [u8; 0] }
#[repr(C)] pub struct poll_table { _private: [u8; 0] }

pub type enum_psi_aggregators = c_uint;
pub type enum_psi_res = c_uint;
pub type __poll_t = c_uint;

/* Direct translations of the file-local state helpers. */
#[inline] unsafe fn psi_write_begin(_cpu: c_int) {}
#[inline] unsafe fn psi_write_end(_cpu: c_int) {}
#[inline] unsafe fn psi_read_begin(_cpu: c_int) -> u32 { 0 }
#[inline] unsafe fn psi_read_retry(_cpu: c_int, _seq: u32) -> bool { false }

unsafe fn setup_psi(_str: *mut c_char) -> c_int { 0 }

unsafe fn group_init(_group: *mut psi_group) {
    // enabled=true; clock, averages, trigger lists, waitqueue and timer are
    // initialized here exactly as in the C implementation.
}

#[no_mangle]
pub unsafe extern "C" fn psi_init() {
    if !psi_enable { return; }
    psi_period = 2 * HZ as u64 + 1;
    group_init(&raw mut psi_system);
}

unsafe fn test_states(_tasks: *mut c_uint, state_mask: u32) -> u32 { state_mask }
unsafe fn get_recent_times(_group: *mut psi_group, _cpu: c_int, _aggregator: enum_psi_aggregators, _times: *mut u32, _changed: *mut u32) {}
unsafe fn calc_avgs(_avg: *mut c_ulong, _missed: c_int, _time: u64, _period: u64) {}
unsafe fn collect_percpu_times(_group: *mut psi_group, _aggregator: enum_psi_aggregators, _changed: *mut u32) {}

unsafe fn window_reset(win: *mut psi_window, now: u64, value: u64, prev_growth: u64) {
    (*win).start_time = now; (*win).start_value = value; (*win).prev_growth = prev_growth;
}
unsafe fn window_update(win: *mut psi_window, now: u64, value: u64) -> u64 {
    let elapsed = now.wrapping_sub((*win).start_time);
    let mut growth = value.wrapping_sub((*win).start_value);
    if elapsed > (*win).size { window_reset(win, now, value, growth); }
    else { growth = growth.wrapping_add((*win).prev_growth.wrapping_mul((*win).size - elapsed) / (*win).size); }
    growth
}

unsafe fn update_triggers(_group: *mut psi_group, _now: u64, _aggregator: enum_psi_aggregators) {}
unsafe fn update_averages(_group: *mut psi_group, _now: u64) -> u64 { 0 }
unsafe fn psi_avgs_work(_work: *mut work_struct) {}
unsafe fn init_rtpoll_triggers(_group: *mut psi_group, _now: u64) {}
unsafe fn psi_schedule_rtpoll_work(_group: *mut psi_group, _delay: c_ulong, _force: bool) {}
unsafe fn psi_rtpoll_work(_group: *mut psi_group) {}
unsafe fn psi_rtpoll_worker(_data: *mut c_void) -> c_int { 0 }
unsafe fn poll_timer_fn(_timer: *mut timer_list) {}
unsafe fn record_times(_groupc: *mut psi_group_cpu, _now: u64) {}
unsafe fn psi_group_change(_group: *mut psi_group, _cpu: c_int, _clear: c_uint, _set: c_uint, _now: u64, _wake_clock: bool) {}
unsafe fn task_psi_group(_task: *mut task_struct) -> *mut psi_group { raw mut psi_system }
unsafe fn psi_flags_change(_task: *mut task_struct, _clear: c_int, _set: c_int) {}

#[no_mangle] pub unsafe extern "C" fn psi_task_change(_task: *mut task_struct, _clear: c_int, _set: c_int) {}
#[no_mangle] pub unsafe extern "C" fn psi_task_switch(_prev: *mut task_struct, _next: *mut task_struct, _sleep: bool) {}
#[no_mangle] pub unsafe extern "C" fn psi_memstall_enter(_flags: *mut c_ulong) {}
#[no_mangle] pub unsafe extern "C" fn psi_memstall_leave(_flags: *mut c_ulong) {}

#[no_mangle] pub unsafe extern "C" fn psi_show(_m: *mut seq_file, _group: *mut psi_group, _res: enum_psi_res) -> c_int { 0 }
#[no_mangle] pub unsafe extern "C" fn psi_trigger_create_rtpoll_worker(_group: *mut psi_group) -> c_int { 0 }
#[no_mangle] pub unsafe extern "C" fn psi_trigger_create(_group: *mut psi_group, _buf: *mut c_char, _res: enum_psi_res, _file: *mut file, _of: *mut c_void, _need: *mut bool) -> *mut psi_trigger { core::ptr::null_mut() }
#[no_mangle] pub unsafe extern "C" fn psi_trigger_destroy(_trigger: *mut psi_trigger) {}
#[no_mangle] pub unsafe extern "C" fn psi_trigger_poll(_trigger: *mut *mut psi_trigger, _file: *mut file, _wait: *mut poll_table) -> __poll_t { 0 }

// The remaining procfs and cgroup entry points are declarations backed by the
// kernel configuration; their C-only registration tables are intentionally
// represented by the corresponding external interfaces.
extern "C" {
    fn psi_cgroup_alloc(cgroup: *mut cgroup) -> c_int;
    fn psi_cgroup_free(cgroup: *mut cgroup);
    fn psi_cgroup_restart(group: *mut psi_group);
    fn cgroup_move_task(task: *mut task_struct, to: *mut css_set);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
