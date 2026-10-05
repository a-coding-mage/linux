// SPDX-License-Identifier: GPL-2.0-only
// Continued from 126a30fae3bba11420ec2fcbde51a0a01bab1b5b; source-only.
// Native headers own every ABI layout, constant and synchronization primitive.
#![no_std]
#![cfg(CONFIG_SCHED_CORE)]

#[cfg(CONFIG_RUST_SCHED_CORE_COOKIE)]
compile_error!("SOURCE ONLY HOLD: scheduler core-cookie owner is not admitted");

use core::mem::MaybeUninit;
use core::ptr::addr_of_mut;
use kernel::bindings::sched_core_cookie_native::*;
use kernel::ffi::{c_int, c_uint, c_ulong, c_void};

const ENODEV: c_int = LUPOS_CORE_COOKIE_ENODEV as c_int;
const EINVAL: c_int = LUPOS_CORE_COOKIE_EINVAL as c_int;
const ESRCH: c_int = LUPOS_CORE_COOKIE_ESRCH as c_int;
const EPERM: c_int = LUPOS_CORE_COOKIE_EPERM as c_int;
const ENOMEM: c_int = LUPOS_CORE_COOKIE_ENOMEM as c_int;
const DEQUEUE_SAVE: c_int = LUPOS_CORE_COOKIE_DEQUEUE_SAVE as c_int;
const PR_SCHED_CORE_GET: c_uint = LUPOS_CORE_COOKIE_GET as c_uint;
const PR_SCHED_CORE_CREATE: c_uint = LUPOS_CORE_COOKIE_CREATE as c_uint;
const PR_SCHED_CORE_SHARE_TO: c_uint = LUPOS_CORE_COOKIE_SHARE_TO as c_uint;
const PR_SCHED_CORE_SHARE_FROM: c_uint = LUPOS_CORE_COOKIE_SHARE_FROM as c_uint;
const PR_SCHED_CORE_MAX: c_uint = LUPOS_CORE_COOKIE_MAX as c_uint;
const PTRACE_MODE_READ_REALCREDS: c_uint = LUPOS_CORE_COOKIE_PTRACE_MODE as c_uint;

/*
 * A simple wrapper around refcount. An allocated sched_core_cookie's
 * address is used to compute the cookie of the task.
 */
// The exact private C struct is declared in sched_core_cookie_bindings.h.

unsafe fn sched_core_alloc_cookie() -> c_ulong {
    // SAFETY: A successful native allocation owns writable refcount storage.
    unsafe {
        let ck = lupos_core_cookie_alloc();
        if ck.is_null() {
            return 0;
        }

        lupos_core_cookie_refcount_set(addr_of_mut!((*ck).refcnt), 1);
        sched_core_get();

        ck as c_ulong
    }
}

unsafe fn sched_core_put_cookie(cookie: c_ulong) {
    // SAFETY: Nonzero cookies represent references held by the caller.
    unsafe {
        let ptr = cookie as *mut sched_core_cookie;

        if !ptr.is_null()
            && lupos_core_cookie_refcount_dec_and_test(addr_of_mut!((*ptr).refcnt))
        {
            kfree(ptr.cast::<c_void>());
            sched_core_put();
        }
    }
}

