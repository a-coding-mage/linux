// SPDX-License-Identifier: GPL-2.0-or-later
/* Function to determine if a thread group is single threaded or not
 *
 * Copyright (C) 2008 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 * - Derived from security/selinux/hooks.c
 */

// Dependencies: linux/sched/signal.h, linux/sched/task.h, linux/sched/mm.h.

use core::mem::offset_of;
use core::ptr::{addr_of, addr_of_mut, read_volatile};
use kernel::bindings::{
    get_current, init_task, list_head, mm_struct, rcu_read_lock, rcu_read_unlock, smp_rmb,
    task_struct, PF_KTHREAD,
};

/// list_entry_rcu(head->next, struct task_struct, member) for the RCU task
/// and thread lists, with the rcu_dereference() load of the next pointer.
///
/// # Safety
/// `entry` is a list member reachable under rcu_read_lock().
unsafe fn next_rcu_entry(entry: *const list_head, member_offset: usize) -> *mut task_struct {
    // SAFETY: the RCU list's next pointer is read once, as READ_ONCE() does.
    let next = unsafe { read_volatile(addr_of!((*entry).next)) };
    next.wrapping_byte_sub(member_offset).cast::<task_struct>()
}

/*
 * Returns true if the task does not share ->mm with another thread/process.
 */
/// Returns true when no other user task shares the current task's mm.
#[no_mangle]
pub extern "C" fn current_is_single_threaded() -> bool {
    // SAFETY: current, its signal_struct and mm are live for the caller;
    // other tasks and threads are only walked under rcu_read_lock().
    unsafe {
        let task = get_current();
        let mm: *mut mm_struct = (*task).mm;

        if read_volatile(addr_of!((*(*task).signal).live.counter)) != 1 {
            return false;
        }

        if read_volatile(addr_of!((*mm).__bindgen_anon_1.mm_users.counter)) == 1 {
            return true;
        }

        let mut ret = false;
        rcu_read_lock();
        let init = addr_of_mut!(init_task);
        let tasks = offset_of!(task_struct, tasks);
        let thread_node = offset_of!(task_struct, thread_node);
        // for_each_process(p)
        let mut p = next_rcu_entry(addr_of!((*init).tasks), tasks);
        'found: {
            while p != init {
                'next_process: {
                    if read_volatile(addr_of!((*p).flags)) & PF_KTHREAD != 0 {
                        break 'next_process;
                    }
                    if p == (*task).group_leader {
                        break 'next_process;
                    }

                    // for_each_thread(p, t)
                    let head = addr_of!((*(*p).signal).thread_head);
                    let mut t = next_rcu_entry(head, thread_node);
                    while addr_of!((*t).thread_node) != head {
                        let t_mm = read_volatile(addr_of!((*t).mm));
                        if t_mm == mm {
                            break 'found;
                        }
                        if !t_mm.is_null() {
                            break;
                        }
                        /*
                         * t->mm == NULL. Make sure next_thread/next_task
                         * will see other CLONE_VM tasks which might be
                         * forked before exiting.
                         */
                        smp_rmb();
                        t = next_rcu_entry(addr_of!((*t).thread_node), thread_node);
                    }
                }
                p = next_rcu_entry(addr_of!((*p).tasks), tasks);
            }
            ret = true;
        }
        // found:
        rcu_read_unlock();

        ret
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
