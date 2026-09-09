// SPDX-License-Identifier: GPL-2.0-only

/*
 * A simple wrapper around refcount. An allocated sched_core_cookie's
 * address is used to compute the cookie of the task.
 */
// Dependency declarations from sched.h are supplied by the surrounding build.

#[repr(C)]
struct sched_core_cookie {
    refcnt: refcount_t,
}

unsafe fn sched_core_alloc_cookie() -> ::core::ffi::c_ulong {
    let ck: *mut sched_core_cookie = kmalloc_obj::<sched_core_cookie>();
    if ck.is_null() {
        return 0;
    }

    refcount_set(&mut (*ck).refcnt, 1);
    sched_core_get();

    ck as ::core::ffi::c_ulong
}

unsafe fn sched_core_put_cookie(cookie: ::core::ffi::c_ulong) {
    let ptr = cookie as *mut sched_core_cookie;

    if !ptr.is_null() && refcount_dec_and_test(&mut (*ptr).refcnt) {
        kfree(ptr);
        sched_core_put();
    }
}

unsafe fn sched_core_get_cookie(cookie: ::core::ffi::c_ulong) -> ::core::ffi::c_ulong {
    let ptr = cookie as *mut sched_core_cookie;

    if !ptr.is_null() {
        refcount_inc(&mut (*ptr).refcnt);
    }

    cookie
}

/*
 * sched_core_update_cookie - replace the cookie on a task
 * @p: the task to update
 * @cookie: the new cookie
 *
 * Effectively exchange the task cookie; caller is responsible for lifetimes on
 * both ends.
 *
 * Returns: the old cookie
 */
unsafe fn sched_core_update_cookie(
    p: *mut task_struct,
    cookie: ::core::ffi::c_ulong,
) -> ::core::ffi::c_ulong {
    let old_cookie: ::core::ffi::c_ulong;
    let mut rf: rq_flags;
    let rq: *mut rq;

    rq = task_rq_lock(p, &mut rf);

    /*
     * Since creating a cookie implies sched_core_get(), and we cannot set
     * a cookie until after we've created it, similarly, we cannot destroy
     * a cookie until after we've removed it, we must have core scheduling
     * enabled here.
     */
    WARN_ON_ONCE(((*p).core_cookie != 0 || cookie != 0) && !sched_core_enabled(rq));

    if sched_core_enqueued(p) {
        sched_core_dequeue(rq, p, DEQUEUE_SAVE);
    }

    old_cookie = (*p).core_cookie;
    (*p).core_cookie = cookie;

    /*
     * Consider the cases: !prev_cookie and !cookie.
     */
    if cookie != 0 && task_on_rq_queued(p) {
        sched_core_enqueue(rq, p);
    }

    /*
     * If task is currently running, it may not be compatible anymore after
     * the cookie change, so enter the scheduler on its CPU to schedule it
     * away.
     *
     * Note that it is possible that as a result of this cookie change, the
     * core has now entered/left forced idle state. Defer accounting to the
     * next scheduling edge, rather than always forcing a reschedule here.
     */
    if task_on_cpu(rq, p) {
        resched_curr(rq);
    }

    task_rq_unlock(rq, p, &mut rf);

    old_cookie
}

unsafe fn sched_core_clone_cookie(p: *mut task_struct) -> ::core::ffi::c_ulong {
    let cookie: ::core::ffi::c_ulong;
    let mut flags: ::core::ffi::c_ulong = 0;

    raw_spin_lock_irqsave(&mut (*p).pi_lock, &mut flags);
    cookie = sched_core_get_cookie((*p).core_cookie);
    raw_spin_unlock_irqrestore(&mut (*p).pi_lock, flags);

    cookie
}

pub unsafe fn sched_core_fork(p: *mut task_struct) {
    RB_CLEAR_NODE(&mut (*p).core_node);
    (*p).core_cookie = sched_core_clone_cookie(current);
}

pub unsafe fn sched_core_free(p: *mut task_struct) {
    sched_core_put_cookie((*p).core_cookie);
}

unsafe fn __sched_core_set(p: *mut task_struct, mut cookie: ::core::ffi::c_ulong) {
    cookie = sched_core_get_cookie(cookie);
    cookie = sched_core_update_cookie(p, cookie);
    sched_core_put_cookie(cookie);
}

