// SPDX-License-Identifier: GPL-2.0
// Dependencies supplied by the Linux cgroup, scheduler, signal, and trace headers.

/*
 * Update CGRP_FROZEN of cgroup.flag
 * Return true if flags is updated; false if flags has no change
 */
unsafe fn cgroup_update_frozen_flag(cgrp: *mut cgroup, frozen: bool) -> bool {
    lockdep_assert_held(&css_set_lock);

    if test_bit(CGRP_FROZEN, &(*cgrp).flags) == frozen {
        return false;
    }

    if frozen {
        set_bit(CGRP_FROZEN, &mut (*cgrp).flags);
    } else {
        clear_bit(CGRP_FROZEN, &mut (*cgrp).flags);
    }

    cgroup_file_notify(&mut (*cgrp).events_file);
    TRACE_CGROUP_PATH!(notify_frozen, cgrp, frozen);
    true
}

/* Propagate the cgroup frozen state upwards by the cgroup tree. */
unsafe fn cgroup_propagate_frozen(mut cgrp: *mut cgroup, frozen: bool) {
    let mut desc: i32 = 1;

    while {
        cgrp = cgroup_parent(cgrp);
        !cgrp.is_null()
    } {
        if frozen {
            (*cgrp).freezer.nr_frozen_descendants += desc;
            if !test_bit(CGRP_FREEZE, &(*cgrp).flags)
                || (*cgrp).freezer.nr_frozen_descendants != (*cgrp).nr_descendants
            {
                continue;
            }
        } else {
            (*cgrp).freezer.nr_frozen_descendants -= desc;
        }

        if cgroup_update_frozen_flag(cgrp, frozen) {
            desc += 1;
        }
    }
}

/* Revisit the cgroup frozen state. */
#[no_mangle]
pub unsafe extern "C" fn cgroup_update_frozen(cgrp: *mut cgroup) {
    let frozen = test_bit(CGRP_FREEZE, &(*cgrp).flags)
        && (*cgrp).freezer.nr_frozen_tasks == __cgroup_task_count(cgrp);

    if cgroup_update_frozen_flag(cgrp, frozen) {
        cgroup_propagate_frozen(cgrp, frozen);
    }
}

/* Increment cgroup's nr_frozen_tasks. */
unsafe fn cgroup_inc_frozen_cnt(cgrp: *mut cgroup) {
    (*cgrp).freezer.nr_frozen_tasks += 1;
}

/* Decrement cgroup's nr_frozen_tasks. */
unsafe fn cgroup_dec_frozen_cnt(cgrp: *mut cgroup) {
    (*cgrp).freezer.nr_frozen_tasks -= 1;
    WARN_ON_ONCE((*cgrp).freezer.nr_frozen_tasks < 0);
}

#[no_mangle]
pub unsafe extern "C" fn cgroup_enter_frozen() {
    if (*current).frozen {
        return;
    }

    spin_lock_irq(&mut css_set_lock);
    (*current).frozen = true;
    let cgrp = task_dfl_cgroup(current);
    cgroup_inc_frozen_cnt(cgrp);
    cgroup_update_frozen(cgrp);
    spin_unlock_irq(&mut css_set_lock);
}

#[no_mangle]
pub unsafe extern "C" fn cgroup_leave_frozen(always_leave: bool) {
    spin_lock_irq(&mut css_set_lock);
    let cgrp = task_dfl_cgroup(current);
    if always_leave || !test_bit(CGRP_FREEZE, &(*cgrp).flags) {
        cgroup_dec_frozen_cnt(cgrp);
        cgroup_update_frozen(cgrp);
        WARN_ON_ONCE(!(*current).frozen);
        (*current).frozen = false;
    } else if ((*current).jobctl & JOBCTL_TRAP_FREEZE) == 0 {
        spin_lock(&mut (*(*current).sighand).siglock);
        (*current).jobctl |= JOBCTL_TRAP_FREEZE;
        set_thread_flag(TIF_SIGPENDING);
        spin_unlock(&mut (*(*current).sighand).siglock);
    }
    spin_unlock_irq(&mut css_set_lock);
}

