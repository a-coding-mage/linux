// SPDX-License-Identifier: GPL-2.0
// Lifecycle/cgroup-notifier family from pinned sub.c:1230–2188.
// Included by the existing sub.rs owner only with CONFIG_EXT_SUB_SCHED.
// Native bindings, never handwritten layouts, own all kernel ABI details.
compile_error!("SOURCE ONLY HOLD: sched_ext lifecycle is not admitted");

/// Re-home a task whose initialization on `to` has already completed.
///
/// # Safety
/// Both schedulers and p are pinned. Caller holds p->pi_lock and its rq lock;
/// `to` owns the completed init. The old scheduler's native task state may
/// include NONE after a failed parent's earlier punt; the exit helper handles it.
unsafe fn scx_rehome_task(to: *mut scx_sched, p: *mut task_struct) {
    // SAFETY: The caller holds both locks. Every transition stays within the
    // exact native dequeue/requeue interval; no branch bypasses change_end.
    unsafe {
        lupos_scx_sub_lifecycle_assert_rehome_task_locked(p);
        let change = lupos_scx_sub_lifecycle_change_begin(p);
        scx_disable_and_exit_task(lupos_scx_sub_lifecycle_task_sched(p), p);
        scx_set_task_state(p, LUPOS_SCX_SUB_LIFECYCLE_TASK_INIT_BEGIN as u32);
        scx_set_task_state(p, LUPOS_SCX_SUB_LIFECYCLE_TASK_INIT as u32);
        scx_set_task_sched(p, to);
        scx_set_task_state(p, LUPOS_SCX_SUB_LIFECYCLE_TASK_READY as u32);
        if lupos_scx_sub_lifecycle_task_is_ext(p) {
            scx_enable_task(to, p);
        }
        lupos_scx_sub_lifecycle_change_end(change);
    }
}

/// Transfer a task without initialization to a dying, bypassed scheduler.
///
/// # Safety
/// p and to are pinned, p's pi/rq locks are held, and to is already bypassed.
/// Its disable path must subsequently visit and re-home this SCX_TASK_NONE task.
unsafe fn scx_punt_task(to: *mut scx_sched, p: *mut task_struct) {
    // SAFETY: Punting preserves the original sched_change interval and never
    // enables an uninitialized task; the failed scheduler retains custody.
    unsafe {
        lupos_scx_sub_lifecycle_assert_punt_task_locked(p);
        lupos_scx_sub_lifecycle_warn_not_bypassed(to);
        let change = lupos_scx_sub_lifecycle_change_begin(p);
        scx_disable_and_exit_task(lupos_scx_sub_lifecycle_task_sched(p), p);
        scx_set_task_sched(p, to);
        lupos_scx_sub_lifecycle_change_end(change);
    }
}

/// Fail the parent and punt any remaining tasks after an init failure.
///
/// # Safety
/// Caller holds enable mutex, fork writer and cgroup locks. sch is linked and
/// pinned, and failed retains the explicit task reference across its unlocked init.
unsafe fn scx_fail_parent(sch: *mut scx_sched, failed: *mut task_struct, fail_code: i32) {
    // SAFETY: Shared serialization pins the parent and css subtree. The native
    // iterator pins each yielded task and holds its pi/rq locks until next/stop.
    unsafe {
        let parent = lupos_scx_sub_parent(sch);
        lupos_scx_sub_lifecycle_fail_parent(parent, failed, fail_code);
        scx_bypass(parent, true);
        let mut sti = core::mem::MaybeUninit::<scx_task_iter>::uninit();
        scx_task_iter_start(sti.as_mut_ptr(), (*sch).cgrp);
        loop {
            let p = scx_task_iter_next_locked(sti.as_mut_ptr());
            if p.is_null() {
                break;
            }
            if lupos_scx_sub_lifecycle_task_on_sched(parent, p) {
                continue;
            }
            scx_punt_task(parent, p);
        }
        scx_task_iter_stop(sti.as_mut_ptr());
    }
}

