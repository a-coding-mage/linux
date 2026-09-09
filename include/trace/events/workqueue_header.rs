/* SPDX-License-Identifier: GPL-2.0 */
// TRACE_SYSTEM: workqueue
// C header guard: _TRACE_WORKQUEUE_H (also available for multi-read inclusion)
// C dependencies: linux/tracepoint.h, linux/workqueue.h, trace/define_trace.h

use core::ffi::c_void;

// External types supplied by the Linux workqueue/tracepoint dependencies.
#[repr(C)]
pub struct pool_workqueue {
    pub wq: *mut workqueue_struct,
    pub pool: *mut worker_pool,
}

#[repr(C)]
pub struct workqueue_struct {
    pub name: *const core::ffi::c_char,
}

#[repr(C)]
pub struct worker_pool {
    pub cpu: i32,
}

#[repr(C)]
pub struct work_struct {
    pub func: *mut c_void,
}

pub type work_func_t = *mut c_void;

#[repr(C)]
pub struct WorkqueueQueueWorkEntry {
    pub work: *mut c_void,
    pub function: *mut c_void,
    pub workqueue: *const core::ffi::c_char,
    pub req_cpu: i32,
    pub cpu: i32,
}

/// workqueue_queue_work - called when a work gets queued
/// @req_cpu: the requested cpu
/// @pwq: pointer to struct pool_workqueue
/// @work: pointer to struct work_struct
///
/// This event occurs when a work is queued immediately or once a delayed work
/// is actually queued on a workqueue (ie: once the delay has been reached).
#[inline]
pub unsafe fn workqueue_queue_work(
    req_cpu: i32,
    pwq: *mut pool_workqueue,
    work: *mut work_struct,
) -> WorkqueueQueueWorkEntry {
    WorkqueueQueueWorkEntry {
        work: work.cast(),
        function: (*work).func,
        workqueue: (*(*pwq).wq).name,
        req_cpu,
        cpu: (*(*pwq).pool).cpu,
    }
}

#[repr(C)]
pub struct WorkqueueActivateWorkEntry {
    pub work: *mut c_void,
    pub function: *mut c_void,
}

/// workqueue_activate_work - called when a work gets activated
/// @work: pointer to struct work_struct
///
/// This event occurs when a queued work is put on the active queue, which
/// happens immediately after queueing unless @max_active limit is reached.
#[inline]
pub unsafe fn workqueue_activate_work(work: *mut work_struct) -> WorkqueueActivateWorkEntry {
    WorkqueueActivateWorkEntry {
        work: work.cast(),
        function: (*work).func,
    }
}

/// workqueue_execute_start - called immediately before the workqueue callback
/// @work: pointer to struct work_struct
///
/// Allows to track workqueue execution.
#[inline]
pub unsafe fn workqueue_execute_start(work: *mut work_struct) -> WorkqueueActivateWorkEntry {
    WorkqueueActivateWorkEntry {
        work: work.cast(),
        function: (*work).func,
    }
}

/// workqueue_execute_end - called immediately after the workqueue callback
/// @work: pointer to struct work_struct
/// @function: pointer to worker function
///
/// Allows to track workqueue execution.
#[inline]
pub unsafe fn workqueue_execute_end(
    work: *mut work_struct,
    function: work_func_t,
) -> WorkqueueActivateWorkEntry {
    WorkqueueActivateWorkEntry {
        work: work.cast(),
        function,
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
