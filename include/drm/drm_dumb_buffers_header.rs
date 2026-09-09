/* SPDX-License-Identifier: MIT */

#[repr(C)]
pub struct drm_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct drm_mode_create_dumb {
    _private: [u8; 0],
}

pub unsafe extern "C" fn drm_mode_size_dumb(
    dev: *mut drm_device,
    args: *mut drm_mode_create_dumb,
    hw_pitch_align: core::ffi::c_ulong,
    hw_size_align: core::ffi::c_ulong,
) -> core::ffi::c_int;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
