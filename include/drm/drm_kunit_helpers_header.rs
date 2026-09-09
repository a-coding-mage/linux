// SPDX-License-Identifier: GPL-2.0

// Translated from drm_kunit_helpers.h. C header dependencies are supplied by
// the surrounding kernel bindings.

use core::mem::{offset_of, size_of};

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}
#[repr(C)]
pub struct drm_connector {
    _private: [u8; 0],
}
#[repr(C)]
pub struct drm_crtc_funcs {
    _private: [u8; 0],
}
#[repr(C)]
pub struct drm_crtc_helper_funcs {
    _private: [u8; 0],
}
#[repr(C)]
pub struct drm_device {
    _private: [u8; 0],
}
#[repr(C)]
pub struct drm_plane_funcs {
    _private: [u8; 0],
}
#[repr(C)]
pub struct drm_plane_helper_funcs {
    _private: [u8; 0],
}
#[repr(C)]
pub struct kunit {
    _private: [u8; 0],
}
#[repr(C)]
pub struct drm_atomic_commit {
    _private: [u8; 0],
}
#[repr(C)]
pub struct drm_modeset_acquire_ctx {
    _private: [u8; 0],
}
#[repr(C)]
pub struct drm_plane {
    _private: [u8; 0],
}
#[repr(C)]
pub struct drm_crtc {
    _private: [u8; 0],
}
#[repr(C)]
pub struct drm_display_mode {
    _private: [u8; 0],
}

#[repr(C)]
pub struct drm_driver {
    pub driver_features: u32,
}

pub const GFP_KERNEL: u32 = 0;

extern "C" {
    pub fn drm_kunit_helper_alloc_device(test: *mut kunit) -> *mut device;
    pub fn drm_kunit_helper_free_device(test: *mut kunit, dev: *mut device);

    pub fn __drm_kunit_helper_alloc_drm_device_with_driver(
        test: *mut kunit,
        dev: *mut device,
        size: usize,
        offset: usize,
        driver: *const drm_driver,
    ) -> *mut drm_device;

    pub fn devm_kzalloc(dev: *mut device, size: usize, gfp: u32) -> *mut core::ffi::c_void;
    pub fn kunit_assert_not_null(test: *mut kunit, ptr: *const core::ffi::c_void);
    pub fn drm_kunit_helper_atomic_state_alloc(
        test: *mut kunit,
        drm: *mut drm_device,
        ctx: *mut drm_modeset_acquire_ctx,
    ) -> *mut drm_atomic_commit;
    pub fn drm_kunit_helper_create_primary_plane(
        test: *mut kunit,
        drm: *mut drm_device,
        funcs: *const drm_plane_funcs,
        helper_funcs: *const drm_plane_helper_funcs,
        formats: *const u32,
        num_formats: u32,
        modifiers: *const u64,
    ) -> *mut drm_plane;
    pub fn drm_kunit_helper_create_crtc(
        test: *mut kunit,
        drm: *mut drm_device,
        primary: *mut drm_plane,
        cursor: *mut drm_plane,
        funcs: *const drm_crtc_funcs,
        helper_funcs: *const drm_crtc_helper_funcs,
    ) -> *mut drm_crtc;
    pub fn drm_kunit_helper_enable_crtc_connector(
        test: *mut kunit,
        drm: *mut drm_device,
        crtc: *mut drm_crtc,
        connector: *mut drm_connector,
        mode: *const drm_display_mode,
        ctx: *mut drm_modeset_acquire_ctx,
    ) -> i32;
    pub fn drm_kunit_add_mode_destroy_action(
        test: *mut kunit,
        mode: *mut drm_display_mode,
    ) -> i32;
    pub fn drm_kunit_display_mode_from_cea_vic(
        test: *mut kunit,
        dev: *mut drm_device,
        video_code: u8,
    ) -> *mut drm_display_mode;
}

#[macro_export]
macro_rules! drm_kunit_helper_alloc_drm_device_with_driver {
    ($test:expr, $dev:expr, $type:ty, $member:tt, $drv:expr) => {
        unsafe {
            $crate::__drm_kunit_helper_alloc_drm_device_with_driver(
                $test,
                $dev,
                core::mem::size_of::<$type>(),
                core::mem::offset_of!($type, $member),
                $drv,
            ) as *mut $type
        }
    };
}

#[inline]
pub unsafe fn __drm_kunit_helper_alloc_drm_device(
    test: *mut kunit,
    dev: *mut device,
    size: usize,
    offset: usize,
    features: u32,
) -> *mut drm_device {
    let driver = devm_kzalloc(dev, size_of::<drm_driver>(), GFP_KERNEL) as *mut drm_driver;
    kunit_assert_not_null(test, driver as *const core::ffi::c_void);
    (*driver).driver_features = features;
    __drm_kunit_helper_alloc_drm_device_with_driver(test, dev, size, offset, driver)
}

#[macro_export]
macro_rules! drm_kunit_helper_alloc_drm_device {
    ($test:expr, $dev:expr, $type:ty, $member:tt, $feat:expr) => {
        unsafe {
            $crate::__drm_kunit_helper_alloc_drm_device(
                $test,
                $dev,
                core::mem::size_of::<$type>(),
                core::mem::offset_of!($type, $member),
                $feat,
            ) as *mut $type
        }
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
