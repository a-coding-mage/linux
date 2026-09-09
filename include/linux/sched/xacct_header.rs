/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Extended task accounting methods.
 *
 * The original header includes linux/sched.h.  The task_struct and ssize_t
 * definitions are supplied by that dependency.
 */

// CONFIG_TASK_XACCT is a build-time configuration condition inherited from
// the C header and is represented here with the corresponding cfg predicate.
#[cfg(CONFIG_TASK_XACCT)]
#[inline]
pub unsafe fn add_rchar(tsk: *mut task_struct, amt: ssize_t) {
    (*tsk).ioac.rchar += amt;
}

#[cfg(CONFIG_TASK_XACCT)]
#[inline]
pub unsafe fn add_wchar(tsk: *mut task_struct, amt: ssize_t) {
    (*tsk).ioac.wchar += amt;
}

#[cfg(CONFIG_TASK_XACCT)]
#[inline]
pub unsafe fn inc_syscr(tsk: *mut task_struct) {
    (*tsk).ioac.syscr += 1;
}

#[cfg(CONFIG_TASK_XACCT)]
#[inline]
pub unsafe fn inc_syscw(tsk: *mut task_struct) {
    (*tsk).ioac.syscw += 1;
}

#[cfg(not(CONFIG_TASK_XACCT))]
#[inline]
pub unsafe fn add_rchar(_tsk: *mut task_struct, _amt: ssize_t) {}

#[cfg(not(CONFIG_TASK_XACCT))]
#[inline]
pub unsafe fn add_wchar(_tsk: *mut task_struct, _amt: ssize_t) {}

#[cfg(not(CONFIG_TASK_XACCT))]
#[inline]
pub unsafe fn inc_syscr(_tsk: *mut task_struct) {}

#[cfg(not(CONFIG_TASK_XACCT))]
#[inline]
pub unsafe fn inc_syscw(_tsk: *mut task_struct) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
