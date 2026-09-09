// SPDX-License-Identifier: GPL-2.0
/*
 * /proc/sys/fs shared sysctls
 *
 * These sysctls are shared between different filesystems.
 */

use core::ffi::{c_char, c_int, c_void};

// Supplied by the kernel sysctl implementation.
#[repr(C)]
pub struct ctl_table {
    pub procname: *const c_char,
    pub data: *mut c_void,
    pub maxlen: usize,
    pub mode: u16,
    pub proc_handler: Option<
        unsafe extern "C" fn(
            table: *mut ctl_table,
            write: c_int,
            buffer: *mut c_void,
            lenp: *mut usize,
            ppos: *mut i64,
        ) -> c_int,
    >,
    pub child: *mut ctl_table,
    pub poll: *mut c_void,
    pub extra1: *mut c_void,
    pub extra2: *mut c_void,
}

extern "C" {
    static mut fs_overflowuid: c_int;
    static mut fs_overflowgid: c_int;

    static SYSCTL_ZERO: *mut c_void;
    static SYSCTL_MAXOLDUID: *mut c_void;

    fn proc_dointvec_minmax(
        table: *mut ctl_table,
        write: c_int,
        buffer: *mut c_void,
        lenp: *mut usize,
        ppos: *mut i64,
    ) -> c_int;

    fn register_sysctl_init(
        table: *const c_char,
        table_header: *const ctl_table,
    ) -> *mut c_void;
}

static FS_SHARED_SYSCTLS: [ctl_table; 2] = [
    ctl_table {
        procname: b"overflowuid\0".as_ptr() as *const c_char,
        data: unsafe { &raw mut fs_overflowuid as *mut c_void },
        maxlen: core::mem::size_of::<c_int>(),
        mode: 0o644,
        proc_handler: Some(proc_dointvec_minmax),
        child: core::ptr::null_mut(),
        poll: core::ptr::null_mut(),
        extra1: unsafe { SYSCTL_ZERO },
        extra2: unsafe { SYSCTL_MAXOLDUID },
    },
    ctl_table {
        procname: b"overflowgid\0".as_ptr() as *const c_char,
        data: unsafe { &raw mut fs_overflowgid as *mut c_void },
        maxlen: core::mem::size_of::<c_int>(),
        mode: 0o644,
        proc_handler: Some(proc_dointvec_minmax),
        child: core::ptr::null_mut(),
        poll: core::ptr::null_mut(),
        extra1: unsafe { SYSCTL_ZERO },
        extra2: unsafe { SYSCTL_MAXOLDUID },
    },
];

unsafe extern "C" fn init_fs_sysctls() -> c_int {
    register_sysctl_init(b"fs\0".as_ptr() as *const c_char, FS_SHARED_SYSCTLS.as_ptr());
    0
}

// The Linux `early_initcall(init_fs_sysctls)` registration is supplied by the
// kernel build system and is intentionally preserved here as a declaration-level comment.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
