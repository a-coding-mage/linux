// SPDX-License-Identifier: GPL-2.0-only
/*
 * Landlock - Cross-thread ruleset enforcement
 *
 * Copyright © 2025 Google LLC
 */

/* Dependencies from:
 * linux/atomic.h, linux/cleanup.h, linux/completion.h, linux/cred.h,
 * linux/errno.h, linux/overflow.h, linux/rcupdate.h, linux/sched.h,
 * linux/sched/signal.h, linux/sched/task.h, linux/slab.h, linux/task_work.h,
 * uapi/linux/landlock.h, cred.h, tsync.h, trace/events/landlock.h
 */

use core::ffi::{c_int, c_uint, c_void};
use core::mem;
use core::ptr;

type bool_ = bool;
type size_t = usize;
type gfp_t = c_uint;
type u32 = c_uint;

const ENOMEM: c_int = 12;
const EOVERFLOW: c_int = 75;
const ERESTARTNOINTR: c_int = 513;
const GFP_KERNEL_ACCOUNT: gfp_t = 0;
const LANDLOCK_RESTRICT_SELF_NO_NEW_PRIVS: u32 = 1 << 0;
const PF_EXITING: c_uint = 0x00000004;
const TWA_SIGNAL: c_int = 0;

#[repr(C)]
pub struct atomic_t {
    counter: c_int,
}

#[repr(C)]
pub struct completion {
    _private: [u8; 0],
}

#[repr(C)]
pub struct callback_head {
    _private: [u8; 0],
}

#[repr(C)]
pub struct cred {
    _private: [u8; 0],
}

#[repr(C)]
pub struct landlock_domain {
    _private: [u8; 0],
}

#[repr(C)]
pub struct landlock_cred_security {
    domain: *const landlock_domain,
}

#[repr(C)]
pub struct rw_semaphore {
    _private: [u8; 0],
}

#[repr(C)]
pub struct signal_struct {
    exec_update_lock: rw_semaphore,
}

#[repr(C)]
pub struct task_struct {
    flags: c_uint,
    signal: *mut signal_struct,
}

extern "C" {
    static mut current: *mut task_struct;

    fn current_cred() -> *const cred;
    fn get_cred(cred: *const cred) -> *const cred;
    fn prepare_creds() -> *mut cred;
    fn abort_creds(cred: *mut cred);
    fn commit_creds(cred: *mut cred) -> c_int;
    fn landlock_cred(cred: *const cred) -> *mut landlock_cred_security;
    fn landlock_cred_copy(
        dst: *mut landlock_cred_security,
        src: *const landlock_cred_security,
    );
    fn atomic_set(v: *mut atomic_t, i: c_int);
    fn atomic_read(v: *const atomic_t) -> c_int;
    fn atomic_inc(v: *mut atomic_t);
    fn atomic_dec(v: *mut atomic_t);
    fn atomic_dec_return(v: *mut atomic_t) -> c_int;
    fn init_completion(x: *mut completion);
    fn reinit_completion(x: *mut completion);
    fn complete_all(x: *mut completion);
    fn wait_for_completion(x: *mut completion);
    fn wait_for_completion_interruptible(x: *mut completion) -> c_int;
    fn task_set_no_new_privs(task: *mut task_struct);
    fn task_no_new_privs(task: *mut task_struct) -> bool_;
    fn trace_landlock_enforce_domain(
        domain: *const landlock_domain,
        complete: bool_,
        process: bool_,
        no_new_privs: bool_,
    );
    fn get_task_struct(task: *mut task_struct) -> *mut task_struct;
    fn put_task_struct(task: *mut task_struct);
    fn check_add_overflow(a: size_t, b: size_t, d: *mut size_t) -> bool_;
    fn krealloc_array(p: *mut c_void, n: size_t, size: size_t, flags: gfp_t) -> *mut c_void;
    fn kzalloc(size: size_t, flags: gfp_t) -> *mut c_void;
    fn kfree(p: *mut c_void);
    fn WARN_ON_ONCE(condition: bool_) -> bool_;
    fn init_task_work(work: *mut callback_head, func: unsafe extern "C" fn(*mut callback_head));
    fn task_work_add(task: *mut task_struct, work: *mut callback_head, notify: c_int) -> c_int;
    fn task_work_cancel(task: *mut task_struct, work: *mut callback_head) -> bool_;
    fn down_write_trylock(sem: *mut rw_semaphore) -> c_int;
    fn up_write(sem: *mut rw_semaphore);
    fn restart_syscall() -> c_int;
    fn next_thread(caller: *mut task_struct, thread: *mut task_struct) -> *mut task_struct;
}

