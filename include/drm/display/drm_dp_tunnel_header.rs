/* SPDX-License-Identifier: MIT */
/*
 * Copyright © 2023 Intel Corporation
 */

// C header dependencies: linux/err.h, linux/errno.h, linux/types.h.

use core::ffi::c_char;

#[repr(C)]
pub struct drm_dp_aux {
    _private: [u8; 0],
}
#[repr(C)]
pub struct drm_device {
    _private: [u8; 0],
}
#[repr(C)]
pub struct drm_atomic_commit {
    _private: [u8; 0],
}
#[repr(C)]
pub struct drm_dp_tunnel_mgr {
    _private: [u8; 0],
}
#[repr(C)]
pub struct drm_dp_tunnel_state {
    _private: [u8; 0],
}
#[repr(C)]
pub struct drm_dp_tunnel {
    _private: [u8; 0],
}
#[repr(C)]
pub struct ref_tracker {
    _private: [u8; 0],
}

#[repr(C)]
pub struct drm_dp_tunnel_ref {
    pub tunnel: *mut drm_dp_tunnel,
    pub tracker: *mut ref_tracker,
}

#[cfg(feature = "CONFIG_DRM_DISPLAY_DP_TUNNEL")]
extern "C" {
    pub fn drm_dp_tunnel_get(tunnel: *mut drm_dp_tunnel,
                              tracker: *mut *mut ref_tracker) -> *mut drm_dp_tunnel;
    pub fn drm_dp_tunnel_put(tunnel: *mut drm_dp_tunnel,
                             tracker: *mut *mut ref_tracker);
    pub fn drm_dp_tunnel_detect(mgr: *mut drm_dp_tunnel_mgr,
                                aux: *mut drm_dp_aux) -> *mut drm_dp_tunnel;
    pub fn drm_dp_tunnel_destroy(tunnel: *mut drm_dp_tunnel) -> i32;
    pub fn drm_dp_tunnel_enable_bw_alloc(tunnel: *mut drm_dp_tunnel) -> i32;
    pub fn drm_dp_tunnel_disable_bw_alloc(tunnel: *mut drm_dp_tunnel) -> i32;
    pub fn drm_dp_tunnel_bw_alloc_is_enabled(tunnel: *const drm_dp_tunnel) -> bool;
    pub fn drm_dp_tunnel_pr_optimization_supported(tunnel: *const drm_dp_tunnel) -> bool;
    pub fn drm_dp_tunnel_alloc_bw(tunnel: *mut drm_dp_tunnel, bw: i32) -> i32;
    pub fn drm_dp_tunnel_get_allocated_bw(tunnel: *mut drm_dp_tunnel) -> i32;
    pub fn drm_dp_tunnel_update_state(tunnel: *mut drm_dp_tunnel) -> i32;
    pub fn drm_dp_tunnel_set_io_error(tunnel: *mut drm_dp_tunnel);
    pub fn drm_dp_tunnel_handle_irq(mgr: *mut drm_dp_tunnel_mgr,
                                    aux: *mut drm_dp_aux) -> i32;
    pub fn drm_dp_tunnel_128b132b_supported(tunnel: *const drm_dp_tunnel) -> bool;
    pub fn drm_dp_tunnel_128b132b_lane0_mapping_supported(tunnel: *const drm_dp_tunnel) -> bool;
    pub fn drm_dp_tunnel_128b132b_dprx_rates(tunnel: *const drm_dp_tunnel) -> u8;
    pub fn drm_dp_tunnel_max_dprx_rate(tunnel: *const drm_dp_tunnel) -> i32;
    pub fn drm_dp_tunnel_max_dprx_lane_count(tunnel: *const drm_dp_tunnel) -> i32;
    pub fn drm_dp_tunnel_available_bw(tunnel: *const drm_dp_tunnel) -> i32;
    pub fn drm_dp_tunnel_name(tunnel: *const drm_dp_tunnel) -> *const c_char;
    pub fn drm_dp_tunnel_atomic_get_state(state: *mut drm_atomic_commit,
                                           tunnel: *mut drm_dp_tunnel) -> *mut drm_dp_tunnel_state;
    pub fn drm_dp_tunnel_atomic_get_old_state(state: *mut drm_atomic_commit,
                                              tunnel: *const drm_dp_tunnel) -> *mut drm_dp_tunnel_state;
    pub fn drm_dp_tunnel_atomic_get_new_state(state: *mut drm_atomic_commit,
                                              tunnel: *const drm_dp_tunnel) -> *mut drm_dp_tunnel_state;
    pub fn drm_dp_tunnel_atomic_set_stream_bw(state: *mut drm_atomic_commit,
                                              tunnel: *mut drm_dp_tunnel,
                                              stream_id: u8, bw: i32) -> i32;
    pub fn drm_dp_tunnel_atomic_get_group_streams_in_state(state: *mut drm_atomic_commit,
                                                           tunnel: *const drm_dp_tunnel,
                                                           stream_mask: *mut u32) -> i32;
    pub fn drm_dp_tunnel_atomic_check_stream_bws(state: *mut drm_atomic_commit,
                                                 failed_stream_mask: *mut u32) -> i32;
    pub fn drm_dp_tunnel_atomic_get_required_bw(tunnel_state: *const drm_dp_tunnel_state) -> i32;
    pub fn drm_dp_tunnel_mgr_create(dev: *mut drm_device, max_group_count: i32) -> *mut drm_dp_tunnel_mgr;
    pub fn drm_dp_tunnel_mgr_destroy(mgr: *mut drm_dp_tunnel_mgr);
}

