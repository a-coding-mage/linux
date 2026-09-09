/* SPDX-License-Identifier: GPL-2.0 */
/*
 *    Hypervisor filesystem for Linux on s390. Diag 204 and 224
 *    implementation.
 *
 *    Copyright IBM Corp. 2006, 2008
 *    Author(s): Michael Holzheu <holzheu@de.ibm.com>
 */

use core::ffi::{c_int, c_void};

// Supplied by the translated asm/diag dependency.
extern "C" {
    pub fn diag204_get_info_type() -> diag204_format;
    pub fn diag204_get_buffer(fmt: diag204_format, pages: *mut c_int) -> *mut c_void;
    pub fn diag204_store(buf: *mut c_void, pages: c_int) -> c_int;

    pub fn __hypfs_diag_fs_init() -> c_int;
    pub fn __hypfs_diag_fs_exit();
}

// `diag204_format` is declared by the supplied asm/diag dependency.
// `CONFIG_S390_HYPFS_FS` corresponds to the C build-time configuration.

#[inline(always)]
pub unsafe fn hypfs_diag_fs_init() -> c_int {
    if cfg!(feature = "CONFIG_S390_HYPFS_FS") {
        __hypfs_diag_fs_init()
    } else {
        0
    }
}

#[inline]
pub unsafe fn hypfs_diag_fs_exit() {
    if cfg!(feature = "CONFIG_S390_HYPFS_FS") {
        __hypfs_diag_fs_exit();
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
