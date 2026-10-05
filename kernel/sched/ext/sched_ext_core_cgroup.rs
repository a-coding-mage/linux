// SPDX-License-Identifier: GPL-2.0
// F08 source repair of ext.c:4676-4995,5145-5207 at
// 126a30fae3bba11420ec2fcbde51a0a01bab1b5b. Native leaves remain explicit,
// unqualified C runtime boundaries. No executable admission is claimed.

compile_error!("SOURCE ONLY HOLD: sched_ext cgroup ABI and protection qualification incomplete");

use super::*;
use core::ptr;
use kernel::ffi::{c_int, c_ulong};

/// Resolve a callback argument at its native macro evaluation point.
///
/// # Safety
/// tg is NULL or live. The synchronous native caller keeps the selected cgroup
/// live under the original cgroup/rq protection. This adapter acquires no reference and
/// delegates all group/root policy to the single F06 Rust helper owner.
#[cfg(CONFIG_EXT_GROUP_SCHED)]
#[no_mangle]
pub unsafe extern "C" fn lupos_scx_core_cgroup_tg_cgrp(tg: *mut task_group) -> *mut cgroup {
    // SAFETY: Keep the helper call inside SCX_CALL_OP's argument evaluation,
    // including the task/locked-rq envelope of the move-task callback.
    unsafe { super::tg_cgrp(tg) }
}

/// Initialize exactly the original task-group defaults, leaving burst alone.
///
/// # Safety
/// tg is a live, exclusively writable native task_group undergoing its original
/// pre-publication initialization. Its other fields retain caller initialization.
#[cfg(CONFIG_EXT_GROUP_SCHED)]
#[no_mangle]
pub unsafe extern "C" fn scx_tg_init(tg: *mut task_group) {
    // SAFETY: The configured native fields and macro values are authoritative;
    // the original function does not initialize flags, sched or bw_burst_us.
    unsafe {
        (*tg).scx.weight = LUPOS_SCX_CORE_CGROUP_WEIGHT_DFL as u32;
        (*tg).scx.bw_period_us = lupos_scx_core_cgroup_default_bw_period_us();
        (*tg).scx.bw_quota_us = LUPOS_SCX_CORE_CGROUP_RUNTIME_INF as u64;
        (*tg).scx.idle = false;
    }
}

/// Resolve only the scheduler on which this task group's init succeeded.
///
/// # Safety
/// tg is live under cgroup_mutex or the cgroup ops rwsem read/write side. A
/// file-write caller pins tg through the CSS file lifetime; an autogroup has
/// no cgroup and uses root_task_group. The returned scheduler is borrowed only
/// for that same protection interval; this plain load does not pin it anew.
#[cfg(CONFIG_EXT_GROUP_SCHED)]
pub(crate) unsafe fn scx_tg_sched(mut tg: *mut task_group) -> *mut scx_sched {
    // SAFETY: Preserve the original assertion site before the autogroup/root
    // substitution. INITED, not a non-NULL stale sched pointer, gates the result.
    unsafe {
        lupos_scx_core_cgroup_assert_tg_sched();
        if (*tg).css.cgroup.is_null() {
            tg = lupos_scx_core_cgroup_root_task_group();
        }
        if (*tg).scx.flags & SCX_TG_INITED != 0 {
            (*tg).scx.sched
        } else {
            ptr::null_mut()
        }
    }
}

/// Resolve the parent task group's scheduler for cgroup knob callbacks.
///
/// # Safety
/// tg and its CSS ancestry are live under the protection required by
/// scx_tg_sched. A parent CSS outlives its children's cgroup files, allowing a
/// file-write caller's ops-rwsem read side to cover the parent scheduler read.
#[cfg(CONFIG_EXT_GROUP_SCHED)]
pub(crate) unsafe fn scx_tg_knob_sched(tg: *mut task_group) -> *mut scx_sched {
    // SAFETY: Check absent cgroup/parent before css_tg; callback ownership is
    // the parent at a sub attach point, and can legitimately resolve to NULL.
    unsafe {
        lupos_scx_core_cgroup_assert_tg_knob_sched();
        if (*tg).css.cgroup.is_null() || (*tg).css.parent.is_null() {
            scx_tg_sched(lupos_scx_core_cgroup_root_task_group())
        } else {
            scx_tg_sched(lupos_scx_core_cgroup_css_tg((*tg).css.parent))
        }
    }
}