/// Claim the subtree's task_groups in two passes, before claiming any tasks.
///
/// # Safety
/// Enable mutex, fork writer and cgroup locks are held; cgroup ownership was
/// already published to sch. The parent, sch, and all visited css objects are
/// pinned by those locks. Dying, not-yet-offlined task_groups must be included.
#[cfg(CONFIG_EXT_GROUP_SCHED)]
unsafe fn scx_cgroup_claim_subtree(sch: *mut scx_sched) -> i32 {
    // SAFETY: Native css iteration includes the ecss root and does not filter
    // dying groups. SUB_INIT records precisely the initialized set for either
    // commit or unwind; no parent exit runs before the entire first pass succeeds.
    unsafe {
        let sub_cgrp = sch_cgroup(sch);
        let ecss = lupos_scx_sub_lifecycle_cpu_ecss(sub_cgrp);
        let parent = lupos_scx_sub_parent(sch);
        let mut css = ptr::null_mut();
        let mut ret = 0;
        loop {
            css = lupos_scx_sub_lifecycle_css_pre(css, ecss);
            if css.is_null() {
                break;
            }
            let tg = lupos_scx_sub_lifecycle_css_tg(css);
            let mut args = scx_cgroup_init_args {
                weight: (*tg).scx.weight,
                bw_period_us: (*tg).scx.bw_period_us,
                bw_quota_us: (*tg).scx.bw_quota_us,
                bw_burst_us: (*tg).scx.bw_burst_us,
            };
            if (*tg).scx.sched != parent ||
                !lupos_scx_sub_lifecycle_cgroup_descendant((*css).cgroup, sub_cgrp) {
                continue;
            }
            if lupos_scx_sub_lifecycle_has_cgroup_init(sch) {
                ret = lupos_scx_sub_lifecycle_call_cgroup_claim_init(sch, (*css).cgroup, &mut args);
                if ret != 0 {
                    lupos_scx_sub_lifecycle_cgroup_init_failed(sch, ret);
                    break;
                }
            }
            (*tg).scx.flags |= LUPOS_SCX_SUB_LIFECYCLE_TG_SUB_INIT as u32;
        }
        css = ptr::null_mut();
        if ret != 0 {
            // Error pass: parent ownership/INITED is untouched, including the
            // failing group which never acquired the SUB_INIT progress mark.
            loop {
                css = lupos_scx_sub_lifecycle_css_post(css, ecss);
                if css.is_null() {
                    break;
                }
                let tg = lupos_scx_sub_lifecycle_css_tg(css);
                if (*tg).scx.flags & LUPOS_SCX_SUB_LIFECYCLE_TG_SUB_INIT as u32 == 0 {
                    continue;
                }
                if lupos_scx_sub_lifecycle_has_cgroup_exit(sch) {
                    lupos_scx_sub_lifecycle_call_cgroup_claim_cancel_exit(sch, (*css).cgroup);
                }
                (*tg).scx.flags &= !(LUPOS_SCX_SUB_LIFECYCLE_TG_SUB_INIT as u32);
            }
            return ret;
        }
        loop {
            css = lupos_scx_sub_lifecycle_css_post(css, ecss);
            if css.is_null() {
                break;
            }
            let tg = lupos_scx_sub_lifecycle_css_tg(css);
            if (*tg).scx.flags & LUPOS_SCX_SUB_LIFECYCLE_TG_SUB_INIT as u32 == 0 {
                continue;
            }
            if (*tg).scx.flags & LUPOS_SCX_SUB_LIFECYCLE_TG_INITED as u32 != 0 &&
                lupos_scx_sub_lifecycle_has_cgroup_exit(parent) {
                lupos_scx_sub_lifecycle_call_cgroup_claim_parent_exit(parent, (*css).cgroup);
            }
            (*tg).scx.sched = sch;
            (*tg).scx.flags |= LUPOS_SCX_SUB_LIFECYCLE_TG_INITED as u32;
            (*tg).scx.flags &= !(LUPOS_SCX_SUB_LIFECYCLE_TG_SUB_INIT as u32);
        }
        0
    }
}

/// Return the subtree's task_groups before tasks are re-homed to the parent.
///
/// # Safety
/// The three enable/fork/cgroup locks are held and subtree cgroup ownership has
/// already been reset to parent. All css/scheduler pointers stay pinned.
#[cfg(CONFIG_EXT_GROUP_SCHED)]
unsafe fn scx_cgroup_return_subtree(sch: *mut scx_sched) {
    // SAFETY: Postorder exit precedes preorder init using current tg settings.
    // A failed parent init leaves that and all later marked groups uninitialized
    // on the dying parent; earlier unrelated punted groups cannot match SUB_INIT.
    unsafe {
        let sub_cgrp = sch_cgroup(sch);
        let ecss = lupos_scx_sub_lifecycle_cpu_ecss(sub_cgrp);
        let parent = lupos_scx_sub_parent(sch);
        let mut css = ptr::null_mut();
        loop {
            css = lupos_scx_sub_lifecycle_css_post(css, ecss);
            if css.is_null() {
                break;
            }
            let tg = lupos_scx_sub_lifecycle_css_tg(css);
            if (*tg).scx.sched != sch ||
                !lupos_scx_sub_lifecycle_cgroup_descendant((*css).cgroup, sub_cgrp) {
                continue;
            }
            if (*tg).scx.flags & LUPOS_SCX_SUB_LIFECYCLE_TG_INITED as u32 != 0 &&
                lupos_scx_sub_lifecycle_has_cgroup_exit(sch) {
                lupos_scx_sub_lifecycle_call_cgroup_return_exit(sch, (*css).cgroup);
            }
            (*tg).scx.sched = parent;
            (*tg).scx.flags |= LUPOS_SCX_SUB_LIFECYCLE_TG_SUB_INIT as u32;
        }
        css = ptr::null_mut();
        let mut parent_failed = false;
        loop {
            css = lupos_scx_sub_lifecycle_css_pre(css, ecss);
            if css.is_null() {
                break;
            }
            let tg = lupos_scx_sub_lifecycle_css_tg(css);
            let mut args = scx_cgroup_init_args {
                weight: (*tg).scx.weight,
                bw_period_us: (*tg).scx.bw_period_us,
                bw_quota_us: (*tg).scx.bw_quota_us,
                bw_burst_us: (*tg).scx.bw_burst_us,
            };
            lupos_scx_sub_lifecycle_warn_unreturned(tg, sch);
            if (*tg).scx.flags & LUPOS_SCX_SUB_LIFECYCLE_TG_SUB_INIT as u32 == 0 {
                continue;
            }
            (*tg).scx.flags &= !(LUPOS_SCX_SUB_LIFECYCLE_TG_SUB_INIT as u32 |
                LUPOS_SCX_SUB_LIFECYCLE_TG_INITED as u32);
            if parent_failed {
                continue;
            }
            if lupos_scx_sub_lifecycle_has_cgroup_init(parent) {
                let ret = lupos_scx_sub_lifecycle_call_cgroup_return_init(parent, (*css).cgroup, &mut args);
                if ret != 0 {
                    lupos_scx_sub_lifecycle_cgroup_return_failed(parent, ret);
                    parent_failed = true;
                    continue;
                }
            }
            (*tg).scx.flags |= LUPOS_SCX_SUB_LIFECYCLE_TG_INITED as u32;
        }
    }
}

