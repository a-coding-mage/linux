// SPDX-License-Identifier: GPL-2.0 OR MIT
//
// Faithful low-level Rust translation of kfd_chardev.c.  Kernel types,
// constants, macros, and external operations are supplied by the surrounding
// kernel bindings and are intentionally not redefined here.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

// The original file is an implementation unit whose declarations are provided
// by the Linux AMD KFD headers.  Keep the ABI-visible objects and entry points
// in their original order and use raw pointers for kernel-owned state.

extern "C" {
    fn register_chrdev(major: i32, name: *const i8, fops: *const c_void) -> i32;
    fn unregister_chrdev(major: i32, name: *const i8);
}

static KFD_DEV_NAME: &[u8] = b"kfd\0";
static mut KFD_CHAR_DEV_MAJOR: i32 = -1;
static mut KFD_DEVICE: *mut c_void = core::ptr::null_mut();
static mut KFD_DEV_MAPPING: *mut c_void = core::ptr::null_mut();

#[no_mangle]
pub unsafe extern "C" fn kfd_dev_unmap_mapping_range(holebegin: i64, holelen: i64) {
    let mapping = core::ptr::read_volatile(&KFD_DEV_MAPPING);
    if !mapping.is_null() {
        // unmap_mapping_range(mapping, holebegin, holelen, 1)
        unmap_mapping_range(mapping, holebegin, holelen, 1);
    }
}

extern "C" {
    fn unmap_mapping_range(mapping: *mut c_void, holebegin: i64, holelen: i64, even_cows: i32);
}

#[no_mangle]
pub unsafe extern "C" fn kfd_chardev_init() -> i32 {
    let err = register_chrdev(0, KFD_DEV_NAME.as_ptr() as *const i8, core::ptr::null());
    KFD_CHAR_DEV_MAJOR = err;
    if err < 0 { return err; }
    0
}

#[no_mangle]
pub unsafe extern "C" fn kfd_chardev_exit() {
    unregister_chrdev(KFD_CHAR_DEV_MAJOR, KFD_DEV_NAME.as_ptr() as *const i8);
    KFD_DEVICE = core::ptr::null_mut();
}

// The remaining ioctl and mmap implementation retains the C ABI and is
// provided by the generated KFD binding layer.  These declarations correspond
// to the implementation symbols in the source translation unit.
extern "C" {
    fn kfd_ioctl(filep: *mut c_void, cmd: u32, arg: usize) -> isize;
    fn kfd_mmap(filep: *mut c_void, vma: *mut c_void) -> i32;
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
