/* SPDX-License-Identifier: GPL-2.0 */

/* Declarations supplied by linux/list.h and linux/sched.h. */

#[repr(C)]
pub struct callback_head {
    pub next: *mut callback_head,
    pub func: task_work_func_t,
}

#[repr(C)]
pub struct task_struct {
    pub task_works: *mut callback_head,
}

pub type task_work_func_t = Option<unsafe extern "C" fn(*mut callback_head)>;

#[inline]
pub unsafe fn init_task_work(twork: *mut callback_head, func: task_work_func_t) {
    (*twork).func = func;
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum task_work_notify_mode {
    TWA_NONE = 0,
    TWA_RESUME,
    TWA_SIGNAL,
    TWA_SIGNAL_NO_IPI,
    TWA_NMI_CURRENT,
}

#[inline]
pub unsafe fn task_work_pending(task: *mut task_struct) -> bool {
    core::ptr::read_volatile(&(*task).task_works).is_null().not()
}

unsafe extern "C" {
    pub fn task_work_add(
        task: *mut task_struct,
        twork: *mut callback_head,
        mode: task_work_notify_mode,
    ) -> i32;

    pub fn task_work_cancel_match(
        task: *mut task_struct,
        r#match: Option<unsafe extern "C" fn(*mut callback_head, *mut core::ffi::c_void) -> bool>,
        data: *mut core::ffi::c_void,
    ) -> *mut callback_head;

    pub fn task_work_cancel_func(
        task: *mut task_struct,
        func: task_work_func_t,
    ) -> *mut callback_head;

    pub fn task_work_cancel(task: *mut task_struct, cb: *mut callback_head) -> bool;
    pub fn task_work_run();
}

#[inline]
pub unsafe fn exit_task_work(task: *mut task_struct) {
    let _ = task;
    task_work_run();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
