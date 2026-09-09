/* SPDX-License-Identifier: GPL-2.0 */
/*
 * BPF extensible scheduler class: Documentation/scheduler/sched-ext.rst
 *
 * Sub-scheduler hierarchy support.
 *
 * Copyright (c) 2026 Meta Platforms, Inc. and affiliates.
 * Copyright (c) 2026 Tejun Heo <tj@kernel.org>
 */

// Dependency: internal.h

#[cfg(feature = "CONFIG_EXT_SUB_SCHED")]
extern "C" {
    pub fn scx_skip_subtree_pre(pos: *mut scx_sched, root: *mut scx_sched) -> *mut scx_sched;
    pub fn scx_next_descendant_pre(pos: *mut scx_sched, root: *mut scx_sched) -> *mut scx_sched;
    pub fn scx_set_task_sched(p: *mut task_struct, sch: *mut scx_sched);
    pub fn sch_cgroup(sch: *mut scx_sched) -> *mut cgroup;
    pub fn set_cgroup_sched(cgrp: *mut cgroup, sch: *mut scx_sched);
    pub fn scx_pstack_recursion_on_dispatch(prog: *mut bpf_prog);
    pub fn scx_pstack_recursion_on_caps_updated(prog: *mut bpf_prog);
    pub fn drain_descendants(sch: *mut scx_sched);
    pub fn scx_sub_disable(sch: *mut scx_sched);
    pub fn scx_sub_enable_workfn(work: *mut kthread_work);
    pub fn scx_bpf_sub_dispatch(cgroup_id: u64, aux: *const bpf_prog_aux) -> bool;
    pub fn scx_free_pshards(sch: *mut scx_sched);
    pub fn scx_alloc_pshards(sch: *mut scx_sched) -> i32;
    pub fn scx_init_root_caps(sch: *mut scx_sched);
    pub fn scx_process_sync_ecaps(rq: *mut rq, prev: *mut task_struct);
    pub fn scx_unbypass_replay_ecaps(rq: *mut rq, sch: *mut scx_sched);
    pub fn scx_online_ecaps(rq: *mut rq);
    pub fn scx_offline_ecaps(rq: *mut rq);
    pub fn scx_discard_ecaps_to_sync(cpu: i32, pcpu: *mut scx_sched_pcpu);
    pub fn scx_discard_stale_ecaps_syncs();
    pub fn scx_resolve_local_dsq(sch: *mut scx_sched, rq: *mut rq, p: *mut task_struct,
                                 enq_flags: *mut u64) -> *mut scx_dispatch_q;
    pub fn scx_task_reenq_on_cap_revoke(rq: *mut rq, p: *mut task_struct) -> bool;
    pub fn scx_reenq_reject(rq: *mut rq);
    pub fn scx_rescue_charge(rq: *mut rq, delta_exec: i64);
    pub fn scx_rescue_end(rq: *mut rq);
    pub fn scx_rescue_keep(rq: *mut rq, p: *mut task_struct) -> bool;
    pub fn scx_rescue_flush(rq: *mut rq);
    pub fn scx_rescue_dump(s: *mut seq_buf, rq: *mut rq);
    pub fn scx_rescue_set_knobs(sch: *mut scx_sched);
    pub fn scx_rescue_init(rq: *mut rq);
}

#[cfg(feature = "CONFIG_EXT_SUB_SCHED")]
#[inline]
pub unsafe fn scx_cgroup_sched(cgrp: *mut cgroup) -> *mut scx_sched {
    rcu_dereference_check((*cgrp).scx_sched,
        lockdep_is_held(&cgroup_mutex) ||
        percpu_rwsem_is_held(&scx_fork_rwsem) ||
        lockdep_is_held(&scx_enable_mutex))
}

#[cfg(feature = "CONFIG_EXT_SUB_SCHED")]
#[inline]
pub unsafe fn sch_cgrp_path(sch: *mut scx_sched) -> *const core::ffi::c_char { (*sch).cgrp_path }

/* a dying sub's hot-path influence ends in scx_sched_free_rcu_work() */
#[cfg(feature = "CONFIG_EXT_SUB_SCHED")]
#[inline]
pub unsafe fn scx_dec_has_subs(sch: *mut scx_sched) {
    if (*sch).level != 0 { static_branch_dec(&__scx_has_subs); }
}

