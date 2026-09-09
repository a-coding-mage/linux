// SPDX-License-Identifier: GPL-2.0
/*
 * fs/ioprio.c
 *
 * Copyright (C) 2004 Jens Axboe <axboe@kernel.dk>
 *
 * Helper functions for setting/querying io priorities of processes. The
 * system calls closely mimmick getpriority/setpriority, see the man page for
 * those. The prio argument is a composite of prio class and prio data, where
 * the data argument has meaning within that class. The standard scheduling
 * classes have 8 distinct prio levels, with 0 being the highest prio and 7
 * being the lowest.
 */

// Kernel headers and build-time macros are supplied by the surrounding kernel
// translation unit.

pub unsafe fn ioprio_check_cap(ioprio: i32) -> i32 {
    let class = IOPRIO_PRIO_CLASS(ioprio);
    let level = IOPRIO_PRIO_LEVEL(ioprio);

    match class {
        IOPRIO_CLASS_RT => {
            if !capable(CAP_SYS_ADMIN) && !capable(CAP_SYS_NICE) {
                return -EPERM;
            }
        }
        IOPRIO_CLASS_BE | IOPRIO_CLASS_IDLE => {}
        IOPRIO_CLASS_NONE => {
            if level != 0 {
                return -EINVAL;
            }
        }
        IOPRIO_CLASS_INVALID => return -EINVAL,
        _ => return -EINVAL,
    }

    0
}

pub unsafe fn ioprio_set(which: i32, who: i32, ioprio: i32) -> i32 {
    let mut p: *mut task_struct;
    let mut g: *mut task_struct;
    let mut user: *mut user_struct;
    let mut pgrp: *mut pid;
    let mut uid: kuid_t;
    let mut ret: i32;

    ret = ioprio_check_cap(ioprio);
    if ret != 0 {
        return ret;
    }

    ret = -ESRCH;
    rcu_read_lock();
    match which {
        IOPRIO_WHO_PROCESS => {
            p = if who == 0 { current } else { find_task_by_vpid(who) };
            if !p.is_null() {
                ret = set_task_ioprio(p, ioprio);
            }
        }
        IOPRIO_WHO_PGRP => {
            pgrp = if who == 0 { task_pgrp(current) } else { find_vpid(who) };
            read_lock(&tasklist_lock);
            // do_each_pid_thread / while_each_pid_thread expand to the kernel
            // pid-thread iteration sequence.
            unsafe {
                for_each_pid_thread(pgrp, PIDTYPE_PGID, p, {
                    ret = set_task_ioprio(p, ioprio);
                    if ret != 0 {
                        read_unlock(&tasklist_lock);
                        rcu_read_unlock();
                        return ret;
                    }
                });
            }
            read_unlock(&tasklist_lock);
        }
        IOPRIO_WHO_USER => {
            uid = make_kuid(current_user_ns(), who);
            if !uid_valid(uid) {
                rcu_read_unlock();
                return ret;
            }
            user = if who == 0 { current_user() } else { find_user(uid) };
            if user.is_null() {
                rcu_read_unlock();
                return ret;
            }
            for_each_process_thread!(g, p, {
                if !uid_eq(task_uid(p), uid) || task_pid_vnr(p) == 0 {
                    continue;
                }
                ret = set_task_ioprio(p, ioprio);
                if ret != 0 {
                    break;
                }
            });
            if who != 0 {
                free_uid(user);
            }
        }
        _ => ret = -EINVAL,
    }

    rcu_read_unlock();
    ret
}

unsafe fn get_task_ioprio(p: *mut task_struct) -> i32 {
    let mut ret = security_task_getioprio(p);
    if ret != 0 {
        return ret;
    }
    task_lock(p);
    ret = __get_task_ioprio(p);
    task_unlock(p);
    ret
}

/* Return raw IO priority value as set by userspace. */
unsafe fn get_task_raw_ioprio(p: *mut task_struct) -> i32 {
    let mut ret = security_task_getioprio(p);
    if ret != 0 {
        return ret;
    }
    task_lock(p);
    ret = if !(*p).io_context.is_null() {
        (*(*p).io_context).ioprio
    } else {
        IOPRIO_DEFAULT
    };
    task_unlock(p);
    ret
}

unsafe fn ioprio_best(aprio: u16, bprio: u16) -> i32 {
    core::cmp::min(aprio, bprio) as i32
}

pub unsafe fn ioprio_get(which: i32, who: i32) -> i32 {
    let mut g: *mut task_struct;
    let mut p: *mut task_struct;
    let mut user: *mut user_struct;
    let mut pgrp: *mut pid;
    let mut uid: kuid_t;
    let mut ret: i32 = -ESRCH;
    let mut tmpio: i32;

    rcu_read_lock();
    match which {
        IOPRIO_WHO_PROCESS => {
            p = if who == 0 { current } else { find_task_by_vpid(who) };
            if !p.is_null() {
                ret = get_task_raw_ioprio(p);
            }
        }
        IOPRIO_WHO_PGRP => {
            pgrp = if who == 0 { task_pgrp(current) } else { find_vpid(who) };
            read_lock(&tasklist_lock);
            for_each_pid_thread!(pgrp, PIDTYPE_PGID, p, {
                tmpio = get_task_ioprio(p);
                if tmpio >= 0 {
                    ret = if ret == -ESRCH { tmpio } else { ioprio_best(ret as u16, tmpio as u16) };
                }
            });
            read_unlock(&tasklist_lock);
        }
        IOPRIO_WHO_USER => {
            uid = make_kuid(current_user_ns(), who);
            user = if who == 0 { current_user() } else { find_user(uid) };
            if !user.is_null() {
                for_each_process_thread!(g, p, {
                    if !uid_eq(task_uid(p), (*user).uid) || task_pid_vnr(p) == 0 {
                        continue;
                    }
                    tmpio = get_task_ioprio(p);
                    if tmpio >= 0 {
                        ret = if ret == -ESRCH { tmpio } else { ioprio_best(ret as u16, tmpio as u16) };
                    }
                });
                if who != 0 {
                    free_uid(user);
                }
            }
        }
        _ => ret = -EINVAL,
    }
    rcu_read_unlock();
    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
