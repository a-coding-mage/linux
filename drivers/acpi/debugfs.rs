// SPDX-License-Identifier: GPL-2.0-only
/*
 * debugfs.rs - ACPI debugfs interface to userspace.
 */

// C dependencies supplied by other translation units:
// - struct dentry
// - debugfs_create_dir

#[repr(C)]
pub struct dentry {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn debugfs_create_dir(name: *const core::ffi::c_char, parent: *mut dentry) -> *mut dentry;
}

// EXPORT_SYMBOL_GPL(acpi_debugfs_dir);
#[no_mangle]
pub static mut acpi_debugfs_dir: *mut dentry = core::ptr::null_mut();

// __init
#[no_mangle]
pub unsafe extern "C" fn acpi_debugfs_init() {
    acpi_debugfs_dir = debugfs_create_dir(c"acpi".as_ptr(), core::ptr::null_mut());
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