#[cfg(not(feature = "CONFIG_EXT_SUB_SCHED"))]
#[inline] pub unsafe fn scx_next_descendant_pre(pos: *mut scx_sched, root: *mut scx_sched) -> *mut scx_sched { if !pos.is_null() { core::ptr::null_mut() } else { root } }
#[cfg(not(feature = "CONFIG_EXT_SUB_SCHED"))]
#[inline] pub unsafe fn scx_skip_subtree_pre(_pos: *mut scx_sched, _root: *mut scx_sched) -> *mut scx_sched { core::ptr::null_mut() }
#[cfg(not(feature = "CONFIG_EXT_SUB_SCHED"))]
#[inline] pub unsafe fn scx_set_task_sched(_p: *mut task_struct, _sch: *mut scx_sched) {}
#[cfg(not(feature = "CONFIG_EXT_SUB_SCHED"))]
#[inline] pub unsafe fn sch_cgroup(_sch: *mut scx_sched) -> *mut cgroup { core::ptr::null_mut() }
#[cfg(not(feature = "CONFIG_EXT_SUB_SCHED"))]
#[inline] pub unsafe fn sch_cgrp_path(_sch: *mut scx_sched) -> *const core::ffi::c_char { b"/\0".as_ptr() as *const _ }
#[cfg(not(feature = "CONFIG_EXT_SUB_SCHED"))]
#[inline] pub unsafe fn set_cgroup_sched(_cgrp: *mut cgroup, _sch: *mut scx_sched) {}
#[cfg(not(feature = "CONFIG_EXT_SUB_SCHED"))]
#[inline] pub unsafe fn drain_descendants(_sch: *mut scx_sched) {}
#[cfg(not(feature = "CONFIG_EXT_SUB_SCHED"))]
#[inline] pub unsafe fn scx_sub_disable(_sch: *mut scx_sched) {}
#[cfg(not(feature = "CONFIG_EXT_SUB_SCHED"))]
#[inline] pub unsafe fn scx_free_pshards(_sch: *mut scx_sched) {}
#[cfg(not(feature = "CONFIG_EXT_SUB_SCHED"))]
#[inline] pub unsafe fn scx_alloc_pshards(_sch: *mut scx_sched) -> i32 { 0 }
#[cfg(not(feature = "CONFIG_EXT_SUB_SCHED"))]
#[inline] pub unsafe fn scx_init_root_caps(_sch: *mut scx_sched) {}
#[cfg(not(feature = "CONFIG_EXT_SUB_SCHED"))]
#[inline] pub unsafe fn scx_process_sync_ecaps(_rq: *mut rq, _prev: *mut task_struct) {}
#[cfg(not(feature = "CONFIG_EXT_SUB_SCHED"))]
#[inline] pub unsafe fn scx_unbypass_replay_ecaps(_rq: *mut rq, _sch: *mut scx_sched) {}
#[cfg(not(feature = "CONFIG_EXT_SUB_SCHED"))]
#[inline] pub unsafe fn scx_online_ecaps(_rq: *mut rq) {}
#[cfg(not(feature = "CONFIG_EXT_SUB_SCHED"))]
#[inline] pub unsafe fn scx_offline_ecaps(_rq: *mut rq) {}
#[cfg(not(feature = "CONFIG_EXT_SUB_SCHED"))]
#[inline] pub unsafe fn scx_discard_ecaps_to_sync(_cpu: i32, _pcpu: *mut scx_sched_pcpu) {}
#[cfg(not(feature = "CONFIG_EXT_SUB_SCHED"))]
#[inline] pub unsafe fn scx_discard_stale_ecaps_syncs() {}
#[cfg(not(feature = "CONFIG_EXT_SUB_SCHED"))]
#[inline] pub unsafe fn scx_resolve_local_dsq(_sch: *mut scx_sched, rq: *mut rq, _p: *mut task_struct, _enq_flags: *mut u64) -> *mut scx_dispatch_q { &mut (*rq).scx.local_dsq }
#[cfg(not(feature = "CONFIG_EXT_SUB_SCHED"))]
#[inline] pub unsafe fn scx_task_reenq_on_cap_revoke(_rq: *mut rq, _p: *mut task_struct) -> bool { false }
#[cfg(not(feature = "CONFIG_EXT_SUB_SCHED"))]
#[inline] pub unsafe fn scx_reenq_reject(_rq: *mut rq) {}
#[cfg(not(feature = "CONFIG_EXT_SUB_SCHED"))]
#[inline] pub unsafe fn scx_rescue_charge(_rq: *mut rq, _delta_exec: i64) {}
#[cfg(not(feature = "CONFIG_EXT_SUB_SCHED"))]
#[inline] pub unsafe fn scx_rescue_end(_rq: *mut rq) {}
#[cfg(not(feature = "CONFIG_EXT_SUB_SCHED"))]
#[inline] pub unsafe fn scx_rescue_keep(_rq: *mut rq, _p: *mut task_struct) -> bool { false }
#[cfg(not(feature = "CONFIG_EXT_SUB_SCHED"))]
#[inline] pub unsafe fn scx_rescue_flush(_rq: *mut rq) {}
#[cfg(not(feature = "CONFIG_EXT_SUB_SCHED"))]
#[inline] pub unsafe fn scx_rescue_dump(_s: *mut seq_buf, _rq: *mut rq) {}
#[cfg(not(feature = "CONFIG_EXT_SUB_SCHED"))]
#[inline] pub unsafe fn scx_rescue_set_knobs(_sch: *mut scx_sched) {}
#[cfg(not(feature = "CONFIG_EXT_SUB_SCHED"))]
#[inline] pub unsafe fn scx_rescue_init(_rq: *mut rq) {}
#[cfg(not(feature = "CONFIG_EXT_SUB_SCHED"))]
#[inline] pub unsafe fn scx_dec_has_subs(_sch: *mut scx_sched) {}

