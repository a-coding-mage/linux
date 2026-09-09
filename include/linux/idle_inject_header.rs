/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2018 Linaro Ltd
 *
 * Author: Daniel Lezcano <daniel.lezcano@linaro.org>
 *
 */

/* private idle injection device structure */
#[repr(C)]
pub struct idle_inject_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct cpumask {
    _private: [u8; 0],
}

extern "C" {
    pub fn idle_inject_register(cpumask: *mut cpumask) -> *mut idle_inject_device;

    pub fn idle_inject_register_full(
        cpumask: *mut cpumask,
        update: Option<unsafe extern "C" fn() -> bool>,
    ) -> *mut idle_inject_device;

    pub fn idle_inject_unregister(ii_dev: *mut idle_inject_device);

    pub fn idle_inject_start(ii_dev: *mut idle_inject_device) -> ::core::ffi::c_int;

    pub fn idle_inject_stop(ii_dev: *mut idle_inject_device);

    pub fn idle_inject_set_duration(
        ii_dev: *mut idle_inject_device,
        run_duration_us: ::core::ffi::c_uint,
        idle_duration_us: ::core::ffi::c_uint,
    );

    pub fn idle_inject_get_duration(
        ii_dev: *mut idle_inject_device,
        run_duration_us: *mut ::core::ffi::c_uint,
        idle_duration_us: *mut ::core::ffi::c_uint,
    );

    pub fn idle_inject_set_latency(
        ii_dev: *mut idle_inject_device,
        latency_us: ::core::ffi::c_uint,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
