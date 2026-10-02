// SPDX-License-Identifier: GPL-2.0
/* Functions for starting/stopping processes on suspend transitions.
 * Originally from swsusp; translated from the retained process.c.
 */
#![allow(missing_docs, unsafe_op_in_unsafe_fn)]

#[allow(clippy::all, dead_code, non_camel_case_types, non_snake_case,
         non_upper_case_globals, improper_ctypes, unreachable_pub)]
mod bindings {
    use kernel::ffi;
    type __kernel_size_t = usize;
    type __kernel_ssize_t = isize;
    type __kernel_ptrdiff_t = isize;
    include!(concat!(env!("OBJTREE"), "/rust/bindings/power_process_generated.rs"));
}

use bindings::*;
use core::{mem::{align_of, offset_of, size_of}, ptr::addr_of_mut};
use kernel::ffi::{c_char, c_int, c_long, c_uint, c_ulong};

// The configured C preprocessor supplies __read_mostly's actual section or
// absence of a section; this included item defines the public state in Rust.
include!(concat!(env!("OBJTREE"), "/kernel/power/process-state-generated.rs"));

// Bindgen must use the same configured task/signal layouts as the C adapter.
// No Rust reference to a shared task, signal, list or global is constructed.
const _: () = {
    assert!(size_of::<c_uint>() == LUPOS_PROCESS_TIMEOUT_SIZE as usize);
    assert!(align_of::<c_uint>() == LUPOS_PROCESS_TIMEOUT_ALIGN as usize);
    assert!(size_of::<task_struct>() == LUPOS_PROCESS_TASK_SIZE as usize);
    assert!(align_of::<task_struct>() == LUPOS_PROCESS_TASK_ALIGN as usize);
    assert!(offset_of!(task_struct, flags) == LUPOS_PROCESS_FLAGS_OFFSET as usize);
    assert!(offset_of!(task_struct, signal) == LUPOS_PROCESS_SIGNAL_OFFSET as usize);
    assert!(offset_of!(task_struct, thread_node) == LUPOS_PROCESS_THREAD_NODE_OFFSET as usize);
    assert!(size_of::<signal_struct>() == LUPOS_PROCESS_SIGNAL_SIZE as usize);
    assert!(align_of::<signal_struct>() == LUPOS_PROCESS_SIGNAL_ALIGN as usize);
    assert!(offset_of!(signal_struct, thread_head) == LUPOS_PROCESS_THREAD_HEAD_OFFSET as usize);
};

struct TasklistReadGuard;

// Each original for_each_process_thread expansion owns a separate RCU
// diagnostic's warn-once state. Do not merge those four macro call sites.
enum ProcessWalk {
    FreezeTasks,
    ShowUnfrozenTasks,
    ThawProcesses,
    ThawKernelThreads,
}

impl TasklistReadGuard {
    unsafe fn new() -> Self {
        lupos_process_tasklist_read_lock();
        Self
    }
}

impl Drop for TasklistReadGuard {
    fn drop(&mut self) {
        unsafe { lupos_process_tasklist_read_unlock(); }
    }
}

// Exact for_each_process_thread order: exclude init_task, walk each process's
// signal->thread_head, and include every thread (including the group leader).
// The guard covers the entire double loop and each visit. The private callers
// neither retain a task pointer beyond this call nor sleep in their visits.
// C adapters preserve READ_ONCE and the original __list_check_rcu diagnostic;
// all traversal and sentinel decisions are here, not in a C callback loop.
unsafe fn for_each_process_thread(walk: ProcessWalk, mut visit: impl FnMut(*mut task_struct)) {
    let _guard = TasklistReadGuard::new();
    let init = addr_of_mut!(init_task);
    let mut group = lupos_process_next_task(init);
    while group != init {
        let head = addr_of_mut!((*(*group).signal).thread_head);
        match walk {
            ProcessWalk::FreezeTasks => lupos_process_freeze_tasks_check_rcu(),
            ProcessWalk::ShowUnfrozenTasks => lupos_process_show_tasks_check_rcu(),
            ProcessWalk::ThawProcesses => lupos_process_thaw_processes_check_rcu(),
            ProcessWalk::ThawKernelThreads => lupos_process_thaw_kernel_threads_check_rcu(),
        }
        let mut node = lupos_process_list_next_rcu(head);
        while node != head {
            // Never form a task pointer from the thread-head sentinel.
            let task = lupos_process_thread_entry(node);
            visit(task);
            node = lupos_process_list_next_rcu(node);
        }
        group = lupos_process_next_task(group);
    }
}

