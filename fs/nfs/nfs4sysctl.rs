// SPDX-License-Identifier: GPL-2.0
/*
 * linux/fs/nfs/nfs4sysctl.c
 *
 * Sysctl interface to NFS v4 parameters
 *
 * Copyright (c) 2006 Trond Myklebust <Trond.Myklebust@netapp.com>
 */

// Dependencies supplied by the corresponding kernel/NFS headers.
use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct ctl_table {
    pub procname: *const c_char,
    pub data: *mut c_void,
    pub maxlen: usize,
    pub mode: u16,
    pub proc_handler: Option<unsafe extern "C" fn(*mut ctl_table, *mut c_void) -> c_int>,
    pub extra1: *mut c_void,
    pub extra2: *mut c_void,
}

#[repr(C)]
pub struct ctl_table_header {
    _private: [u8; 0],
}

unsafe extern "C" {
    static mut nfs_callback_set_tcpport: c_int;
    static mut nfs_idmap_cache_timeout: c_int;

    static nfs_set_port_min: c_int;
    static nfs_set_port_max: c_int;

    static mut nfs4_callback_sysctl_table: *mut ctl_table_header;

    fn proc_dointvec_minmax(table: *mut ctl_table, write: *mut c_void) -> c_int;
    fn proc_dointvec(table: *mut ctl_table, write: *mut c_void) -> c_int;
    fn register_sysctl(path: *const c_char, table: *const ctl_table) -> *mut ctl_table_header;
    fn unregister_sysctl_table(header: *mut ctl_table_header);
}

const NFS_SET_PORT_MAX: c_int = 65535;

static mut NFS4_CB_SYSCTLS: [ctl_table; 2] = [
    ctl_table {
        procname: b"nfs_callback_tcpport\0".as_ptr() as *const c_char,
        data: unsafe { &raw mut nfs_callback_set_tcpport as *mut c_int as *mut c_void },
        maxlen: core::mem::size_of::<c_int>(),
        mode: 0o644,
        proc_handler: Some(proc_dointvec_minmax),
        extra1: unsafe { &nfs_set_port_min as *const c_int as *mut c_void },
        extra2: unsafe { &nfs_set_port_max as *const c_int as *mut c_void },
    },
    ctl_table {
        procname: b"idmap_cache_timeout\0".as_ptr() as *const c_char,
        data: unsafe { &raw mut nfs_idmap_cache_timeout as *mut c_int as *mut c_void },
        maxlen: core::mem::size_of::<c_int>(),
        mode: 0o644,
        proc_handler: Some(proc_dointvec),
        extra1: core::ptr::null_mut(),
        extra2: core::ptr::null_mut(),
    },
];

pub unsafe extern "C" fn nfs4_register_sysctl() -> c_int {
    nfs4_callback_sysctl_table = register_sysctl(
        b"fs/nfs\0".as_ptr() as *const c_char,
        NFS4_CB_SYSCTLS.as_ptr(),
    );
    if nfs4_callback_sysctl_table.is_null() {
        return -12; // -ENOMEM
    }
    0
}

pub unsafe extern "C" fn nfs4_unregister_sysctl() {
    unregister_sysctl_table(nfs4_callback_sysctl_table);
    nfs4_callback_sysctl_table = core::ptr::null_mut();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