/// Online a task group and publish its scheduler only after successful init.
///
/// # Safety
/// The cgroup core owns tg's online transition under cgroup_mutex. tg, its CSS
/// and the selected scheduler remain live through the synchronous callback.
/// Native callback presence bits/table and the lifetime notifier association
/// are initialized; the original online-before-file-publication order holds.
#[cfg(CONFIG_EXT_GROUP_SCHED)]
#[no_mangle]
pub unsafe extern "C" fn scx_tg_online(tg: *mut task_group) -> c_int {
    // SAFETY: Original cgroup serialization protects each plain field access.
    // Separate native macro leaves retain callback typing and warning sites.
    unsafe {
        let mut ret = 0;
        lupos_scx_core_cgroup_warn_online(tg);
        if lupos_scx_core_cgroup_enabled() {
            let sch = if lupos_scx_core_cgroup_on_dfl((*tg).css.cgroup) {
                lupos_scx_core_cgroup_online_sched((*tg).css.cgroup)
            } else {
                scx_tg_sched(lupos_scx_core_cgroup_root_task_group())
            };
            if lupos_scx_core_cgroup_has_online_init(sch) {
                let mut args = scx_cgroup_init_args {
                    weight: (*tg).scx.weight,
                    bw_period_us: (*tg).scx.bw_period_us,
                    bw_quota_us: (*tg).scx.bw_quota_us,
                    bw_burst_us: (*tg).scx.bw_burst_us,
                };
                ret = lupos_scx_core_cgroup_call_online_init(sch, tg, &mut args);
                if ret != 0 {
                    ret = lupos_scx_core_cgroup_sanitize_online_err(sch, ret);
                }
            }
            if ret == 0 {
                (*tg).scx.sched = sch;
                (*tg).scx.flags |= SCX_TG_ONLINE | SCX_TG_INITED;
            }
        } else {
            (*tg).scx.flags |= SCX_TG_ONLINE;
        }
        ret
    }
}

/// Exit an initialized online task group, then clear its association and flags.
///
/// # Safety
/// The cgroup core owns tg's offline transition under cgroup_mutex after its
/// files are drained. If INITED is set, tg's recorded scheduler is non-NULL,
/// initialized and live through the synchronous cgroup_exit callback.
#[cfg(CONFIG_EXT_GROUP_SCHED)]
#[no_mangle]
pub unsafe extern "C" fn scx_tg_offline(tg: *mut task_group) {
    // SAFETY: INITED is tested before the presence leaf dereferences sch. The
    // callback runs before association and ONLINE/INITED are cleared.
    unsafe {
        let sch = (*tg).scx.sched;
        lupos_scx_core_cgroup_warn_offline(tg);
        if lupos_scx_core_cgroup_enabled()
            && (*tg).scx.flags & SCX_TG_INITED != 0
            && lupos_scx_core_cgroup_has_offline_exit(sch)
        {
            lupos_scx_core_cgroup_call_offline_exit(sch, tg);
        }
        (*tg).scx.sched = ptr::null_mut();
        (*tg).scx.flags &= !(SCX_TG_ONLINE | SCX_TG_INITED);
    }
}

/// Borrow a migrating task's scheduler with the original protected RCU read.
///
/// # Safety
/// p is live and cgroup_mutex is held. Re-homing is excluded except at this
/// migration's own CGROUP_TASK_MIGRATED point; the borrow ends with protection.
#[cfg(CONFIG_EXT_GROUP_SCHED)]
pub(crate) unsafe fn scx_cgroup_task_sched(p: *mut task_struct) -> *mut scx_sched {
    // SAFETY: The native rcu_dereference_protected expression retains the exact
    // cgroup_mutex lockdep check, rather than relying on an unrelated task load.
    unsafe { lupos_scx_core_cgroup_task_sched_protected(p) }
}

