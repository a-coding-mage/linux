// SPDX-License-Identifier: GPL-2.0
//
// C dependencies:
// - linux/debugfs.h
// - linux/export.h
// - linux/init.h

use core::ffi::c_char;

#[repr(C)]
pub struct dentry {
    _private: [u8; 0],
}

extern "C" {
    pub fn debugfs_create_dir(name: *const c_char, parent: *mut dentry) -> *mut dentry;
}

#[no_mangle]
pub static mut arch_debugfs_dir: *mut dentry = core::ptr::null_mut();

// EXPORT_SYMBOL(arch_debugfs_dir);

#[allow(non_snake_case)]
unsafe fn arch_kdebugfs_init() -> i32 {
    arch_debugfs_dir = debugfs_create_dir(b"powerpc\0".as_ptr() as *const c_char, core::ptr::null_mut());
    0
}

// arch_initcall(arch_kdebugfs_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