#[macro_export]
macro_rules! scx_for_each_descendant_pre {
    ($pos:ident, $root:expr, $body:block) => {{
        $pos = unsafe { scx_next_descendant_pre(core::ptr::null_mut(), $root) };
        while !$pos.is_null() { $body; $pos = unsafe { scx_next_descendant_pre($pos, $root) }; }
    }};
}

#[cfg(feature = "CONFIG_EXT_SUB_SCHED")]
#[inline]
pub unsafe fn scx_missing_caps(sch: *mut scx_sched, cpu: i32, needed: u64) -> u64 {
    if !scx_has_subs() || (*sch).level == 0 { return 0; }
    let ecaps = READ_ONCE((*per_cpu_ptr((*sch).pcpu, cpu)).ecaps);
    needed & !ecaps
}
#[cfg(not(feature = "CONFIG_EXT_SUB_SCHED"))]
#[inline] pub unsafe fn scx_missing_caps(_sch: *mut scx_sched, _cpu: i32, _needed: u64) -> u64 { 0 }

#[cfg(feature = "CONFIG_EXT_SUB_SCHED")]
#[inline] pub unsafe fn scx_caps_for_enq(enq_flags: u64) -> u64 {
    if unlikely(enq_flags & SCX_ENQ_IGNORE_CAPS != 0) { return 0; }
    if enq_flags & SCX_ENQ_IMMED != 0 { SCX_CAP_ENQ_IMMED } else { SCX_CAP_ENQ }
}
#[cfg(feature = "CONFIG_EXT_SUB_SCHED")]
#[inline] pub unsafe fn scx_caps_for_task(p: *mut task_struct) -> u64 { if (*p).scx.flags & SCX_TASK_IMMED != 0 { SCX_CAP_ENQ_IMMED } else { SCX_CAP_ENQ } }
#[cfg(feature = "CONFIG_EXT_SUB_SCHED")]
#[inline] pub unsafe fn scx_caps_for_preempt(sch: *mut scx_sched, rq: *mut rq, enq_flags: u64) -> u64 {
    let curr = (*rq).curr;
    if unlikely(enq_flags & SCX_ENQ_IGNORE_CAPS != 0) || (*curr).sched_class != &ext_sched_class || scx_is_descendant(scx_task_sched(curr), sch) { 0 } else { SCX_CAP_PREEMPT }
}
#[cfg(feature = "CONFIG_EXT_SUB_SCHED")]
#[inline] pub unsafe fn scx_caps_implied(cap: u64) -> u64 { match cap { SCX_CAP_PREEMPT => SCX_CAP_ENQ | SCX_CAP_ENQ_IMMED, SCX_CAP_ENQ => SCX_CAP_ENQ_IMMED, _ => 0 } }
#[cfg(not(feature = "CONFIG_EXT_SUB_SCHED"))]
#[inline] pub unsafe fn scx_caps_for_preempt(_sch: *mut scx_sched, _rq: *mut rq, _enq_flags: u64) -> u64 { 0 }
#[cfg(not(feature = "CONFIG_EXT_SUB_SCHED"))]
#[inline] pub unsafe fn scx_task_can_stay_on_cpu(_rq: *mut rq, _p: *mut task_struct) -> bool { true }
#[cfg(feature = "CONFIG_EXT_SUB_SCHED")]
#[inline] pub unsafe fn scx_task_can_stay_on_cpu(rq: *mut rq, p: *mut task_struct) -> bool { !scx_has_subs() || is_migration_disabled(p) || likely(scx_missing_caps(scx_task_sched(p), cpu_of(rq), SCX_CAP_BASE) == 0) }
#[inline] pub unsafe fn scx_rescuee(rq: *mut rq) -> *mut task_struct { lockdep_assert_rq_held(rq); #[cfg(feature = "CONFIG_EXT_SUB_SCHED")] { if scx_has_subs() { return (*rq).scx.rescue.curr; } } core::ptr::null_mut() }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
