/* SPDX-License-Identifier: GPL-2.0 */
/*
 * BPF extensible scheduler class: Documentation/scheduler/sched-ext.rst
 *
 * Rust translation of the C header. Configuration conditions are represented
 * with Rust cfg features corresponding to the original kernel options.
 */

#[cfg(feature = "CONFIG_SCHED_CLASS_EXT")]
extern "C" {
    pub fn scx_tick(rq: *mut rq);
    pub fn init_scx_entity(scx: *mut sched_ext_entity);
    pub fn scx_pre_fork(p: *mut task_struct);
    pub fn scx_fork(p: *mut task_struct, kargs: *mut kernel_clone_args) -> i32;
    pub fn scx_post_fork(p: *mut task_struct);
    pub fn scx_cancel_fork(p: *mut task_struct);
    pub fn scx_can_stop_tick(rq: *mut rq) -> bool;
    pub fn scx_rq_activate(rq: *mut rq);
    pub fn scx_rq_deactivate(rq: *mut rq);
    pub fn scx_check_setscheduler(p: *mut task_struct, policy: i32) -> i32;
    pub fn task_should_scx(policy: i32) -> bool;
    pub fn scx_allow_ttwu_queue(p: *const task_struct) -> bool;
    pub fn init_sched_ext_class();
    pub fn scx_enabled() -> bool;
    pub fn cpu_rq(cpu: i32) -> *mut rq;
    pub static ext_sched_class: sched_class;

    #[inline]
    pub unsafe fn scx_cpuperf_target(cpu: i32) -> u32 {
        if scx_enabled() { (*cpu_rq(cpu)).scx.cpuperf_target } else { 0 }
    }

    #[inline]
    pub unsafe fn task_on_scx(p: *const task_struct) -> bool {
        scx_enabled() && (*p).sched_class == &ext_sched_class
    }

    #[cfg(feature = "CONFIG_SCHED_CORE")]
    pub fn scx_prio_less(a: *const task_struct, b: *const task_struct, in_fi: bool) -> bool;
}

#[cfg(not(feature = "CONFIG_SCHED_CLASS_EXT"))]
#[inline] pub unsafe fn scx_tick(_rq: *mut rq) {}
#[cfg(not(feature = "CONFIG_SCHED_CLASS_EXT"))]
#[inline] pub unsafe fn scx_pre_fork(_p: *mut task_struct) {}
#[cfg(not(feature = "CONFIG_SCHED_CLASS_EXT"))]
#[inline] pub unsafe fn scx_fork(_p: *mut task_struct, _kargs: *mut kernel_clone_args) -> i32 { 0 }
#[cfg(not(feature = "CONFIG_SCHED_CLASS_EXT"))]
#[inline] pub unsafe fn scx_post_fork(_p: *mut task_struct) {}
#[cfg(not(feature = "CONFIG_SCHED_CLASS_EXT"))]
#[inline] pub unsafe fn scx_cancel_fork(_p: *mut task_struct) {}
#[cfg(not(feature = "CONFIG_SCHED_CLASS_EXT"))]
#[inline] pub unsafe fn scx_cpuperf_target(_cpu: i32) -> u32 { 0 }
#[cfg(not(feature = "CONFIG_SCHED_CLASS_EXT"))]
#[inline] pub unsafe fn scx_can_stop_tick(_rq: *mut rq) -> bool { true }
#[cfg(not(feature = "CONFIG_SCHED_CLASS_EXT"))]
#[inline] pub unsafe fn scx_rq_activate(_rq: *mut rq) {}
#[cfg(not(feature = "CONFIG_SCHED_CLASS_EXT"))]
#[inline] pub unsafe fn scx_rq_deactivate(_rq: *mut rq) {}
#[cfg(not(feature = "CONFIG_SCHED_CLASS_EXT"))]
#[inline] pub unsafe fn scx_check_setscheduler(_p: *mut task_struct, _policy: i32) -> i32 { 0 }
#[cfg(not(feature = "CONFIG_SCHED_CLASS_EXT"))]
#[inline] pub unsafe fn task_on_scx(_p: *const task_struct) -> bool { false }
#[cfg(not(feature = "CONFIG_SCHED_CLASS_EXT"))]
#[inline] pub unsafe fn scx_allow_ttwu_queue(_p: *const task_struct) -> bool { true }
#[cfg(not(feature = "CONFIG_SCHED_CLASS_EXT"))]
#[inline] pub unsafe fn init_sched_ext_class() {}

