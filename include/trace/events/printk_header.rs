/* SPDX-License-Identifier: GPL-2.0 */

//! Rust translation of the Linux `printk` trace-event header.
//!
//! The trace-event framework supplies the event registration and formatting
//! machinery represented by `TRACE_EVENT`, `TP_STRUCT__entry`, and friends in
//! the original header.

use core::ffi::c_char;

/// Equivalent to the dynamically sized `msg` field of the trace entry.
#[repr(C)]
pub struct ConsoleEntry {
    /// Storage for `len + 1` characters, as allocated by the trace framework.
    pub msg: *mut c_char,
}

/// Fast assignment for the `console` trace event.
///
/// The caller supplies storage for the dynamic `msg` array, matching the
/// original `__dynamic_array(char, msg, len + 1)` entry.
#[inline]
pub unsafe fn console_fast_assign(
    entry: *mut ConsoleEntry,
    text: *const c_char,
    mut len: usize,
) {
    /*
     * Each trace entry is printed in a new line.
     * If the msg finishes with '\n', cut it off
     * to avoid blank lines in the trace.
     */
    if len > 0 && *text.add(len - 1) == b'\n' as c_char {
        len -= 1;
    }

    // Equivalent to: memcpy(__get_str(msg), text, len);
    core::ptr::copy_nonoverlapping(text, (*entry).msg, len);
    // Equivalent to: __get_str(msg)[len] = 0;
    *(*entry).msg.add(len) = 0;
}

/// Trace-event declaration corresponding to:
///
/// `TRACE_EVENT(console, TP_PROTO(const char *text, size_t len), ...)`
///
/// The trace framework provides the generated registration and invocation
/// interfaces. Its print format is `%s`, using the entry's `msg` string.
pub const CONSOLE_TRACE_PRINT_FORMAT: &str = "%s";


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
