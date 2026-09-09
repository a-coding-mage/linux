/* SPDX-License-Identifier: GPL-2.0 */
/*
 * kernel/workqueue_internal.h
 *
 * Workqueue internal header file.  Only to be included by workqueue and
 * core kernel subsystems.
 */

// Dependency intent: linux/workqueue.h, linux/kthread.h, and linux/preempt.h
// provide the types, constants, and functions referenced below.

use core::mem::ManuallyDrop;

pub struct worker_pool;

/*
 * The poor guys doing the actual heavy lifting.  All on-duty workers are
 * either serving the manager role, on idle list or on busy hash.  For
 * details on the locking annotation (L, I, X...), refer to workqueue.c.
 *
 * Only to be used in workqueue and async.
 */
#[repr(C)]
pub union worker_entry {
	pub entry: ManuallyDrop<list_head>,
	pub hentry: ManuallyDrop<hlist_node>,
}

#[repr(C)]
pub struct worker {
	/* on idle list while idle, on busy hash table while busy */
	pub entry: worker_entry,

	pub current_work: *mut work_struct, /* K: work being processed and its */
	pub current_func: work_func_t,      /* K: function */
	pub current_pwq: *mut pool_workqueue, /* K: pwq */
	pub current_at: u64,                /* K: runtime at start or last wakeup */
	pub current_start: c_ulong,         /* K: start time of current work item */
	pub current_color: c_uint,          /* K: color */

	pub sleeping: c_int,                /* S: is worker sleeping? */

	/* used by the scheduler to determine a worker's last known identity */
	pub last_func: work_func_t,         /* K: last work's fn */

	pub scheduled: list_head,           /* L: scheduled works */

	pub task: *mut task_struct,          /* I: worker task */
	pub pool: *mut worker_pool,          /* A: the associated pool */
	                                      /* L: for rescuers */
	pub node: list_head,                 /* A: anchored at pool->workers */
	                                      /* A: runs through worker->node */

	pub last_active: c_ulong,            /* K: last active timestamp */
	pub flags: c_uint,                   /* L: flags */
	pub id: c_int,                       /* I: worker id */

	/*
	 * Opaque string set with work_set_desc().  Printed out with task
	 * dump for debugging - WARN, BUG, panic or sysrq.
	 */
	pub desc: [c_char; WORKER_DESC_LEN],

	/* used only by rescuers to point to the target workqueue */
	pub rescue_wq: *mut workqueue_struct, /* I: the workqueue to rescue */
}

/**
 * current_wq_worker - return struct worker if %current is a workqueue worker
 */
#[inline]
pub unsafe fn current_wq_worker() -> *mut worker {
	if in_task() && ((*current).flags & PF_WQ_WORKER) != 0 {
		return kthread_data(current);
	}
	core::ptr::null_mut()
}

/*
 * Scheduler hooks for concurrency managed workqueue.  Only to be used from
 * sched/ and workqueue.c.
 */
extern "C" {
	pub fn wq_worker_running(task: *mut task_struct);
	pub fn wq_worker_sleeping(task: *mut task_struct);
	pub fn wq_worker_tick(task: *mut task_struct);
	pub fn wq_worker_last_func(task: *mut task_struct) -> work_func_t;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