unsafe fn sched_core_get_cookie(cookie: c_ulong) -> c_ulong {
    // SAFETY: The caller holds an existing reference or the task's PI lock.
    unsafe {
        let ptr = cookie as *mut sched_core_cookie;

        if !ptr.is_null() {
            lupos_core_cookie_refcount_inc(addr_of_mut!((*ptr).refcnt));
        }

        cookie
    }
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
unsafe fn sched_core_update_cookie(p: *mut task_struct, cookie: c_ulong) -> c_ulong {
    // SAFETY: The task reference and cookie remain live through the locked swap.
    unsafe {
        let old_cookie: c_ulong;
        let mut rf = MaybeUninit::<rq_flags>::uninit();
        let rq = lupos_core_cookie_task_rq_lock(p, rf.as_mut_ptr());

        /*
         * Since creating a cookie implies sched_core_get(), and we cannot set
         * a cookie until after we've created it, similarly, we cannot destroy
         * a cookie until after we've removed it, we must have core scheduling
         * enabled here.
         */
        lupos_core_cookie_warn_disabled(
            ((*p).core_cookie != 0 || cookie != 0) && !lupos_core_cookie_enabled(rq),
        );

        if lupos_core_cookie_enqueued(p) {
            sched_core_dequeue(rq, p, DEQUEUE_SAVE);
        }

        old_cookie = (*p).core_cookie;
        (*p).core_cookie = cookie;

        /*
         * Consider the cases: !prev_cookie and !cookie.
         */
        if cookie != 0 && lupos_core_cookie_task_on_rq_queued(p) {
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
        if lupos_core_cookie_task_on_cpu(rq, p) {
            resched_curr(rq);
        }

        lupos_core_cookie_task_rq_unlock(rq, p, rf.as_mut_ptr());

        old_cookie
    }
}

unsafe fn sched_core_clone_cookie(p: *mut task_struct) -> c_ulong {
    // SAFETY: The caller retains the task; the native PI lock protects its cookie.
    unsafe {
        let flags = lupos_core_cookie_pi_lock_irqsave(p);
        let cookie = sched_core_get_cookie((*p).core_cookie);
        lupos_core_cookie_pi_unlock_irqrestore(p, flags);
        cookie
    }
}

/// Initialize the newborn task's core-scheduling cookie.
///
/// # Safety
/// The caller owns the unpublished child and runs in its parent's context.
#[no_mangle]
pub unsafe extern "C" fn sched_core_fork(p: *mut task_struct) {
    // SAFETY: The native fork caller supplies the lifetime and parent context.
    unsafe {
        lupos_core_cookie_clear_node(addr_of_mut!((*p).core_node));
        (*p).core_cookie = sched_core_clone_cookie(lupos_core_cookie_current());
    }
}

/// Release the cookie owned by an exiting task.
///
/// # Safety
/// The caller owns the final task teardown and must release its cookie once.
#[no_mangle]
pub unsafe extern "C" fn sched_core_free(p: *mut task_struct) {
    // SAFETY: The task teardown contract supplies the outstanding cookie ref.
    unsafe { sched_core_put_cookie((*p).core_cookie) };
}

unsafe fn __sched_core_set(p: *mut task_struct, mut cookie: c_ulong) {
    // SAFETY: The task and incoming cookie reference outlive the exchange.
    unsafe {
        cookie = sched_core_get_cookie(cookie);
        cookie = sched_core_update_cookie(p, cookie);
        sched_core_put_cookie(cookie);
    }
}

/* Called from prctl interface: PR_SCHED_CORE */
/// Implement PR_SCHED_CORE with the native permission and reference ordering.
///
/// # Safety
/// The caller must be the current process's native prctl path. User memory is
/// accessed only through the native put_user leaf.
#[no_mangle]
pub unsafe extern "C" fn sched_core_share_pid(
    cmd: c_uint,
    pid: pid_t,
    ty: pid_type,
    uaddr: c_ulong,
) -> c_int {
    // SAFETY: The native entry contract applies; acquired task/cookie references
    // and tasklist lock are released on exactly the original C exit paths.
    unsafe {
        let mut cookie: c_ulong = 0;
        let mut id: c_ulong = 0;
        let task: *mut task_struct;
        let mut err: c_int = 0;

        if !lupos_core_cookie_smt_active() {
            return -ENODEV;
        }

        // The native scope BUILD_BUG_ON checks live in the bindings header.
        if ty.0 > PIDTYPE_PGID.0 || cmd >= PR_SCHED_CORE_MAX || pid < 0
            || (cmd != PR_SCHED_CORE_GET && uaddr != 0)
        {
            return -EINVAL;
        }

        lupos_core_cookie_rcu_read_lock();
        if pid == 0 {
            task = lupos_core_cookie_current();
        } else {
            task = find_task_by_vpid(pid);
            if task.is_null() {
                lupos_core_cookie_rcu_read_unlock();
                return -ESRCH;
            }
        }
        lupos_core_cookie_get_task_struct(task);
        lupos_core_cookie_rcu_read_unlock();

        'out: {
            /* Check whether this process may modify the specified process. */
            if !ptrace_may_access(task, PTRACE_MODE_READ_REALCREDS) {
                err = -EPERM;
                break 'out;
            }

            match cmd {
                PR_SCHED_CORE_GET => {
                    if ty.0 != PIDTYPE_PID.0 || uaddr & 7 != 0 {
                        err = -EINVAL;
                        break 'out;
                    }
                    cookie = sched_core_clone_cookie(task);
                    if cookie != 0 {
                        // Preserve the C owner's ignored hash status.
                        let _ = ptr_to_hashval(cookie as *const c_void, &mut id);
                    }
                    err = lupos_core_cookie_put_user_id(id, uaddr);
                    break 'out;
                }
                PR_SCHED_CORE_CREATE => {
                    cookie = sched_core_alloc_cookie();
                    if cookie == 0 {
                        err = -ENOMEM;
                        break 'out;
                    }
                }
                PR_SCHED_CORE_SHARE_TO => {
                    cookie = sched_core_clone_cookie(lupos_core_cookie_current());
                }
                PR_SCHED_CORE_SHARE_FROM => {
                    if ty.0 != PIDTYPE_PID.0 {
                        err = -EINVAL;
                        break 'out;
                    }
                    cookie = sched_core_clone_cookie(task);
                    __sched_core_set(lupos_core_cookie_current(), cookie);
                    break 'out;
                }
                _ => {
                    err = -EINVAL;
                    break 'out;
                }
            }

            if ty.0 == PIDTYPE_PID.0 {
                __sched_core_set(task, cookie);
                break 'out;
            }

            lupos_core_cookie_tasklist_read_lock();
            'out_tasklist: {
                let grp = lupos_core_cookie_task_pid_type(task, ty);
                // Preserve both complete do_each_pid_thread passes. No target is
                // modified before all permissions pass under tasklist_lock.
                let mut group = lupos_core_cookie_pid_first_check(grp, ty);
                while !group.is_null() {
                    let mut p = lupos_core_cookie_thread_first_check(group);
                    while !p.is_null() {
                        if !ptrace_may_access(p, PTRACE_MODE_READ_REALCREDS) {
                            err = -EPERM;
                            break 'out_tasklist;
                        }
                        p = lupos_core_cookie_thread_next(group, p);
                    }
                    if ty.0 == PIDTYPE_PID.0 {
                        break;
                    }
                    group = lupos_core_cookie_pid_next(group, ty);
                }

                let mut group = lupos_core_cookie_pid_first_set(grp, ty);
                while !group.is_null() {
                    let mut p = lupos_core_cookie_thread_first_set(group);
                    while !p.is_null() {
                        __sched_core_set(p, cookie);
                        p = lupos_core_cookie_thread_next(group, p);
                    }
                    if ty.0 == PIDTYPE_PID.0 {
                        break;
                    }
                    group = lupos_core_cookie_pid_next(group, ty);
                }
            }
            lupos_core_cookie_tasklist_read_unlock();
        }

        sched_core_put_cookie(cookie);
        lupos_core_cookie_put_task_struct(task);
        err
    }
}

