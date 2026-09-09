/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2000, 2001 Jeff Dike (jdike@karaya.com)
 */

// Dependencies corresponding to <init.h> and <linux/types.h> are supplied externally.
use core::ffi::{c_char, c_void};

#[repr(C)]
pub struct chan_opts {
    pub announce: Option<unsafe extern "C" fn(dev_name: *mut c_char, dev: i32)>,
    pub xterm_title: *mut c_char,
    pub raw: i32,
}

#[repr(C)]
pub struct chan_ops {
    pub type_: *mut c_char,
    pub init: Option<unsafe extern "C" fn(*mut c_char, i32, *const chan_opts) -> *mut c_void>,
    pub open: Option<unsafe extern "C" fn(i32, i32, i32, *mut c_void, *mut *mut c_char) -> i32>,
    pub close: Option<unsafe extern "C" fn(i32, *mut c_void)>,
    pub read: Option<unsafe extern "C" fn(i32, *mut u8, *mut c_void) -> i32>,
    pub write: Option<unsafe extern "C" fn(i32, *const u8, usize, *mut c_void) -> i32>,
    pub console_write: Option<unsafe extern "C" fn(i32, *const c_char, i32) -> i32>,
    pub window_size:
        Option<unsafe extern "C" fn(i32, *mut c_void, *mut u16, *mut u16) -> i32>,
    pub free: Option<unsafe extern "C" fn(*mut c_void)>,
    pub winch: i32,
}

extern "C" {
    pub static fd_ops: chan_ops;
    pub static null_ops: chan_ops;
    pub static port_ops: chan_ops;
    pub static pts_ops: chan_ops;
    pub static pty_ops: chan_ops;
    pub static tty_ops: chan_ops;
    pub static xterm_ops: chan_ops;

    pub fn generic_close(fd: i32, unused: *mut c_void);
    pub fn generic_read(fd: i32, c_out: *mut u8, unused: *mut c_void) -> i32;
    pub fn generic_write(fd: i32, buf: *const u8, n: usize, unused: *mut c_void) -> i32;
    pub fn generic_console_write(fd: i32, buf: *const c_char, n: i32) -> i32;
    pub fn generic_window_size(
        fd: i32,
        unused: *mut c_void,
        rows_out: *mut u16,
        cols_out: *mut u16,
    ) -> i32;
    pub fn generic_free(data: *mut c_void);
}

#[repr(C)]
pub struct tty_port {
    _private: [u8; 0],
}

extern "C" {
    pub fn register_winch(fd: i32, port: *mut tty_port);
    pub fn register_winch_irq(
        fd: i32,
        tty_fd: i32,
        pid: i32,
        port: *mut tty_port,
        stack: c_ulong,
    );
}

// C's unsigned long, used by register_winch_irq.
pub type c_ulong = usize;

// Equivalent of the source macro; __uml_help! is supplied by <init.h>.
#[macro_export]
macro_rules! __channel_help {
    ($fn:expr, $prefix:expr) => {
        __uml_help!(
            $fn,
            concat!(
                $prefix,
                "[0-9]*=<channel description>\n",
                "    Attach a console or serial line to a host channel.  See\n",
                "    http://user-mode-linux.sourceforge.net/old/input.html for a complete\n",
                "    description of this switch.\n\n"
            )
        );
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
