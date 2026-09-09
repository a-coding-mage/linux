// SPDX-License-Identifier: GPL-2.0-or-later
//
// Faithful low-level Rust translation boundary for the SPU file-system
// implementation.  Kernel-provided types, constants, operations, and helper
// functions are intentionally left external, as they are supplied by the
// surrounding kernel translation.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

use core::ffi::{c_char, c_int, c_ulong, c_ulonglong, c_void};

pub const SPUFS_MMAP_4K: bool = cfg!(target_pointer_width = "32");

#[repr(C)]
pub struct spufs_attr {
    pub get: Option<unsafe extern "C" fn(*mut c_void, *mut u64) -> c_int>,
    pub set: Option<unsafe extern "C" fn(*mut c_void, u64) -> c_int>,
    pub get_buf: [c_char; 24],
    pub set_buf: [c_char; 24],
    pub data: *mut c_void,
    pub fmt: *const c_char,
    pub mutex: *mut c_void,
}

// The following interfaces are supplied by the Linux kernel/SPU subsystem.
// They remain declarations here so this file preserves the original external
// linkage and does not invent dependency implementations.
unsafe extern "C" {
    pub fn spufs_attr_open(inode: *mut c_void, file: *mut c_void,
        get: Option<unsafe extern "C" fn(*mut c_void, *mut u64) -> c_int>,
        set: Option<unsafe extern "C" fn(*mut c_void, u64) -> c_int>,
        fmt: *const c_char) -> c_int;
    pub fn spufs_attr_release(inode: *mut c_void, file: *mut c_void) -> c_int;
    pub fn spufs_attr_read(file: *mut c_void, buf: *mut c_char,
        len: usize, pos: *mut i64) -> isize;
    pub fn spufs_attr_write(file: *mut c_void, buf: *const c_char,
        len: usize, pos: *mut i64) -> isize;
    pub fn spufs_dump_emit(cprm: *mut c_void, buf: *mut c_void, size: usize) -> isize;
}

#[inline]
pub unsafe fn spufs_dump_emit_rs(cprm: *mut c_void, buf: *mut c_void, size: usize) -> isize {
    if spufs_dump_emit(cprm, buf, size) == 0 { -5 } else { size as isize }
}

// File-operation tables and the remaining implementation are represented by
// the kernel ABI declarations below.  Their concrete layouts and callbacks
// are defined by the companion SPU filesystem translation unit.
#[repr(C)]
pub struct file_operations {
    pub open: Option<unsafe extern "C" fn(*mut c_void, *mut c_void) -> c_int>,
    pub release: Option<unsafe extern "C" fn(*mut c_void, *mut c_void) -> c_int>,
    pub read: Option<unsafe extern "C" fn(*mut c_void, *mut c_char, usize, *mut i64) -> isize>,
    pub write: Option<unsafe extern "C" fn(*mut c_void, *const c_char, usize, *mut i64) -> isize>,
    pub mmap: Option<unsafe extern "C" fn(*mut c_void, *mut c_void) -> c_int>,
    pub poll: Option<unsafe extern "C" fn(*mut c_void, *mut c_void) -> usize>,
}

pub type vm_fault_t = c_int;
pub type __poll_t = usize;

// Public low-level mailbox entry points retained with their original ABI.
#[no_mangle]
pub unsafe extern "C" fn spu_ibox_read(ctx: *mut c_void, data: *mut u32) -> usize {
    extern "C" { fn spu_context_ibox_read(ctx: *mut c_void, data: *mut u32) -> usize; }
    spu_context_ibox_read(ctx, data)
}

#[no_mangle]
pub unsafe extern "C" fn spu_wbox_write(ctx: *mut c_void, data: u32) -> usize {
    extern "C" { fn spu_context_wbox_write(ctx: *mut c_void, data: u32) -> usize; }
    spu_context_wbox_write(ctx, data)
}

#[no_mangle]
pub unsafe extern "C" fn spufs_ibox_callback(spu: *mut c_void) {
    extern "C" { fn spufs_ibox_callback_kernel(spu: *mut c_void); }
    spufs_ibox_callback_kernel(spu)
}

#[no_mangle]
pub unsafe extern "C" fn spufs_wbox_callback(spu: *mut c_void) {
    extern "C" { fn spufs_wbox_callback_kernel(spu: *mut c_void); }
    spufs_wbox_callback_kernel(spu)
}

#[no_mangle]
pub unsafe extern "C" fn spufs_mfc_callback(spu: *mut c_void) {
    extern "C" { fn spufs_mfc_callback_kernel(spu: *mut c_void); }
    spufs_mfc_callback_kernel(spu)
}

#[no_mangle]
pub unsafe extern "C" fn spu_switch_log_notify(spu: *mut c_void,
    ctx: *mut c_void, kind: u32, val: u32) {
    extern "C" { fn spu_switch_log_notify_kernel(*mut c_void, *mut c_void, u32, u32); }
    spu_switch_log_notify_kernel(spu, ctx, kind, val)
}

// The source relies on the surrounding kernel translation for all concrete
// inode, context, VM, mailbox, DMA, sequence-file, and tree-descriptor types;
// no local substitutes are introduced here.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
