/* SPDX-License-Identifier: GPL-2.0-only */
/* Copyright 2024-2025 Tomeu Vizoso <tomeu@tomeuvizoso.net> */

// Dependencies supplied by the corresponding kernel and Rocket declarations.

#[repr(C)]
pub struct rocket_device {
    pub ddev: drm_device,
    pub sched_lock: mutex,
    pub cores: *mut rocket_core,
    pub num_cores: core::ffi::c_uint,
}

unsafe extern "C" {
    pub fn rocket_device_init(
        pdev: *mut platform_device,
        rocket_drm_driver: *const drm_driver,
    ) -> *mut rocket_device;
    pub fn rocket_device_fini(rdev: *mut rocket_device);
}

#[inline]
pub unsafe fn to_rocket_device(drm_dev: *mut drm_device) -> *mut rocket_device {
    (drm_dev as *mut u8)
        .sub(core::mem::offset_of!(rocket_device, ddev))
        .cast::<rocket_device>()
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
