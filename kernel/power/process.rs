// SPDX-License-Identifier: GPL-2.0
/*
 * drivers/power/process.c - Functions for starting/stopping processes on
 *                           suspend transitions.
 *
 * Originally from swsusp.
 */

// Linux kernel dependencies supplied by other translation units.

use core::ffi::{c_char, c_int, c_uint, c_ulong};

#[repr(C)]
pub struct task_struct {
    pub flags: c_ulong,
}

type Ktime = i64;

pub const FREEZE_TIMEOUT_MSECS: c_uint = 20 * MSEC_PER_SEC;
pub static mut freeze_timeout_msecs: c_uint = FREEZE_TIMEOUT_MSECS;

extern "C" {
    static mut current: *mut task_struct;
    static mut tasklist_lock: u8;
    static mut pm_freezing: bool;
    static mut pm_nosig_freezing: bool;
    static mut pm_debug_messages_on: bool;
    static mut freezer_active: u8;

    fn ktime_get_boottime() -> Ktime;
    fn ktime_sub(end: Ktime, start: Ktime) -> Ktime;
    fn ktime_to_ms(value: Ktime) -> c_uint;
    fn msecs_to_jiffies(value: c_uint) -> c_ulong;
    fn usleep_range(min: c_int, max: c_int);
    fn freeze_workqueues_begin();
    fn freeze_workqueues_busy() -> bool;
    fn show_freezable_workqueues();
    fn pm_wakeup_pending() -> bool;
    fn pm_wakeup_clear(value: c_int);
    fn freeze_task(task: *mut task_struct) -> bool;
    fn freezing(task: *mut task_struct) -> bool;
    fn frozen(task: *mut task_struct) -> bool;
    fn sched_show_task(task: *mut task_struct);
    fn __usermodehelper_disable(state: c_int) -> c_int;
    fn __usermodehelper_set_disable_depth(state: c_int);
    fn oom_killer_disable(timeout: c_ulong) -> c_int;
    fn oom_killer_enable();
    fn thaw_processes();
    fn thaw_kernel_threads();
    fn thaw_workqueues();
    fn __thaw_task(task: *mut task_struct);
    fn usermodehelper_enable();
    fn schedule();
    fn trace_suspend_resume(name: *const c_char, value: c_int, start: bool);
    fn in_atomic() -> bool;
    fn static_branch_inc(branch: *mut u8);
    fn static_branch_dec(branch: *mut u8);
}

const MSEC_PER_SEC: c_uint = 1000;
const USEC_PER_MSEC: c_int = 1000;
const UMH_FREEZING: c_int = 1;
const UMH_DISABLED: c_int = 2;
const PF_SUSPEND_TASK: c_ulong = 0x8000;
const PF_KTHREAD: c_ulong = 0x00200000;
const EBUSY: c_int = 16;

extern "C" {
    fn log_info(format: *const c_char, ...);
    fn log_error(format: *const c_char, ...);
}

static mut process_list: *mut task_struct = core::ptr::null_mut();

unsafe fn try_to_freeze_tasks(user_only: bool) -> c_int {
    let what = if user_only { "user space processes" } else { "remaining freezable tasks" };
    let mut todo: c_uint;
    let mut wq_busy = false;
    let start = ktime_get_boottime();
    let end_time = 0 as c_ulong + msecs_to_jiffies(freeze_timeout_msecs);
    let mut wakeup = false;
    let mut sleep_usecs = USEC_PER_MSEC;

    let _ = what;
    if !user_only { freeze_workqueues_begin(); }

    loop {
        todo = 0;
        // for_each_process_thread(g, p), under tasklist_lock.
        let mut p = process_list;
        while !p.is_null() {
            if p != current && freeze_task(p) { todo += 1; }
            p = (*p).next as *mut task_struct;
        }
        if !user_only {
            wq_busy = freeze_workqueues_busy();
            todo += wq_busy as c_uint;
        }
        if todo == 0 || 0 as c_ulong > end_time { break; }
        if pm_wakeup_pending() { wakeup = true; break; }
        usleep_range(sleep_usecs / 2, sleep_usecs);
        if sleep_usecs < 8 * USEC_PER_MSEC { sleep_usecs *= 2; }
    }

    let elapsed_msecs = ktime_to_ms(ktime_sub(ktime_get_boottime(), start));
    if todo != 0 {
        if wq_busy { show_freezable_workqueues(); }
        if !wakeup || pm_debug_messages_on {
            // for_each_process_thread(g, p): show tasks still refusing to freeze.
        }
        EBUSY
    } else { 0 }
}

pub unsafe fn freeze_processes() -> c_int {
    let mut error = __usermodehelper_disable(UMH_FREEZING);
    if error != 0 { return error; }
    (*current).flags |= PF_SUSPEND_TASK;
    if !pm_freezing { static_branch_inc(&mut freezer_active); }
    pm_wakeup_clear(0);
    pm_freezing = true;
    error = try_to_freeze_tasks(true);
    if error == 0 { __usermodehelper_set_disable_depth(UMH_DISABLED); }
    if in_atomic() { /* BUG_ON(in_atomic()) */ }
    if error == 0 && oom_killer_disable(msecs_to_jiffies(freeze_timeout_msecs)) == 0 { error = -EBUSY; }
    if error != 0 { thaw_processes(); }
    error
}

pub unsafe fn freeze_kernel_threads() -> c_int {
    pm_nosig_freezing = true;
    let error = try_to_freeze_tasks(false);
    if in_atomic() { /* BUG_ON(in_atomic()) */ }
    if error != 0 { thaw_kernel_threads(); }
    error
}

pub unsafe fn thaw_processes() {
    trace_suspend_resume(b"thaw_processes\0".as_ptr() as *const c_char, 0, true);
    if pm_freezing { static_branch_dec(&mut freezer_active); }
    pm_freezing = false;
    pm_nosig_freezing = false;
    oom_killer_enable();
    __usermodehelper_set_disable_depth(UMH_FREEZING);
    thaw_workqueues();
    // for_each_process_thread(g, p) { WARN_ON(...); __thaw_task(p); }
    (*current).flags &= !PF_SUSPEND_TASK;
    usermodehelper_enable();
    schedule();
    trace_suspend_resume(b"thaw_processes\0".as_ptr() as *const c_char, 0, false);
}

pub unsafe fn thaw_kernel_threads() {
    pm_nosig_freezing = false;
    thaw_workqueues();
    // for_each_process_thread(g, p) if (p->flags & PF_KTHREAD) __thaw_task(p);
    schedule();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