#[cfg(feature = "CONFIG_DRM_DISPLAY_DP_TUNNEL")]
#[inline]
pub unsafe fn drm_dp_tunnel_ref_get(tunnel: *mut drm_dp_tunnel,
                                     tunnel_ref: *mut drm_dp_tunnel_ref) {
    (*tunnel_ref).tunnel = drm_dp_tunnel_get(tunnel, &mut (*tunnel_ref).tracker);
}

#[cfg(feature = "CONFIG_DRM_DISPLAY_DP_TUNNEL")]
#[inline]
pub unsafe fn drm_dp_tunnel_ref_put(tunnel_ref: *mut drm_dp_tunnel_ref) {
    drm_dp_tunnel_put((*tunnel_ref).tunnel, &mut (*tunnel_ref).tracker);
    (*tunnel_ref).tunnel = core::ptr::null_mut();
}

// When CONFIG_DRM_DISPLAY_DP_TUNNEL is disabled, the declarations above are
// replaced by the following inline stubs. ERR_PTR values preserve the C ABI
// intent; the external helper and errno constants are supplied by dependencies.
#[cfg(not(feature = "CONFIG_DRM_DISPLAY_DP_TUNNEL"))]
extern "C" {
    fn ERR_PTR(error: isize) -> *mut core::ffi::c_void;
}

#[cfg(not(feature = "CONFIG_DRM_DISPLAY_DP_TUNNEL"))]
#[inline]
pub unsafe fn drm_dp_tunnel_get(_tunnel: *mut drm_dp_tunnel, _tracker: *mut *mut ref_tracker) -> *mut drm_dp_tunnel { core::ptr::null_mut() }
#[cfg(not(feature = "CONFIG_DRM_DISPLAY_DP_TUNNEL"))]
#[inline]
pub unsafe fn drm_dp_tunnel_put(_tunnel: *mut drm_dp_tunnel, _tracker: *mut *mut ref_tracker) {}
#[cfg(not(feature = "CONFIG_DRM_DISPLAY_DP_TUNNEL"))]
#[inline]
pub unsafe fn drm_dp_tunnel_ref_get(_tunnel: *mut drm_dp_tunnel, _tunnel_ref: *mut drm_dp_tunnel_ref) {}
#[cfg(not(feature = "CONFIG_DRM_DISPLAY_DP_TUNNEL"))]
#[inline]
pub unsafe fn drm_dp_tunnel_ref_put(_tunnel_ref: *mut drm_dp_tunnel_ref) {}