/// Prepare nonidentity migrations which do not re-home a task's scheduler.
///
/// # Safety
/// The cgroup migration core holds cgroup_mutex and owns the live taskset,
/// destination CSS pointers and migration csets. Tasks, source/destination
/// groups and schedulers stay live through both passes and callbacks. A
/// successful preparation is paired with the original move/cancel lifecycle.
#[cfg(CONFIG_EXT_GROUP_SCHED)]
#[no_mangle]
pub unsafe extern "C" fn scx_cgroup_can_attach(tset: *mut cgroup_taskset) -> c_int {
    // SAFETY: Native first/next update the local destination-CSS slot only for
    // this taskset walk. Every returned task is handled before advancing it.
    unsafe {
        if !lupos_scx_core_cgroup_enabled() {
            return 0;
        }
        let mut css = ptr::null_mut();
        let mut p = lupos_scx_core_cgroup_taskset_first(tset, &mut css);
        let ret = 'prepare: {
            while !p.is_null() {
                let sch = scx_cgroup_task_sched(p);
                let from = tg_cgrp(lupos_scx_core_task_group(p));
                let to = tg_cgrp(lupos_scx_core_cgroup_css_tg(css));
                lupos_scx_core_cgroup_warn_moving_from(p);
                if from != to
                    && !sch.is_null()
                    && sch == lupos_scx_core_cgroup_migration_dst_sched(p)
                {
                    if lupos_scx_core_cgroup_has_prep_move(sch) {
                        let err = lupos_scx_core_cgroup_call_prep_move(
                            sch, p, from, css);
                        if err != 0 {
                            break 'prepare lupos_scx_core_cgroup_sanitize_prep_err(sch, err);
                        }
                    }
                    (*p).scx.cgrp_moving_from = from;
                }
                p = lupos_scx_core_cgroup_taskset_next(tset, &mut css);
            }
            return 0;
        };
        // Restart the entire taskset, including entries after the failure.
        // The saved source pointer gates the callback before sch is dereferenced.
        p = lupos_scx_core_cgroup_taskset_first(tset, &mut css);
        while !p.is_null() {
            let sch = scx_cgroup_task_sched(p);
            if !(*p).scx.cgrp_moving_from.is_null()
                && lupos_scx_core_cgroup_has_prepare_cancel(sch)
            {
                lupos_scx_core_cgroup_call_prepare_cancel(sch, p, css);
            }
            (*p).scx.cgrp_moving_from = ptr::null_mut();
            p = lupos_scx_core_cgroup_taskset_next(tset, &mut css);
        }
        ret
    }
}

/// Report the prepared move under the task's rq lock, then clear preparation.
///
/// # Safety
/// The original sched_move_task path holds cgroup_mutex and p's current rq
/// lock. p, its group and its associated scheduler remain live. A non-NULL
/// cgrp_moving_from represents this task's matching successful preparation;
/// task-based callback guards must not nest on the executing task.
#[cfg(CONFIG_EXT_GROUP_SCHED)]
#[no_mangle]
pub unsafe extern "C" fn scx_cgroup_move_task(p: *mut task_struct) {
    // SAFETY: CSS-only/identity migrations can have no saved source. Testing it
    // first avoids dereferencing an absent scheduler or emitting an unpaired op.
    unsafe {
        if !lupos_scx_core_cgroup_enabled() {
            return;
        }
        let sch = scx_cgroup_task_sched(p);
        if !(*p).scx.cgrp_moving_from.is_null() && lupos_scx_core_cgroup_has_move(sch) {
            // Evaluate task_rq(p), the saved source and the destination inside
            // the native task/locked-rq callback envelope, not before entry.
            lupos_scx_core_cgroup_call_move(sch, p);
        }
        (*p).scx.cgrp_moving_from = ptr::null_mut();
    }
}