macro_rules! unlikely {
    ($x:expr) => {
        $x
    };
}

/*
 * Shared state between multiple threads which are enforcing Landlock rulesets
 * in lockstep with each other.
 */
#[repr(C)]
struct tsync_shared_context {
    /* The old and tentative new creds of the calling thread. */
    old_cred: *const cred,
    new_cred: *const cred,

    /* True if sibling tasks need to set the no_new_privs flag. */
    set_no_new_privs: bool_,

    /* An error encountered in preparation step, or 0. */
    preparation_error: atomic_t,

    /*
     * Barrier after preparation step in restrict_one_thread.
     * The calling thread waits for completion.
     *
     * Re-initialized on every round of looking for newly spawned threads.
     */
    num_preparing: atomic_t,
    all_prepared: completion,

    /* Sibling threads wait for completion. */
    ready_to_commit: completion,

    /*
     * Barrier after commit step (used by syscall impl to wait for
     * completion).
     */
    num_unfinished: atomic_t,
    all_finished: completion,
}

#[repr(C)]
struct tsync_work {
    work: callback_head,
    task: *mut task_struct,
    shared_ctx: *mut tsync_shared_context,
}

/*
 * restrict_one_thread - update a thread's Landlock domain in lockstep with the
 * other threads in the same process
 *
 * When this is run, the same function gets run in all other threads in the same
 * process (except for the calling thread which called landlock_restrict_self).
 * The concurrently running invocations of restrict_one_thread coordinate
 * through the shared ctx object to do their work in lockstep to implement
 * all-or-nothing semantics for enforcing the new Landlock domain.
 *
 * Afterwards, depending on the presence of an error, all threads either commit
 * or abort the prepared credentials.  The commit operation can not fail any
 * more.
 */
unsafe extern "C" fn restrict_one_thread(ctx: *mut tsync_shared_context) {
    let new_dom: *const landlock_domain = (*landlock_cred((*ctx).new_cred)).domain;
    let mut err: c_int;
    let mut cred: *mut cred = ptr::null_mut();

    if current_cred() == (*ctx).old_cred {
        /*
         * Switch out old_cred with new_cred, if possible.
         *
         * In the common case, where all threads initially point to the
         * same struct cred, this optimization avoids creating separate
         * redundant credentials objects for each, which would all have
         * the same contents.
         *
         * Note: We are intentionally dropping the const qualifier
         * here, because it is required by commit_creds() and
         * abort_creds().
         */
        cred = get_cred((*ctx).new_cred) as *mut cred;
    } else {
        /* Else, prepare new creds and populate them. */
        cred = prepare_creds();

        if cred.is_null() {
            atomic_set(&mut (*ctx).preparation_error, -ENOMEM);

            /*
             * Even on error, we need to adhere to the protocol and
             * coordinate with concurrently running invocations.
             */
            if atomic_dec_return(&mut (*ctx).num_preparing) == 0 {
                complete_all(&mut (*ctx).all_prepared);
            }

            goto_out(ctx);
            return;
        }

        landlock_cred_copy(landlock_cred(cred), landlock_cred((*ctx).new_cred));
    }

    /*
     * Barrier: Wait until all threads are done preparing.
     * After this point, we can have no more failures.
     */
    if atomic_dec_return(&mut (*ctx).num_preparing) == 0 {
        complete_all(&mut (*ctx).all_prepared);
    }

    /*
     * Wait for signal from calling thread that it's safe to read the
     * preparation error now and we are ready to commit (or abort).
     */
    wait_for_completion(&mut (*ctx).ready_to_commit);

    /* Abort the commit if any of the other threads had an error. */
    err = atomic_read(&(*ctx).preparation_error);
    if err != 0 {
        abort_creds(cred);
        goto_out(ctx);
        return;
    }

    /*
     * Make sure that all sibling tasks fulfill the no_new_privs
     * prerequisite.  (This is in line with Seccomp's
     * SECCOMP_FILTER_FLAG_TSYNC logic in kernel/seccomp.c)
     */
    if (*ctx).set_no_new_privs {
        task_set_no_new_privs(current);
    }

    commit_creds(cred);

    /*
     * Emitted strictly after commit_creds() and before the out: label, so
     * it fires only for a thread now enforcing new_dom, and every
     * non-concluding (complete == false) event happens-before the
     * operation's single concluding one.  Skipped on the flags-only path,
     * where old_cred and new_cred carry the same domain.  A sibling never
     * concludes the operation and its enforcement is always process-wide.
     */
    if new_dom != (*landlock_cred((*ctx).old_cred)).domain {
        trace_landlock_enforce_domain(new_dom, false, true, task_no_new_privs(current));
    }

    goto_out(ctx);
}

