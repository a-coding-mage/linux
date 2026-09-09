/* SPDX-License-Identifier: GPL-2.0 */
/*
 * linux/include/linux/sunrpc/sched.h
 *
 * Scheduling primitives for kernel Sun RPC.
 *
 * Copyright (C) 1996, Olaf Kirch <okir@monad.swb.de>
 */

/* C header dependencies are supplied by other translated files. */

/* This is the actual RPC procedure call info. */
pub struct rpc_procinfo;

#[repr(C)]
pub struct rpc_message {
    pub rpc_proc: *const rpc_procinfo,
    pub rpc_argp: *mut core::ffi::c_void,
    pub rpc_resp: *mut core::ffi::c_void,
    pub rpc_cred: *const cred,
}

pub struct rpc_call_ops;
pub struct rpc_wait_queue;

#[repr(C)]
pub struct rpc_wait {
    pub list: list_head,
    pub links: list_head,
    pub timer_list: list_head,
}

#[repr(C)]
pub struct rpc_timeout {
    pub to_initval: c_ulong,
    pub to_maxval: c_ulong,
    pub to_increment: c_ulong,
    pub to_retries: c_uint,
    pub to_exponential: u8,
}

#[repr(C)]
pub union rpc_task_u {
    pub tk_work: work_struct,
    pub tk_wait: rpc_wait,
}

#[repr(C)]
pub struct rpc_task {
    pub tk_count: atomic_t,
    pub tk_status: c_int,
    pub tk_task: list_head,
    pub tk_callback: Option<unsafe extern "C" fn(*mut rpc_task)>,
    pub tk_action: Option<unsafe extern "C" fn(*mut rpc_task)>,
    pub tk_timeout: c_ulong,
    pub tk_runstate: c_ulong,
    pub tk_waitqueue: *mut rpc_wait_queue,
    pub u: rpc_task_u,
    pub tk_msg: rpc_message,
    pub tk_calldata: *mut core::ffi::c_void,
    pub tk_ops: *const rpc_call_ops,
    pub tk_client: *mut rpc_clnt,
    pub tk_xprt: *mut rpc_xprt,
    pub tk_op_cred: *mut rpc_cred,
    pub tk_rqstp: *mut rpc_rqst,
    pub tk_workqueue: *mut workqueue_struct,
    pub tk_start: ktime_t,
    pub tk_owner: pid_t,
    pub tk_rpc_status: c_int,
    pub tk_flags: u16,
    pub tk_timeouts: u16,
    pub tk_pid: u16,
    /* C bitfields: tk_priority:2, tk_garb_retry:2, tk_cred_retry:2. */
    pub tk_priority: u8,
    pub tk_garb_retry: u8,
    pub tk_cred_retry: u8,
}

pub type rpc_action = unsafe extern "C" fn(*mut rpc_task);

#[repr(C)]
pub struct rpc_call_ops {
    pub rpc_call_prepare: Option<unsafe extern "C" fn(*mut rpc_task, *mut core::ffi::c_void)>,
    pub rpc_call_done: Option<unsafe extern "C" fn(*mut rpc_task, *mut core::ffi::c_void)>,
    pub rpc_count_stats: Option<unsafe extern "C" fn(*mut rpc_task, *mut core::ffi::c_void)>,
    pub rpc_release: Option<unsafe extern "C" fn(*mut core::ffi::c_void)>,
}

#[repr(C)]
pub struct rpc_task_setup {
    pub task: *mut rpc_task,
    pub rpc_client: *mut rpc_clnt,
    pub rpc_xprt: *mut rpc_xprt,
    pub rpc_op_cred: *mut rpc_cred,
    pub rpc_message: *const rpc_message,
    pub callback_ops: *const rpc_call_ops,
    pub callback_data: *mut core::ffi::c_void,
    pub workqueue: *mut workqueue_struct,
    pub flags: u16,
    pub priority: i8,
}

pub const RPC_TASK_ASYNC: c_int = 0x0001;
pub const RPC_TASK_SWAPPER: c_int = 0x0002;
pub const RPC_TASK_MOVEABLE: c_int = 0x0004;
pub const RPC_TASK_NULLCREDS: c_int = 0x0010;
pub const RPC_CALL_MAJORSEEN: c_int = 0x0020;
pub const RPC_TASK_NETUNREACH_FATAL: c_int = 0x0040;
pub const RPC_TASK_DYNAMIC: c_int = 0x0080;
pub const RPC_TASK_NO_ROUND_ROBIN: c_int = 0x0100;
pub const RPC_TASK_SOFT: c_int = 0x0200;
pub const RPC_TASK_SOFTCONN: c_int = 0x0400;
pub const RPC_TASK_SENT: c_int = 0x0800;
pub const RPC_TASK_TIMEOUT: c_int = 0x1000;
pub const RPC_TASK_NOCONNECT: c_int = 0x2000;
pub const RPC_TASK_NO_RETRANS_TIMEOUT: c_int = 0x4000;
pub const RPC_TASK_CRED_NOREF: c_int = 0x8000;