/// Cancel every prepared task in a failed cgroup attachment.
///
/// # Safety
/// The migration core holds cgroup_mutex and owns the live taskset and its
/// destination CSS records. A saved source implies a matching non-NULL task
/// scheduler, kept live by the caller's migration protection until cancellation.
#[cfg(CONFIG_EXT_GROUP_SCHED)]
#[no_mangle]
pub unsafe extern "C" fn scx_cgroup_cancel_attach(tset: *mut cgroup_taskset) {
    // SAFETY: The saved source is checked before the scheduler presence leaf;
    // callback arguments remain borrowed synchronously and every slot is cleared.
    unsafe {
        if !lupos_scx_core_cgroup_enabled() {
            return;
        }
        let mut css = ptr::null_mut();
        let mut p = lupos_scx_core_cgroup_taskset_first(tset, &mut css);
        while !p.is_null() {
            let sch = scx_cgroup_task_sched(p);
            if !(*p).scx.cgrp_moving_from.is_null()
                && lupos_scx_core_cgroup_has_attach_cancel(sch)
            {
                lupos_scx_core_cgroup_call_attach_cancel(sch, p, css);
            }
            (*p).scx.cgrp_moving_from = ptr::null_mut();
            p = lupos_scx_core_cgroup_taskset_next(tset, &mut css);
        }
    }
}

/// Deliver a changed weight to the parent-owned knob scheduler before storing it.
///
/// # Safety
/// tg is live under the cgroup file-write/CSS lifetime or original init path.
/// The caller permits sleeping on the ops rwsem and owns native knob mutation
/// serialization. Native callback state stays valid under the acquired read side.
#[cfg(CONFIG_EXT_GROUP_SCHED)]
#[no_mangle]
pub unsafe extern "C" fn scx_group_set_weight(tg: *mut task_group, weight: c_ulong) {
    // SAFETY: The read side spans lookup, callback and write. Comparison promotes
    // the native u32 to unsigned long; callback/store retain native narrowing.
    unsafe {
        lupos_scx_core_cgroup_weight_down_read();
        let sch = scx_tg_knob_sched(tg);
        if lupos_scx_core_cgroup_enabled()
            && !sch.is_null()
            && lupos_scx_core_cgroup_has_set_weight(sch)
            && (*tg).scx.weight as c_ulong != weight
        {
            lupos_scx_core_cgroup_call_set_weight(sch, tg, weight);
        }
        (*tg).scx.weight = weight as u32;
        lupos_scx_core_cgroup_weight_up_read();
    }
}

/// Deliver an idle knob write, including unchanged values, then store the state.
///
/// # Safety
/// Same live-CSS, knob serialization and sleepable ops-rwsem contract as
/// scx_group_set_weight. The callback is synchronous under its read protection.
#[cfg(CONFIG_EXT_GROUP_SCHED)]
#[no_mangle]
pub unsafe extern "C" fn scx_group_set_idle(tg: *mut task_group, idle: bool) {
    // SAFETY: Preserve callback-before-store and the absence of an equality
    // filter. Lookup may return NULL when a parent initialization has failed.
    unsafe {
        lupos_scx_core_cgroup_idle_down_read();
        let sch = scx_tg_knob_sched(tg);
        if lupos_scx_core_cgroup_enabled()
            && !sch.is_null()
            && lupos_scx_core_cgroup_has_set_idle(sch)
        {
            lupos_scx_core_cgroup_call_set_idle(sch, tg, idle);
        }
        (*tg).scx.idle = idle;
        lupos_scx_core_cgroup_idle_up_read();
    }
}

