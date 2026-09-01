/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2019  Arm Limited
 * Original author: Dave Martin <Dave.Martin@arm.com>
 */

/* Depends on C declarations from <linux/signal.h> and "system.h". */

pub type sighandler_t = __sighandler_t;

unsafe extern "C" {
    pub fn sigemptyset(s: *mut sigset_t) -> ::std::os::raw::c_int;
    pub fn sigaddset(s: *mut sigset_t, n: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
    pub fn sigaction(
        n: ::std::os::raw::c_int,
        sa: *mut sigaction,
        old: *const sigaction,
    ) -> ::std::os::raw::c_int;
    pub fn sigprocmask(
        how: ::std::os::raw::c_int,
        mask: *const sigset_t,
        old: *mut sigset_t,
    ) -> ::std::os::raw::c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
