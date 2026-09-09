// SPDX-License-Identifier: GPL-2.0
/*
 * wq_stall - Test module for the workqueue stall detector.
 *
 * Deliberately creates a workqueue stall so the watchdog fires and
 * prints diagnostic output.  Useful for verifying that the stall
 * detector correctly identifies stuck workers and produces useful
 * backtraces.
 *
 * The stall is triggered by clearing PF_WQ_WORKER before sleeping,
 * which hides the worker from the concurrency manager.  A second
 * work item queued on the same pool then sits in the worklist with
 * no worker available to process it.
 *
 * After ~30s the workqueue watchdog fires:
 *   BUG: workqueue lockup - pool cpus=N ...
 *
 * Build:
 *   make -C <kernel tree> M=samples/workqueue/stall_detector modules
 *
 * Copyright (c) 2026 Meta Platforms, Inc. and affiliates.
 * Copyright (c) 2026 Breno Leitao <leitao@debian.org>
 */

use core::ffi::{c_char, c_int};

// C headers and kernel-provided symbols are external dependencies of this translation.
#[repr(C)]
pub struct work_struct {
    _private: [u8; 0],
}
#[repr(C)]
pub struct wait_queue_head {
    _private: [u8; 0],
}
#[repr(C)]
pub struct atomic_t {
    pub counter: c_int,
}
#[repr(C)]
pub struct task_struct {
    pub flags: usize,
}

extern "C" {
    fn pr_info(fmt: *const c_char, ...);
    fn init_work(work: *mut work_struct, function: unsafe extern "C" fn(*mut work_struct));
    fn schedule_work(work: *mut work_struct) -> c_int;
    fn wait_event_idle(head: *mut wait_queue_head, condition: bool);
    fn atomic_read(v: *const atomic_t) -> c_int;
    fn atomic_set(v: *mut atomic_t, value: c_int);
    fn wake_up(head: *mut wait_queue_head);
    fn flush_work(work: *mut work_struct);
    fn raw_smp_processor_id() -> c_int;
    static mut current: *mut task_struct;
}

const PF_WQ_WORKER: usize = 1usize << 26;

static mut stall_wq_head: wait_queue_head = wait_queue_head { _private: [] };
static mut wake_condition: atomic_t = atomic_t { counter: 0 };
static mut stall_work1: work_struct = work_struct { _private: [] };
static mut stall_work2: work_struct = work_struct { _private: [] };

unsafe extern "C" fn stall_work2_fn(_work: *mut work_struct) {
    pr_info(b"wq_stall: second work item finally ran\n\0".as_ptr() as *const c_char);
}

unsafe extern "C" fn stall_work1_fn(_work: *mut work_struct) {
    pr_info(
        b"wq_stall: first work item running on cpu %d\n\0".as_ptr() as *const c_char,
        raw_smp_processor_id(),
    );

    /*
     * Queue second item while we're still counted as running
     * (pool->nr_running > 0).  Since schedule_work() on a per-CPU
     * workqueue targets raw_smp_processor_id(), item 2 lands on the
     * same pool.  __queue_work -> kick_pool -> need_more_worker()
     * sees nr_running > 0 and does NOT wake a new worker.
     */
    schedule_work(&raw mut stall_work2);

    /*
     * Hide from the workqueue concurrency manager.  Without
     * PF_WQ_WORKER, schedule() won't call wq_worker_sleeping(),
     * so nr_running is never decremented and no replacement
     * worker is created.  Item 2 stays stuck in pool->worklist.
     */
    (*current).flags &= !PF_WQ_WORKER;

    pr_info(b"wq_stall: entering wait_event_idle (PF_WQ_WORKER cleared)\n\0".as_ptr() as *const c_char);
    pr_info(b"wq_stall: expect 'BUG: workqueue lockup' in ~30-60s\n\0".as_ptr() as *const c_char);
    wait_event_idle(&raw mut stall_wq_head, atomic_read(&raw const wake_condition) != 0);

    /* Restore so process_one_work() cleanup works correctly */
    (*current).flags |= PF_WQ_WORKER;
    pr_info(b"wq_stall: woke up, PF_WQ_WORKER restored\n\0".as_ptr() as *const c_char);
}

pub unsafe extern "C" fn wq_stall_init() -> c_int {
    pr_info(b"wq_stall: loading\n\0".as_ptr() as *const c_char);

    init_work(&raw mut stall_work1, stall_work1_fn);
    init_work(&raw mut stall_work2, stall_work2_fn);
    schedule_work(&raw mut stall_work1);

    0
}

pub unsafe extern "C" fn wq_stall_exit() {
    pr_info(b"wq_stall: unloading\n\0".as_ptr() as *const c_char);
    atomic_set(&raw mut wake_condition, 1);
    wake_up(&raw mut stall_wq_head);
    flush_work(&raw mut stall_work1);
    flush_work(&raw mut stall_work2);
    pr_info(b"wq_stall: all work flushed, module unloaded\n\0".as_ptr() as *const c_char);
}

// module_init(wq_stall_init);
// module_exit(wq_stall_exit);
// MODULE_LICENSE("GPL");
// MODULE_DESCRIPTION("Reproduce workqueue stall caused by PF_WQ_WORKER misuse");
// MODULE_AUTHOR("Breno Leitao <leitao@debian.org>");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
