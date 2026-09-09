/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Generic task switch macro wrapper.
 *
 * It should be possible to use these on really simple architectures,
 * but it serves more as a starting point for new ports.
 *
 * Copyright (C) 2007 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

// Dependency supplied by linux/thread_info.h.
#[repr(C)]
pub struct task_struct {
    _private: [u8; 0],
}

/*
 * Context switching is now performed out-of-line in switch_to.S
 */
unsafe extern "C" {
    pub fn __switch_to(
        prev: *mut task_struct,
        next: *mut task_struct,
    ) -> *mut task_struct;
}

macro_rules! switch_to {
    ($prev:expr, $next:expr, $last:expr) => {{
        $last = unsafe { __switch_to($prev, $next) };
    }};
}

pub(crate) use switch_to;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
