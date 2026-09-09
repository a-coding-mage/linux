// SPDX-License-Identifier: GPL-2.0
/*
 * sysctl_net_llc.c: sysctl interface to LLC net subsystem.
 *
 * Arnaldo Carvalho de Melo <acme@conectiva.com.br>
 */

use core::ffi::{c_char, c_int, c_void};

// Dependency declarations supplied by the Linux kernel headers.
#[repr(C)]
pub struct ctl_table {
    pub procname: *const c_char,
    pub data: *mut c_void,
    pub maxlen: usize,
    pub mode: u16,
    pub proc_handler: Option<unsafe extern "C" fn()>,
}

#[repr(C)]
pub struct ctl_table_header {
    _private: [u8; 0],
}

#[repr(C)]
pub struct net {
    _private: [u8; 0],
}

unsafe extern "C" {
    static mut init_net: net;
    static mut sysctl_llc2_ack_timeout: c_int;
    static mut sysctl_llc2_busy_timeout: c_int;
    static mut sysctl_llc2_p_timeout: c_int;
    static mut sysctl_llc2_rej_timeout: c_int;

    unsafe fn proc_dointvec_jiffies();
    unsafe fn register_net_sysctl(
        net: *mut net,
        path: *const c_char,
        table: *mut ctl_table,
    ) -> *mut ctl_table_header;
    unsafe fn register_net_sysctl_sz(
        net: *mut net,
        path: *const c_char,
        table: *mut ctl_table,
        table_size: usize,
    ) -> *mut ctl_table_header;
    unsafe fn unregister_net_sysctl_table(header: *mut ctl_table_header);
}

static mut llc2_timeout_table: [ctl_table; 4] = [
    ctl_table {
        procname: b"ack\0".as_ptr() as *const c_char,
        data: core::ptr::addr_of_mut!(sysctl_llc2_ack_timeout) as *mut c_void,
        maxlen: core::mem::size_of::<c_int>(),
        mode: 0o644,
        proc_handler: Some(proc_dointvec_jiffies),
    },
    ctl_table {
        procname: b"busy\0".as_ptr() as *const c_char,
        data: core::ptr::addr_of_mut!(sysctl_llc2_busy_timeout) as *mut c_void,
        maxlen: core::mem::size_of::<c_int>(),
        mode: 0o644,
        proc_handler: Some(proc_dointvec_jiffies),
    },
    ctl_table {
        procname: b"p\0".as_ptr() as *const c_char,
        data: core::ptr::addr_of_mut!(sysctl_llc2_p_timeout) as *mut c_void,
        maxlen: core::mem::size_of::<c_int>(),
        mode: 0o644,
        proc_handler: Some(proc_dointvec_jiffies),
    },
    ctl_table {
        procname: b"rej\0".as_ptr() as *const c_char,
        data: core::ptr::addr_of_mut!(sysctl_llc2_rej_timeout) as *mut c_void,
        maxlen: core::mem::size_of::<c_int>(),
        mode: 0o644,
        proc_handler: Some(proc_dointvec_jiffies),
    },
];

static mut llc2_timeout_header: *mut ctl_table_header = core::ptr::null_mut();
static mut llc_station_header: *mut ctl_table_header = core::ptr::null_mut();

// __init
pub unsafe extern "C" fn llc_sysctl_init() -> c_int {
    static mut empty: [ctl_table; 1] = [ctl_table {
        procname: core::ptr::null(),
        data: core::ptr::null_mut(),
        maxlen: 0,
        mode: 0,
        proc_handler: None,
    }];

    llc2_timeout_header = register_net_sysctl(
        core::ptr::addr_of_mut!(init_net),
        b"net/llc/llc2/timeout\0".as_ptr() as *const c_char,
        core::ptr::addr_of_mut!(llc2_timeout_table) as *mut ctl_table,
    );
    llc_station_header = register_net_sysctl_sz(
        core::ptr::addr_of_mut!(init_net),
        b"net/llc/station\0".as_ptr() as *const c_char,
        core::ptr::addr_of_mut!(empty) as *mut ctl_table,
        0,
    );

    if llc2_timeout_header.is_null() || llc_station_header.is_null() {
        llc_sysctl_exit();
        return -12;
    }
    0
}

pub unsafe extern "C" fn llc_sysctl_exit() {
    if !llc2_timeout_header.is_null() {
        unregister_net_sysctl_table(llc2_timeout_header);
        llc2_timeout_header = core::ptr::null_mut();
    }
    if !llc_station_header.is_null() {
        unregister_net_sysctl_table(llc_station_header);
        llc_station_header = core::ptr::null_mut();
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
