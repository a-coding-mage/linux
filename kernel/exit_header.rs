// SPDX-License-Identifier: GPL-2.0-only

#[repr(C)]
pub struct waitid_info {
    pub pid: pid_t,
    pub uid: uid_t,
    pub status: ::core::ffi::c_int,
    pub cause: ::core::ffi::c_int,
}

#[repr(C)]
pub struct wait_opts {
    pub wo_type: pid_type,
    pub wo_flags: ::core::ffi::c_int,
    pub wo_pid: *mut pid,

    pub wo_info: *mut waitid_info,
    pub wo_stat: ::core::ffi::c_int,
    pub wo_rusage: *mut rusage,

    pub child_wait: wait_queue_entry_t,
    pub notask_error: ::core::ffi::c_int,
}

unsafe extern "C" {
    pub fn pid_child_should_wake(wo: *mut wait_opts, p: *mut task_struct) -> bool;
    pub fn __do_wait(wo: *mut wait_opts) -> ::core::ffi::c_long;
    pub fn kernel_waitid_prepare(
        wo: *mut wait_opts,
        which: ::core::ffi::c_int,
        upid: pid_t,
        infop: *mut waitid_info,
        options: ::core::ffi::c_int,
        ru: *mut rusage,
    ) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
