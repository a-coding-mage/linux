/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Dependency: `__kernel_old_time_t` is supplied by the translated linux/types.h.
#[repr(C)]
pub struct utimbuf {
    pub actime: __kernel_old_time_t,
    pub modtime: __kernel_old_time_t,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
