// SPDX-License-Identifier: GPL-2.0

#[repr(C)]
pub struct io_sq_data {
    pub refs: refcount_t,
    pub park_pending: atomic_t,
    pub lock: mutex,

    /* ctx's that are using this sqd */
    pub ctx_list: list_head,

    pub thread: *mut task_struct,
    pub wait: wait_queue_head,

    pub sq_thread_idle: ::core::ffi::c_uint,
    pub sq_cpu: ::core::ffi::c_int,
    pub task_pid: pid_t,
    pub task_tgid: pid_t,

    pub work_time: u64,
    pub state: ::core::ffi::c_ulong,
    pub exited: completion,
}

extern "C" {
    pub fn io_sq_offload_create(
        ctx: *mut io_ring_ctx,
        p: *mut io_uring_params,
    ) -> ::core::ffi::c_int;
    pub fn io_sq_thread_finish(ctx: *mut io_ring_ctx);
    pub fn io_sq_thread_stop(sqd: *mut io_sq_data);
    pub fn io_sq_thread_park(sqd: *mut io_sq_data);
    pub fn io_sq_thread_unpark(sqd: *mut io_sq_data);
    pub fn io_put_sq_data(sqd: *mut io_sq_data);
    pub fn io_sqpoll_wait_sq(ctx: *mut io_ring_ctx);
    pub fn io_sqpoll_wq_cpu_affinity(
        ctx: *mut io_ring_ctx,
        mask: cpumask_var_t,
    ) -> ::core::ffi::c_int;
    pub fn io_sq_cpu_usec(tsk: *mut task_struct) -> u64;
}

#[inline]
pub unsafe fn sqpoll_task_locked(sqd: *mut io_sq_data) -> *mut task_struct {
    rcu_dereference_protected(
        (*sqd).thread,
        lockdep_is_held(&(*sqd).lock),
    )
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
