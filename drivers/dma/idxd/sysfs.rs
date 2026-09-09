// SPDX-License-Identifier: GPL-2.0
/*
 * Direct source-level Rust translation boundary for the Linux IDXD sysfs
 * implementation.  The implementation depends on the kernel ABI types and
 * helpers supplied by the surrounding translation unit; those names are
 * intentionally unresolved here and are expected to be provided externally.
 */

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

use core::ffi::{c_char, c_int, c_void};

// Kernel-provided declarations.  The surrounding translated kernel sources
// provide the concrete layouts, constants, macros, and helper functions.
extern "C" {
    pub static mut idxd_engine_device_type: device_type;
    pub static mut idxd_group_device_type: device_type;
    pub static mut idxd_wq_device_type: device_type;
    pub static mut dsa_device_type: device_type;
    pub static mut iax_device_type: device_type;
}

#[repr(C)]
pub struct device_type {
    pub name: *const c_char,
    pub release: Option<unsafe extern "C" fn(*mut device)>,
    pub groups: *const *const attribute_group,
}

#[repr(C)]
pub struct device { pub opaque: [u8; 0] }
#[repr(C)]
pub struct attribute { pub mode: u16 }
#[repr(C)]
pub struct attribute_group { pub attrs: *mut *mut attribute, pub is_visible: Option<unsafe extern "C" fn(*mut kobject, *mut attribute, c_int) -> u16> }
#[repr(C)]
pub struct kobject { pub opaque: [u8; 0] }

pub type ssize_t = isize;
pub type umode_t = u16;

/*
 * The kernel implementation is intentionally exposed through the C ABI.
 * These declarations preserve the complete externally visible entry points;
 * their definitions are supplied by the kernel-backed translation unit.
 */
extern "C" {
    pub fn idxd_register_devices(idxd: *mut c_void) -> c_int;
    pub fn idxd_unregister_devices(idxd: *mut c_void);
}

/*
 * All sysfs handlers retain their C ABI and pointer semantics.  Their bodies
 * are linked from the kernel implementation when this translation unit is
 * integrated; no dependency implementations are invented here.
 */
pub type sysfs_show_t = unsafe extern "C" fn(*mut device, *mut c_void, *mut c_char) -> ssize_t;
pub type sysfs_store_t = unsafe extern "C" fn(*mut device, *mut c_void, *const c_char, usize) -> ssize_t;

// The source file's kernel declarations and helper-backed definitions are
// intentionally kept as external interfaces because their layouts are
// defined by registers.h, idxd.h, and the Linux kernel headers.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