unsafe fn goto_out(ctx: *mut tsync_shared_context) {
    /* Notify the calling thread once all threads are done */
    if atomic_dec_return(&mut (*ctx).num_unfinished) == 0 {
        complete_all(&mut (*ctx).all_finished);
    }
}

/*
 * restrict_one_thread_callback - task_work callback for restricting a thread
 *
 * Calls restrict_one_thread with the struct landlock_shared_tsync_context.
 */
unsafe extern "C" fn restrict_one_thread_callback(work: *mut callback_head) {
    let ctx: *mut tsync_work = work as *mut tsync_work;

    restrict_one_thread((*ctx).shared_ctx);
}

/*
 * struct tsync_works - a growable array of per-task contexts
 *
 * The zero-initialized struct represents the empty array.
 */
#[repr(C)]
struct tsync_works {
    works: *mut *mut tsync_work,
    size: size_t,
    capacity: size_t,
}

/*
 * tsync_works_provide - provides a preallocated tsync_work for the given task
 *
 * This also stores a task pointer in the context and increments the reference
 * count of the task.
 *
 * This function may fail in the case where we did not preallocate sufficient
 * capacity.  This can legitimately happen if new threads get started after we
 * grew the capacity.
 *
 * Return: A pointer to the preallocated context struct with task filled in, or
 * NULL if preallocated context structs ran out.
 */
unsafe extern "C" fn tsync_works_provide(
    s: *mut tsync_works,
    task: *mut task_struct,
) -> *mut tsync_work {
    let ctx: *mut tsync_work;

    if (*s).size >= (*s).capacity {
        return ptr::null_mut();
    }

    ctx = *(*s).works.add((*s).size);
    (*s).size += 1;

    (*ctx).task = get_task_struct(task);
    ctx
}

/**
 * tsync_works_trim - Put the last tsync_work element
 *
 * @s: TSYNC works to trim.
 *
 * Put the last task and decrement the size of @s.
 *
 * This helper does not cancel a running task, but just reset the last element
 * to zero.
 */
unsafe extern "C" fn tsync_works_trim(s: *mut tsync_works) {
    let ctx: *mut tsync_work;

    if WARN_ON_ONCE((*s).size <= 0) {
        return;
    }

    ctx = *(*s).works.add((*s).size - 1);

    /*
     * For consistency, remove the task from ctx so that it does not look
     * like we handed it a task_work.
     */
    put_task_struct((*ctx).task);
    ptr::write_bytes(ctx, 0, 1);

    /*
     * Cancel the tsync_works_provide() change to recycle the reserved
     * memory for the next thread, if any.  This also ensures that
     * cancel_tsync_works() and tsync_works_release() do not see any NULL
     * task pointers.
     */
    (*s).size -= 1;
}

/*
 * tsync_works_grow_by - preallocates space for n more contexts in s
 *
 * On a successful return, the subsequent n calls to tsync_works_provide() are
 * guaranteed to succeed.  (size + n <= capacity)
 *
 * Return: 0 if sufficient space for n more elements could be provided, -ENOMEM
 * on allocation errors, -EOVERFLOW in case of integer overflow.
 */