/// Native CONFIG_EXT_GROUP_SCHED-disabled behavior.
///
/// # Safety
/// No pointer is accessed; the surrounding sub enable retains its normal locks.
#[cfg(not(CONFIG_EXT_GROUP_SCHED))]
unsafe fn scx_cgroup_claim_subtree(_sch: *mut scx_sched) -> i32 { 0 }

/// Native CONFIG_EXT_GROUP_SCHED-disabled behavior.
///
/// # Safety
/// No pointer is accessed; the surrounding sub disable retains its normal locks.
#[cfg(not(CONFIG_EXT_GROUP_SCHED))]
unsafe fn scx_cgroup_return_subtree(_sch: *mut scx_sched) {}

/// Wait until every linked descendant has reached its unlinking stage.
///
/// # Safety
/// sch and its children-list head must remain live throughout the wait. Caller
/// is in an IRQ-enabled sleepable context with forward progress guaranteed and
/// holds no lock needed by descendant teardown, including scx_enable_mutex.
#[no_mangle]
pub unsafe extern "C" fn drain_descendants(sch: *mut scx_sched) {
    // SAFETY: Native wait_event retains its predicate/recheck/wakeup protocol
    // and the single family-owned waitqueue; no polling or copied list is used.
    unsafe { lupos_scx_sub_lifecycle_wait_children(sch) };
}