unsafe fn try_to_freeze_tasks(user_only: bool) -> c_int {
    let what: *const c_char = if user_only {
        b"user space processes\0".as_ptr().cast()
    } else {
        b"remaining freezable tasks\0".as_ptr().cast()
    };
    let mut wq_busy = false;
    let mut wakeup = false;
    let mut sleep_usecs = LUPOS_PROCESS_USEC_PER_MSEC as c_int;

    lupos_process_log_freezing(what);
    let start = lupos_process_ktime_get_boottime();
    let end_time = lupos_process_jiffies()
        .wrapping_add(lupos_process_msecs_to_jiffies(freeze_timeout_msecs));

    if !user_only {
        freeze_workqueues_begin();
    }

    let mut todo: c_uint;
    loop {
        todo = 0;
        for_each_process_thread(ProcessWalk::FreezeTasks, |task| {
            if task != lupos_process_current() && freeze_task(task) {
                todo = todo.wrapping_add(1);
            }
        });

        if !user_only {
            wq_busy = freeze_workqueues_busy();
            todo = todo.wrapping_add(wq_busy as c_uint);
        }

        // Match C's strict time_after, including unsigned-long wraparound.
        // Do not read jiffies if todo is zero, or test wakeup after expiry.
        if todo == 0 || lupos_process_time_after(lupos_process_jiffies(), end_time) {
            break;
        }
        if lupos_process_pm_wakeup_pending() {
            wakeup = true;
            break;
        }

        lupos_process_usleep_range((sleep_usecs / 2) as c_ulong, sleep_usecs as c_ulong);
        if sleep_usecs < 8 * LUPOS_PROCESS_USEC_PER_MSEC as c_int {
            sleep_usecs *= 2;
        }
    }

    let end = lupos_process_ktime_get_boottime();
    let elapsed = end.wrapping_sub(start);
    // ktime_to_ms returns signed 64-bit; C then narrows to unsigned int.
    let elapsed_msecs = lupos_process_ktime_to_ms(elapsed) as c_uint;
    if todo != 0 {
        let result: *const c_char = if wakeup {
            b"aborted\0".as_ptr().cast()
        } else {
            b"failed\0".as_ptr().cast()
        };
        lupos_process_log_failure(what, result, elapsed_msecs / 1000,
            elapsed_msecs % 1000, todo.wrapping_sub(wq_busy as c_uint), wq_busy);

        if wq_busy {
            show_freezable_workqueues();
        }
        if !wakeup || lupos_process_pm_debug_messages_on() {
            for_each_process_thread(ProcessWalk::ShowUnfrozenTasks, |task| {
                if task != lupos_process_current() && lupos_process_freezing(task) && !frozen(task) {
                    sched_show_task(task);
                }
            });
        }
    } else {
        lupos_process_log_complete(what, elapsed_msecs / 1000, elapsed_msecs % 1000);
    }

    if todo != 0 { -(LUPOS_PROCESS_EBUSY as c_int) } else { 0 }
}

/// Signal user space tasks to freeze. The calling task must later thaw them.
/// On failure all tasks are thawed; on success return zero.
#[no_mangle]
pub unsafe extern "C" fn freeze_processes() -> c_int {
    let mut error = __usermodehelper_disable(LUPOS_PROCESS_UMH_FREEZING);
    if error != 0 {
        return error;
    }

    (*lupos_process_current()).flags |= LUPOS_PROCESS_PF_SUSPEND_TASK;
    if !pm_freezing {
        lupos_process_freezer_active_inc();
    }
    lupos_process_pm_wakeup_clear();
    pm_freezing = true;
    error = try_to_freeze_tasks(true);
    if error == 0 {
        __usermodehelper_set_disable_depth(LUPOS_PROCESS_UMH_DISABLED);
    }

    lupos_process_freeze_processes_bug_on_atomic();

    // Preserve the original short-circuit, signed-long timeout and bool result.
    if error == 0
        && !oom_killer_disable(lupos_process_msecs_to_jiffies(freeze_timeout_msecs) as c_long)
    {
        error = -(LUPOS_PROCESS_EBUSY as c_int);
    }
    if error != 0 {
        thaw_processes();
    }
    error
}

/// Freeze remaining kernel tasks. Failure thaws only kernel threads; the
/// caller remains responsible for thawing user space after its own cleanup.
#[no_mangle]
pub unsafe extern "C" fn freeze_kernel_threads() -> c_int {
    pm_nosig_freezing = true;
    let error = try_to_freeze_tasks(false);
    lupos_process_freeze_kernel_threads_bug_on_atomic();
    if error != 0 {
        thaw_kernel_threads();
    }
    error
}

#[no_mangle]
pub unsafe extern "C" fn thaw_processes() {
    let curr = lupos_process_current();

    lupos_process_trace_thaw_begin();
    if pm_freezing {
        lupos_process_freezer_active_dec();
    }
    pm_freezing = false;
    pm_nosig_freezing = false;
    oom_killer_enable();

    lupos_process_log_restart_begin();
    __usermodehelper_set_disable_depth(LUPOS_PROCESS_UMH_FREEZING);
    thaw_workqueues();

    for_each_process_thread(ProcessWalk::ThawProcesses, |task| {
        lupos_process_warn_other_suspend_task(task, curr);
        __thaw_task(task);
    });

    lupos_process_warn_current_not_suspend_task(curr);
    (*curr).flags &= !LUPOS_PROCESS_PF_SUSPEND_TASK;
    lupos_process_usermodehelper_enable();

    schedule();
    lupos_process_log_restart_done();
    lupos_process_trace_thaw_end();
}

#[no_mangle]
pub unsafe extern "C" fn thaw_kernel_threads() {
    pm_nosig_freezing = false;
    lupos_process_log_restart_kernel_begin();
    thaw_workqueues();

    for_each_process_thread(ProcessWalk::ThawKernelThreads, |task| {
        if (*task).flags & LUPOS_PROCESS_PF_KTHREAD != 0 {
            __thaw_task(task);
        }
    });

    schedule();
    lupos_process_log_restart_kernel_done();
}

// SOURCE-COMMIT: 84c953500b7e02473337375b4ffeb610a6e455fd
