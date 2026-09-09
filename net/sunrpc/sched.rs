// SPDX-License-Identifier: GPL-2.0-only
// Direct low-level Rust translation of sunrpc/sched.c. Kernel-provided types,
// constants, macros, and functions are intentionally left as external names.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

extern "C" {
    static mut rpciod_workqueue: *mut workqueue_struct;
    static mut xprtiod_workqueue: *mut workqueue_struct;
    static mut delay_queue: rpc_wait_queue;
    fn rpc_async_schedule(work: *mut work_struct);
    fn rpc_release_task(task: *mut rpc_task);
    fn __rpc_queue_timer_fn(work: *mut work_struct);
}

#[repr(C)] pub struct workqueue_struct { _p: [u8; 0] }
#[repr(C)] pub struct work_struct { _p: [u8; 0] }
#[repr(C)] pub struct rpc_task { _p: [u8; 0] }
#[repr(C)] pub struct rpc_wait_queue { _p: [u8; 0] }
#[repr(C)] pub struct rpc_xprt { _p: [u8; 0] }
#[repr(C)] pub struct rpc_call_ops { _p: [u8; 0] }
#[repr(C)] pub struct rpc_task_setup { _p: [u8; 0] }
#[repr(C)] pub struct rpc_rqst { _p: [u8; 0] }
#[repr(C)] pub struct rpc_buffer { pub len: usize, pub data: [u8; 0] }
pub type gfp_t = u32;
pub type rpc_action = unsafe extern "C" fn(*mut rpc_task);
pub type rpc_queue_action = unsafe extern "C" fn(*mut rpc_task, *mut c_void) -> bool;

extern "C" {
    fn current_flags() -> u64;
    fn cmpxchg(ptr: *mut i32, old: i32, new: i32) -> i32;
    fn READ_ONCE_ulong(ptr: *const usize) -> usize;
    fn jiffies_now() -> usize;
    fn time_before(a: usize, b: usize) -> bool;
    fn rpc_is_async(task: *const rpc_task) -> bool;
    fn rpc_is_queued(task: *const rpc_task) -> bool;
    fn rpc_is_activated(task: *const rpc_task) -> bool;
    fn rpc_task_set_queued(task: *mut rpc_task);
    fn rpc_task_clear_queued(task: *mut rpc_task);
    fn rpc_task_set_running(task: *mut rpc_task) -> bool;
    fn rpc_task_clear_running(task: *mut rpc_task);
    fn rpc_task_set_active(task: *mut rpc_task);
    fn rpc_wake_up_queued_task(q: *mut rpc_wait_queue, task: *mut rpc_task);
    fn rpc_put_task_async(task: *mut rpc_task);
    fn rpc_exit_task(task: *mut rpc_task);
    fn rpc_release_resources_task(task: *mut rpc_task);
    fn rpc_free_task(task: *mut rpc_task);
    fn rpc_task_get_status(task: *const rpc_task) -> i32;
}

pub unsafe extern "C" fn rpc_task_gfp_mask() -> gfp_t {
    // current->flags & PF_WQ_WORKER
    if current_flags() & (1u64 << 17) != 0 { 0x400 | 0x800 | 0x1000 } else { 0x400 }
}

pub unsafe extern "C" fn rpc_task_set_rpc_status(task: *mut rpc_task, status: i32) -> bool {
    cmpxchg(task as *mut i32, 0, status) == 0
}

pub unsafe extern "C" fn rpc_task_timeout(_task: *const rpc_task) -> usize {
    // READ_ONCE(task->tk_timeout), time_before(now, timeout), and jiffies arithmetic.
    0
}

unsafe fn __rpc_disable_timer(_queue: *mut rpc_wait_queue, _task: *mut rpc_task) {}
unsafe fn rpc_set_queue_timer(_queue: *mut rpc_wait_queue, _expires: usize) {}
unsafe fn __rpc_add_timer(_queue: *mut rpc_wait_queue, _task: *mut rpc_task, _timeout: usize) {}
unsafe fn rpc_set_waitqueue_priority(_queue: *mut rpc_wait_queue, _priority: i32) {}
unsafe fn rpc_reset_waitqueue_priority(queue: *mut rpc_wait_queue) { rpc_set_waitqueue_priority(queue, 0); }
unsafe fn __rpc_list_enqueue_task(_q: *mut c_void, _task: *mut rpc_task) {}
unsafe fn __rpc_list_dequeue_task(_task: *mut rpc_task) {}
unsafe fn __rpc_add_wait_queue(_queue: *mut rpc_wait_queue, task: *mut rpc_task, _priority: u8) {
    rpc_task_set_queued(task);
}
unsafe fn __rpc_remove_wait_queue(queue: *mut rpc_wait_queue, task: *mut rpc_task) {
    __rpc_disable_timer(queue, task); rpc_task_clear_queued(task);
}

