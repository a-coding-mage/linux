/* SPDX-License-Identifier: GPL-2.0 */

// Dependency equivalent of: #include <uapi/linux/resource.h>

use std::os::raw::c_int;

#[repr(C)]
pub struct task_struct {
    _private: [u8; 0],
}

extern "C" {
    pub fn getrusage(p: *mut task_struct, who: c_int, ru: *mut crate::rusage);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