/// Deliver any bandwidth change, then store all three values under the read side.
///
/// # Safety
/// Same live-CSS, knob serialization and sleepable ops-rwsem contract as
/// scx_group_set_weight; period, quota and burst are the caller-validated values.
#[cfg(CONFIG_EXT_GROUP_SCHED)]
#[no_mangle]
pub unsafe extern "C" fn scx_group_set_bandwidth(
    tg: *mut task_group, period_us: u64, quota_us: u64, burst_us: u64,
) {
    // SAFETY: Compare all native u64 values without conversion and preserve the
    // original callback-before-all-three-stores order, even if no op is present.
    unsafe {
        lupos_scx_core_cgroup_bandwidth_down_read();
        let sch = scx_tg_knob_sched(tg);
        if lupos_scx_core_cgroup_enabled()
            && !sch.is_null()
            && lupos_scx_core_cgroup_has_set_bandwidth(sch)
            && ((*tg).scx.bw_period_us != period_us
                || (*tg).scx.bw_quota_us != quota_us
                || (*tg).scx.bw_burst_us != burst_us)
        {
            lupos_scx_core_cgroup_call_set_bandwidth(
                sch, tg, period_us, quota_us, burst_us);
        }
        (*tg).scx.bw_period_us = period_us;
        (*tg).scx.bw_quota_us = quota_us;
        (*tg).scx.bw_burst_us = burst_us;
        lupos_scx_core_cgroup_bandwidth_up_read();
    }
}

/// Borrow the configured default root, or preserve the original NULL alternative.
///
/// # Safety
/// With GROUP or SUB enabled, the native default hierarchy is initialized and
/// the caller retains its required cgroup/scheduler lifetime protection.
pub(crate) unsafe fn root_cgroup() -> *mut cgroup {
    #[cfg(any(CONFIG_EXT_GROUP_SCHED, CONFIG_EXT_SUB_SCHED))]
    // SAFETY: Native storage supplies the actual embedded default-root cgroup.
    unsafe { lupos_scx_core_cgroup_default_root() }
    #[cfg(not(any(CONFIG_EXT_GROUP_SCHED, CONFIG_EXT_SUB_SCHED)))]
    { ptr::null_mut() }
}

/// Lock cgroups before taking the optional group-ops write side.
///
/// # Safety
/// Caller may sleep, does not already own either nonrecursive lock, and follows
/// the original external enable/fork lock order. Every acquisition must be
/// paired with scx_cgroup_unlock by the same task. cgroup_mutex must be outermost
/// to avoid deadlocking teardown against a blocked cgroup file write.
#[cfg(any(CONFIG_EXT_GROUP_SCHED, CONFIG_EXT_SUB_SCHED))]
#[no_mangle]
pub unsafe extern "C" fn scx_cgroup_lock() {
    // SAFETY: Keep cgroup_lock first; GROUP contributes the nested write side,
    // whereas SUB alone only locks cgroup_mutex, exactly as the native source.
    unsafe {
        lupos_scx_core_cgroup_mutex_lock();
        #[cfg(CONFIG_EXT_GROUP_SCHED)]
        lupos_scx_core_cgroup_ops_down_write();
    }
}

/// Release the optional group-ops write side before unlocking cgroups.
///
/// # Safety
/// This task owns one outstanding scx_cgroup_lock acquisition and has ended all
/// borrows protected solely by it; the release must follow original lock order.
#[cfg(any(CONFIG_EXT_GROUP_SCHED, CONFIG_EXT_SUB_SCHED))]
#[no_mangle]
pub unsafe extern "C" fn scx_cgroup_unlock() {
    // SAFETY: Reverse precisely the locks acquired by the enabled lock body.
    unsafe {
        #[cfg(CONFIG_EXT_GROUP_SCHED)]
        lupos_scx_core_cgroup_ops_up_write();
        lupos_scx_core_cgroup_mutex_unlock();
    }
}

/// Preserve the original private no-op when neither cgroup feature is enabled.
///
/// # Safety
/// No cgroup protection is supplied in this configuration; callers must not
/// assume this no-op stabilizes any cgroup or scheduler object.
#[cfg(not(any(CONFIG_EXT_GROUP_SCHED, CONFIG_EXT_SUB_SCHED)))]
pub(crate) unsafe fn scx_cgroup_lock() {}