/* CONFIG_SCHEDSTATS conditional section preserved from the source. */
#[cfg(CONFIG_SCHEDSTATS)]
/// Charge forced idle to selected/current non-idle SMT siblings.
///
/// # Safety
/// The caller holds the runqueue lock and has recently updated rq->core's clock.
#[no_mangle]
pub unsafe extern "C" fn __sched_core_account_forceidle(rq: *mut rq) {
    // SAFETY: The native caller supplies the lock and clock invariants.
    unsafe {
        let smt_mask = lupos_core_cookie_smt_mask(rq);
        let mut delta: u64;
        let now = lupos_core_cookie_rq_clock((*rq).core);

        lupos_core_cookie_assert_rq_held(rq);
        lupos_core_cookie_warn_forceidle_count((*(*rq).core).core_forceidle_count == 0);
        if (*(*rq).core).core_forceidle_start == 0 {
            return;
        }

        delta = now.wrapping_sub((*(*rq).core).core_forceidle_start);
        if (delta as i64) <= 0 {
            return;
        }
        (*(*rq).core).core_forceidle_start = now;

        if lupos_core_cookie_warn_forceidle_occupation(
            (*(*rq).core).core_forceidle_occupation == 0,
        ) {
            /* can't be forced idle without a running task */
        } else if (*(*rq).core).core_forceidle_count > 1
            || (*(*rq).core).core_forceidle_occupation > 1
        {
            delta = delta.wrapping_mul((*(*rq).core).core_forceidle_count as u64);
            delta = lupos_core_cookie_div_u64(delta, (*(*rq).core).core_forceidle_occupation);
        }

        let mut i = lupos_core_cookie_cpu_first(smt_mask);
        // for_each_cpu compares its signed int cursor with an unsigned bound.
        while (i as c_uint) < lupos_core_cookie_cpu_limit() {
            let rq_i = lupos_core_cookie_cpu_rq(i);
            let pick = (*rq_i).core_pick;
            let p = if !pick.is_null() {
                pick
            } else {
                lupos_core_cookie_rq_curr(rq_i)
            };
            if p != (*rq_i).idle {
                __account_forceidle_time(p, delta);
            }
            i = lupos_core_cookie_cpu_next(i, smt_mask);
        }
    }
}

#[cfg(CONFIG_SCHEDSTATS)]
/// Update core forced-idle accounting at a scheduler tick.
///
/// # Safety
/// The caller holds rq's native scheduler lock and supplies a live runqueue.
#[no_mangle]
pub unsafe extern "C" fn __sched_core_tick(rq: *mut rq) {
    // SAFETY: The native tick path supplies locking and object lifetime.
    unsafe {
        if (*(*rq).core).core_forceidle_count == 0 {
            return;
        }
        if rq != (*rq).core {
            update_rq_clock((*rq).core);
        }
        __sched_core_account_forceidle(rq);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
