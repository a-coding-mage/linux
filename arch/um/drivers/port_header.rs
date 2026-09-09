/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2001 Jeff Dike (jdike@karaya.com)
 */

use core::ffi::c_void;
use std::os::raw::c_int;

unsafe extern "C" {
    pub fn port_data(port: c_int) -> *mut c_void;
    pub fn port_wait(data: *mut c_void) -> c_int;
    pub fn port_kern_close(d: *mut c_void);
    pub fn port_connection(fd: c_int, socket_out: *mut c_int, pid_out: *mut c_int) -> c_int;
    pub fn port_listen_fd(port: c_int) -> c_int;
    pub fn port_read(fd: c_int, data: *mut c_void);
    pub fn port_kern_free(d: *mut c_void);
    pub fn port_rcv_fd(fd: c_int) -> c_int;
    pub fn port_remove_dev(d: *mut c_void);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
