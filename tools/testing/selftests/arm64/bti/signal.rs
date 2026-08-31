// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2019  Arm Limited
 * Original author: Dave Martin <Dave.Martin@arm.com>
 */

// C dependencies: "system.h", "signal.h"

use core::ffi::{c_int, c_long, c_uint, c_ulong};

extern "C" {
    fn syscall(number: c_long, ...) -> c_int;
}

pub unsafe extern "C" fn sigemptyset(s: *mut sigset_t) -> c_int {
    let mut i: c_uint = 0;

    while i < _NSIG_WORDS {
        (*s).sig[i as usize] = 0;
        i += 1;
    }

    0
}

pub unsafe extern "C" fn sigaddset(s: *mut sigset_t, n: c_int) -> c_int {
    if n < 1 || n > _NSIG {
        return -EINVAL;
    }

    (*s).sig[((n - 1) / _NSIG_BPW) as usize] |=
        (1 as c_ulong) << ((n - 1) % _NSIG_BPW);
    0
}

pub unsafe extern "C" fn sigaction(
    n: c_int,
    sa: *mut sigaction,
    old: *const sigaction,
) -> c_int {
    syscall(
        __NR_rt_sigaction as c_long,
        n,
        sa,
        old,
        core::mem::size_of::<sigset_t>(),
    )
}

pub unsafe extern "C" fn sigprocmask(
    how: c_int,
    mask: *const sigset_t,
    old: *mut sigset_t,
) -> c_int {
    syscall(
        __NR_rt_sigprocmask as c_long,
        how,
        mask,
        old,
        core::mem::size_of::<sigset_t>(),
    )
}
