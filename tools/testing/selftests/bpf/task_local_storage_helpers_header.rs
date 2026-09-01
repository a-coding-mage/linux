// SPDX-License-Identifier: GPL-2.0
//
// C header dependencies:
// - <unistd.h> provides syscall
// - <sys/syscall.h> may provide __NR_pidfd_open
// - <sys/types.h> provides pid_t

use std::os::raw::{c_int, c_long, c_uint};

// Fallback definition used when __NR_pidfd_open is not provided by sys/syscall.h.
// The C source uses __alpha__ to select the architecture-specific syscall number.
pub const __NR_pidfd_open: c_long = if cfg!(target_arch = "alpha") {
    544
} else {
    434
};

unsafe extern "C" {
    pub fn syscall(num: c_long, ...) -> c_long;
}

#[inline]
pub unsafe fn sys_pidfd_open(pid: libc::pid_t, flags: c_uint) -> c_int {
    unsafe { syscall(__NR_pidfd_open, pid, flags) as c_int }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
