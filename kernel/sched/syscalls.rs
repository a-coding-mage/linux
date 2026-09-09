// SPDX-License-Identifier: GPL-2.0-only
/* Rust translation of kernel/sched/syscalls.c.  Kernel types and helpers are
 * supplied by the surrounding scheduler translation unit. */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_int, c_long, c_uint, c_ulong, c_void};

#[repr(C)] pub struct task_struct { pub policy: c_int, pub rt_priority: c_int,
    pub static_prio: c_int, pub prio: c_int, pub normal_prio: c_int,
    pub sched_reset_on_fork: bool, pub timer_slack_ns: u64,
    pub default_timer_slack_ns: u64, pub flags: c_ulong }
#[repr(C)] pub struct sched_param { pub sched_priority: c_int }
#[repr(C)] pub struct sched_attr { pub size: u32, pub sched_policy: u32,
    pub sched_flags: u64, pub sched_nice: i32, pub sched_priority: u32,
    pub sched_runtime: u64, pub sched_deadline: u64, pub sched_period: u64,
    pub sched_util_min: u32, pub sched_util_max: u32 }
#[repr(C)] pub struct cpumask { _private: [u8; 0] }
#[repr(C)] pub struct affinity_context { pub new_mask: *const cpumask,
    pub user_mask: *mut cpumask, pub flags: c_uint }
#[repr(C)] pub struct timespec64 { pub tv_sec: i64, pub tv_nsec: i32 }

extern "C" {
    static mut current: *mut task_struct;
    fn dl_policy(policy: c_int) -> bool; fn rt_policy(policy: c_int) -> bool;
    fn fair_policy(policy: c_int) -> bool; fn valid_policy(policy: c_int) -> bool;
    fn task_nice(p: *const task_struct) -> c_int; fn capable(cap: c_int) -> bool;
    fn security_task_setnice(p: *mut task_struct, nice: c_long) -> c_long;
    fn security_task_setscheduler(p: *mut task_struct) -> c_int;
    fn security_task_getscheduler(p: *mut task_struct) -> c_int;
    fn set_load_weight(p: *mut task_struct, update: bool); fn sched_uclamp_enable();
    fn find_task_by_vpid(pid: i32) -> *mut task_struct;
    fn get_task_struct(p: *mut task_struct); fn put_task_struct(p: *mut task_struct);
    fn task_has_dl_policy(p: *const task_struct) -> bool;
    fn task_has_rt_policy(p: *const task_struct) -> bool;
    fn task_has_idle_policy(p: *const task_struct) -> bool;
    fn rt_or_dl_prio(prio: c_int) -> bool; fn rt_effective_prio(p: *mut task_struct, n: c_int) -> c_int;
    fn __setparam_dl(p: *mut task_struct, a: *const sched_attr);
    fn __setparam_fair(p: *mut task_struct, a: *const sched_attr);
    fn __sched_setscheduler(p: *mut task_struct, a: *const sched_attr, user: bool, pi: bool) -> c_int;
    fn __set_cpus_allowed_ptr(p: *mut task_struct, c: *mut affinity_context) -> c_int;
    fn dl_task_check_affinity(p: *mut task_struct, m: *const cpumask) -> c_int;
    fn sched_rr_get_interval_impl(pid: i32, t: *mut timespec64) -> c_int;
}

const MAX_DL_PRIO: c_int = 0; const MAX_RT_PRIO: c_int = 100;
const MIN_NICE: c_int = -20; const MAX_NICE: c_int = 19;
const NICE_WIDTH: c_int = 40; const CAP_SYS_NICE: c_int = 23;
const SETPARAM_POLICY: c_int = -1;