/// Disable a sub-scheduler, returning its cgroups and tasks to its parent.
///
/// # Safety
/// sch is a live, allocated non-root scheduler owned by its single disable
/// worker; it may be unlinked after a failed enable. Its kobject and retained
/// parent reference remain valid through all callbacks and sysfs removal.
/// Caller is IRQ-enabled, permits sleeping and both RCU waits, and holds none
/// of the enable/fork/cgroup or other locks needed by descendant teardown.
#[no_mangle]
pub unsafe extern "C" fn scx_sub_disable(sch: *mut scx_sched) {
    // SAFETY: Bypass guarantees progress before any blocking lock. Iterator
    // references are extended explicitly before dropping task locks; every
    // successful init is consumed by re-home or cancelled after the DEAD race.
    // Locks are released before sub_detach/exit exactly as in native sub.c.
    unsafe {
        let parent = lupos_scx_sub_parent(sch);
        scx_bypass(sch, true);
        drain_descendants(sch);
        lupos_scx_sub_lifecycle_enable_lock();
        lupos_scx_sub_lifecycle_fork_down();
        scx_cgroup_lock();
        if (*sch).linked {
            // An unlinked failed enable never owned tasks or cgroups.
            set_cgroup_sched(sch_cgroup(sch), parent);
            scx_cgroup_return_subtree(sch);
            let mut sti = core::mem::MaybeUninit::<scx_task_iter>::uninit();
            scx_task_iter_start(sti.as_mut_ptr(), (*sch).cgrp);
            loop {
                let p = scx_task_iter_next_locked(sti.as_mut_ptr());
                if p.is_null() {
                    break;
                }
                if lupos_scx_sub_lifecycle_task_on_sched(parent, p) {
                    continue;
                }
                lupos_scx_sub_lifecycle_warn_wrong_sched(sch, p);
                lupos_scx_sub_lifecycle_get_task(p);
                scx_task_iter_unlock(sti.as_mut_ptr());
                let ret = __scx_init_task(parent, p, ptr::null_mut(), false);
                if ret != 0 {
                    scx_fail_parent(sch, p, ret);
                    lupos_scx_sub_lifecycle_put_task(p);
                    break;
                }
                let mut rf = core::mem::MaybeUninit::<rq_flags>::uninit();
                let rq = lupos_scx_sub_lifecycle_task_lock(p, rf.as_mut_ptr());
                if scx_get_task_state(p) == LUPOS_SCX_SUB_LIFECYCLE_TASK_DEAD as u32 {
                    // The old scheduler got exit_task from sched_ext_dead;
                    // the parent's just-completed initialization is still owed exit.
                    scx_sub_init_cancel_task(parent, p);
                    lupos_scx_sub_lifecycle_task_unlock(rq, p, rf.as_mut_ptr());
                    lupos_scx_sub_lifecycle_put_task(p);
                    continue;
                }
                scx_rehome_task(parent, p);
                lupos_scx_sub_lifecycle_task_unlock(rq, p, rf.as_mut_ptr());
                lupos_scx_sub_lifecycle_put_task(p);
            }
            scx_task_iter_stop(sti.as_mut_ptr());
        }
        scx_disable_dump(sch);
        scx_cgroup_unlock();
        lupos_scx_sub_lifecycle_fork_up();
        lupos_scx_sub_lifecycle_synchronize_expedited();
        scx_disable_bypass_dsp(sch);
        scx_unlink_sched(sch);
        lupos_scx_sub_lifecycle_enable_unlock();
        lupos_scx_sub_lifecycle_wake_unlink();
        if lupos_scx_sub_lifecycle_has_sub_detach(parent) && (*sch).sub_attached {
            let mut args = scx_sub_detach_args {
                ops: lupos_scx_sub_ops(sch),
                cgroup_path: (*sch).cgrp_path,
            };
            lupos_scx_sub_lifecycle_call_sub_detach(parent, &mut args);
        }
        scx_log_sched_disable(sch);
        if lupos_scx_sub_lifecycle_has_exit(sch) {
            lupos_scx_sub_lifecycle_call_exit(sch);
        }
        // Non-ops programs may still run after ops.exit; stop new resolvers,
        // then drain old ones before removing sysfs objects.
        lupos_scx_sub_lifecycle_mark_dead(sch);
        synchronize_rcu();
        if !(*sch).sub_kset.is_null() {
            kobject_del(ptr::addr_of_mut!((*(*sch).sub_kset).kobj));
        }
        if lupos_scx_sub_lifecycle_in_sysfs(sch) {
            kobject_del(ptr::addr_of_mut!((*sch).kobj));
        }
    }
}

/// Find the existing owner to which a new cgroup scheduler may attach.
///
/// # Safety
/// cgrp is reference-pinned, and enable mutex plus scx_sched_lock are held.
/// The returned scheduler is borrowed until the caller gets its kobject
/// reference before unlocking scx_sched_lock.
unsafe fn find_parent_sched(cgrp: *mut cgroup) -> *mut scx_sched {
    // SAFETY: Native RCU/check accessor and ordinary locked list iteration use
    // the one shared scheduler tree. Error pointers are formed by native ERR_PTR.
    unsafe {
        let parent = lupos_scx_sub_lifecycle_cgroup_sched(cgrp);
        lupos_scx_sub_lifecycle_assert_sched_locked();
        if (*parent).cgrp == cgrp {
            return lupos_scx_sub_lifecycle_err_ptr(-(LUPOS_SCX_SUB_LIFECYCLE_EBUSY as i32)).cast();
        }
        if !lupos_scx_sub_lifecycle_has_sub_attach(parent) {
            return lupos_scx_sub_lifecycle_err_ptr(-(LUPOS_SCX_SUB_LIFECYCLE_EOPNOTSUPP as i32)).cast();
        }
        let mut pos = ptr::null_mut();
        loop {
            pos = lupos_scx_sub_lifecycle_next_child(parent, pos);
            if pos.is_null() {
                break;
            }
            if lupos_scx_sub_lifecycle_cgroup_descendant((*pos).cgrp, cgrp) {
                return lupos_scx_sub_lifecycle_err_ptr(-(LUPOS_SCX_SUB_LIFECYCLE_EBUSY as i32)).cast();
            }
        }
        parent
    }
}

/// Validate a locked task before either enable pass.
///
/// # Safety
/// The iterator pins p and holds its task/rq locks. This reports invalid states
/// without changing them; the caller decides whether it can unwind or proceed.
unsafe fn assert_task_ready_or_enabled(p: *mut task_struct) -> bool {
    // SAFETY: The held locks serialize the state. p remains pinned for the
    // native diagnostic's name/pid accesses, as in the original assertion.
    unsafe {
        let state = scx_get_task_state(p);
        if state == LUPOS_SCX_SUB_LIFECYCLE_TASK_READY as u32 ||
            state == LUPOS_SCX_SUB_LIFECYCLE_TASK_ENABLED as u32 {
            return true;
        }
        lupos_scx_sub_lifecycle_warn_task_state(p, state);
        false
    }
}

