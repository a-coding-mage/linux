/*
 * JFFS2 -- Journalling Flash File System, Version 2.
 *
 * Copyright © 2001-2007 Red Hat, Inc.
 * Copyright © 2004-2010 David Woodhouse <dwmw2@infradead.org>
 *
 * Created by David Woodhouse <dwmw2@infradead.org>
 *
 * For licensing information, see the file 'LICENCE' in this directory.
 */

// pr_fmt(fmt) KBUILD_MODNAME ": " fmt

use core::ffi::c_void;

#[repr(C)]
pub struct jffs2_sb_info {
    pub erase_completion_lock: c_void,
    pub gc_task: *mut task_struct,
    pub gc_thread_start: c_void,
    pub gc_thread_exit: c_void,
    pub mtd: *mut mtd_info,
}

#[repr(C)]
pub struct task_struct {
    pub pid: i32,
}

#[repr(C)]
pub struct mtd_info {
    pub index: i32,
}

extern "C" {
    fn jffs2_thread_should_wake(c: *mut jffs2_sb_info) -> bool;
    fn send_sig(sig: i32, t: *mut task_struct, group: i32) -> i32;
    fn init_completion(completion: *mut c_void);
    fn kthread_run(threadfn: unsafe extern "C" fn(*mut c_void) -> i32,
                   data: *mut c_void,
                   name: *const i8,
                   ...) -> *mut task_struct;
    fn is_err(ptr: *mut task_struct) -> bool;
    fn ptr_err(ptr: *mut task_struct) -> i32;
    fn complete(completion: *mut c_void);
    fn wait_for_completion(completion: *mut c_void);
    fn spin_lock(lock: *mut c_void);
    fn spin_unlock(lock: *mut c_void);
    fn siginitset(set: *mut sigset_t, mask: u64);
    fn sigmask(sig: i32) -> u64;
    fn allow_signal(sig: i32) -> i32;
    fn set_user_nice(task: *mut task_struct, nice: i32) -> i32;
    fn set_freezable();
    fn sigprocmask(how: i32, set: *mut sigset_t, oldset: *mut sigset_t) -> i32;
    fn set_current_state(state: i32);
    fn schedule();
    fn schedule_timeout_interruptible(timeout: u64) -> i64;
    fn msecs_to_jiffies(msecs: u64) -> u64;
    fn kthread_should_stop() -> bool;
    fn signal_pending(task: *mut task_struct) -> bool;
    fn freezing(task: *mut task_struct) -> bool;
    fn try_to_freeze() -> bool;
    fn kernel_dequeue_signal() -> u64;
    fn kernel_signal_stop();
    fn jffs2_garbage_collect_pass(c: *mut jffs2_sb_info) -> i32;
    fn kthread_complete_and_exit(completion: *mut c_void, code: i32) -> !;
    static mut current: *mut task_struct;
}

#[repr(C)]
pub struct sigset_t {
    pub __val: [u64; 16],
}

const SIGHUP: i32 = 1;
const SIGKILL: i32 = 9;
const SIGSTOP: i32 = 19;
const SIG_UNBLOCK: i32 = 1;
const SIG_BLOCK: i32 = 0;
const TASK_INTERRUPTIBLE: i32 = 1;
const ENOSPC: i32 = 28;

pub unsafe fn jffs2_garbage_collect_trigger(c: *mut jffs2_sb_info) {
    // assert_spin_locked(&c->erase_completion_lock);
    if !(*c).gc_task.is_null() && jffs2_thread_should_wake(c) {
        send_sig(SIGHUP, (*c).gc_task, 1);
    }
}

/* This must only ever be called when no GC thread is currently running */
pub unsafe fn jffs2_start_garbage_collect_thread(c: *mut jffs2_sb_info) -> i32 {
    let mut tsk: *mut task_struct;
    let mut ret: i32 = 0;

    // BUG_ON(c->gc_task);

    init_completion(&mut (*c).gc_thread_start as *mut c_void);
    init_completion(&mut (*c).gc_thread_exit as *mut c_void);

    tsk = kthread_run(jffs2_garbage_collect_thread, c as *mut c_void,
                      b"jffs2_gcd_mtd%d\0".as_ptr() as *const i8, (*(*c).mtd).index);
    if is_err(tsk) {
        // pr_warn("fork failed for JFFS2 garbage collect thread: %pe\n", tsk);
        complete(&mut (*c).gc_thread_exit as *mut c_void);
        ret = ptr_err(tsk);
    } else {
        // jffs2_dbg(1, "Garbage collect thread is pid %d\n", tsk->pid);
        wait_for_completion(&mut (*c).gc_thread_start as *mut c_void);
        ret = (*tsk).pid;
    }

    ret
}

