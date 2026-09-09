// SPDX-License-Identifier: GPL-2.0
/*
 * linux/fs/nfs/sysctl.c
 *
 * Sysctl interface to NFS parameters
 */

use core::ffi::{c_char, c_int, c_void};

// Dependencies supplied by the Linux kernel sources.
#[repr(C)]
pub struct ctl_table_header {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ctl_table {
    pub procname: *const c_char,
    pub data: *mut c_void,
    pub maxlen: usize,
    pub mode: u16,
    pub proc_handler: *mut c_void,
}

unsafe extern "C" {
    static mut nfs_mountpoint_expiry_timeout: usize;
    static mut nfs_congestion_kb: c_int;

    static mut proc_dointvec_jiffies: *mut c_void;
    static mut proc_dointvec: *mut c_void;

    fn register_sysctl(path: *const c_char, table: *const ctl_table) -> *mut ctl_table_header;
    fn unregister_sysctl_table(header: *mut ctl_table_header);
}

static mut nfs_callback_sysctl_table: *mut ctl_table_header = core::ptr::null_mut();

static mut nfs_cb_sysctls: [ctl_table; 2] = [
    ctl_table {
        procname: b"nfs_mountpoint_timeout\0".as_ptr() as *const c_char,
        data: unsafe { &raw mut nfs_mountpoint_expiry_timeout as *mut c_void },
        maxlen: core::mem::size_of::<usize>(),
        mode: 0o644,
        proc_handler: unsafe { &raw mut proc_dointvec_jiffies },
    },
    ctl_table {
        procname: b"nfs_congestion_kb\0".as_ptr() as *const c_char,
        data: unsafe { &raw mut nfs_congestion_kb as *mut c_void },
        maxlen: core::mem::size_of::<c_int>(),
        mode: 0o644,
        proc_handler: unsafe { &raw mut proc_dointvec },
    },
];

#[no_mangle]
pub unsafe extern "C" fn nfs_register_sysctl() -> c_int {
    nfs_callback_sysctl_table = register_sysctl(
        b"fs/nfs\0".as_ptr() as *const c_char,
        nfs_cb_sysctls.as_ptr(),
    );
    if nfs_callback_sysctl_table.is_null() {
        return -12;
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn nfs_unregister_sysctl() {
    unregister_sysctl_table(nfs_callback_sysctl_table);
    nfs_callback_sysctl_table = core::ptr::null_mut();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
