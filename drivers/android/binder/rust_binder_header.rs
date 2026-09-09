/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2025 Google, Inc.
 */

// Dependency declarations from <uapi/linux/android/binder.h> and
// <uapi/linux/android/binderfs.h> are supplied by other files.

use core::ffi::c_void;

/*
 * These symbols are exposed by `rust_binderfs.c` and exist here so that Rust
 * Binder can call them.
 */
unsafe extern "C" {
    pub fn init_rust_binderfs() -> ::core::ffi::c_int;
}

#[repr(C)]
pub struct dentry {
    _private: [u8; 0],
}

#[repr(C)]
pub struct inode {
    _private: [u8; 0],
}

#[repr(C)]
pub struct task_struct {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn rust_binderfs_create_proc_file(nodp: *mut inode, pid: ::core::ffi::c_int)
        -> *mut dentry;
    pub fn rust_binderfs_remove_file(dentry: *mut dentry);
}

/*
 * The internal data types in the Rust Binder driver are opaque to C, so we use
 * void pointer typedefs for these types.
 */

pub type rust_binder_transaction = *mut c_void;
pub type rust_binder_process = *mut c_void;
pub type rust_binder_node = *mut c_void;

#[repr(C)]
pub struct rb_process_layout {
    pub arc_offset: usize,
    pub task: usize,
}

#[repr(C)]
pub struct rb_transaction_layout {
    pub debug_id: usize,
    pub code: usize,
    pub flags: usize,
    pub from_thread: usize,
    pub to_proc: usize,
    pub target_node: usize,
}

#[repr(C)]
pub struct rb_node_layout {
    pub arc_offset: usize,
    pub debug_id: usize,
    pub ptr: usize,
}

#[repr(C)]
pub struct rust_binder_layout {
    pub t: rb_transaction_layout,
    pub p: rb_process_layout,
    pub n: rb_node_layout,
}

unsafe extern "C" {
    pub static RUST_BINDER_LAYOUT: rust_binder_layout;
}

#[inline]
pub unsafe fn rust_binder_transaction_debug_id(t: rust_binder_transaction) -> usize {
    *(t.byte_add(RUST_BINDER_LAYOUT.t.debug_id) as *const usize)
}

#[inline]
pub unsafe fn rust_binder_transaction_code(t: rust_binder_transaction) -> u32 {
    *(t.byte_add(RUST_BINDER_LAYOUT.t.code) as *const u32)
}

#[inline]
pub unsafe fn rust_binder_transaction_flags(t: rust_binder_transaction) -> u32 {
    *(t.byte_add(RUST_BINDER_LAYOUT.t.flags) as *const u32)
}

// Nullable!
#[inline]
pub unsafe fn rust_binder_transaction_target_node(
    t: rust_binder_transaction,
) -> rust_binder_node {
    let mut p = *(t.byte_add(RUST_BINDER_LAYOUT.t.target_node) as *const *mut c_void);

    if !p.is_null() {
        p = p.byte_add(RUST_BINDER_LAYOUT.n.arc_offset);
    }
    p
}

#[inline]
pub unsafe fn rust_binder_transaction_to_proc(t: rust_binder_transaction) -> rust_binder_process {
    let p = *(t.byte_add(RUST_BINDER_LAYOUT.t.to_proc) as *const *mut c_void);

    p.byte_add(RUST_BINDER_LAYOUT.p.arc_offset)
}

#[inline]
pub unsafe fn rust_binder_process_task(t: rust_binder_process) -> *mut task_struct {
    *(t.byte_add(RUST_BINDER_LAYOUT.p.task) as *const *mut task_struct)
}

#[inline]
pub unsafe fn rust_binder_node_debug_id(t: rust_binder_node) -> usize {
    *(t.byte_add(RUST_BINDER_LAYOUT.n.debug_id) as *const usize)
}

#[inline]
pub unsafe fn rust_binder_node_ptr(t: rust_binder_node) -> binder_uintptr_t {
    *(t.byte_add(RUST_BINDER_LAYOUT.n.ptr) as *const binder_uintptr_t)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
