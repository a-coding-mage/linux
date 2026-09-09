// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * FIPS 200 support.
 *
 * Copyright (c) 2008 Neil Horman <nhorman@tuxdriver.com>
 */

use core::ffi::{c_char, c_int, c_void};

// Kernel-provided declarations from the included headers.
extern "C" {
    fn kstrtoint(s: *const c_char, base: c_uint, result: *mut c_int) -> c_int;
    fn str_enabled_disabled(enabled: c_int) -> *const c_char;
    fn pr_info(format: *const c_char, ...);
    fn register_sysctl(path: *const c_char, table: *const ctl_table) -> *mut ctl_table_header;
    fn unregister_sysctl_table(header: *mut ctl_table_header);
    fn proc_dointvec(
        table: *mut ctl_table,
        write: c_int,
        buffer: *mut c_void,
        lenp: *mut usize,
        ppos: *mut i64,
    ) -> c_int;
    fn proc_dostring(
        table: *mut ctl_table,
        write: c_int,
        buffer: *mut c_void,
        lenp: *mut usize,
        ppos: *mut i64,
    ) -> c_int;
    fn atomic_notifier_call_chain(
        chain: *mut c_void,
        value: c_ulong,
        v: *mut c_void,
    ) -> c_int;
}

type c_uint = u32;
type c_ulong = usize;

#[repr(C)]
pub struct ctl_table {
    pub procname: *const c_char,
    pub data: *mut c_void,
    pub maxlen: usize,
    pub mode: u16,
    pub proc_handler: Option<unsafe extern "C" fn(
        *mut ctl_table,
        c_int,
        *mut c_void,
        *mut usize,
        *mut i64,
    ) -> c_int>,
}

#[repr(C)]
pub struct ctl_table_header {
    _private: [u8; 0],
}

#[no_mangle]
pub static mut fips_enabled: c_int = 0;

// ATOMIC_NOTIFIER_HEAD(fips_fail_notif_chain);
#[no_mangle]
pub static mut fips_fail_notif_chain: c_void = unsafe { core::mem::zeroed() };

/* Process kernel command-line parameter at boot time. fips=0 or fips=1 */
unsafe extern "C" fn fips_enable(str_: *mut c_char) -> c_int {
    if kstrtoint(str_, 0, &mut fips_enabled) != 0 {
        return 0;
    }

    fips_enabled = (fips_enabled != 0) as c_int;
    pr_info(b"fips mode: %s\n\0".as_ptr() as *const c_char, str_enabled_disabled(fips_enabled));
    1
}

// __setup("fips=", fips_enable);

// FIPS_MODULE_NAME is CONFIG_CRYPTO_FIPS_NAME.  When
// CONFIG_CRYPTO_FIPS_CUSTOM_VERSION is unset, FIPS_MODULE_VERSION is UTS_RELEASE.
#[cfg(feature = "CONFIG_CRYPTO_FIPS_NAME")]
const FIPS_MODULE_NAME: &[u8] = env!("CONFIG_CRYPTO_FIPS_NAME").as_bytes();
#[cfg(not(feature = "CONFIG_CRYPTO_FIPS_NAME"))]
const FIPS_MODULE_NAME: &[u8] = b"";

#[cfg(feature = "CONFIG_CRYPTO_FIPS_CUSTOM_VERSION")]
const FIPS_MODULE_VERSION: &[u8] = env!("CONFIG_CRYPTO_FIPS_VERSION").as_bytes();
#[cfg(not(feature = "CONFIG_CRYPTO_FIPS_CUSTOM_VERSION"))]
const FIPS_MODULE_VERSION: &[u8] = env!("CARGO_PKG_VERSION").as_bytes();

static mut fips_name: [u8; 1] = [0];
static mut fips_version: [u8; 1] = [0];

static mut crypto_sysctl_table: [ctl_table; 3] = [
    ctl_table {
        procname: b"fips_enabled\0".as_ptr() as *const c_char,
        data: unsafe { &mut fips_enabled as *mut c_int as *mut c_void },
        maxlen: core::mem::size_of::<c_int>(),
        mode: 0o444,
        proc_handler: Some(proc_dointvec),
    },
    ctl_table {
        procname: b"fips_name\0".as_ptr() as *const c_char,
        data: unsafe { &mut fips_name as *mut [u8; 1] as *mut c_void },
        maxlen: 64,
        mode: 0o444,
        proc_handler: Some(proc_dostring),
    },
    ctl_table {
        procname: b"fips_version\0".as_ptr() as *const c_char,
        data: unsafe { &mut fips_version as *mut [u8; 1] as *mut c_void },
        maxlen: 64,
        mode: 0o444,
        proc_handler: Some(proc_dostring),
    },
];

static mut crypto_sysctls: *mut ctl_table_header = core::ptr::null_mut();

unsafe fn crypto_proc_fips_init() {
    crypto_sysctls = register_sysctl(
        b"crypto\0".as_ptr() as *const c_char,
        crypto_sysctl_table.as_ptr(),
    );
}

unsafe fn crypto_proc_fips_exit() {
    unregister_sysctl_table(crypto_sysctls);
}

#[no_mangle]
pub unsafe extern "C" fn fips_fail_notify() {
    if fips_enabled != 0 {
        atomic_notifier_call_chain(&mut fips_fail_notif_chain as *mut c_void, 0, core::ptr::null_mut());
    }
}

// EXPORT_SYMBOL_GPL(fips_fail_notify);

unsafe extern "C" fn fips_init() -> c_int {
    crypto_proc_fips_init();
    0
}

unsafe extern "C" fn fips_exit() {
    crypto_proc_fips_exit();
}

// module_init(fips_init);
// module_exit(fips_exit);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