/// Initialize, then claim all subtree tasks; cancel every partial init on error.
///
/// # Safety
/// The enable/fork/cgroup locks are held, cgroups have been claimed, and sch
/// remains bypassed. parent is pinned. No other sub enable may be in progress.
unsafe fn scx_sub_enable_tasks(sch: *mut scx_sched, parent: *mut scx_sched) -> i32 {
    // SAFETY: SUB_INIT marks exact per-task progress across duplicate iterator
    // visits and exiting tasks. Keep the shared enabling pointer published until
    // every mark is transferred or cancelled, including the failure unwind.
    unsafe {
        lupos_scx_sub_lifecycle_warn_enabling();
        lupos_scx_sub_lifecycle_set_enabling(sch);
        let mut sti = core::mem::MaybeUninit::<scx_task_iter>::uninit();
        let mut ret = 0;
        scx_task_iter_start(sti.as_mut_ptr(), (*sch).cgrp);
        loop {
            let p = scx_task_iter_next_locked(sti.as_mut_ptr());
            if p.is_null() {
                break;
            }
            if (*p).scx.flags & LUPOS_SCX_SUB_LIFECYCLE_TASK_SUB_INIT as u32 != 0 {
                continue;
            }
            lupos_scx_sub_lifecycle_get_task(p);
            if !assert_task_ready_or_enabled(p) {
                ret = -(LUPOS_SCX_SUB_LIFECYCLE_EINVAL as i32);
                lupos_scx_sub_lifecycle_put_task(p);
                break;
            }
            scx_task_iter_unlock(sti.as_mut_ptr());
            ret = __scx_init_task(sch, p, ptr::null_mut(), false);
            if ret != 0 {
                lupos_scx_sub_lifecycle_put_task(p);
                break;
            }
            let mut rf = core::mem::MaybeUninit::<rq_flags>::uninit();
            let rq = lupos_scx_sub_lifecycle_task_lock(p, rf.as_mut_ptr());
            if scx_get_task_state(p) == LUPOS_SCX_SUB_LIFECYCLE_TASK_DEAD as u32 {
                scx_sub_init_cancel_task(sch, p);
                lupos_scx_sub_lifecycle_task_unlock(rq, p, rf.as_mut_ptr());
                lupos_scx_sub_lifecycle_put_task(p);
                continue;
            }
            (*p).scx.flags |= LUPOS_SCX_SUB_LIFECYCLE_TASK_SUB_INIT as u32;
            lupos_scx_sub_lifecycle_task_unlock(rq, p, rf.as_mut_ptr());
            lupos_scx_sub_lifecycle_put_task(p);
        }
        scx_task_iter_stop(sti.as_mut_ptr());
        if ret != 0 {
            scx_task_iter_start(sti.as_mut_ptr(), (*sch).cgrp);
            loop {
                let p = scx_task_iter_next_locked(sti.as_mut_ptr());
                if p.is_null() {
                    break;
                }
                if (*p).scx.flags & LUPOS_SCX_SUB_LIFECYCLE_TASK_SUB_INIT as u32 != 0 {
                    // No enable ran on sch: cancel init only, never disable.
                    scx_sub_init_cancel_task(sch, p);
                    (*p).scx.flags &= !(LUPOS_SCX_SUB_LIFECYCLE_TASK_SUB_INIT as u32);
                }
            }
            scx_task_iter_stop(sti.as_mut_ptr());
            lupos_scx_sub_lifecycle_set_enabling(ptr::null_mut());
            return ret;
        }
        scx_task_iter_start(sti.as_mut_ptr(), (*sch).cgrp);
        loop {
            let p = scx_task_iter_next_locked(sti.as_mut_ptr());
            if p.is_null() {
                break;
            }
            if (*p).scx.flags & LUPOS_SCX_SUB_LIFECYCLE_TASK_SUB_INIT as u32 == 0 {
                continue;
            }
            let change = lupos_scx_sub_lifecycle_change_begin(p);
            assert_task_ready_or_enabled(p);
            __scx_disable_and_exit_task(parent, p);
            scx_set_task_sched(p, sch);
            if lupos_scx_sub_lifecycle_task_is_ext(p) {
                scx_enable_task(sch, p);
            }
            (*p).scx.flags &= !(LUPOS_SCX_SUB_LIFECYCLE_TASK_SUB_INIT as u32);
            lupos_scx_sub_lifecycle_change_end(change);
        }
        scx_task_iter_stop(sti.as_mut_ptr());
        lupos_scx_sub_lifecycle_set_enabling(ptr::null_mut());
        0
    }
}

