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

/* Dependencies supplied by the surrounding kernel translation unit. */
unsafe extern "C" {
    fn __debug_locks_off() -> i32;
    fn console_verbose();
}

/*
 * We want to turn all lock-debugging facilities on/off at once,
 * via a global flag. The reason is that once a single bug has been
 * detected and reported, there might be cascade of followup bugs
 * that would just muddy the log. So we report the first one and
 * shut up after that.
 */
#[no_mangle]
pub static mut debug_locks: i32 = 1;
// EXPORT_SYMBOL_GPL(debug_locks);

/*
 * The locking-testsuite uses <debug_locks_silent> to get a
 * 'silent failure': nothing is printed to the console when
 * a locking bug is detected.
 */
#[no_mangle]
pub static mut debug_locks_silent: i32 = 0;
// EXPORT_SYMBOL_GPL(debug_locks_silent);

/*
 * Generic 'turn off all lock debugging' function:
 */
#[no_mangle]
pub unsafe extern "C" fn debug_locks_off() -> i32 {
    if debug_locks != 0 && __debug_locks_off() != 0 {
        if debug_locks_silent == 0 {
            console_verbose();
            return 1;
        }
    }
    0
}
// EXPORT_SYMBOL_GPL(debug_locks_off);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