pub unsafe fn jffs2_stop_garbage_collect_thread(c: *mut jffs2_sb_info) {
    let mut wait = 0;
    spin_lock(&mut (*c).erase_completion_lock as *mut c_void);
    if !(*c).gc_task.is_null() {
        // jffs2_dbg(1, "Killing GC task %d\n", c->gc_task->pid);
        send_sig(SIGKILL, (*c).gc_task, 1);
        wait = 1;
    }
    spin_unlock(&mut (*c).erase_completion_lock as *mut c_void);
    if wait != 0 {
        wait_for_completion(&mut (*c).gc_thread_exit as *mut c_void);
    }
}

unsafe extern "C" fn jffs2_garbage_collect_thread(_c: *mut c_void) -> i32 {
    let c = _c as *mut jffs2_sb_info;
    let mut hupmask = sigset_t { __val: [0; 16] };

    siginitset(&mut hupmask, sigmask(SIGHUP));
    allow_signal(SIGKILL);
    allow_signal(SIGSTOP);
    allow_signal(SIGHUP);

    (*c).gc_task = current;
    complete(&mut (*c).gc_thread_start as *mut c_void);

    set_user_nice(current, 10);

    set_freezable();
    'main: loop {
        sigprocmask(SIG_UNBLOCK, &mut hupmask, core::ptr::null_mut());
        spin_lock(&mut (*c).erase_completion_lock as *mut c_void);
        if !jffs2_thread_should_wake(c) {
            set_current_state(TASK_INTERRUPTIBLE);
            spin_unlock(&mut (*c).erase_completion_lock as *mut c_void);
            // jffs2_dbg(1, "%s(): sleeping...\n", __func__);
            schedule();
        } else {
            spin_unlock(&mut (*c).erase_completion_lock as *mut c_void);
        }

        /* Problem - immediately after bootup, the GCD spends a lot
         * of time in places like jffs2_kill_fragtree(); so much so
         * that userspace processes (like gdm and X) are starved
         * despite plenty of cond_resched()s and renicing.  Yield()
         * doesn't help, either (presumably because userspace and GCD
         * are generally competing for a higher latency resource -
         * disk).
         * This forces the GCD to slow the hell down.   Pulling an
         * inode in with read_inode() is much preferable to having
         * the GC thread get there first. */
        schedule_timeout_interruptible(msecs_to_jiffies(50));

        if kthread_should_stop() {
            // jffs2_dbg(1, "%s(): kthread_stop() called\n", __func__);
            break;
        }

        /* Put_super will send a SIGKILL and then wait on the sem. */
        while signal_pending(current) || freezing(current) {
            let signr: u64;

            if try_to_freeze() {
                continue 'main;
            }

            signr = kernel_dequeue_signal();

            match signr as i32 {
                SIGSTOP => {
                    // jffs2_dbg(1, "%s(): SIGSTOP received\n", __func__);
                    kernel_signal_stop();
                }
                SIGKILL => {
                    // jffs2_dbg(1, "%s(): SIGKILL received\n", __func__);
                    break 'main;
                }
                SIGHUP => {
                    // jffs2_dbg(1, "%s(): SIGHUP received\n", __func__);
                }
                _ => {
                    // jffs2_dbg(1, "%s(): signal %ld received\n", __func__, signr);
                }
            }
        }

        /* We don't want SIGHUP to interrupt us. STOP and KILL are OK though. */
        sigprocmask(SIG_BLOCK, &mut hupmask, core::ptr::null_mut());

        // jffs2_dbg(1, "%s(): pass\n", __func__);
        if jffs2_garbage_collect_pass(c) == -ENOSPC {
            // pr_notice("No space for garbage collection. Aborting GC thread\n");
            break;
        }
    }

    spin_lock(&mut (*c).erase_completion_lock as *mut c_void);
    (*c).gc_task = core::ptr::null_mut();
    spin_unlock(&mut (*c).erase_completion_lock as *mut c_void);
    kthread_complete_and_exit(&mut (*c).gc_thread_exit as *mut c_void, 0);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
