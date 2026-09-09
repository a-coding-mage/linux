// SPDX-License-Identifier: GPL-2.0-or-later
/* Function to determine if a thread group is single threaded or not
 *
 * Copyright (C) 2008 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 * - Derived from security/selinux/hooks.c
 */
// Dependencies supplied by linux/sched/signal.h, linux/sched/task.h,
// and linux/sched/mm.h are expected to be available to this translation.

/*
 * Returns true if the task does not share ->mm with another thread/process.
 */
pub unsafe fn current_is_single_threaded() -> bool {
    let task: *mut task_struct = current;
    let mm: *mut mm_struct = (*task).mm;
    let mut p: *mut task_struct;
    let mut t: *mut task_struct;
    let mut ret: bool;

    if atomic_read(&(*(*task).signal).live) != 1 {
        return false;
    }

    if atomic_read(&(*mm).mm_users) == 1 {
        return true;
    }

    ret = false;
    rcu_read_lock();
    'search: {
        for_each_process!(p) {
            if unlikely!((*p).flags & PF_KTHREAD != 0) {
                continue;
            }
            if unlikely!(p == (*task).group_leader) {
                continue;
            }

            for_each_thread!(p, t) {
                if unlikely!((*t).mm == mm) {
                    break 'search;
                }
                if likely!((*t).mm != core::ptr::null_mut()) {
                    break;
                }
                /*
                 * t->mm == NULL. Make sure next_thread/next_task
                 * will see other CLONE_VM tasks which might be
                 * forked before exiting.
                 */
                smp_rmb();
            }
        }
        ret = true;
    }
    rcu_read_unlock();

    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