#[inline]
pub unsafe fn RPC_IS_ASYNC(t: *const rpc_task) -> u16 { (*t).tk_flags & RPC_TASK_ASYNC as u16 }
#[inline]
pub unsafe fn RPC_IS_SWAPPER(t: *const rpc_task) -> u16 { (*t).tk_flags & RPC_TASK_SWAPPER as u16 }
#[inline]
pub unsafe fn RPC_IS_SOFT(t: *const rpc_task) -> u16 { (*t).tk_flags & (RPC_TASK_SOFT | RPC_TASK_TIMEOUT) as u16 }
#[inline]
pub unsafe fn RPC_IS_SOFTCONN(t: *const rpc_task) -> u16 { (*t).tk_flags & RPC_TASK_SOFTCONN as u16 }
#[inline]
pub unsafe fn RPC_WAS_SENT(t: *const rpc_task) -> u16 { (*t).tk_flags & RPC_TASK_SENT as u16 }
#[inline]
pub unsafe fn RPC_IS_MOVEABLE(t: *const rpc_task) -> u16 { (*t).tk_flags & RPC_TASK_MOVEABLE as u16 }

pub const RPC_TASK_RUNNING: c_uint = 0;
pub const RPC_TASK_QUEUED: c_uint = 1;
pub const RPC_TASK_ACTIVE: c_uint = 2;
pub const RPC_TASK_NEED_XMIT: c_uint = 3;
pub const RPC_TASK_NEED_RECV: c_uint = 4;
pub const RPC_TASK_MSG_PIN_WAIT: c_uint = 5;

/* Bit operations and READ_ONCE are supplied by the kernel translation. */
extern "C" {
    pub fn rpc_test_and_set_running(task: *mut rpc_task) -> c_ulong;
    pub fn rpc_clear_running(task: *mut rpc_task);
    pub fn RPC_IS_QUEUED(task: *const rpc_task) -> bool;
    pub fn rpc_set_queued(task: *mut rpc_task);
    pub fn rpc_clear_queued(task: *mut rpc_task);
    pub fn RPC_IS_ACTIVATED(task: *const rpc_task) -> bool;
}

#[inline]
pub unsafe fn RPC_SIGNALLED(task: *const rpc_task) -> bool {
    (*task).tk_rpc_status == -512 /* -ERESTARTSYS */
}

pub const RPC_PRIORITY_LOW: c_int = -1;
pub const RPC_PRIORITY_NORMAL: c_int = 0;
pub const RPC_PRIORITY_HIGH: c_int = 1;
pub const RPC_PRIORITY_PRIVILEGED: c_int = 2;
pub const RPC_NR_PRIORITY: usize = 4;

#[repr(C)]
pub struct rpc_timer {
    pub list: list_head,
    pub expires: c_ulong,
    pub dwork: delayed_work,
}

#[repr(C)]
pub struct rpc_wait_queue {
    pub lock: spinlock_t,
    pub tasks: [list_head; RPC_NR_PRIORITY],
    pub maxpriority: u8,
    pub priority: u8,
    pub nr: u8,
    pub qlen: c_uint,
    pub timer_list: rpc_timer,
    /* Present when CONFIG_SUNRPC_DEBUG or CONFIG_TRACEPOINTS is enabled. */
    pub name: *const c_char,
}

#[inline]
pub unsafe fn RPC_IS_PRIORITY(q: *const rpc_wait_queue) -> bool { (*q).maxpriority > 0 }