#[cfg(not(feature = "CONFIG_DRM_DISPLAY_DP_TUNNEL"))]
#[inline]
pub unsafe fn drm_dp_tunnel_detect(_mgr: *mut drm_dp_tunnel_mgr, _aux: *mut drm_dp_aux) -> *mut drm_dp_tunnel { ERR_PTR(-95) as *mut drm_dp_tunnel }
#[cfg(not(feature = "CONFIG_DRM_DISPLAY_DP_TUNNEL"))]
#[inline]
pub unsafe fn drm_dp_tunnel_destroy(_tunnel: *mut drm_dp_tunnel) -> i32 { 0 }
#[cfg(not(feature = "CONFIG_DRM_DISPLAY_DP_TUNNEL"))]
#[inline]
pub unsafe fn drm_dp_tunnel_enable_bw_alloc(_tunnel: *mut drm_dp_tunnel) -> i32 { -95 }
#[cfg(not(feature = "CONFIG_DRM_DISPLAY_DP_TUNNEL"))]
#[inline]
pub unsafe fn drm_dp_tunnel_disable_bw_alloc(_tunnel: *mut drm_dp_tunnel) -> i32 { -95 }
#[cfg(not(feature = "CONFIG_DRM_DISPLAY_DP_TUNNEL"))]
#[inline]
pub unsafe fn drm_dp_tunnel_bw_alloc_is_enabled(_tunnel: *const drm_dp_tunnel) -> bool { false }
#[cfg(not(feature = "CONFIG_DRM_DISPLAY_DP_TUNNEL"))]
#[inline]
pub unsafe fn drm_dp_tunnel_pr_optimization_supported(_tunnel: *const drm_dp_tunnel) -> bool { false }
#[cfg(not(feature = "CONFIG_DRM_DISPLAY_DP_TUNNEL"))]
#[inline]
pub unsafe fn drm_dp_tunnel_alloc_bw(_tunnel: *mut drm_dp_tunnel, _bw: i32) -> i32 { -95 }
#[cfg(not(feature = "CONFIG_DRM_DISPLAY_DP_TUNNEL"))]
#[inline]
pub unsafe fn drm_dp_tunnel_get_allocated_bw(_tunnel: *mut drm_dp_tunnel) -> i32 { -1 }
#[cfg(not(feature = "CONFIG_DRM_DISPLAY_DP_TUNNEL"))]
#[inline]
pub unsafe fn drm_dp_tunnel_update_state(_tunnel: *mut drm_dp_tunnel) -> i32 { -95 }
#[cfg(not(feature = "CONFIG_DRM_DISPLAY_DP_TUNNEL"))]
#[inline]
pub unsafe fn drm_dp_tunnel_set_io_error(_tunnel: *mut drm_dp_tunnel) {}
#[cfg(not(feature = "CONFIG_DRM_DISPLAY_DP_TUNNEL"))]
#[inline]
pub unsafe fn drm_dp_tunnel_handle_irq(_mgr: *mut drm_dp_tunnel_mgr, _aux: *mut drm_dp_aux) -> i32 { -95 }
#[cfg(not(feature = "CONFIG_DRM_DISPLAY_DP_TUNNEL"))]
#[inline]
pub unsafe fn drm_dp_tunnel_128b132b_supported(_tunnel: *const drm_dp_tunnel) -> bool { false }
#[cfg(not(feature = "CONFIG_DRM_DISPLAY_DP_TUNNEL"))]
#[inline]
pub unsafe fn drm_dp_tunnel_128b132b_lane0_mapping_supported(_tunnel: *const drm_dp_tunnel) -> bool { false }
#[cfg(not(feature = "CONFIG_DRM_DISPLAY_DP_TUNNEL"))]
#[inline]
pub unsafe fn drm_dp_tunnel_128b132b_dprx_rates(_tunnel: *const drm_dp_tunnel) -> u8 { 0 }
#[cfg(not(feature = "CONFIG_DRM_DISPLAY_DP_TUNNEL"))]
#[inline]
pub unsafe fn drm_dp_tunnel_max_dprx_rate(_tunnel: *const drm_dp_tunnel) -> i32 { 0 }
#[cfg(not(feature = "CONFIG_DRM_DISPLAY_DP_TUNNEL"))]
#[inline]
pub unsafe fn drm_dp_tunnel_max_dprx_lane_count(_tunnel: *const drm_dp_tunnel) -> i32 { 0 }
#[cfg(not(feature = "CONFIG_DRM_DISPLAY_DP_TUNNEL"))]
#[inline]
pub unsafe fn drm_dp_tunnel_available_bw(_tunnel: *const drm_dp_tunnel) -> i32 { -1 }
#[cfg(not(feature = "CONFIG_DRM_DISPLAY_DP_TUNNEL"))]
#[inline]
pub unsafe fn drm_dp_tunnel_name(_tunnel: *const drm_dp_tunnel) -> *const c_char { core::ptr::null() }
#[cfg(not(feature = "CONFIG_DRM_DISPLAY_DP_TUNNEL"))]
#[inline]
pub unsafe fn drm_dp_tunnel_atomic_get_state(_state: *mut drm_atomic_commit, _tunnel: *mut drm_dp_tunnel) -> *mut drm_dp_tunnel_state { ERR_PTR(-95) as *mut drm_dp_tunnel_state }
#[cfg(not(feature = "CONFIG_DRM_DISPLAY_DP_TUNNEL"))]
#[inline]
pub unsafe fn drm_dp_tunnel_atomic_get_new_state(_state: *mut drm_atomic_commit, _tunnel: *const drm_dp_tunnel) -> *mut drm_dp_tunnel_state { ERR_PTR(-95) as *mut drm_dp_tunnel_state }
#[cfg(not(feature = "CONFIG_DRM_DISPLAY_DP_TUNNEL"))]
#[inline]
pub unsafe fn drm_dp_tunnel_atomic_set_stream_bw(_state: *mut drm_atomic_commit, _tunnel: *mut drm_dp_tunnel, _stream_id: u8, _bw: i32) -> i32 { -95 }
#[cfg(not(feature = "CONFIG_DRM_DISPLAY_DP_TUNNEL"))]
#[inline]
pub unsafe fn drm_dp_tunnel_atomic_get_group_streams_in_state(_state: *mut drm_atomic_commit, _tunnel: *const drm_dp_tunnel, _stream_mask: *mut u32) -> i32 { -95 }
#[cfg(not(feature = "CONFIG_DRM_DISPLAY_DP_TUNNEL"))]
#[inline]
pub unsafe fn drm_dp_tunnel_atomic_check_stream_bws(_state: *mut drm_atomic_commit, _failed_stream_mask: *mut u32) -> i32 { -95 }
#[cfg(not(feature = "CONFIG_DRM_DISPLAY_DP_TUNNEL"))]
#[inline]
pub unsafe fn drm_dp_tunnel_atomic_get_required_bw(_tunnel_state: *const drm_dp_tunnel_state) -> i32 { 0 }
#[cfg(not(feature = "CONFIG_DRM_DISPLAY_DP_TUNNEL"))]
#[inline]
pub unsafe fn drm_dp_tunnel_mgr_create(_dev: *mut drm_device, _max_group_count: i32) -> *mut drm_dp_tunnel_mgr { ERR_PTR(-95) as *mut drm_dp_tunnel_mgr }
#[cfg(not(feature = "CONFIG_DRM_DISPLAY_DP_TUNNEL"))]
#[inline]
pub unsafe fn drm_dp_tunnel_mgr_destroy(_mgr: *mut drm_dp_tunnel_mgr) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
