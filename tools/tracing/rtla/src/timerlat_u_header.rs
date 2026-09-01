// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2023 Red Hat Inc, Daniel Bristot de Oliveira <bristot@kernel.org>
 */

// Dependencies from included C headers / surrounding bindings:
// cpu_set_t
// sched_attr

#[repr(C)]
pub struct timerlat_u_params {
    /* timerlat -> timerlat_u: user-space threads can keep running */
    pub should_run: ::std::os::raw::c_int,
    /* timerlat_u -> timerlat: all timerlat_u threads left, no reason to continue */
    pub stopped_running: ::std::os::raw::c_int,

    /* threads config */
    pub set: *mut cpu_set_t,
    pub cgroup_name: *mut ::std::os::raw::c_char,
    pub sched_param: *mut sched_attr,
}

extern "C" {
    pub fn timerlat_u_dispatcher(data: *mut ::std::os::raw::c_void) -> *mut ::std::os::raw::c_void;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