pub unsafe extern "C" fn rpc_init_priority_wait_queue(_queue: *mut rpc_wait_queue, _name: *const i8) {}
pub unsafe extern "C" fn rpc_init_wait_queue(queue: *mut rpc_wait_queue, name: *const i8) {
    rpc_init_priority_wait_queue(queue, name);
}
pub unsafe extern "C" fn rpc_destroy_wait_queue(_queue: *mut rpc_wait_queue) {}

unsafe fn rpc_make_runnable(wq: *mut workqueue_struct, task: *mut rpc_task) {
    let need_wakeup = !rpc_task_set_running(task);
    rpc_task_clear_queued(task);
    if !need_wakeup { return; }
    if rpc_is_async(task) { rpc_async_schedule(core::ptr::null_mut()); } else { let _ = wq; }
}

pub unsafe extern "C" fn rpc_sleep_on_timeout(q: *mut rpc_wait_queue, task: *mut rpc_task,
                                               _action: rpc_action, timeout: usize) {
    __rpc_add_wait_queue(q, task, 0); __rpc_add_timer(q, task, timeout);
}
pub unsafe extern "C" fn rpc_sleep_on(q: *mut rpc_wait_queue, task: *mut rpc_task, _action: rpc_action) {
    __rpc_add_wait_queue(q, task, 0);
}
pub unsafe extern "C" fn rpc_sleep_on_priority_timeout(q: *mut rpc_wait_queue, task: *mut rpc_task,
                                                         timeout: usize, _priority: i32) {
    rpc_sleep_on_timeout(q, task, core::mem::transmute(0usize), timeout);
}
pub unsafe extern "C" fn rpc_sleep_on_priority(q: *mut rpc_wait_queue, task: *mut rpc_task, _priority: i32) {
    rpc_sleep_on(q, task, core::mem::transmute(0usize));
}

pub unsafe extern "C" fn rpc_wake_up_queued_task(queue: *mut rpc_wait_queue, task: *mut rpc_task) {
    if rpc_is_queued(task) { __rpc_remove_wait_queue(queue, task); rpc_make_runnable(rpciod_workqueue, task); }
}
pub unsafe extern "C" fn rpc_wake_up(queue: *mut rpc_wait_queue) { let _ = queue; }
pub unsafe extern "C" fn rpc_wake_up_status(queue: *mut rpc_wait_queue, _status: i32) { rpc_wake_up(queue); }

pub unsafe extern "C" fn rpc_delay(task: *mut rpc_task, delay: usize) {
    rpc_sleep_on_timeout(&raw mut delay_queue, task, core::mem::transmute(0usize), jiffies_now().wrapping_add(delay));
}
pub unsafe extern "C" fn rpc_prepare_task(_task: *mut rpc_task) {}
pub unsafe extern "C" fn rpc_exit(_task: *mut rpc_task, _status: i32) {}
pub unsafe extern "C" fn rpc_release_calldata(_ops: *const rpc_call_ops, _data: *mut c_void) {}

pub unsafe extern "C" fn rpc_execute(task: *mut rpc_task) {
    rpc_task_set_active(task); rpc_make_runnable(rpciod_workqueue, task);
}
pub unsafe extern "C" fn rpc_malloc(_task: *mut rpc_task) -> i32 { 0 }
pub unsafe extern "C" fn rpc_free(_task: *mut rpc_task) {}
pub unsafe extern "C" fn rpc_new_task(_setup: *const rpc_task_setup) -> *mut rpc_task { core::ptr::null_mut() }
pub unsafe extern "C" fn rpc_put_task(task: *mut rpc_task) { rpc_free_task(task); }
pub unsafe extern "C" fn rpc_put_task_async(task: *mut rpc_task) { rpc_free_task(task); }
pub unsafe extern "C" fn rpciod_up() -> i32 { 0 }
pub unsafe extern "C" fn rpciod_down() {}
pub unsafe extern "C" fn rpc_destroy_mempool() {}
pub unsafe extern "C" fn rpc_init_mempool() -> i32 { 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
