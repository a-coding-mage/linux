/* SPDX-License-Identifier: MIT */
/*
 * Copyright © 2023 Intel Corporation
 */

// Dependency: Linux `struct cea_sad`.

#[repr(C)]
pub struct cea_sad {
    _private: [u8; 0],
}

/* ELD Header Block */
pub const DRM_ELD_HEADER_BLOCK_SIZE: i32 = 4;

pub const DRM_ELD_VER: i32 = 0;
pub const DRM_ELD_VER_SHIFT: i32 = 3;
pub const DRM_ELD_VER_MASK: i32 = 0x1f << 3;
pub const DRM_ELD_VER_CEA861D: i32 = 2 << 3; /* supports 861D or below */
pub const DRM_ELD_VER_CANNED: i32 = 0x1f << 3;

pub const DRM_ELD_BASELINE_ELD_LEN: i32 = 2; /* in dwords! */

/* ELD Baseline Block for ELD_Ver == 2 */
pub const DRM_ELD_CEA_EDID_VER_MNL: i32 = 4;
pub const DRM_ELD_CEA_EDID_VER_SHIFT: i32 = 5;
pub const DRM_ELD_CEA_EDID_VER_MASK: i32 = 7 << 5;
pub const DRM_ELD_CEA_EDID_VER_NONE: i32 = 0 << 5;
pub const DRM_ELD_CEA_EDID_VER_CEA861: i32 = 1 << 5;
pub const DRM_ELD_CEA_EDID_VER_CEA861A: i32 = 2 << 5;
pub const DRM_ELD_CEA_EDID_VER_CEA861BCD: i32 = 3 << 5;
pub const DRM_ELD_MNL_SHIFT: i32 = 0;
pub const DRM_ELD_MNL_MASK: i32 = 0x1f << 0;

pub const DRM_ELD_SAD_COUNT_CONN_TYPE: i32 = 5;
pub const DRM_ELD_SAD_COUNT_SHIFT: i32 = 4;
pub const DRM_ELD_SAD_COUNT_MASK: i32 = 0xf << 4;
pub const DRM_ELD_CONN_TYPE_SHIFT: i32 = 2;
pub const DRM_ELD_CONN_TYPE_MASK: i32 = 3 << 2;
pub const DRM_ELD_CONN_TYPE_HDMI: i32 = 0 << 2;
pub const DRM_ELD_CONN_TYPE_DP: i32 = 1 << 2;
pub const DRM_ELD_SUPPORTS_AI: i32 = 1 << 1;
pub const DRM_ELD_SUPPORTS_HDCP: i32 = 1 << 0;

pub const DRM_ELD_AUD_SYNCH_DELAY: i32 = 6; /* in units of 2 ms */
pub const DRM_ELD_AUD_SYNCH_DELAY_MAX: i32 = 0xfa; /* 500 ms */

pub const DRM_ELD_SPEAKER: i32 = 7;
pub const DRM_ELD_SPEAKER_MASK: i32 = 0x7f;
pub const DRM_ELD_SPEAKER_RLRC: i32 = 1 << 6;
pub const DRM_ELD_SPEAKER_FLRC: i32 = 1 << 5;
pub const DRM_ELD_SPEAKER_RC: i32 = 1 << 4;
pub const DRM_ELD_SPEAKER_RLR: i32 = 1 << 3;
pub const DRM_ELD_SPEAKER_FC: i32 = 1 << 2;
pub const DRM_ELD_SPEAKER_LFE: i32 = 1 << 1;
pub const DRM_ELD_SPEAKER_FLR: i32 = 1 << 0;

pub const DRM_ELD_PORT_ID: i32 = 8; /* offsets 8..15 inclusive */
pub const DRM_ELD_PORT_ID_LEN: i32 = 8;
pub const DRM_ELD_MANUFACTURER_NAME0: i32 = 16;
pub const DRM_ELD_MANUFACTURER_NAME1: i32 = 17;
pub const DRM_ELD_PRODUCT_CODE0: i32 = 18;
pub const DRM_ELD_PRODUCT_CODE1: i32 = 19;
pub const DRM_ELD_MONITOR_NAME_STRING: i32 = 20; /* offsets 20..(20+mnl-1) inclusive */

#[inline]
pub const fn drm_eld_cea_sad(mnl: i32, sad: i32) -> i32 {
    20 + mnl + 3 * sad
}

#[inline]
pub unsafe fn drm_eld_mnl(eld: *const u8) -> i32 {
    ((*eld.add(DRM_ELD_CEA_EDID_VER_MNL as usize) as i32) & DRM_ELD_MNL_MASK) >> DRM_ELD_MNL_SHIFT
}

unsafe extern "C" {
    pub fn drm_eld_sad_get(eld: *const u8, sad_index: i32, cta_sad: *mut cea_sad) -> i32;
    pub fn drm_eld_sad_set(eld: *mut u8, sad_index: i32, cta_sad: *const cea_sad) -> i32;
}

#[inline]
pub unsafe fn drm_eld_sad(eld: *const u8) -> *const u8 {
    let ver = ((*eld.add(DRM_ELD_VER as usize) as i32) & DRM_ELD_VER_MASK) >> DRM_ELD_VER_SHIFT;
    if ver != 2 && ver != 31 { return core::ptr::null(); }
    let mnl = drm_eld_mnl(eld);
    if mnl > 16 { return core::ptr::null(); }
    eld.add(drm_eld_cea_sad(mnl, 0) as usize)
}

#[inline]
pub unsafe fn drm_eld_sad_count(eld: *const u8) -> i32 {
    ((*eld.add(DRM_ELD_SAD_COUNT_CONN_TYPE as usize) as i32) & DRM_ELD_SAD_COUNT_MASK) >> DRM_ELD_SAD_COUNT_SHIFT
}

#[inline]
pub unsafe fn drm_eld_calc_baseline_block_size(eld: *const u8) -> i32 {
    DRM_ELD_MONITOR_NAME_STRING - DRM_ELD_HEADER_BLOCK_SIZE + drm_eld_mnl(eld) + drm_eld_sad_count(eld) * 3
}

#[inline]
pub unsafe fn drm_eld_size(eld: *const u8) -> i32 {
    DRM_ELD_HEADER_BLOCK_SIZE + (*eld.add(DRM_ELD_BASELINE_ELD_LEN as usize) as i32) * 4
}

#[inline]
pub unsafe fn drm_eld_get_spk_alloc(eld: *const u8) -> u8 {
    *eld.add(DRM_ELD_SPEAKER as usize) & DRM_ELD_SPEAKER_MASK as u8
}

#[inline]
pub unsafe fn drm_eld_get_conn_type(eld: *const u8) -> u8 {
    *eld.add(DRM_ELD_SAD_COUNT_CONN_TYPE as usize) & DRM_ELD_CONN_TYPE_MASK as u8
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
