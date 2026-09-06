// SPDX-License-Identifier: GPL-2.0-or-later
/* Key management controls
 *
 * Copyright (C) 2008 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

use core::ffi::{c_char, c_int, c_uint, c_void};

extern "C" {
    static mut key_quota_maxkeys: c_uint;
    static mut key_quota_maxbytes: c_uint;
    static mut key_quota_root_maxkeys: c_uint;
    static mut key_quota_root_maxbytes: c_uint;
    static mut key_gc_delay: c_uint;

    /* Present when CONFIG_PERSISTENT_KEYRINGS is enabled. */
    static mut persistent_keyring_expiry: c_uint;

    static SYSCTL_ONE: c_int;
    static SYSCTL_ZERO: c_int;
    static SYSCTL_INT_MAX: c_int;

    fn proc_dointvec_minmax(
        table: *mut ctl_table,
        write: c_int,
        buffer: *mut c_void,
        lenp: *mut usize,
        ppos: *mut i64,
    ) -> c_int;

    fn register_sysctl_init(path: *const c_char, table: *const ctl_table);
}

type ProcHandler = unsafe extern "C" fn(
    table: *mut ctl_table,
    write: c_int,
    buffer: *mut c_void,
    lenp: *mut usize,
    ppos: *mut i64,
) -> c_int;

#[repr(C)]
pub struct ctl_table {
    pub procname: *const c_char,
    pub data: *mut c_void,
    pub maxlen: usize,
    pub mode: c_uint,
    pub proc_handler: Option<ProcHandler>,
    pub extra1: *mut c_void,
    pub extra2: *mut c_void,
}

static KEY_SYSCTLS: [ctl_table; 6] = unsafe {
    [
        ctl_table {
            procname: b"maxkeys\0".as_ptr() as *const c_char,
            data: &key_quota_maxkeys as *const c_uint as *mut c_void,
            maxlen: core::mem::size_of::<c_uint>(),
            mode: 0o644,
            proc_handler: Some(proc_dointvec_minmax),
            extra1: &SYSCTL_ONE as *const c_int as *mut c_void,
            extra2: &SYSCTL_INT_MAX as *const c_int as *mut c_void,
        },
        ctl_table {
            procname: b"maxbytes\0".as_ptr() as *const c_char,
            data: &key_quota_maxbytes as *const c_uint as *mut c_void,
            maxlen: core::mem::size_of::<c_uint>(),
            mode: 0o644,
            proc_handler: Some(proc_dointvec_minmax),
            extra1: &SYSCTL_ONE as *const c_int as *mut c_void,
            extra2: &SYSCTL_INT_MAX as *const c_int as *mut c_void,
        },
        ctl_table {
            procname: b"root_maxkeys\0".as_ptr() as *const c_char,
            data: &key_quota_root_maxkeys as *const c_uint as *mut c_void,
            maxlen: core::mem::size_of::<c_uint>(),
            mode: 0o644,
            proc_handler: Some(proc_dointvec_minmax),
            extra1: &SYSCTL_ONE as *const c_int as *mut c_void,
            extra2: &SYSCTL_INT_MAX as *const c_int as *mut c_void,
        },
        ctl_table {
            procname: b"root_maxbytes\0".as_ptr() as *const c_char,
            data: &key_quota_root_maxbytes as *const c_uint as *mut c_void,
            maxlen: core::mem::size_of::<c_uint>(),
            mode: 0o644,
            proc_handler: Some(proc_dointvec_minmax),
            extra1: &SYSCTL_ONE as *const c_int as *mut c_void,
            extra2: &SYSCTL_INT_MAX as *const c_int as *mut c_void,
        },
        ctl_table {
            procname: b"gc_delay\0".as_ptr() as *const c_char,
            data: &key_gc_delay as *const c_uint as *mut c_void,
            maxlen: core::mem::size_of::<c_uint>(),
            mode: 0o644,
            proc_handler: Some(proc_dointvec_minmax),
            extra1: &SYSCTL_ZERO as *const c_int as *mut c_void,
            extra2: &SYSCTL_INT_MAX as *const c_int as *mut c_void,
        },
        /*
         * Original C includes this entry only under CONFIG_PERSISTENT_KEYRINGS.
         */
        ctl_table {
            procname: b"persistent_keyring_expiry\0".as_ptr() as *const c_char,
            data: &persistent_keyring_expiry as *const c_uint as *mut c_void,
            maxlen: core::mem::size_of::<c_uint>(),
            mode: 0o644,
            proc_handler: Some(proc_dointvec_minmax),
            extra1: &SYSCTL_ZERO as *const c_int as *mut c_void,
            extra2: &SYSCTL_INT_MAX as *const c_int as *mut c_void,
        },
    ]
};

unsafe extern "C" fn init_security_keys_sysctls() -> c_int {
    register_sysctl_init(
        b"kernel/keys\0".as_ptr() as *const c_char,
        KEY_SYSCTLS.as_ptr(),
    );
    0
}

/* early_initcall(init_security_keys_sysctls); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
