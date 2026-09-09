/* SPDX-License-Identifier: GPL-2.0-or-later */
/*  Syslog internals
 *
 *  Copyright 2010 Canonical, Ltd.
 *  Author: Kees Cook <kees.cook@canonical.com>
 */

// Dependency supplied by linux/wait.h in the translated source tree.

/* Close the log.  Currently a NOP. */
pub const SYSLOG_ACTION_CLOSE: i32 = 0;
/* Open the log. Currently a NOP. */
pub const SYSLOG_ACTION_OPEN: i32 = 1;
/* Read from the log. */
pub const SYSLOG_ACTION_READ: i32 = 2;
/* Read all messages remaining in the ring buffer. */
pub const SYSLOG_ACTION_READ_ALL: i32 = 3;
/* Read and clear all messages remaining in the ring buffer */
pub const SYSLOG_ACTION_READ_CLEAR: i32 = 4;
/* Clear ring buffer. */
pub const SYSLOG_ACTION_CLEAR: i32 = 5;
/* Disable printk's to console */
pub const SYSLOG_ACTION_CONSOLE_OFF: i32 = 6;
/* Enable printk's to console */
pub const SYSLOG_ACTION_CONSOLE_ON: i32 = 7;
/* Set level of messages printed to console */
pub const SYSLOG_ACTION_CONSOLE_LEVEL: i32 = 8;
/* Return number of unread characters in the log buffer */
pub const SYSLOG_ACTION_SIZE_UNREAD: i32 = 9;
/* Return size of the log buffer */
pub const SYSLOG_ACTION_SIZE_BUFFER: i32 = 10;

pub const SYSLOG_FROM_READER: i32 = 0;
pub const SYSLOG_FROM_PROC: i32 = 1;

unsafe extern "C" {
    pub fn do_syslog(
        type_: ::core::ffi::c_int,
        buf: *mut ::core::ffi::c_char,
        count: ::core::ffi::c_int,
        source: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;

    pub static mut log_wait: wait_queue_head_t;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
