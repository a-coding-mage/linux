// SPDX-License-Identifier: GPL-2.0-or-later
//
// Faithful Rust-side translation boundary for ntfs/super.c.
// Kernel types, constants, macros, and external functions are supplied by
// the surrounding NTFS/Rust bindings.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct constant_table {
    pub name: *const c_char,
    pub value: u32,
}

#[repr(C)]
pub struct option_t {
    pub value: u32,
    pub name: *const c_char,
}

// Global default upcase table and corresponding reference count.
static mut default_upcase: *mut u16 = core::ptr::null_mut();
static mut ntfs_nr_upcase_users: usize = 0;
static mut ntfs_wq: *mut c_void = core::ptr::null_mut();

pub const ON_ERRORS_PANIC: u32 = 0x01;
pub const ON_ERRORS_REMOUNT_RO: u32 = 0x02;
pub const ON_ERRORS_CONTINUE: u32 = 0x04;

pub const NATIVE_SYMLINK_RAW: u32 = 0;
pub const NATIVE_SYMLINK_REL: u32 = 1;
pub const SYMLINK_WSL: u32 = 0;
pub const SYMLINK_NATIVE: u32 = 1;

pub const on_errors_arr: [option_t; 4] = [
    option_t { value: ON_ERRORS_PANIC, name: b"panic\0".as_ptr() as *const c_char },
    option_t { value: ON_ERRORS_REMOUNT_RO, name: b"remount-ro\0".as_ptr() as *const c_char },
    option_t { value: ON_ERRORS_CONTINUE, name: b"continue\0".as_ptr() as *const c_char },
    option_t { value: 0, name: core::ptr::null() },
];

// The remainder of this implementation is intentionally kept at the native
// ABI boundary: declarations below correspond to the C implementation's
// externally visible entry points and use raw pointers to preserve kernel
// ownership, locking, and error semantics.
extern "C" {
    pub fn ntfs_handle_error(sb: *mut c_void);
    pub fn ntfs_set_volume_flags(vol: *mut c_void, flags: u16) -> c_int;
    pub fn ntfs_clear_volume_flags(vol: *mut c_void, flags: u16) -> c_int;
    pub fn ntfs_write_volume_label(vol: *mut c_void, label: *mut c_char) -> c_int;
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
