/*
 * Copyright (c) 2006 Oracle.  All rights reserved.
 *
 * This software is available to you under a choice of one of two
 * licenses.  You may choose to be licensed under the terms of the GNU
 * General Public License (GPL) Version 2, available from the file
 * COPYING in the main directory of this source tree, or the
 * OpenIB.org BSD license below:
 *
 *     Redistribution and use in source and binary forms, with or
 *     without modification, are permitted provided that the following
 *     conditions are met:
 *
 *      - Redistributions of source code must retain the above
 *        copyright notice, this list of conditions and the following
 *        disclaimer.
 *
 *      - Redistributions in binary form must reproduce the above
 *        copyright notice, this list of conditions and the following
 *        disclaimer in the documentation and/or other materials
 *        provided with the distribution.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
 * EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
 * MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
 * NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS
 * BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN
 * ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
 * CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

use std::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

// Types and symbols supplied by the Linux kernel and rds.h.
#[repr(C)]
pub struct ctl_table {
    pub procname: *const c_char,
    pub data: *mut c_void,
    pub maxlen: usize,
    pub mode: c_uint,
    pub proc_handler: Option<unsafe extern "C" fn()>,
    pub extra1: *mut c_void,
    pub extra2: *mut c_void,
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
    static HZ: c_ulong;
    fn msecs_to_jiffies(m: c_ulong) -> c_ulong;
    fn register_net_sysctl(
        net: *mut net,
        path: *const c_char,
        table: *mut ctl_table,
    ) -> *mut ctl_table_header;
    fn unregister_net_sysctl_table(header: *mut ctl_table_header);
    fn proc_doulongvec_ms_jiffies_minmax();
    fn proc_dointvec();
}

static mut rds_sysctl_reg_table: *mut ctl_table_header = std::ptr::null_mut();

static mut rds_sysctl_reconnect_min: c_ulong = 1;
static mut rds_sysctl_reconnect_max: c_ulong = !0;

pub static mut rds_sysctl_reconnect_min_jiffies: c_ulong = 0;
pub static mut rds_sysctl_reconnect_max_jiffies: c_ulong = HZ;

pub static mut rds_sysctl_max_unacked_packets: c_uint = 8;
pub static mut rds_sysctl_max_unacked_bytes: c_uint = 16 << 20;

pub static mut rds_sysctl_ping_enable: c_uint = 1;

static mut rds_sysctl_rds_table: [ctl_table; 5] = [
    ctl_table {
        procname: b"reconnect_min_delay_ms\0".as_ptr() as *const c_char,
        data: unsafe { &raw mut rds_sysctl_reconnect_min_jiffies as *mut c_ulong as *mut c_void },
        maxlen: std::mem::size_of::<c_ulong>(),
        mode: 0o644,
        proc_handler: Some(proc_doulongvec_ms_jiffies_minmax),
        extra1: unsafe { &raw mut rds_sysctl_reconnect_min as *mut c_ulong as *mut c_void },
        extra2: unsafe { &raw mut rds_sysctl_reconnect_max_jiffies as *mut c_ulong as *mut c_void },
    },
    ctl_table {
        procname: b"reconnect_max_delay_ms\0".as_ptr() as *const c_char,
        data: unsafe { &raw mut rds_sysctl_reconnect_max_jiffies as *mut c_ulong as *mut c_void },
        maxlen: std::mem::size_of::<c_ulong>(),
        mode: 0o644,
        proc_handler: Some(proc_doulongvec_ms_jiffies_minmax),
        extra1: unsafe { &raw mut rds_sysctl_reconnect_min_jiffies as *mut c_ulong as *mut c_void },
        extra2: unsafe { &raw mut rds_sysctl_reconnect_max as *mut c_ulong as *mut c_void },
    },
    ctl_table {
        procname: b"max_unacked_packets\0".as_ptr() as *const c_char,
        data: unsafe { &raw mut rds_sysctl_max_unacked_packets as *mut c_uint as *mut c_void },
        maxlen: std::mem::size_of::<c_int>(),
        mode: 0o644,
        proc_handler: Some(proc_dointvec),
        extra1: std::ptr::null_mut(),
        extra2: std::ptr::null_mut(),
    },
    ctl_table {
        procname: b"max_unacked_bytes\0".as_ptr() as *const c_char,
        data: unsafe { &raw mut rds_sysctl_max_unacked_bytes as *mut c_uint as *mut c_void },
        maxlen: std::mem::size_of::<c_int>(),
        mode: 0o644,
        proc_handler: Some(proc_dointvec),
        extra1: std::ptr::null_mut(),
        extra2: std::ptr::null_mut(),
    },
    ctl_table {
        procname: b"ping_enable\0".as_ptr() as *const c_char,
        data: unsafe { &raw mut rds_sysctl_ping_enable as *mut c_uint as *mut c_void },
        maxlen: std::mem::size_of::<c_int>(),
        mode: 0o644,
        proc_handler: Some(proc_dointvec),
        extra1: std::ptr::null_mut(),
        extra2: std::ptr::null_mut(),
    },
];

pub unsafe extern "C" fn rds_sysctl_exit() {
    unregister_net_sysctl_table(rds_sysctl_reg_table);
}

pub unsafe extern "C" fn rds_sysctl_init() -> c_int {
    rds_sysctl_reconnect_min = msecs_to_jiffies(1);
    rds_sysctl_reconnect_min_jiffies = rds_sysctl_reconnect_min;

    rds_sysctl_reg_table = register_net_sysctl(
        &raw mut init_net,
        b"net/rds\0".as_ptr() as *const c_char,
        rds_sysctl_rds_table.as_mut_ptr(),
    );
    if rds_sysctl_reg_table.is_null() {
        return -12; // -ENOMEM
    }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