/// Prepare an allocated scheduler, attach it, then claim its cgroups and tasks.
///
/// # Safety
/// Caller holds enable mutex and owns the allocated sch. cgrp was consumed by
/// that allocation, parent is retained by sch, and ops belongs to the live cmd.
/// Returns with enable mutex held; any acquired fork/cgroup locks are released.
unsafe fn scx_sub_enable_prepared(sch: *mut scx_sched, parent: *mut scx_sched,
    cgrp: *mut cgroup, ops: *mut sched_ext_ops) -> i32 {
    // SAFETY: Validation and arena/pshard allocation precede publication. Every
    // error preserves the same resources for the disable worker. Once bypass is
    // entered, failures leave it on. Group/task ownership changes share one
    // fork-writer/cgroup interval and keep cgroups-before-tasks ordering.
    unsafe {
        let mut ret = scx_validate_ops(sch, ops);
        if ret != 0 { return ret; }
        scx_rescue_check_timeout(sch);
        ret = scx_arena_pool_init(sch);
        if ret != 0 { return ret; }
        ret = scx_alloc_pshards(sch);
        if ret != 0 { return ret; }
        ret = scx_link_sched(sch);
        if ret != 0 { return ret; }
        ret = scx_sched_sysfs_add(sch);
        if ret != 0 { return ret; }
        if (*sch).level >= LUPOS_SCX_SUB_LIFECYCLE_MAX_DEPTH as i32 {
            lupos_scx_sub_lifecycle_max_depth_error(sch);
            return -(LUPOS_SCX_SUB_LIFECYCLE_EINVAL as i32);
        }
        if lupos_scx_sub_lifecycle_has_init(sch) {
            ret = lupos_scx_sub_lifecycle_call_init(sch);
            if ret != 0 {
                ret = lupos_scx_sub_lifecycle_sanitize_init(sch, ret);
                lupos_scx_sub_lifecycle_init_error(sch, ret);
                return ret;
            }
            (*(*sch).exit_info).flags |= LUPOS_SCX_SUB_LIFECYCLE_EFLAG_INITIALIZED as u64;
        }
        ret = scx_set_cmask_scratch_alloc(sch);
        if ret != 0 { return ret; }
        let mut args = scx_sub_attach_args {
            ops: lupos_scx_sub_ops(sch),
            cgroup_path: (*sch).cgrp_path,
        };
        ret = lupos_scx_sub_lifecycle_call_sub_attach(parent, &mut args);
        if ret != 0 {
            ret = lupos_scx_sub_lifecycle_sanitize_attach(sch, ret);
            lupos_scx_sub_lifecycle_attach_error(sch, ret);
            return ret;
        }
        (*sch).sub_attached = true;
        scx_bypass(sch, true);
        for i in LUPOS_SCX_SUB_LIFECYCLE_OPI_BEGIN as i32..LUPOS_SCX_SUB_LIFECYCLE_OPI_END as i32 {
            if lupos_scx_sub_lifecycle_op_present(ops, i) {
                lupos_scx_sub_lifecycle_set_has_op(sch, i);
            }
        }
        lupos_scx_sub_lifecycle_fork_down();
        scx_cgroup_lock();
        set_cgroup_sched(sch_cgroup(sch), sch);
        if !lupos_scx_sub_lifecycle_cgroup_online(cgrp) {
            lupos_scx_sub_lifecycle_cgroup_offline_error(sch);
            ret = -(LUPOS_SCX_SUB_LIFECYCLE_ENODEV as i32);
        } else {
            ret = scx_cgroup_claim_subtree(sch);
            if ret == 0 {
                ret = scx_sub_enable_tasks(sch, parent);
            }
        }
        scx_cgroup_unlock();
        lupos_scx_sub_lifecycle_fork_up();
        if ret != 0 {
            return ret;
        }
        scx_bypass(sch, false);
        scx_sub_seed_caps(sch);
        lupos_scx_sub_lifecycle_log_enable(sch);
        lupos_scx_sub_lifecycle_uevent_add(sch);
        0
    }
}