extern "C" {
    pub fn rpc_new_task(setup: *const rpc_task_setup) -> *mut rpc_task;
    pub fn rpc_run_task(setup: *const rpc_task_setup) -> *mut rpc_task;
    pub fn rpc_run_bc_task(req: *mut rpc_rqst, timeout: *mut rpc_timeout) -> *mut rpc_task;
    pub fn rpc_put_task(task: *mut rpc_task);
    pub fn rpc_put_task_async(task: *mut rpc_task);
    pub fn rpc_task_set_rpc_status(task: *mut rpc_task, rpc_status: c_int) -> bool;
    pub fn rpc_task_try_cancel(task: *mut rpc_task, error: c_int);
    pub fn rpc_signal_task(task: *mut rpc_task);
    pub fn rpc_exit_task(task: *mut rpc_task);
    pub fn rpc_exit(task: *mut rpc_task, status: c_int);
    pub fn rpc_release_calldata(ops: *const rpc_call_ops, data: *mut core::ffi::c_void);
    pub fn rpc_killall_tasks(clnt: *mut rpc_clnt);
    pub fn rpc_cancel_tasks(clnt: *mut rpc_clnt, error: c_int,
                            fnmatch: Option<unsafe extern "C" fn(*const rpc_task, *const core::ffi::c_void) -> bool>,
                            data: *const core::ffi::c_void) -> c_ulong;
    pub fn rpc_execute(task: *mut rpc_task);
    pub fn rpc_init_priority_wait_queue(queue: *mut rpc_wait_queue, name: *const c_char);
    pub fn rpc_init_wait_queue(queue: *mut rpc_wait_queue, name: *const c_char);
    pub fn rpc_destroy_wait_queue(queue: *mut rpc_wait_queue);
    pub fn rpc_task_timeout(task: *const rpc_task) -> c_ulong;
    pub fn rpc_sleep_on_timeout(queue: *mut rpc_wait_queue, task: *mut rpc_task,
                                action: rpc_action, timeout: c_ulong);
    pub fn rpc_sleep_on(queue: *mut rpc_wait_queue, task: *mut rpc_task, action: rpc_action);
    pub fn rpc_sleep_on_priority_timeout(queue: *mut rpc_wait_queue, task: *mut rpc_task,
                                         timeout: c_ulong, priority: c_int);
    pub fn rpc_sleep_on_priority(queue: *mut rpc_wait_queue, task: *mut rpc_task,
                                 priority: c_int);
    pub fn rpc_wake_up_queued_task(queue: *mut rpc_wait_queue, task: *mut rpc_task);
    pub fn rpc_wake_up_queued_task_set_status(queue: *mut rpc_wait_queue, task: *mut rpc_task,
                                              status: c_int);
    pub fn rpc_wake_up(queue: *mut rpc_wait_queue);
    pub fn rpc_wake_up_next(queue: *mut rpc_wait_queue) -> *mut rpc_task;
    pub fn rpc_wake_up_first_on_wq(wq: *mut workqueue_struct, queue: *mut rpc_wait_queue,
                                   predicate: Option<unsafe extern "C" fn(*mut rpc_task, *mut core::ffi::c_void) -> bool>,
                                   data: *mut core::ffi::c_void) -> *mut rpc_task;
    pub fn rpc_wake_up_first(queue: *mut rpc_wait_queue,
                             predicate: Option<unsafe extern "C" fn(*mut rpc_task, *mut core::ffi::c_void) -> bool>,
                             data: *mut core::ffi::c_void) -> *mut rpc_task;
    pub fn rpc_wake_up_status(queue: *mut rpc_wait_queue, status: c_int);
    pub fn rpc_delay(task: *mut rpc_task, delay: c_ulong);
    pub fn rpc_malloc(task: *mut rpc_task) -> c_int;
    pub fn rpc_free(task: *mut rpc_task);
    pub fn rpciod_up() -> c_int;
    pub fn rpciod_down();
    pub fn rpc_wait_for_completion_task(task: *mut rpc_task) -> c_int;
    pub fn rpc_init_mempool() -> c_int;
    pub fn rpc_destroy_mempool();
    pub static mut rpciod_workqueue: *mut workqueue_struct;
    pub static mut xprtiod_workqueue: *mut workqueue_struct;
    pub fn rpc_prepare_task(task: *mut rpc_task);
    pub fn rpc_task_gfp_mask() -> gfp_t;
}

#[cfg(any(feature = "CONFIG_SUNRPC_DEBUG", feature = "CONFIG_TRACEPOINTS"))]
#[inline]
pub unsafe fn rpc_qname(q: *const rpc_wait_queue) -> *const c_char {
    if !q.is_null() && !(*q).name.is_null() { (*q).name } else { b"unknown\0".as_ptr() as *const c_char }
}

#[cfg(any(feature = "CONFIG_SUNRPC_DEBUG", feature = "CONFIG_TRACEPOINTS"))]
#[inline]
pub unsafe fn rpc_assign_waitqueue_name(q: *mut rpc_wait_queue, name: *const c_char) { (*q).name = name; }

#[cfg(not(any(feature = "CONFIG_SUNRPC_DEBUG", feature = "CONFIG_TRACEPOINTS")))]
#[inline]
pub unsafe fn rpc_assign_waitqueue_name(_q: *mut rpc_wait_queue, _name: *const c_char) {}

#[cfg(feature = "CONFIG_SUNRPC_SWAP")]
extern "C" {
    pub fn rpc_clnt_swap_activate(clnt: *mut rpc_clnt) -> c_int;
    pub fn rpc_clnt_swap_deactivate(clnt: *mut rpc_clnt);
}

#[cfg(not(feature = "CONFIG_SUNRPC_SWAP"))]
#[inline]
pub unsafe fn rpc_clnt_swap_activate(_clnt: *mut rpc_clnt) -> c_int { -22 }

#[cfg(not(feature = "CONFIG_SUNRPC_SWAP"))]
#[inline]
pub unsafe fn rpc_clnt_swap_deactivate(_clnt: *mut rpc_clnt) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
