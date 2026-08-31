/* SPDX-License-Identifier: GPL-2.0 */
/*
 * A minimal userland scheduler.
 *
 * In terms of scheduling, this provides two different types of behaviors:
 * 1. A global FIFO scheduling order for _any_ tasks that have CPU affinity.
 *    All such tasks are direct-dispatched from the kernel, and are never
 *    enqueued in user space.
 * 2. A primitive vruntime scheduler that is implemented in user space, for all
 *    other tasks.
 *
 * Some parts of this example user space scheduler could be implemented more
 * efficiently using more complex and sophisticated data structures. For
 * example, rather than using BPF_MAP_TYPE_QUEUE's,
 * BPF_MAP_TYPE_{USER_}RINGBUF's could be used for exchanging messages between
 * user space and kernel space. Similarly, we use a simple vruntime-sorted list
 * in user space, but an rbtree could be used instead.
 *
 * Copyright (c) 2022 Meta Platforms, Inc. and affiliates.
 * Copyright (c) 2022 Tejun Heo <tj@kernel.org>
 * Copyright (c) 2022 David Vernet <dvernet@meta.com>
 */

// Dependencies from <scx/common.bpf.h> and "scx_userland.h" are expected to be
// supplied by the BPF build environment.

/*
 * Maximum amount of tasks enqueued/dispatched between kernel and user-space.
 */
pub const MAX_ENQUEUED_TASKS: u32 = 4096;

#[link_section = "license"]
#[no_mangle]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[no_mangle]
pub static usersched_pid: core::ffi::c_int = 0;

/* !0 for veristat, set during init */
#[no_mangle]
pub static num_possible_cpus: u32 = 64;

/* Stats that are printed by user space. */
#[no_mangle]
pub static mut nr_failed_enqueues: u64 = 0;
#[no_mangle]
pub static mut nr_kernel_enqueues: u64 = 0;
#[no_mangle]
pub static mut nr_user_enqueues: u64 = 0;

/*
 * Number of tasks that are queued for scheduling.
 *
 * This number is incremented by the BPF component when a task is queued to the
 * user-space scheduler and it must be decremented by the user-space scheduler
 * when a task is consumed.
 */
#[no_mangle]
pub static mut nr_queued: u64 = 0;

/*
 * Number of tasks that are waiting for scheduling.
 *
 * This number must be updated by the user-space scheduler to keep track if
 * there is still some scheduling work to do.
 */
#[no_mangle]
pub static mut nr_scheduled: u64 = 0;

UEI_DEFINE!(uei);

/*
 * The map containing tasks that are enqueued in user space from the kernel.
 *
 * This map is drained by the user space scheduler.
 */
#[repr(C)]
pub struct EnqueuedMap {
    pub type_: u32,
    pub max_entries: u32,
    pub value: scx_userland_enqueued_task,
}

#[link_section = ".maps"]
#[no_mangle]
pub static mut enqueued: EnqueuedMap = EnqueuedMap {
    type_: BPF_MAP_TYPE_QUEUE,
    max_entries: MAX_ENQUEUED_TASKS,
    value: unsafe { core::mem::zeroed() },
};

/*
 * The map containing tasks that are dispatched to the kernel from user space.
 *
 * Drained by the kernel in userland_dispatch().
 */
#[repr(C)]
pub struct DispatchedMap {
    pub type_: u32,
    pub max_entries: u32,
    pub value: core::ffi::c_int,
}

#[link_section = ".maps"]
#[no_mangle]
pub static mut dispatched: DispatchedMap = DispatchedMap {
    type_: BPF_MAP_TYPE_QUEUE,
    max_entries: MAX_ENQUEUED_TASKS,
    value: 0,
};

/* Per-task scheduling context */
#[repr(C)]
pub struct task_ctx {
    pub force_local: bool, /* Dispatch directly to local DSQ */
}

/* Map that contains task-local storage. */
#[repr(C)]
pub struct TaskCtxStorMap {
    pub type_: u32,
    pub map_flags: u32,
    pub key: core::ffi::c_int,
    pub value: task_ctx,
}

#[link_section = ".maps"]
#[no_mangle]
pub static mut task_ctx_stor: TaskCtxStorMap = TaskCtxStorMap {
    type_: BPF_MAP_TYPE_TASK_STORAGE,
    map_flags: BPF_F_NO_PREALLOC,
    key: 0,
    value: task_ctx { force_local: false },
};

/*
 * Flag used to wake-up the user-space scheduler.
 */