/// Enable a requested sub-scheduler on the dedicated system-wide RT worker.
///
/// # Safety
/// work is the live embedded work member of a scx_enable_cmd, submitted exactly
/// once to the dedicated system-wide RT worker. The submitting path pins the
/// command and its native ops union until completion; cmd->ret is writable and
/// any arena_map reference is available for transfer. Caller is IRQ-enabled
/// and sleepable, and holds none of the enable/fork/cgroup or sched/task locks.
#[no_mangle]
pub unsafe extern "C" fn scx_sub_enable_workfn(work: *mut kthread_work) {
    // SAFETY: Native container_of/union access retrieves the authoritative cmd.
    // Parent's borrowed pointer is ref-pinned under sched_lock before unlock.
    // alloc_and_add consumes cgrp on success AND failure; no double put occurs.
    unsafe {
        let cmd = lupos_scx_sub_lifecycle_enable_cmd(work);
        let ops = lupos_scx_sub_lifecycle_cmd_ops(cmd);
        lupos_scx_sub_lifecycle_enable_lock();
        let ret = 'out_unlock: {
            if !lupos_scx_sub_lifecycle_enabled() {
                break 'out_unlock -(LUPOS_SCX_SUB_LIFECYCLE_ENODEV as i32);
            }
            if lupos_scx_sub_lifecycle_ops_published(ops) {
                break 'out_unlock -(LUPOS_SCX_SUB_LIFECYCLE_EBUSY as i32);
            }
            let cgrp = cgroup_get_from_id((*ops).sub_cgroup_id);
            if lupos_scx_sub_lifecycle_is_err(cgrp.cast()) {
                break 'out_unlock lupos_scx_sub_lifecycle_ptr_err(cgrp.cast());
            }
            lupos_scx_sub_lifecycle_sched_lock();
            let parent = find_parent_sched(cgrp);
            if lupos_scx_sub_lifecycle_is_err(parent.cast()) {
                lupos_scx_sub_lifecycle_sched_unlock();
                let ret = lupos_scx_sub_lifecycle_ptr_err(parent.cast());
                lupos_scx_sub_lifecycle_cgroup_put(cgrp);
                break 'out_unlock ret;
            }
            kobject_get(ptr::addr_of_mut!((*parent).kobj));
            lupos_scx_sub_lifecycle_sched_unlock();
            // Flip hot-path gates before ops->priv becomes visible.
            lupos_scx_sub_lifecycle_has_subs_inc();
            let sch = scx_alloc_and_add_sched(cmd, cgrp, parent);
            kobject_put(ptr::addr_of_mut!((*parent).kobj));
            if lupos_scx_sub_lifecycle_is_err(sch.cast()) {
                lupos_scx_sub_lifecycle_has_subs_dec();
                break 'out_unlock lupos_scx_sub_lifecycle_ptr_err(sch.cast());
            }
            let ret = scx_sub_enable_prepared(sch, parent, cgrp, ops);
            if ret != 0 {
                // All allocated-scheduler failures must record an error and
                // flush disable outside enable mutex; command returns zero
                // because scheduler exit_info carries the failure.
                lupos_scx_sub_lifecycle_enable_unlock();
                lupos_scx_sub_lifecycle_enable_error(sch, ret);
                scx_flush_disable_work(sch);
                (*cmd).ret = 0;
                return;
            }
            0
        };
        lupos_scx_sub_lifecycle_enable_unlock();
        (*cmd).ret = ret;
    }
}

/// Prepare a cross-scheduler cgroup migration before it commits.
///
/// # Safety
/// ctx and its referenced task/destination are pinned by the cgroup migration
/// notifier caller, which holds cgroup_mutex. The enabled flag is stabilized by
/// that mutex, so root task teardown cannot overlap an initialized destination.
unsafe fn scx_cgroup_task_migrating(ctx: *mut cgroup_task_migrate_ctx) -> i32 {
    // SAFETY: cgroup_mutex stabilizes scheduler ownership across preparation;
    // the notifier's task reference survives the potentially sleeping init.
    unsafe {
        let p = (*ctx).task;
        if !lupos_scx_sub_lifecycle_cgroup_enabled() {
            return LUPOS_SCX_SUB_LIFECYCLE_NOTIFY_OK as i32;
        }
        let to = lupos_scx_sub_lifecycle_cgroup_sched((*ctx).dst_dcgrp);
        if lupos_scx_sub_lifecycle_task_on_sched(to, p) {
            return LUPOS_SCX_SUB_LIFECYCLE_NOTIFY_OK as i32;
        }
        let ret = __scx_init_task(to, p, (*ctx).dst_dcgrp, false);
        if ret != 0 {
            return lupos_scx_sub_lifecycle_notifier_errno(ret);
        }
        LUPOS_SCX_SUB_LIFECYCLE_NOTIFY_OK as i32
    }
}

/// Commit the already initialized task's change of owning scheduler.
///
/// # Safety
/// ctx is the committed migration corresponding to successful preparation;
/// cgroup_mutex and notifier references retain task and destination ownership.
unsafe fn scx_cgroup_task_migrated(ctx: *mut cgroup_task_migrate_ctx) {
    // SAFETY: Disable resets ownership and re-homes tasks under cgroup_mutex;
    // the destination here is either the parent or is still obliged to re-home
    // this now-member task. Native task_rq_lock protects the actual transition.
    unsafe {
        let p = (*ctx).task;
        if !lupos_scx_sub_lifecycle_cgroup_enabled() {
            return;
        }
        let to = lupos_scx_sub_lifecycle_cgroup_sched((*ctx).dst_dcgrp);
        if lupos_scx_sub_lifecycle_task_on_sched(to, p) {
            return;
        }
        let mut rf = core::mem::MaybeUninit::<rq_flags>::uninit();
        let rq = lupos_scx_sub_lifecycle_task_lock(p, rf.as_mut_ptr());
        scx_rehome_task(to, p);
        lupos_scx_sub_lifecycle_task_unlock(rq, p, rf.as_mut_ptr());
    }
}