unsafe extern "C" fn tsync_works_grow_by(
    s: *mut tsync_works,
    n: size_t,
    flags: gfp_t,
) -> c_int {
    let mut i: size_t;
    let mut new_capacity: size_t = 0;
    let works: *mut *mut tsync_work;
    let mut work: *mut tsync_work;

    if check_add_overflow((*s).size, n, &mut new_capacity) {
        return -EOVERFLOW;
    }

    /* No need to reallocate if s already has sufficient capacity. */
    if new_capacity <= (*s).capacity {
        return 0;
    }

    works = krealloc_array(
        (*s).works as *mut c_void,
        new_capacity,
        mem::size_of::<*mut tsync_work>(),
        flags,
    ) as *mut *mut tsync_work;
    if works.is_null() {
        return -ENOMEM;
    }

    (*s).works = works;

    i = (*s).capacity;
    while i < new_capacity {
        work = kzalloc(mem::size_of::<tsync_work>(), flags) as *mut tsync_work;
        if work.is_null() {
            /*
             * Leave the object in a consistent state,
             * but return an error.
             */
            (*s).capacity = i;
            return -ENOMEM;
        }
        *(*s).works.add(i) = work;
        i += 1;
    }
    (*s).capacity = new_capacity;
    0
}

/*
 * tsync_works_contains - checks for presence of task in s
 */
unsafe extern "C" fn tsync_works_contains_task(
    s: *const tsync_works,
    task: *const task_struct,
) -> bool_ {
    let mut i: size_t;

    i = 0;
    while i < (*s).size {
        if (**(*s).works.add(i)).task == task as *mut task_struct {
            return true;
        }
        i += 1;
    }

    false
}

/*
 * tsync_works_release - frees memory held by s and drops all task references
 *
 * This does not free s itself, only the data structures held by it.
 */
unsafe extern "C" fn tsync_works_release(s: *mut tsync_works) {
    let mut i: size_t;

    i = 0;
    while i < (*s).size {
        if WARN_ON_ONCE((**(*s).works.add(i)).task.is_null()) {
            i += 1;
            continue;
        }

        put_task_struct((**(*s).works.add(i)).task);
        i += 1;
    }

    i = 0;
    while i < (*s).capacity {
        kfree(*(*s).works.add(i) as *mut c_void);
        i += 1;
    }

    kfree((*s).works as *mut c_void);
    (*s).works = ptr::null_mut();
    (*s).size = 0;
    (*s).capacity = 0;
}

/*
 * count_additional_threads - counts the sibling threads that are not in works
 */
unsafe extern "C" fn count_additional_threads(works: *const tsync_works) -> size_t {
    let _caller: *const task_struct;
    let _thread: *const task_struct;
    let mut n: size_t = 0;

    _caller = current;

    /*
     * Original C iterates under guard(rcu)():
     * for_each_thread(caller, thread) {
     *     Skip current, exited threads, and threads already in works.
     *     n++;
     * }
     * The for_each_thread/RCU iteration primitive is provided externally by
     * the kernel and has no file-local Rust expression here.
     */
    let mut thread = _caller as *mut task_struct;
    while !thread.is_null() {
        if thread != _caller as *mut task_struct && ((*thread).flags & PF_EXITING) == 0
            && !tsync_works_contains_task(works, thread) { n += 1; }
        thread = next_thread(current, thread);
    }
    n
}

/*
 * schedule_task_work - adds task_work for all eligible sibling threads
 *                      which have not been scheduled yet
 *
 * For each added task_work, atomically increments shared_ctx->num_preparing and
 * shared_ctx->num_unfinished.
 *
 * Return: True if at least one eligible sibling thread was found, false
 * otherwise.
 */
