/* SPDX-License-Identifier: GPL-2.0 */
/*
 * tsacct_kern.h - kernel header for system accounting over taskstats interface
 *
 * Copyright (C) Jay Lan\tSGI
 */

// Dependency supplied by the translated taskstats header and kernel types:
// `user_namespace`, `pid_namespace`, `taskstats`, and `task_struct`.

#[cfg(feature = "CONFIG_TASKSTATS")]
extern "C" {
    pub fn bacct_add_tsk(
        user_ns: *mut user_namespace,
        pid_ns: *mut pid_namespace,
        stats: *mut taskstats,
        tsk: *mut task_struct,
    );
}

#[cfg(not(feature = "CONFIG_TASKSTATS"))]
#[inline]
pub unsafe fn bacct_add_tsk(
    _user_ns: *mut user_namespace,
    _pid_ns: *mut pid_namespace,
    _stats: *mut taskstats,
    _tsk: *mut task_struct,
) {
}

#[cfg(feature = "CONFIG_TASK_XACCT")]
extern "C" {
    pub fn xacct_add_tsk(stats: *mut taskstats, p: *mut task_struct);
    pub fn acct_update_integrals(tsk: *mut task_struct);
    pub fn acct_account_cputime(tsk: *mut task_struct);
    pub fn acct_clear_integrals(tsk: *mut task_struct);
}

#[cfg(not(feature = "CONFIG_TASK_XACCT"))]
#[inline]
pub unsafe fn xacct_add_tsk(_stats: *mut taskstats, _p: *mut task_struct) {}

#[cfg(not(feature = "CONFIG_TASK_XACCT"))]
#[inline]
pub unsafe fn acct_update_integrals(_tsk: *mut task_struct) {}

#[cfg(not(feature = "CONFIG_TASK_XACCT"))]
#[inline]
pub unsafe fn acct_account_cputime(_tsk: *mut task_struct) {}

#[cfg(not(feature = "CONFIG_TASK_XACCT"))]
#[inline]
pub unsafe fn acct_clear_integrals(_tsk: *mut task_struct) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