/// Cancel destination initialization after the cgroup migration is abandoned.
///
/// # Safety
/// ctx is a prepared but uncommitted migration with live notifier references;
/// cgroup_mutex stabilizes its destination and the enabled flag throughout.
unsafe fn scx_cgroup_task_migrate_canceled(ctx: *mut cgroup_task_migrate_ctx) {
    // SAFETY: The source still owns p. Its pi/rq locks serialize the owed
    // destination exit without disabling or moving the source task.
    unsafe {
        let p = (*ctx).task;
        if !lupos_scx_sub_lifecycle_cgroup_enabled() {
            return;
        }
        let to = lupos_scx_sub_lifecycle_cgroup_sched((*ctx).dst_dcgrp);
        if lupos_scx_sub_lifecycle_task_on_sched(to, p) {
            return;
        }
        let mut rf = core::mem::MaybeUninit::<rq_flags>::uninit();
        let rq = lupos_scx_sub_lifecycle_task_lock(p, rf.as_mut_ptr());
        scx_sub_init_cancel_task(to, p);
        lupos_scx_sub_lifecycle_task_unlock(rq, p, rf.as_mut_ptr());
    }
}

/// Inherit scheduler ownership on online, or terminate an offlining owner.
///
/// # Safety
/// Called by the native lifetime notifier with data pointing to its live
/// cgroup, under cgroup_mutex. nb belongs to the permanently allocated notifier
/// metadata. The caller supplies a native CGROUP_LIFETIME_* action value.
#[no_mangle]
pub unsafe extern "C" fn scx_cgroup_lifetime_notify(_nb: *mut notifier_block,
    action: c_ulong, data: *mut c_void) -> i32 {
    // SAFETY: Cgroup lifetime notifier serialization pins cgrp and parent. RCU
    // assignment and exit metadata are preserved by typed native leaves.
    unsafe {
        let cgrp = data.cast::<cgroup>();
        let parent = lupos_scx_sub_lifecycle_cgroup_parent(cgrp);
        if !lupos_scx_sub_lifecycle_cgroup_on_dfl(cgrp) {
            return LUPOS_SCX_SUB_LIFECYCLE_NOTIFY_OK as i32;
        }
        if action == LUPOS_SCX_SUB_LIFECYCLE_CGROUP_ONLINE as c_ulong {
            if !parent.is_null() {
                lupos_scx_sub_lifecycle_publish_cgroup(cgrp,
                    lupos_scx_sub_lifecycle_cgroup_sched(parent));
            }
        } else if action == LUPOS_SCX_SUB_LIFECYCLE_CGROUP_OFFLINE as c_ulong {
            let sch = lupos_scx_sub_lifecycle_cgroup_sched(cgrp);
            if !sch.is_null() && (*sch).cgrp == cgrp {
                lupos_scx_sub_lifecycle_exit_offline(sch, cgrp);
            }
        }
        LUPOS_SCX_SUB_LIFECYCLE_NOTIFY_OK as i32
    }
}

/// Dispatch migration notifications while retaining preparation's error result.
///
/// # Safety
/// Called by the native migration notifier with data pointing to the matching
/// pinned cgroup_task_migrate_ctx and cgroup_mutex held. nb is native metadata.
#[no_mangle]
pub unsafe extern "C" fn scx_cgroup_task_notify(_nb: *mut notifier_block,
    action: c_ulong, data: *mut c_void) -> i32 {
    // SAFETY: Each action preserves the notifier's matching ctx/task lifetime;
    // unknown actions do not dereference data or change state.
    unsafe {
        let ctx = data.cast::<cgroup_task_migrate_ctx>();
        if action == LUPOS_SCX_SUB_LIFECYCLE_TASK_MIGRATING as c_ulong {
            return scx_cgroup_task_migrating(ctx);
        }
        if action == LUPOS_SCX_SUB_LIFECYCLE_TASK_MIGRATED as c_ulong {
            scx_cgroup_task_migrated(ctx);
        } else if action == LUPOS_SCX_SUB_LIFECYCLE_TASK_MIGRATE_CANCELED as c_ulong {
            scx_cgroup_task_migrate_canceled(ctx);
        }
        LUPOS_SCX_SUB_LIFECYCLE_NOTIFY_OK as i32
    }
}

/// Register the lifetime notifier before the migration notifier at core init.
///
/// # Safety
/// Called once by the native core_initcall wrapper during boot, with both
/// native notifier heads initialized and these static blocks not yet linked.
/// No unloading/re-registration is supported by this original owner contract.
#[no_mangle]
#[cold]
#[link_section = ".init.text"]
pub unsafe extern "C" fn scx_cgroup_notifier_init() -> i32 {
    // SAFETY: Native leaves retain initialized static notifier blocks and typed
    // callback pointers. A second-registration error intentionally does not
    // unregister the first notifier, matching the original initcall policy.
    unsafe {
        let ret = lupos_scx_sub_lifecycle_register_lifetime();
        if ret != 0 {
            return ret;
        }
        lupos_scx_sub_lifecycle_register_task()
    }
}
