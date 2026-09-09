// SPDX-License-Identifier: GPL-2.0
/*
 * Sysctl operations for Coda filesystem
 * Original version: (C) 1996 P. Braam and M. Callahan
 * Rewritten for Linux 2.1. (C) 1997 Carnegie Mellon University
 * 
 * Carnegie Mellon encourages users to contribute improvements to
 * the Coda project. Contact Peter Braam (coda@cs.cmu.edu).
 */

use core::ffi::{c_char, c_int, c_void};

// Supplied by linux/sysctl.h and coda_int.h in the surrounding translation.
extern "C" {
    static mut coda_timeout: c_int;
    static mut coda_hard: c_int;
    static mut coda_fake_statfs: c_int;

    fn proc_dointvec(
        table: *mut crate::ctl_table,
        write: c_int,
        buffer: *mut c_void,
        lenp: *mut usize,
        ppos: *mut i64,
    ) -> c_int;
    fn register_sysctl(
        name: *const c_char,
        table: *const crate::ctl_table,
    ) -> *mut crate::ctl_table_header;
    fn unregister_sysctl_table(header: *mut crate::ctl_table_header);
}

static mut FS_TABLE_HEADER: *mut crate::ctl_table_header = core::ptr::null_mut();

static CODA_TABLE: [crate::ctl_table; 3] = [
    crate::ctl_table {
        procname: b"timeout\0".as_ptr() as *mut c_char,
        data: unsafe { &raw mut coda_timeout as *mut c_void },
        maxlen: core::mem::size_of::<c_int>() as c_int,
        mode: 0o644,
        proc_handler: Some(proc_dointvec),
    },
    crate::ctl_table {
        procname: b"hard\0".as_ptr() as *mut c_char,
        data: unsafe { &raw mut coda_hard as *mut c_void },
        maxlen: core::mem::size_of::<c_int>() as c_int,
        mode: 0o644,
        proc_handler: Some(proc_dointvec),
    },
    crate::ctl_table {
        procname: b"fake_statfs\0".as_ptr() as *mut c_char,
        data: unsafe { &raw mut coda_fake_statfs as *mut c_void },
        maxlen: core::mem::size_of::<c_int>() as c_int,
        mode: 0o600,
        proc_handler: Some(proc_dointvec),
    },
];

pub unsafe extern "C" fn coda_sysctl_init() {
    if FS_TABLE_HEADER.is_null() {
        FS_TABLE_HEADER = register_sysctl(b"coda\0".as_ptr() as *const c_char, CODA_TABLE.as_ptr());
    }
}

pub unsafe extern "C" fn coda_sysctl_clean() {
    if !FS_TABLE_HEADER.is_null() {
        unregister_sysctl_table(FS_TABLE_HEADER);
        FS_TABLE_HEADER = core::ptr::null_mut();
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