static mut usersched_needed: u32 = 0;

/*
 * Set user-space scheduler wake-up flag (equivalent to an atomic release
 * operation).
 */
unsafe fn set_usersched_needed() {
    __sync_fetch_and_or(&raw mut usersched_needed, 1);
}

/*
 * Check and clear user-space scheduler wake-up flag (equivalent to an atomic
 * acquire operation).
 */
unsafe fn test_and_clear_usersched_needed() -> bool {
    __sync_fetch_and_and(&raw mut usersched_needed, 0) == 1
}

unsafe fn is_usersched_task(p: *const task_struct) -> bool {
    (*p).pid == usersched_pid
}

unsafe fn keep_in_kernel(p: *const task_struct) -> bool {
    (*p).nr_cpus_allowed < num_possible_cpus
}

unsafe fn usersched_task() -> *mut task_struct {
    let p: *mut task_struct;

    p = bpf_task_from_pid(usersched_pid);
    /*
     * Should never happen -- the usersched task should always be managed
     * by sched_ext.
     */
    if p.is_null() {
        scx_bpf_error!("Failed to find usersched task %d", usersched_pid);
    }

    p
}

#[no_mangle]
pub unsafe extern "C" fn userland_select_cpu(
    p: *mut task_struct,
    prev_cpu: core::ffi::c_int,
    wake_flags: u64,
) -> core::ffi::c_int {
    let _ = wake_flags;

    if keep_in_kernel(p) {
        let cpu: core::ffi::c_int;
        let tctx: *mut task_ctx;

        tctx = bpf_task_storage_get(&raw mut task_ctx_stor, p, 0, 0);
        if tctx.is_null() {
            scx_bpf_error!("Failed to look up task-local storage for %s", (*p).comm);
            return -ESRCH;
        }

        if (*p).nr_cpus_allowed == 1 || scx_bpf_test_and_clear_cpu_idle(prev_cpu) {
            (*tctx).force_local = true;
            return prev_cpu;
        }

        cpu = scx_bpf_pick_idle_cpu((*p).cpus_ptr, 0);
        if cpu >= 0 {
            (*tctx).force_local = true;
            return cpu;
        }
    }

    prev_cpu
}

unsafe fn dispatch_user_scheduler() {
    let p: *mut task_struct;

    p = usersched_task();
    if !p.is_null() {
        scx_bpf_dsq_insert(p, SCX_DSQ_GLOBAL, SCX_SLICE_DFL, 0);
        bpf_task_release(p);
    }
}

unsafe fn enqueue_task_in_user_space(p: *mut task_struct, enq_flags: u64) {
    let mut task: scx_userland_enqueued_task = core::mem::zeroed();

    task.pid = (*p).pid;
    task.sum_exec_runtime = (*p).se.sum_exec_runtime;
    task.weight = (*p).scx.weight;

    if bpf_map_push_elem(&raw mut enqueued, &raw const task, 0) != 0 {
        /*
         * If we fail to enqueue the task in user space, put it
         * directly on the global DSQ.
         */
        __sync_fetch_and_add(&raw mut nr_failed_enqueues, 1);
        scx_bpf_dsq_insert(p, SCX_DSQ_GLOBAL, SCX_SLICE_DFL, enq_flags);
    } else {
        __sync_fetch_and_add(&raw mut nr_user_enqueues, 1);
        set_usersched_needed();
    }
}

#[no_mangle]
pub unsafe extern "C" fn userland_enqueue(p: *mut task_struct, enq_flags: u64) {
    if keep_in_kernel(p) {
        let mut dsq_id: u64 = SCX_DSQ_GLOBAL;
        let tctx: *mut task_ctx;

        tctx = bpf_task_storage_get(&raw mut task_ctx_stor, p, 0, 0);
        if tctx.is_null() {
            scx_bpf_error!("Failed to lookup task ctx for %s", (*p).comm);
            return;
        }

        if (*tctx).force_local {
            dsq_id = SCX_DSQ_LOCAL;
        }
        (*tctx).force_local = false;
        scx_bpf_dsq_insert(p, dsq_id, SCX_SLICE_DFL, enq_flags);
        __sync_fetch_and_add(&raw mut nr_kernel_enqueues, 1);
        return;
    } else if !is_usersched_task(p) {
        enqueue_task_in_user_space(p, enq_flags);
    }
}

