/* SPDX-License-Identifier: GPL-2.0+ */

// Dependencies supplied by the surrounding Linux/Rust translation.

#[repr(C)]
pub struct drm_atomic_commit {
    _private: [u8; 0],
}

#[repr(C)]
pub struct drm_crtc {
    _private: [u8; 0],
}

// linux/hrtimer_types.h: ktime_t is a signed 64-bit nanosecond value.
pub type ktime_t = i64;

/*
 * VBLANK helpers
 */

unsafe extern "C" {
    pub fn drm_crtc_vblank_atomic_flush(
        crtc: *mut drm_crtc,
        state: *mut drm_atomic_commit,
    );
    pub fn drm_crtc_vblank_atomic_enable(
        crtc: *mut drm_crtc,
        state: *mut drm_atomic_commit,
    );
    pub fn drm_crtc_vblank_atomic_disable(
        crtc: *mut drm_crtc,
        crtc_state: *mut drm_atomic_commit,
    );

    /*
     * VBLANK timer
     */
    pub fn drm_crtc_vblank_helper_enable_vblank_timer(crtc: *mut drm_crtc) -> i32;
    pub fn drm_crtc_vblank_helper_disable_vblank_timer(crtc: *mut drm_crtc);
    pub fn drm_crtc_vblank_helper_get_vblank_timestamp_from_timer(
        crtc: *mut drm_crtc,
        max_error: *mut i32,
        vblank_time: *mut ktime_t,
        in_vblank_irq: bool,
    ) -> bool;
}

/**
 * DRM_CRTC_HELPER_VBLANK_FUNCS - Default implementation for VBLANK helpers
 *
 * This macro initializes struct &drm_crtc_helper_funcs to default helpers
 * for VBLANK handling.
 */
#[macro_export]
macro_rules! DRM_CRTC_HELPER_VBLANK_FUNCS {
    () => {
        .atomic_flush = $crate::drm_crtc_vblank_atomic_flush,
        .atomic_enable = $crate::drm_crtc_vblank_atomic_enable,
        .atomic_disable = $crate::drm_crtc_vblank_atomic_disable,
    };
}

/**
 * DRM_CRTC_VBLANK_TIMER_FUNCS - Default implementation for VBLANK timers
 *
 * This macro initializes struct &drm_crtc_funcs to default helpers for
 * VBLANK timers.
 */
#[macro_export]
macro_rules! DRM_CRTC_VBLANK_TIMER_FUNCS {
    () => {
        .enable_vblank = $crate::drm_crtc_vblank_helper_enable_vblank_timer,
        .disable_vblank = $crate::drm_crtc_vblank_helper_disable_vblank_timer,
        .get_vblank_timestamp = $crate::drm_crtc_vblank_helper_get_vblank_timestamp_from_timer,
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
