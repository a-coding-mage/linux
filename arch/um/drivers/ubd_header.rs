/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2000 Jeff Dike (jdike@karaya.com)
 * Copyright (C) 2001 RidgeRun, Inc (glonnon@ridgerun.com)
 */

// C dependency: <os.h>

use core::ffi::{c_int, c_void};

// Opaque type supplied by <os.h>.
pub enum os_helper_thread {}

extern "C" {
    pub fn start_io_thread(
        td_out: *mut *mut os_helper_thread,
        fd_out: *mut c_int,
    ) -> c_int;
    pub fn io_thread(arg: *mut c_void) -> *mut c_void;
    pub static mut kernel_fd: c_int;

    pub fn ubd_read_poll(timeout: c_int) -> c_int;
    pub fn ubd_write_poll(timeout: c_int) -> c_int;
}

pub const UBD_REQ_BUFFER_SIZE: usize = 64;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
