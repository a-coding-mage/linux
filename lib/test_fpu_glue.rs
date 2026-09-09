// SPDX-License-Identifier: GPL-2.0+
/*
 * Test cases for using floating point operations inside a kernel module.
 *
 * This tests kernel_fpu_begin() and kernel_fpu_end() functions, especially
 * when userland has modified the floating point control registers. The kernel
 * state might depend on the state set by the userland thread that was active
 * before a syscall.
 *
 * To facilitate the test, this module registers file
 * /sys/kernel/debug/selftest_helpers/test_fpu, which when read causes a
 * sequence of floating point operations. If the operations fail, either the
 * read returns error status or the kernel crashes.
 * If the operations succeed, the read returns "1\n".
 */

use core::ffi::{c_int, c_void};

// Kernel and test_fpu.h declarations are supplied by the surrounding build.
#[repr(C)]
pub struct dentry {
    _private: [u8; 0],
}

#[repr(C)]
pub struct file_operations {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn kernel_fpu_begin();
    fn kernel_fpu_end();
    fn kernel_fpu_available() -> bool;
    fn test_fpu() -> c_int;
    fn debugfs_create_dir(name: *const u8, parent: *mut dentry) -> *mut dentry;
    fn debugfs_create_file_unsafe(
        name: *const u8,
        mode: u32,
        parent: *mut dentry,
        data: *mut c_void,
        fops: *const file_operations,
    ) -> *mut dentry;
    fn debugfs_remove(entry: *mut dentry);
}

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;

static mut selftest_dir: *mut dentry = core::ptr::null_mut();

// DEFINE_DEBUGFS_ATTRIBUTE(test_fpu_fops, test_fpu_get, NULL, "%lld\n");
// The generated file-operations object is provided by the kernel debugfs API.
unsafe extern "C" {
    static test_fpu_fops: file_operations;
}

unsafe fn test_fpu_get(_data: *mut c_void, val: *mut u64) -> c_int {
    let mut status: c_int = -EINVAL;

    kernel_fpu_begin();
    status = test_fpu();
    kernel_fpu_end();

    *val = 1;
    status
}

unsafe fn test_fpu_init() -> c_int {
    if !kernel_fpu_available() {
        return -EINVAL;
    }

    selftest_dir = debugfs_create_dir(b"selftest_helpers\0".as_ptr(), core::ptr::null_mut());
    if selftest_dir as isize == -1 {
        return -ENOMEM;
    }

    debugfs_create_file_unsafe(
        b"test_fpu\0".as_ptr(),
        0o444,
        selftest_dir,
        core::ptr::null_mut(),
        &test_fpu_fops,
    );

    0
}

unsafe fn test_fpu_exit() {
    debugfs_remove(selftest_dir);
}

// module_init(test_fpu_init);
// module_exit(test_fpu_exit);
// MODULE_DESCRIPTION("Test cases for floating point operations");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
