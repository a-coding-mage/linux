/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2025 Google, Inc.
 */

// TRACE_SYSTEM: rust_binder
// TRACE_INCLUDE_FILE: rust_binder_events
// TRACE_INCLUDE_PATH: ../drivers/android/binder
//
// The Linux tracepoint declarations below are represented as Rust C-layout
// event payloads and their corresponding assignment/format descriptions.

#[repr(C)]
pub struct BinderIoctlEntry {
    pub cmd: core::ffi::c_uint,
    pub arg: core::ffi::c_ulong,
}

#[repr(C)]
pub struct BinderFunctionReturnEntry {
    pub ret: core::ffi::c_int,
}

#[repr(C)]
pub struct BinderWaitForWorkEntry {
    pub proc_work: bool,
    pub transaction_stack: bool,
    pub thread_todo: bool,
}

#[repr(C)]
pub struct BinderTransactionEntry {
    pub debug_id: core::ffi::c_int,
    pub target_node: core::ffi::c_int,
    pub to_proc: core::ffi::c_int,
    pub to_thread: core::ffi::c_int,
    pub reply: core::ffi::c_int,
    pub code: core::ffi::c_uint,
    pub flags: core::ffi::c_uint,
}

#[repr(C)]
pub struct BinderTransactionReceivedEntry {
    pub debug_id: core::ffi::c_int,
}

#[repr(C)]
pub struct BinderTransactionFdEntry {
    pub debug_id: core::ffi::c_int,
    pub fd: core::ffi::c_int,
    pub offset: usize,
}

#[repr(C)]
pub struct BinderCommandEntry {
    pub cmd: u32,
}

#[repr(C)]
pub struct BinderReturnEntry {
    pub cmd: u32,
}

// TP_fast_assign for binder_ioctl.
#[inline]
pub const fn binder_ioctl_entry(cmd: core::ffi::c_uint, arg: core::ffi::c_ulong) -> BinderIoctlEntry {
    BinderIoctlEntry { cmd, arg }
}

// TP_fast_assign for binder_function_return_class and its events:
// binder_ioctl_done, binder_read_done, and binder_write_done.
#[inline]
pub const fn binder_function_return_entry(ret: core::ffi::c_int) -> BinderFunctionReturnEntry {
    BinderFunctionReturnEntry { ret }
}

#[inline]
pub const fn binder_wait_for_work_entry(
    proc_work: bool,
    transaction_stack: bool,
    thread_todo: bool,
) -> BinderWaitForWorkEntry {
    BinderWaitForWorkEntry { proc_work, transaction_stack, thread_todo }
}

// rust_binder_transaction, rust_binder_process, rust_binder_node, and
// struct task_struct are supplied by the binder implementation.
extern "C" {
    pub type rust_binder_transaction;
    pub type rust_binder_process;
    pub type rust_binder_node;
    pub type task_struct;

    pub fn rust_binder_transaction_to_proc(t: *mut rust_binder_transaction) -> *mut rust_binder_process;
    pub fn rust_binder_transaction_target_node(t: *mut rust_binder_transaction) -> *mut rust_binder_node;
    pub fn rust_binder_transaction_debug_id(t: *mut rust_binder_transaction) -> core::ffi::c_int;
    pub fn rust_binder_transaction_code(t: *mut rust_binder_transaction) -> core::ffi::c_uint;
    pub fn rust_binder_transaction_flags(t: *mut rust_binder_transaction) -> core::ffi::c_uint;
    pub fn rust_binder_node_debug_id(node: *mut rust_binder_node) -> core::ffi::c_int;
    pub fn rust_binder_process_task(process: *mut rust_binder_process) -> *mut task_struct;
}

#[inline]
pub unsafe fn binder_transaction_entry(
    reply: bool,
    t: *mut rust_binder_transaction,
    thread: *mut task_struct,
) -> BinderTransactionEntry {
    let to = rust_binder_transaction_to_proc(t);
    let target_node = rust_binder_transaction_target_node(t);
    BinderTransactionEntry {
        debug_id: rust_binder_transaction_debug_id(t),
        target_node: if !target_node.is_null() { rust_binder_node_debug_id(target_node) } else { 0 },
        to_proc: (*rust_binder_process_task(to)).pid,
        to_thread: if !thread.is_null() { (*thread).pid } else { 0 },
        reply: reply as core::ffi::c_int,
        code: rust_binder_transaction_code(t),
        flags: rust_binder_transaction_flags(t),
    }
}

#[inline]
pub unsafe fn binder_transaction_received_entry(t: *mut rust_binder_transaction) -> BinderTransactionReceivedEntry {
    BinderTransactionReceivedEntry { debug_id: rust_binder_transaction_debug_id(t) }
}

#[inline]
pub const fn binder_transaction_fd_entry(t_debug_id: core::ffi::c_int, fd: core::ffi::c_int, offset: usize) -> BinderTransactionFdEntry {
    BinderTransactionFdEntry { debug_id: t_debug_id, fd, offset }
}

#[inline]
pub const fn binder_command_entry(cmd: u32) -> BinderCommandEntry { BinderCommandEntry { cmd } }

#[inline]
pub const fn binder_return_entry(cmd: u32) -> BinderReturnEntry { BinderReturnEntry { cmd } }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