/// Preserve the matching original private no-op in the feature-absent case.
///
/// # Safety
/// Must be used only with the feature-absent lock alternative, never to release
/// real native locks acquired by another path.
#[cfg(not(any(CONFIG_EXT_GROUP_SCHED, CONFIG_EXT_SUB_SCHED)))]
pub(crate) unsafe fn scx_cgroup_unlock() {}

/// Exit root-scheduler task groups in postorder, clearing failed-init owners too.
///
/// # Safety
/// Caller holds scx_cgroup_lock and the root scheduler's lifecycle protection.
/// sch, root_task_group and the full CSS walk remain live; online/offline and
/// file writes are excluded until exit completes. Callback tables remain live.
#[cfg(CONFIG_EXT_GROUP_SCHED)]
pub(crate) unsafe fn scx_cgroup_exit(sch: *mut scx_sched) {
    // SAFETY: Native CSS iteration supplies each live task_group. Association is
    // cleared unconditionally; INITED is cleared before its optional callback.
    unsafe {
        let mut css = lupos_scx_core_cgroup_next_post(ptr::null_mut());
        while !css.is_null() {
            let tg = lupos_scx_core_cgroup_css_tg(css);
            (*tg).scx.sched = ptr::null_mut();
            if (*tg).scx.flags & SCX_TG_INITED != 0 {
                (*tg).scx.flags &= !SCX_TG_INITED;
                if lupos_scx_core_cgroup_root_exit_present(sch) {
                    lupos_scx_core_cgroup_call_root_exit(sch, css);
                }
            }
            css = lupos_scx_core_cgroup_next_post(css);
        }
    }
}

/// Initialize online, not-yet-initialized root task groups in preorder.
///
/// # Safety
/// Caller holds scx_cgroup_lock and pins sch throughout the walk and callback.
/// The root CSS tree is stable; online/offline and file writes are excluded.
/// The caller owns cleanup of any successfully initialized prefix on failure.
#[cfg(CONFIG_EXT_GROUP_SCHED)]
pub(crate) unsafe fn scx_cgroup_init(sch: *mut scx_sched) -> c_int {
    // SAFETY: Native iteration includes the root. Publish only successful init;
    // the first raw callback error is reported and returned without sanitizing.
    unsafe {
        let mut css = lupos_scx_core_cgroup_next_pre(ptr::null_mut());
        while !css.is_null() {
            let tg = lupos_scx_core_cgroup_css_tg(css);
            if (*tg).scx.flags & (SCX_TG_ONLINE | SCX_TG_INITED) == SCX_TG_ONLINE {
                if lupos_scx_core_cgroup_root_init_present(sch) {
                    let mut args = scx_cgroup_init_args {
                        weight: (*tg).scx.weight,
                        bw_period_us: (*tg).scx.bw_period_us,
                        bw_quota_us: (*tg).scx.bw_quota_us,
                        bw_burst_us: (*tg).scx.bw_burst_us,
                    };
                    let ret = lupos_scx_core_cgroup_call_root_init(sch, css, &mut args);
                    if ret != 0 {
                        lupos_scx_core_cgroup_error_root_init(sch, ret);
                        return ret;
                    }
                }
                (*tg).scx.sched = sch;
                (*tg).scx.flags |= SCX_TG_INITED;
            }
            css = lupos_scx_core_cgroup_next_pre(css);
        }
        0
    }
}

/// Preserve the original !GROUP root-exit no-op, including SUB-only builds.
///
/// # Safety
/// No pointer is dereferenced; this alternative performs no group teardown.
#[cfg(not(CONFIG_EXT_GROUP_SCHED))]
pub(crate) unsafe fn scx_cgroup_exit(_sch: *mut scx_sched) {}

/// Preserve the original !GROUP successful empty root-init alternative.
///
/// # Safety
/// No pointer is dereferenced; success means only that group init is absent.
#[cfg(not(CONFIG_EXT_GROUP_SCHED))]
pub(crate) unsafe fn scx_cgroup_init(_sch: *mut scx_sched) -> c_int { 0 }