/* Called from prctl interface: PR_SCHED_CORE */
pub unsafe fn sched_core_share_pid(
    cmd: ::core::ffi::c_uint,
    pid: pid_t,
    ty: pid_type,
    uaddr: ::core::ffi::c_ulong,
) -> ::core::ffi::c_int {
    let mut cookie: ::core::ffi::c_ulong = 0;
    let mut id: ::core::ffi::c_ulong = 0;
    let mut task: *mut task_struct;
    let mut p: *mut task_struct;
    let mut grp: *mut pid;
    let mut err: ::core::ffi::c_int = 0;

    if !sched_smt_active() {
        return -ENODEV;
    }

    BUILD_BUG_ON(PR_SCHED_CORE_SCOPE_THREAD != PIDTYPE_PID);
    BUILD_BUG_ON(PR_SCHED_CORE_SCOPE_THREAD_GROUP != PIDTYPE_TGID);
    BUILD_BUG_ON(PR_SCHED_CORE_SCOPE_PROCESS_GROUP != PIDTYPE_PGID);

    if ty > PIDTYPE_PGID || cmd >= PR_SCHED_CORE_MAX || pid < 0
        || (cmd != PR_SCHED_CORE_GET && uaddr != 0)
    {
        return -EINVAL;
    }

    rcu_read_lock();
    if pid == 0 {
        task = current;
    } else {
        task = find_task_by_vpid(pid);
        if task.is_null() {
            rcu_read_unlock();
            return -ESRCH;
        }
    }
    get_task_struct(task);
    rcu_read_unlock();

    /* Check whether this process may modify the specified process. */
    if !ptrace_may_access(task, PTRACE_MODE_READ_REALCREDS) {
        err = -EPERM;
        goto out;
    }

    match cmd {
        PR_SCHED_CORE_GET => {
            if ty != PIDTYPE_PID || uaddr & 7 != 0 {
                err = -EINVAL;
                goto out;
            }
            cookie = sched_core_clone_cookie(task);
            if cookie != 0 {
                ptr_to_hashval(cookie as *mut ::core::ffi::c_void, &mut id);
            }
            err = put_user(id, uaddr as *mut u64);
            goto out;
        }
        PR_SCHED_CORE_CREATE => {
            cookie = sched_core_alloc_cookie();
            if cookie == 0 {
                err = -ENOMEM;
                goto out;
            }
        }
        PR_SCHED_CORE_SHARE_TO => cookie = sched_core_clone_cookie(current),
        PR_SCHED_CORE_SHARE_FROM => {
            if ty != PIDTYPE_PID {
                err = -EINVAL;
                goto out;
            }
            cookie = sched_core_clone_cookie(task);
            __sched_core_set(current, cookie);
            goto out;
        }
        _ => {
            err = -EINVAL;
            goto out;
        }
    }

    if ty == PIDTYPE_PID {
        __sched_core_set(task, cookie);
        goto out;
    }

    read_lock(&tasklist_lock);
    grp = task_pid_type(task, ty);
    /* do_each_pid_thread(grp, ty, p) { ... } while_each_pid_thread(grp, ty, p); */
    do_each_pid_thread!(grp, ty, p);
    if !ptrace_may_access(p, PTRACE_MODE_READ_REALCREDS) {
        err = -EPERM;
        goto out_tasklist;
    }
    while_each_pid_thread!(grp, ty, p);

    do_each_pid_thread!(grp, ty, p);
    __sched_core_set(p, cookie);
    while_each_pid_thread!(grp, ty, p);
out_tasklist:
    read_unlock(&tasklist_lock);

out:
    sched_core_put_cookie(cookie);
    put_task_struct(task);
    err
}

/* CONFIG_SCHEDSTATS conditional section preserved from the source. */
#[cfg(CONFIG_SCHEDSTATS)]
unsafe fn __sched_core_account_forceidle(rq: *mut rq) {
    let smt_mask: *const cpumask = cpu_smt_mask(cpu_of((*rq).core));
    let mut delta: u64;
    let now: u64 = rq_clock((*rq).core);
    let mut rq_i: *mut rq;
    let mut p: *mut task_struct;
    let mut i: ::core::ffi::c_int;

    lockdep_assert_rq_held(rq);
    WARN_ON_ONCE((*(*rq).core).core_forceidle_count == 0);
    if (*(*rq).core).core_forceidle_start == 0 {
        return;
    }

    delta = now - (*(*rq).core).core_forceidle_start;
    if (delta as i64) <= 0 {
        return;
    }
    (*(*rq).core).core_forceidle_start = now;

    if WARN_ON_ONCE((*(*rq).core).core_forceidle_occupation == 0) {
        /* can't be forced idle without a running task */
    } else if (*(*rq).core).core_forceidle_count > 1
        || (*(*rq).core).core_forceidle_occupation > 1
    {
        delta *= (*(*rq).core).core_forceidle_count;
        delta = div_u64(delta, (*(*rq).core).core_forceidle_occupation);
    }

    for_each_cpu!(i, smt_mask);
    rq_i = cpu_rq(i);
    p = if !(*rq_i).core_pick.is_null() { (*rq_i).core_pick } else { (*rq_i).curr };
    if p != (*rq_i).idle {
        __account_forceidle_time(p, delta);
    }
    for_each_cpu_end!();
}

#[cfg(CONFIG_SCHEDSTATS)]
unsafe fn __sched_core_tick(rq: *mut rq) {
    if (*(*rq).core).core_forceidle_count == 0 {
        return;
    }
    if rq != (*rq).core {
        update_rq_clock((*rq).core);
    }
    __sched_core_account_forceidle(rq);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
