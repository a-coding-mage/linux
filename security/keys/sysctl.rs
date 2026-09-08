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
    #[cfg(CONFIG_PERSISTENT_KEYRINGS)]
    static mut persistent_keyring_expiry: c_uint;
    static SYSCTL_ONE: c_int;
    static SYSCTL_ZERO: c_int;
    static SYSCTL_INT_MAX: c_int;
    fn proc_dointvec_minmax(table: *mut ctl_table, write: c_int, buffer: *mut c_void, lenp: *mut usize, ppos: *mut i64) -> c_int;
    fn register_sysctl_init(path: *const c_char, table: *const ctl_table);
}

type ProcHandler = unsafe extern "C" fn(*mut ctl_table, c_int, *mut c_void, *mut usize, *mut i64) -> c_int;

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

macro_rules! key_ctl {
    ($name:literal, $data:ident, $min:ident) => {
        ctl_table { procname: concat!($name, "\0").as_ptr() as *const c_char,
            data: unsafe { &mut $data as *mut c_uint as *mut c_void },
            maxlen: core::mem::size_of::<c_uint>(), mode: 0o644,
            proc_handler: Some(proc_dointvec_minmax),
            extra1: unsafe { &$min as *const c_int as *mut c_void },
            extra2: unsafe { &SYSCTL_INT_MAX as *const c_int as *mut c_void } }
    };
}

static KEY_SYSCTLS: &[ctl_table] = &[
    key_ctl!("maxkeys", key_quota_maxkeys, SYSCTL_ONE),
    key_ctl!("maxbytes", key_quota_maxbytes, SYSCTL_ONE),
    key_ctl!("root_maxkeys", key_quota_root_maxkeys, SYSCTL_ONE),
    key_ctl!("root_maxbytes", key_quota_root_maxbytes, SYSCTL_ONE),
    key_ctl!("gc_delay", key_gc_delay, SYSCTL_ZERO),
    #[cfg(CONFIG_PERSISTENT_KEYRINGS)]
    key_ctl!("persistent_keyring_expiry", persistent_keyring_expiry, SYSCTL_ZERO),
];

unsafe extern "C" fn init_security_keys_sysctls() -> c_int {
    register_sysctl_init(b"kernel/keys\0".as_ptr() as *const c_char, KEY_SYSCTLS.as_ptr());
    0
}

// early_initcall(init_security_keys_sysctls);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
