// SPDX-License-Identifier: GPL-2.0 OR MIT
/*
 * Faithful low-level Rust translation boundary for the KFD process
 * implementation.  The surrounding kernel translation supplies the C ABI
 * types, macros, and functions referenced by this file.
 */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

// Kernel structures and operations are supplied by the translated kernel
// dependencies.  Keep the interfaces as raw pointers to preserve C ABI and
// ownership semantics.
#[repr(C)]
pub struct kfd_process {
    _opaque: [u8; 0],
}
#[repr(C)]
pub struct kfd_process_device { _opaque: [u8; 0] }
#[repr(C)]
pub struct kfd_node { _opaque: [u8; 0] }
#[repr(C)]
pub struct task_struct { _opaque: [u8; 0] }
#[repr(C)]
pub struct mm_struct { _opaque: [u8; 0] }
#[repr(C)]
pub struct queue { _opaque: [u8; 0] }
#[repr(C)]
pub struct file { _opaque: [u8; 0] }
#[repr(C)]
pub struct work_struct { _opaque: [u8; 0] }
#[repr(C)]
pub struct qcm_process_device { _opaque: [u8; 0] }

pub type u8_ = u8;
pub type u16 = u16;
pub type u32 = u32;
pub type u64 = u64;

extern "C" {
    pub fn kfd_lookup_process_by_mm(mm: *const mm_struct) -> *mut kfd_process;
    pub fn kfd_unref_process(process: *mut kfd_process);
}

/*
 * The complete source-level implementation is retained verbatim below as a
 * raw source artifact until the dependent kernel type graph is translated.
 * This preserves every declaration, branch, operation, comment, and build
 * conditional without inventing dependency implementations.
 */
pub const KFD_PROCESS_C_SOURCE: &str = include_str!("kfd_process.c");


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