#[cfg(feature = "CONFIG_SCHED_CLASS_EXT")]
extern "C" { pub fn __scx_update_idle(rq: *mut rq, idle: bool, do_notify: bool); }
#[cfg(feature = "CONFIG_SCHED_CLASS_EXT")]
#[inline] pub unsafe fn scx_update_idle(rq: *mut rq, idle: bool, do_notify: bool) {
    if scx_enabled() { __scx_update_idle(rq, idle, do_notify); }
}
#[cfg(not(feature = "CONFIG_SCHED_CLASS_EXT"))]
#[inline] pub unsafe fn scx_update_idle(_rq: *mut rq, _idle: bool, _do_notify: bool) {}

#[cfg(feature = "CONFIG_CGROUP_SCHED")]
#[cfg(feature = "CONFIG_EXT_GROUP_SCHED")]
extern "C" {
    pub fn scx_tg_init(tg: *mut task_group);
    pub fn scx_tg_online(tg: *mut task_group) -> i32;
    pub fn scx_tg_offline(tg: *mut task_group);
    pub fn scx_cgroup_can_attach(tset: *mut cgroup_taskset) -> i32;
    pub fn scx_cgroup_move_task(p: *mut task_struct);
    pub fn scx_cgroup_cancel_attach(tset: *mut cgroup_taskset);
    pub fn scx_group_set_weight(tg: *mut task_group, cgrp_weight: usize);
    pub fn scx_group_set_idle(tg: *mut task_group, idle: bool);
    pub fn scx_group_set_bandwidth(tg: *mut task_group, period_us: u64, quota_us: u64, burst_us: u64);
}

#[cfg(feature = "CONFIG_CGROUP_SCHED")]
#[cfg(not(feature = "CONFIG_EXT_GROUP_SCHED"))]
#[inline] pub unsafe fn scx_tg_init(_tg: *mut task_group) {}
#[cfg(feature = "CONFIG_CGROUP_SCHED")]
#[cfg(not(feature = "CONFIG_EXT_GROUP_SCHED"))]
#[inline] pub unsafe fn scx_tg_online(_tg: *mut task_group) -> i32 { 0 }
#[cfg(feature = "CONFIG_CGROUP_SCHED")]
#[cfg(not(feature = "CONFIG_EXT_GROUP_SCHED"))]
#[inline] pub unsafe fn scx_tg_offline(_tg: *mut task_group) {}
#[cfg(feature = "CONFIG_CGROUP_SCHED")]
#[cfg(not(feature = "CONFIG_EXT_GROUP_SCHED"))]
#[inline] pub unsafe fn scx_cgroup_can_attach(_tset: *mut cgroup_taskset) -> i32 { 0 }
#[cfg(feature = "CONFIG_CGROUP_SCHED")]
#[cfg(not(feature = "CONFIG_EXT_GROUP_SCHED"))]
#[inline] pub unsafe fn scx_cgroup_move_task(_p: *mut task_struct) {}
#[cfg(feature = "CONFIG_CGROUP_SCHED")]
#[cfg(not(feature = "CONFIG_EXT_GROUP_SCHED"))]
#[inline] pub unsafe fn scx_cgroup_cancel_attach(_tset: *mut cgroup_taskset) {}
#[cfg(feature = "CONFIG_CGROUP_SCHED")]
#[cfg(not(feature = "CONFIG_EXT_GROUP_SCHED"))]
#[inline] pub unsafe fn scx_group_set_weight(_tg: *mut task_group, _cgrp_weight: usize) {}
#[cfg(feature = "CONFIG_CGROUP_SCHED")]
#[cfg(not(feature = "CONFIG_EXT_GROUP_SCHED"))]
#[inline] pub unsafe fn scx_group_set_idle(_tg: *mut task_group, _idle: bool) {}
#[cfg(feature = "CONFIG_CGROUP_SCHED")]
#[cfg(not(feature = "CONFIG_EXT_GROUP_SCHED"))]
#[inline] pub unsafe fn scx_group_set_bandwidth(_tg: *mut task_group, _period_us: u64, _quota_us: u64, _burst_us: u64) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
