// SPDX-License-Identifier: GPL-2.0-only
/* rust_binder_events.c
 *
 * Rust Binder tracepoints.
 *
 * Copyright 2025 Google LLC
 */

// Dependency supplied by rust_binder.h in the C source.
use core::ffi::c_char;

pub static binder_command_strings: [*const c_char; 22] = [
    b"BC_TRANSACTION\0".as_ptr() as *const c_char,
    b"BC_REPLY\0".as_ptr() as *const c_char,
    b"BC_ACQUIRE_RESULT\0".as_ptr() as *const c_char,
    b"BC_FREE_BUFFER\0".as_ptr() as *const c_char,
    b"BC_INCREFS\0".as_ptr() as *const c_char,
    b"BC_ACQUIRE\0".as_ptr() as *const c_char,
    b"BC_RELEASE\0".as_ptr() as *const c_char,
    b"BC_DECREFS\0".as_ptr() as *const c_char,
    b"BC_INCREFS_DONE\0".as_ptr() as *const c_char,
    b"BC_ACQUIRE_DONE\0".as_ptr() as *const c_char,
    b"BC_ATTEMPT_ACQUIRE\0".as_ptr() as *const c_char,
    b"BC_REGISTER_LOOPER\0".as_ptr() as *const c_char,
    b"BC_ENTER_LOOPER\0".as_ptr() as *const c_char,
    b"BC_EXIT_LOOPER\0".as_ptr() as *const c_char,
    b"BC_REQUEST_DEATH_NOTIFICATION\0".as_ptr() as *const c_char,
    b"BC_CLEAR_DEATH_NOTIFICATION\0".as_ptr() as *const c_char,
    b"BC_DEAD_BINDER_DONE\0".as_ptr() as *const c_char,
    b"BC_TRANSACTION_SG\0".as_ptr() as *const c_char,
    b"BC_REPLY_SG\0".as_ptr() as *const c_char,
    b"BC_REQUEST_FREEZE_NOTIFICATION\0".as_ptr() as *const c_char,
    b"BC_CLEAR_FREEZE_NOTIFICATION\0".as_ptr() as *const c_char,
    b"BC_FREEZE_NOTIFICATION_DONE\0".as_ptr() as *const c_char,
];

pub static binder_return_strings: [*const c_char; 23] = [
    b"BR_ERROR\0".as_ptr() as *const c_char,
    b"BR_OK\0".as_ptr() as *const c_char,
    b"BR_TRANSACTION\0".as_ptr() as *const c_char,
    b"BR_REPLY\0".as_ptr() as *const c_char,
    b"BR_ACQUIRE_RESULT\0".as_ptr() as *const c_char,
    b"BR_DEAD_REPLY\0".as_ptr() as *const c_char,
    b"BR_TRANSACTION_COMPLETE\0".as_ptr() as *const c_char,
    b"BR_INCREFS\0".as_ptr() as *const c_char,
    b"BR_ACQUIRE\0".as_ptr() as *const c_char,
    b"BR_RELEASE\0".as_ptr() as *const c_char,
    b"BR_DECREFS\0".as_ptr() as *const c_char,
    b"BR_ATTEMPT_ACQUIRE\0".as_ptr() as *const c_char,
    b"BR_NOOP\0".as_ptr() as *const c_char,
    b"BR_SPAWN_LOOPER\0".as_ptr() as *const c_char,
    b"BR_FINISHED\0".as_ptr() as *const c_char,
    b"BR_DEAD_BINDER\0".as_ptr() as *const c_char,
    b"BR_CLEAR_DEATH_NOTIFICATION_DONE\0".as_ptr() as *const c_char,
    b"BR_FAILED_REPLY\0".as_ptr() as *const c_char,
    b"BR_FROZEN_REPLY\0".as_ptr() as *const c_char,
    b"BR_ONEWAY_SPAM_SUSPECT\0".as_ptr() as *const c_char,
    b"BR_TRANSACTION_PENDING_FROZEN\0".as_ptr() as *const c_char,
    b"BR_FROZEN_BINDER\0".as_ptr() as *const c_char,
    b"BR_CLEAR_FREEZE_NOTIFICATION_DONE\0".as_ptr() as *const c_char,
];

// CREATE_TRACE_POINTS
// CREATE_RUST_TRACE_POINTS
// Dependency supplied by rust_binder_events.h in the C source.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
