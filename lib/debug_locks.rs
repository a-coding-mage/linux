// SPDX-License-Identifier: GPL-2.0-only
/*
 * lib/debug_locks.c
 *
 * Generic place for common debugging facilities for various locks:
 * spinlocks, rwlocks, mutexes and rwsems.
 *
 * Started by Ingo Molnar:
 *
 *  Copyright (C) 2006 Red Hat, Inc., Ingo Molnar <mingo@redhat.com>
 */

// Dependencies: linux/debug_locks.h, linux/printk.h, linux/export.h.

use core::ptr::{addr_of, read_volatile};
use kernel::ffi::c_int;

extern "C" {
    fn console_verbose();
    fn lupos_debug_locks_xchg_off() -> c_int;
}

/*
 * We want to turn all lock-debugging facilities on/off at once,
 * via a global flag. The reason is that once a single bug has been
 * detected and reported, there might be cascade of followup bugs
 * that would just muddy the log. So we report the first one and
 * shut up after that.
 */
/// The global lock-debugging enable flag.
#[no_mangle]
#[link_section = ".data..read_mostly"]
pub static mut debug_locks: c_int = 1;
// EXPORT_SYMBOL_GPL(debug_locks);

/*
 * The locking-testsuite uses <debug_locks_silent> to get a
 * 'silent failure': nothing is printed to the console when
 * a locking bug is detected.
 */
/// Suppresses console output for detected locking bugs when nonzero.
#[no_mangle]
#[link_section = ".data..read_mostly"]
pub static mut debug_locks_silent: c_int = 0;
// EXPORT_SYMBOL_GPL(debug_locks_silent);

/*
 * Generic 'turn off all lock debugging' function:
 */
/// Turns all lock debugging off; returns 1 when the caller should report.
#[no_mangle]
pub extern "C" fn debug_locks_off() -> c_int {
    // SAFETY: both flags are always-valid globals; __debug_locks_off() is
    // the native fully ordered xchg() and console_verbose() a plain call.
    unsafe {
        if read_volatile(addr_of!(debug_locks)) != 0 && lupos_debug_locks_xchg_off() != 0 {
            if read_volatile(addr_of!(debug_locks_silent)) == 0 {
                console_verbose();
                return 1;
            }
        }
    }
    0
}
// EXPORT_SYMBOL_GPL(debug_locks_off);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