unsafe extern "C" fn schedule_task_work(
    works: *mut tsync_works,
    shared_ctx: *mut tsync_shared_context,
) -> bool_ {
    let _err: c_int;
    let _caller: *const task_struct;
    let _thread: *mut task_struct;
    let _ctx: *mut tsync_work;
    let mut found_more_threads: bool_ = false;

    _caller = current;

    /*
     * Original C iterates under guard(rcu)():
     * for_each_thread(caller, thread) {
     *     Skip current, exited threads, and already-seen threads.
     *     Set found_more_threads.
     *     Provide a tsync_work, initialize it, increment shared counters,
     *     init_task_work(), and task_work_add(..., TWA_SIGNAL).
     *     On task_work_add() failure, trim and decrement counters.
     * }
     * The for_each_thread/RCU iteration primitive is provided externally by
     * the kernel and has no file-local Rust expression here.
     */
    let mut thread = current;
    while !thread.is_null() {
        if thread != current && ((*thread).flags & PF_EXITING) == 0
            && !tsync_works_contains_task(works, thread) {
            found_more_threads = true;
            let ctx = tsync_works_provide(works, thread);
            if ctx.is_null() { break; }
            (*ctx).shared_ctx = shared_ctx;
            atomic_inc(&mut (*shared_ctx).num_preparing);
            atomic_inc(&mut (*shared_ctx).num_unfinished);
            init_task_work(&mut (*ctx).work, restrict_one_thread_callback);
            if unlikely!(task_work_add(thread, &mut (*ctx).work, TWA_SIGNAL) != 0) {
                tsync_works_trim(works);
                atomic_dec(&mut (*shared_ctx).num_preparing);
                atomic_dec(&mut (*shared_ctx).num_unfinished);
            }
        }
        thread = next_thread(current, thread);
    }
    found_more_threads
}

/*
 * cancel_tsync_works - cancel all task works where it is possible
 *
 * Task works can be canceled as long as they are still queued and have not
 * started running.  If they get canceled, we decrement
 * shared_ctx->num_preparing and shared_ctx->num_unfished and mark the two
 * completions if needed, as if the task was never scheduled.
 */
unsafe extern "C" fn cancel_tsync_works(
    works: *const tsync_works,
    shared_ctx: *mut tsync_shared_context,
) {
    let mut i: size_t;

    i = 0;
    while i < (*works).size {
        if WARN_ON_ONCE((**(*works).works.add(i)).task.is_null()) {
            i += 1;
            continue;
        }

        if !task_work_cancel((**(*works).works.add(i)).task, &mut (**(*works).works.add(i)).work) {
            i += 1;
            continue;
        }

        /* After dequeueing, act as if the task work had executed. */

        if atomic_dec_return(&mut (*shared_ctx).num_preparing) == 0 {
            complete_all(&mut (*shared_ctx).all_prepared);
        }

        if atomic_dec_return(&mut (*shared_ctx).num_unfinished) == 0 {
            complete_all(&mut (*shared_ctx).all_finished);
        }
        i += 1;
    }
}

/*
 * restrict_sibling_threads - enables a Landlock policy for all sibling threads
 */