/* Freeze or unfreeze the task by setting or clearing JOBCTL_TRAP_FREEZE. */
unsafe fn cgroup_freeze_task(task: *mut task_struct, freeze: bool) {
    let mut flags: c_ulong = 0;
    if !lock_task_sighand(task, &mut flags) {
        return;
    }
    if freeze {
        (*task).jobctl |= JOBCTL_TRAP_FREEZE;
        signal_wake_up(task, false);
    } else {
        (*task).jobctl &= !JOBCTL_TRAP_FREEZE;
        wake_up_process(task);
    }
    unlock_task_sighand(task, &mut flags);
}

/* Freeze or unfreeze all tasks in the given cgroup. */
unsafe fn cgroup_do_freeze(cgrp: *mut cgroup, freeze: bool, ts_nsec: u64) {
    let mut it: css_task_iter = core::mem::zeroed();
    let mut task: *mut task_struct;

    lockdep_assert_held(&cgroup_mutex);
    spin_lock_irq(&mut css_set_lock);
    write_seqcount_begin(&mut (*cgrp).freezer.freeze_seq);
    if freeze {
        set_bit(CGRP_FREEZE, &mut (*cgrp).flags);
        (*cgrp).freezer.freeze_start_nsec = ts_nsec;
    } else {
        clear_bit(CGRP_FREEZE, &mut (*cgrp).flags);
        (*cgrp).freezer.frozen_nsec += ts_nsec - (*cgrp).freezer.freeze_start_nsec;
    }
    write_seqcount_end(&mut (*cgrp).freezer.freeze_seq);
    spin_unlock_irq(&mut css_set_lock);

    if freeze { TRACE_CGROUP_PATH!(freeze, cgrp); } else { TRACE_CGROUP_PATH!(unfreeze, cgrp); }
    css_task_iter_start(&mut (*cgrp).self_, 0, &mut it);
    while {
        task = css_task_iter_next(&mut it);
        !task.is_null()
    } {
        if ((*task).flags & PF_KTHREAD) != 0 { continue; }
        cgroup_freeze_task(task, freeze);
    }
    css_task_iter_end(&mut it);

    spin_lock_irq(&mut css_set_lock);
    if (*cgrp).nr_descendants == (*cgrp).freezer.nr_frozen_descendants {
        cgroup_update_frozen(cgrp);
    }
    spin_unlock_irq(&mut css_set_lock);
}

#[no_mangle]
pub unsafe extern "C" fn cgroup_freezer_migrate_task(task: *mut task_struct, src: *mut cgroup, dst: *mut cgroup) {
    lockdep_assert_held(&css_set_lock);
    if ((*task).flags & PF_KTHREAD) != 0 { return; }
    if !test_bit(CGRP_FREEZE, &(*src).flags) && !test_bit(CGRP_FREEZE, &(*dst).flags) && !(*task).frozen { return; }
    if (*task).frozen { cgroup_inc_frozen_cnt(dst); cgroup_dec_frozen_cnt(src); }
    cgroup_update_frozen(dst);
    cgroup_update_frozen(src);
    cgroup_freeze_task(task, test_bit(CGRP_FREEZE, &(*dst).flags));
}

#[no_mangle]
pub unsafe extern "C" fn cgroup_freeze(cgrp: *mut cgroup, freeze: bool) {
    let mut css: *mut cgroup_subsys_state;
    let mut dsct: *mut cgroup;
    let mut applied = false;
    let ts_nsec: u64;
    let old_e: bool;

    lockdep_assert_held(&cgroup_mutex);
    if (*cgrp).freezer.freeze == freeze { return; }
    (*cgrp).freezer.freeze = freeze;
    ts_nsec = ktime_get_ns();

    css_for_each_descendant_pre!(css, &mut (*cgrp).self_);
    while !css.is_null() {
        dsct = (*css).cgroup;
        if cgroup_is_dead(dsct) { css = css_next!(); continue; }
        old_e = (*dsct).freezer.e_freeze;
        let parent = cgroup_parent(dsct);
        (*dsct).freezer.e_freeze = (*dsct).freezer.freeze || (*parent).freezer.e_freeze;
        if (*dsct).freezer.e_freeze == old_e { css = css_rightmost_descendant(css); continue; }
        cgroup_do_freeze(dsct, freeze, ts_nsec);
        applied = true;
        css = css_next!();
    }

    if !applied {
        TRACE_CGROUP_PATH!(notify_frozen, cgrp, test_bit(CGRP_FROZEN, &(*cgrp).flags));
        cgroup_file_notify(&mut (*cgrp).events_file);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
