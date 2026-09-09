/* SPDX-License-Identifier: GPL-2.0 */

// TRACE_SYSTEM capability
//
// The declarations below correspond to the Linux credential, tracepoint, and
// user-namespace headers included by the original C header.

use core::ffi::c_int;

/// Opaque type supplied by the credential subsystem.
#[repr(C)]
pub struct cred {
    _private: [u8; 0],
}

/// Opaque type supplied by the user-namespace subsystem.
#[repr(C)]
pub struct user_namespace {
    _private: [u8; 0],
}

/// Fields stored by the `cap_capable` trace event.
#[repr(C)]
pub struct CapCapableEntry {
    pub cred: *const cred,
    pub target_ns: *mut user_namespace,
    pub capable_ns: *const user_namespace,
    pub cap: c_int,
    pub ret: c_int,
}

/**
 * cap_capable - called after it's determined if a task has a particular
 * effective capability
 *
 * @cred: The credentials used
 * @target_ns: The user namespace of the resource being accessed
 * @capable_ns: The user namespace in which the credential provides the
 *              capability to access the targeted resource.
 *              This will be NULL if ret is not 0.
 * @cap: The capability to check for
 * @ret: The return value of the check: 0 if it does, -ve if it does not
 *
 * Allows to trace calls to cap_capable in commoncap.c
 */

/// Equivalent to the `TP_fast_assign` body for `cap_capable`.
#[inline]
pub unsafe fn cap_capable_fast_assign(
    entry: *mut CapCapableEntry,
    cred: *const cred,
    target_ns: *mut user_namespace,
    capable_ns: *const user_namespace,
    cap: c_int,
    ret: c_int,
) {
    (*entry).cred = cred;
    (*entry).target_ns = target_ns;
    (*entry).capable_ns = if ret == 0 {
        capable_ns
    } else {
        core::ptr::null()
    };
    (*entry).cap = cap;
    (*entry).ret = ret;
}

/// The trace event's printk format:
/// `cred %p, target_ns %p, capable_ns %p, cap %d, ret %d`.
pub const CAP_CAPABLE_PRINTK_FORMAT: &str =
    "cred %p, target_ns %p, capable_ns %p, cap %d, ret %d";


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