#[no_mangle]
pub unsafe extern "C" fn landlock_restrict_sibling_threads(
    old_cred: *const cred,
    new_cred: *const cred,
    restrict_flags: u32,
) -> c_int {
    let mut err: c_int;
    let mut shared_ctx: tsync_shared_context = mem::zeroed();
    let mut works: tsync_works = mem::zeroed();
    let mut newly_discovered_threads: size_t;
    let mut found_more_threads: bool_;

    atomic_set(&mut shared_ctx.preparation_error, 0);
    init_completion(&mut shared_ctx.all_prepared);
    init_completion(&mut shared_ctx.ready_to_commit);
    atomic_set(&mut shared_ctx.num_unfinished, 1);
    init_completion(&mut shared_ctx.all_finished);
    shared_ctx.old_cred = old_cred;
    shared_ctx.new_cred = new_cred;
    shared_ctx.set_no_new_privs =
        (restrict_flags & LANDLOCK_RESTRICT_SELF_NO_NEW_PRIVS) != 0 || task_no_new_privs(current);

    /*
     * Serialize concurrent TSYNC operations to prevent deadlocks when
     * multiple threads call landlock_restrict_self() simultaneously.
     * If the lock is already held, we gracefully yield by restarting the
     * syscall. This allows the current thread to process pending
     * task_works before retrying.
     */
    if down_write_trylock(&mut (*(*current).signal).exec_update_lock) == 0 {
        return restart_syscall();
    }

    /*
     * We schedule a pseudo-signal task_work for each of the calling task's
     * sibling threads.  In the task work, each thread:
     *
     * 1) runs prepare_creds() and writes back the error to
     *    shared_ctx.preparation_error, if needed.
     *
     * 2) signals that it's done with prepare_creds() to the calling task.
     *    (completion "all_prepared").
     *
     * 3) waits for the completion "ready_to_commit".  This is sent by the
     *    calling task after ensuring that all sibling threads have done
     *    with the "preparation" stage.
     *
     *    After this barrier is reached, it's safe to read
     *    shared_ctx.preparation_error.
     *
     * 4) reads shared_ctx.preparation_error and then either does
     *    commit_creds() or abort_creds().
     *
     * 5) signals that it's done altogether (barrier synchronization
     *    "all_finished")
     *
     * Unlike seccomp, which modifies sibling tasks directly, we do not
     * need to acquire the cred_guard_mutex and sighand->siglock:
     *
     * - As in our case, all threads are themselves exchanging their own
     *   struct cred through the credentials API, no locks are needed for
     *   that.
     * - Our for_each_thread() loops are protected by RCU.
     * - We do not acquire a lock to keep the list of sibling threads
     *   stable between our for_each_thread loops.  If the list of
     *   available sibling threads changes between these for_each_thread
     *   loops, we make up for that by continuing to look for threads until
     *   they are all discovered and have entered their task_work, where
     *   they are unable to spawn new threads.
     */
    loop {
        /* In RCU read-lock, count the threads we need. */
        newly_discovered_threads = count_additional_threads(&works);

        if newly_discovered_threads == 0 {
            break; /* done */
        }

        err = tsync_works_grow_by(&mut works, newly_discovered_threads, GFP_KERNEL_ACCOUNT);
        if err != 0 {
            atomic_set(&mut shared_ctx.preparation_error, err);
            break;
        }

        /*
         * The "all_prepared" barrier is used locally to the loop body,
         * this use of for_each_thread().  We can reset it on each loop
         * iteration because all previous loop iterations are done with
         * it already.
         *
         * num_preparing is initialized to 1 so that the counter can
         * not go to 0 and mark the completion as done before all task
         * works are registered.  We decrement it at the end of the
         * loop body.
         */
        atomic_set(&mut shared_ctx.num_preparing, 1);
        reinit_completion(&mut shared_ctx.all_prepared);

        /*
         * In RCU read-lock, schedule task work on newly discovered
         * sibling tasks.
         */
        found_more_threads = schedule_task_work(&mut works, &mut shared_ctx);

        /*
         * Decrement num_preparing for current, to undo that we
         * initialized it to 1 a few lines above.
         */
        if atomic_dec_return(&mut shared_ctx.num_preparing) > 0 {
            if wait_for_completion_interruptible(&mut shared_ctx.all_prepared) != 0 {
                /*
                 * In case of interruption, we need to retry
                 * the system call.
                 */
                atomic_set(&mut shared_ctx.preparation_error, -ERESTARTNOINTR);

                /*
                 * Opportunistic improvement: try to cancel task
                 * works for tasks that did not start running
                 * yet. We do not have a guarantee that it
                 * cancels any of the enqueued task works
                 * because task_work_run() might already have
                 * dequeued them.
                 */
                cancel_tsync_works(&works, &mut shared_ctx);

                /*
                 * Break the loop with error. The cleanup code
                 * after the loop unblocks the remaining
                 * task_works.
                 */
                break;
            }
        }

        if !(found_more_threads && atomic_read(&shared_ctx.preparation_error) == 0) {
            break;
        }
    }

    /*
     * We now have either (a) all sibling threads blocking and in "prepared"
     * state in the task work, or (b) the preparation error is set. Ask all
     * threads to commit (or abort).
     */
    complete_all(&mut shared_ctx.ready_to_commit);

    /*
     * Decrement num_unfinished for current, to undo that we initialized it
     * to 1 at the beginning.
     */
    if atomic_dec_return(&mut shared_ctx.num_unfinished) > 0 {
        wait_for_completion(&mut shared_ctx.all_finished);
    }

    tsync_works_release(&mut works);
    up_write(&mut (*(*current).signal).exec_update_lock);
    atomic_read(&shared_ctx.preparation_error)
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