#[no_mangle]
pub unsafe extern "C" fn userland_dispatch(cpu: core::ffi::c_int, prev: *mut task_struct) {
    let _ = cpu;
    let _ = prev;

    if test_and_clear_usersched_needed() {
        dispatch_user_scheduler();
    }

    for _ in 0..MAX_ENQUEUED_TASKS {
        let mut pid: core::ffi::c_int = 0;
        let p: *mut task_struct;

        if bpf_map_pop_elem(&raw mut dispatched, &raw mut pid) != 0 {
            break;
        }

        /*
         * The task could have exited by the time we get around to
         * dispatching it. Treat this as a normal occurrence, and simply
         * move onto the next iteration.
         */
        p = bpf_task_from_pid(pid);
        if p.is_null() {
            continue;
        }

        scx_bpf_dsq_insert(p, SCX_DSQ_GLOBAL, SCX_SLICE_DFL, 0);
        bpf_task_release(p);
    }
}

/*
 * A CPU is about to change its idle state. If the CPU is going idle, ensure
 * that the user-space scheduler has a chance to run if there is any remaining
 * work to do.
 */
#[no_mangle]
pub unsafe extern "C" fn userland_update_idle(cpu: core::ffi::c_int, idle: bool) {
    /*
     * Don't do anything if we exit from and idle state, a CPU owner will
     * be assigned in .running().
     */
    if !idle {
        return;
    }
    /*
     * A CPU is now available, notify the user-space scheduler that tasks
     * can be dispatched, if there is at least one task waiting to be
     * scheduled, either queued (accounted in nr_queued) or scheduled
     * (accounted in nr_scheduled).
     *
     * NOTE: nr_queued is incremented by the BPF component, more exactly in
     * enqueue(), when a task is sent to the user-space scheduler, then
     * the scheduler drains the queued tasks (updating nr_queued) and adds
     * them to its internal data structures / state; at this point tasks
     * become "scheduled" and the user-space scheduler will take care of
     * updating nr_scheduled accordingly; lastly tasks will be dispatched
     * and the user-space scheduler will update nr_scheduled again.
     *
     * Checking both counters allows to determine if there is still some
     * pending work to do for the scheduler: new tasks have been queued
     * since last check, or there are still tasks "queued" or "scheduled"
     * since the previous user-space scheduler run. If the counters are
     * both zero it is pointless to wake-up the scheduler (even if a CPU
     * becomes idle), because there is nothing to do.
     *
     * Keep in mind that update_idle() doesn't run concurrently with the
     * user-space scheduler (that is single-threaded): this function is
     * naturally serialized with the user-space scheduler code, therefore
     * this check here is also safe from a concurrency perspective.
     */
    if nr_queued != 0 || nr_scheduled != 0 {
        /*
         * Kick the CPU to make it immediately ready to accept
         * dispatched tasks.
         */
        set_usersched_needed();
        scx_bpf_kick_cpu(cpu, 0);
    }
}

#[no_mangle]
pub unsafe extern "C" fn userland_init_task(
    p: *mut task_struct,
    args: *mut scx_init_task_args,
) -> core::ffi::c_int {
    let _ = args;

    if !bpf_task_storage_get(
        &raw mut task_ctx_stor,
        p,
        0,
        BPF_LOCAL_STORAGE_GET_F_CREATE,
    )
    .is_null()
    {
        0
    } else {
        -ENOMEM
    }
}

#[no_mangle]
pub unsafe extern "C" fn userland_init() -> core::ffi::c_int {
    if num_possible_cpus == 0 {
        scx_bpf_error!(
            "User scheduler # CPUs uninitialized (%d)",
            num_possible_cpus
        );
        return -EINVAL;
    }

    if usersched_pid <= 0 {
        scx_bpf_error!("User scheduler pid uninitialized (%d)", usersched_pid);
        return -EINVAL;
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn userland_exit(ei: *mut scx_exit_info) {
    UEI_RECORD!(uei, ei);
}

SCX_OPS_DEFINE!(
    userland_ops,
    select_cpu = userland_select_cpu as *mut core::ffi::c_void,
    enqueue = userland_enqueue as *mut core::ffi::c_void,
    dispatch = userland_dispatch as *mut core::ffi::c_void,
    update_idle = userland_update_idle as *mut core::ffi::c_void,
    init_task = userland_init_task as *mut core::ffi::c_void,
    init = userland_init as *mut core::ffi::c_void,
    exit = userland_exit as *mut core::ffi::c_void,
    flags = SCX_OPS_ENQ_LAST | SCX_OPS_KEEP_BUILTIN_IDLE,
    name = "userland",
);