#[inline] unsafe fn __normal_prio(policy: c_int, rt_prio: c_int, nice: c_int) -> c_int {
    if dl_policy(policy) { MAX_DL_PRIO - 1 } else if rt_policy(policy) { MAX_RT_PRIO - 1 - rt_prio } else { 120 + nice }
}
#[inline] unsafe fn normal_prio(p: *mut task_struct) -> c_int { __normal_prio((*p).policy, (*p).rt_priority, (*p).static_prio - 120) }
unsafe fn effective_prio(p: *mut task_struct) -> c_int {
    (*p).normal_prio = normal_prio(p); if !rt_or_dl_prio((*p).prio) { (*p).normal_prio } else { (*p).prio }
}
#[no_mangle] pub unsafe extern "C" fn set_user_nice(p: *mut task_struct, nice: c_long) {
    let n = nice as c_int; if task_nice(p) == n || n < MIN_NICE || n > MAX_NICE { return; }
    (*p).static_prio = 120 + n; if task_has_dl_policy(p) || task_has_rt_policy(p) { return; }
    set_load_weight(p, true); (*p).prio = effective_prio(p);
}
unsafe fn is_nice_reduction(p: *const task_struct, nice: c_int) -> bool { (20 - nice) <= 40 }
#[no_mangle] pub unsafe extern "C" fn can_nice(p: *const task_struct, nice: c_int) -> c_int { (is_nice_reduction(p, nice) || capable(CAP_SYS_NICE)) as c_int }
#[no_mangle] pub unsafe extern "C" fn task_prio(p: *const task_struct) -> c_int { (*p).prio - MAX_RT_PRIO }
#[no_mangle] pub unsafe extern "C" fn find_process_by_pid(pid: i32) -> *mut task_struct { if pid != 0 { find_task_by_vpid(pid) } else { current } }

#[no_mangle] pub unsafe extern "C" fn sched_setscheduler(p: *mut task_struct, policy: c_int, param: *const sched_param) -> c_int {
    let a = sched_attr { size: 0, sched_policy: policy as u32, sched_flags: 0, sched_nice: (*p).static_prio - 120,
        sched_priority: (*param).sched_priority as u32, sched_runtime: 0, sched_deadline: 0, sched_period: 0, sched_util_min: 0, sched_util_max: 0 };
    __sched_setscheduler(p, &a, true, true)
}
#[no_mangle] pub unsafe extern "C" fn sched_setattr(p: *mut task_struct, a: *const sched_attr) -> c_int { __sched_setscheduler(p, a, true, true) }
#[no_mangle] pub unsafe extern "C" fn sched_setattr_nocheck(p: *mut task_struct, a: *const sched_attr) -> c_int { __sched_setscheduler(p, a, false, true) }
#[no_mangle] pub unsafe extern "C" fn sched_setscheduler_nocheck(p: *mut task_struct, policy: c_int, a: *const sched_param) -> c_int { sched_setscheduler(p, policy, a) }
#[no_mangle] pub unsafe extern "C" fn sched_set_fifo(p: *mut task_struct) { let a=sched_param{sched_priority:MAX_RT_PRIO/2}; let _=sched_setscheduler_nocheck(p,1,&a); }
#[no_mangle] pub unsafe extern "C" fn sched_set_fifo_low(p: *mut task_struct) { let a=sched_param{sched_priority:1}; let _=sched_setscheduler_nocheck(p,1,&a); }
#[no_mangle] pub unsafe extern "C" fn sched_set_fifo_secondary(p: *mut task_struct) { let a=sched_param{sched_priority:MAX_RT_PRIO/2-1}; let _=sched_setscheduler_nocheck(p,1,&a); }
#[no_mangle] pub unsafe extern "C" fn sched_set_normal(p: *mut task_struct, nice: c_int) { let a=sched_attr{size:0,sched_policy:0,sched_flags:0,sched_nice:nice,sched_priority:0,sched_runtime:0,sched_deadline:0,sched_period:0,sched_util_min:0,sched_util_max:0}; let _=sched_setattr_nocheck(p,&a); }

#[no_mangle] pub unsafe extern "C" fn sched_setaffinity(pid: i32, mask: *const cpumask) -> c_long { let p=find_process_by_pid(pid); if p.is_null(){-3}else{let mut a=affinity_context{new_mask:mask,user_mask:core::ptr::null_mut(),flags:1}; __set_cpus_allowed_ptr(p,&mut a) as c_long} }
#[no_mangle] pub unsafe extern "C" fn sched_rr_get_interval(pid: i32, t: *mut timespec64) -> c_int { sched_rr_get_interval_impl(pid,t) }
#[no_mangle] pub unsafe extern "C" fn yield_() { }


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
