// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2026 Christian Brauner <brauner@kernel.org> */

// Declarations supplied by the corresponding kernel headers.
use crate::linux::init::__init;
use crate::linux::rcupdate::rcu_head;
use crate::linux::refcount::refcount_t;
use crate::linux::sched::coredump::task_dumpable;
use crate::linux::user_namespace::user_namespace;

#[repr(C)]
pub struct task_exec_state {
    pub count: refcount_t,
    pub dumpable: task_dumpable,
    pub user_ns: *mut user_namespace,
    pub rcu: rcu_head,
}

extern "C" {
    pub static mut init_task_exec_state: task_exec_state;

    pub fn alloc_task_exec_state(user_ns: *mut user_namespace) -> *mut task_exec_state;
    pub fn put_task_exec_state(exec_state: *mut task_exec_state);
    pub fn task_exec_state_rcu(tsk: *const task_struct) -> *mut task_exec_state;
    pub fn task_exec_state_replace(
        tsk: *mut task_struct,
        exec_state: *mut task_exec_state,
    ) -> *mut task_exec_state;
    pub fn task_exec_state_copy(tsk: *mut task_struct) -> core::ffi::c_int;
    pub fn exec_state_init();
}

// Supplied by the scheduler headers; retained as an external type dependency.
#[repr(C)]
pub struct task_struct {
    _private: [u8; 0],
}

// DEFINE_FREE(put_task_exec_state, struct task_exec_state *,
//             put_task_exec_state(_T))
// Kernel cleanup annotation: release a task_exec_state with put_task_exec_state.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
