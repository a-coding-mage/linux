/* SPDX-License-Identifier: GPL-2.0 */

use core::ffi::{c_int, c_long, c_ulong};

// C dependencies: <unistd.h>, <sys/types.h>, <sys/syscall.h>, <linux/compiler.h>

pub type pid_t = c_int;

#[repr(C)]
pub struct perf_event_attr {
    _unused: [u8; 0],
}

unsafe extern "C" {
    pub static __NR_perf_event_open: c_long;

    pub fn syscall(num: c_long, ...) -> c_long;
}

pub unsafe fn sys_perf_event_open(
    attr: *mut perf_event_attr,
    pid: pid_t,
    cpu: c_int,
    group_fd: c_int,
    flags: c_ulong,
) -> c_int {
    unsafe {
        syscall(
            __NR_perf_event_open,
            attr,
            pid,
            cpu,
            group_fd,
            flags,
        ) as c_int
    }
}
