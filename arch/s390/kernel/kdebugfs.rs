// SPDX-License-Identifier: GPL-2.0

// Translated from the Linux kernel C implementation. The declarations below
// are supplied by the surrounding kernel Rust bindings/build environment.

#[repr(C)]
pub struct dentry {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn debugfs_create_dir(name: *const u8, parent: *mut dentry) -> *mut dentry;
}

#[no_mangle]
pub static mut arch_debugfs_dir: *mut dentry = core::ptr::null_mut();

// EXPORT_SYMBOL(arch_debugfs_dir);

#[allow(non_snake_case)]
unsafe fn arch_kdebugfs_init() -> i32 {
    arch_debugfs_dir = debugfs_create_dir(b"s390\0".as_ptr(), core::ptr::null_mut());
    0
}

// postcore_initcall(arch_kdebugfs_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
