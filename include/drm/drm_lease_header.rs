/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright © 2017 Keith Packard <keithp@keithp.com>
 */

// Dependency intent: types are supplied by the Linux/Rust bindings.

use core::ffi::{c_int, c_void};

#[repr(C)]
pub struct drm_file {
    _private: [u8; 0],
}

#[repr(C)]
pub struct drm_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct drm_master {
    _private: [u8; 0],
}

extern "C" {
    pub fn drm_lease_owner(master: *mut drm_master) -> *mut drm_master;

    pub fn drm_lease_destroy(lessee: *mut drm_master);

    pub fn drm_lease_held(file_priv: *mut drm_file, id: c_int) -> bool;

    pub fn _drm_lease_held(file_priv: *mut drm_file, id: c_int) -> bool;

    pub fn drm_lease_revoke(master: *mut drm_master);

    pub fn drm_lease_filter_crtcs(file_priv: *mut drm_file, crtcs: u32) -> u32;

    pub fn drm_mode_create_lease_ioctl(
        dev: *mut drm_device,
        data: *mut c_void,
        file_priv: *mut drm_file,
    ) -> c_int;

    pub fn drm_mode_list_lessees_ioctl(
        dev: *mut drm_device,
        data: *mut c_void,
        file_priv: *mut drm_file,
    ) -> c_int;

    pub fn drm_mode_get_lease_ioctl(
        dev: *mut drm_device,
        data: *mut c_void,
        file_priv: *mut drm_file,
    ) -> c_int;

    pub fn drm_mode_revoke_lease_ioctl(
        dev: *mut drm_device,
        data: *mut c_void,
        file_priv: *mut drm_file,
    ) -> c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
