/* SPDX-License-Identifier: GPL-2.0 */

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct request_queue {
    _private: [u8; 0],
}

/*
 * block layer runtime pm functions
 *
 * The CONFIG_PM condition is preserved as a Rust configuration condition;
 * the corresponding feature is supplied by the build configuration.
 */
#[cfg(feature = "CONFIG_PM")]
extern "C" {
    pub fn blk_pm_runtime_init(q: *mut request_queue, dev: *mut device);
    pub fn blk_pre_runtime_suspend(q: *mut request_queue) -> ::core::ffi::c_int;
    pub fn blk_post_runtime_suspend(q: *mut request_queue, err: ::core::ffi::c_int);
    pub fn blk_pre_runtime_resume(q: *mut request_queue);
    pub fn blk_post_runtime_resume(q: *mut request_queue);
}

#[cfg(not(feature = "CONFIG_PM"))]
#[inline]
pub unsafe fn blk_pm_runtime_init(_q: *mut request_queue, _dev: *mut device) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
