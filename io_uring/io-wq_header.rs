/* SPDX-License-Identifier: GPL-2.0 */
/* Translated from io-wq.h. */

use core::ffi::c_void;

/* Supplied by the surrounding kernel translation. */
#[repr(C)]
pub struct refcount_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct wait_queue_head {
    _private: [u8; 0],
}

#[repr(C)]
pub struct task_struct {
    pub flags: usize,
    pub worker_private: *mut c_void,
}

#[repr(C)]
pub struct io_uring_task {
    _private: [u8; 0],
}

#[repr(C)]
pub struct io_wq_work {
    pub flags: i32,
}

pub type cpumask_var_t = *mut c_void;

pub enum io_wq {}

pub const IO_WQ_WORK_CANCEL: u32 = 1;
pub const IO_WQ_WORK_HASHED: u32 = 2;
pub const IO_WQ_WORK_UNBOUND: u32 = 4;
pub const IO_WQ_WORK_CONCURRENT: u32 = 16;
pub const IO_WQ_HASH_SHIFT: u32 = 24; /* upper 8 bits are used for hash key */

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum io_wq_cancel {
    IO_WQ_CANCEL_OK,
    IO_WQ_CANCEL_RUNNING,
    IO_WQ_CANCEL_NOTFOUND,
}

#[repr(C)]
pub struct io_wq_hash {
    pub refs: refcount_t,
    pub map: usize,
    pub wait: wait_queue_head,
}

extern "C" {
    fn refcount_dec_and_test(r: *mut refcount_t) -> bool;
    fn kfree(p: *mut c_void);
    fn in_task() -> bool;
    static mut current: *mut task_struct;
}

#[inline]
pub unsafe fn io_wq_put_hash(hash: *mut io_wq_hash) {
    if refcount_dec_and_test(&mut (*hash).refs) {
        kfree(hash.cast::<c_void>());
    }
}

#[repr(C)]
pub struct io_wq_data {
    pub hash: *mut io_wq_hash,
    pub task: *mut task_struct,
}

extern "C" {
    pub fn io_wq_create(bounded: u32, data: *mut io_wq_data) -> *mut io_wq;
    pub fn io_wq_exit_start(wq: *mut io_wq);
    pub fn io_wq_put_and_exit(wq: *mut io_wq);
    pub fn io_wq_set_exit_on_idle(wq: *mut io_wq, enable: bool);

    pub fn io_wq_enqueue(wq: *mut io_wq, work: *mut io_wq_work);
    pub fn io_wq_hash_work(work: *mut io_wq_work, val: *mut c_void);

    pub fn io_wq_cpu_affinity(tctx: *mut io_uring_task, mask: cpumask_var_t) -> i32;
    pub fn io_wq_max_workers(wq: *mut io_wq, new_count: *mut i32) -> i32;
    pub fn io_wq_worker_stopped() -> bool;
}

#[inline]
pub unsafe fn __io_wq_is_hashed(work_flags: u32) -> bool {
    (work_flags & IO_WQ_WORK_HASHED) != 0
}

#[inline]
pub unsafe fn io_wq_is_hashed(work: *mut io_wq_work) -> bool {
    __io_wq_is_hashed((*work).flags as u32)
}

pub type work_cancel_fn = unsafe extern "C" fn(*mut io_wq_work, *mut c_void) -> bool;

extern "C" {
    pub fn io_wq_cancel_cb(
        wq: *mut io_wq,
        cancel: Option<work_cancel_fn>,
        data: *mut c_void,
        cancel_all: bool,
    ) -> io_wq_cancel;

    #[cfg(feature = "CONFIG_IO_WQ")]
    pub fn io_wq_worker_sleeping(tsk: *mut task_struct);
    #[cfg(feature = "CONFIG_IO_WQ")]
    pub fn io_wq_worker_running(tsk: *mut task_struct);
}

#[cfg(not(feature = "CONFIG_IO_WQ"))]
#[inline]
pub unsafe fn io_wq_worker_sleeping(_tsk: *mut task_struct) {}

#[cfg(not(feature = "CONFIG_IO_WQ"))]
#[inline]
pub unsafe fn io_wq_worker_running(_tsk: *mut task_struct) {}

#[inline]
pub unsafe fn io_wq_current_is_worker() -> bool {
    in_task() && ((*current).flags & PF_IO_WORKER) != 0 && !(*current).worker_private.is_null()
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
