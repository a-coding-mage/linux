/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * v4l2-dv-timings - Internal header with dv-timings helper functions
 *
 * Copyright 2013 Cisco Systems, Inc. and/or its affiliates. All rights reserved.
 */

// Dependency declarations corresponding to <linux/debugfs.h> and
// <linux/videodev2.h> are supplied by the surrounding translation.

/// v4l2_calc_timeperframe - helper function to calculate timeperframe based
/// on v4l2_dv_timings fields.
extern "C" {
    pub fn v4l2_calc_timeperframe(t: *const v4l2_dv_timings) -> v4l2_fract;

    /* v4l2_dv_timings_presets: list of all dv_timings presets. */
    pub static v4l2_dv_timings_presets: v4l2_dv_timings;

    /// Returns true if the given timings are valid.
    pub fn v4l2_valid_dv_timings(
        t: *const v4l2_dv_timings,
        cap: *const v4l2_dv_timings_cap,
        fnc: v4l2_check_dv_timings_fnc,
        fnc_handle: *mut core::ffi::c_void,
    ) -> bool;

    pub fn v4l2_enum_dv_timings_cap(
        t: *mut v4l2_enum_dv_timings,
        cap: *const v4l2_dv_timings_cap,
        fnc: v4l2_check_dv_timings_fnc,
        fnc_handle: *mut core::ffi::c_void,
    ) -> i32;

    pub fn v4l2_find_dv_timings_cap(
        t: *mut v4l2_dv_timings,
        cap: *const v4l2_dv_timings_cap,
        pclock_delta: u32,
        fnc: v4l2_check_dv_timings_fnc,
        fnc_handle: *mut core::ffi::c_void,
    ) -> bool;

    pub fn v4l2_find_dv_timings_cea861_vic(t: *mut v4l2_dv_timings, vic: u8) -> bool;

    pub fn v4l2_match_dv_timings(
        measured: *const v4l2_dv_timings,
        standard: *const v4l2_dv_timings,
        pclock_delta: u32,
        match_reduced_fps: bool,
    ) -> bool;

    pub fn v4l2_print_dv_timings(
        dev_prefix: *const core::ffi::c_char,
        prefix: *const core::ffi::c_char,
        t: *const v4l2_dv_timings,
        detailed: bool,
    );

    pub fn v4l2_detect_cvt(
        frame_height: u32,
        hfreq: u32,
        vsync: u32,
        active_width: u32,
        polarities: u32,
        interlaced: bool,
        cap: *const v4l2_dv_timings_cap,
        fmt: *mut v4l2_dv_timings,
    ) -> bool;

    pub fn v4l2_detect_gtf(
        frame_height: u32,
        hfreq: u32,
        vsync: u32,
        polarities: u32,
        interlaced: bool,
        aspect: v4l2_fract,
        cap: *const v4l2_dv_timings_cap,
        fmt: *mut v4l2_dv_timings,
    ) -> bool;

    pub fn v4l2_calc_aspect_ratio(hor_landscape: u8, vert_portrait: u8) -> v4l2_fract;
    pub fn v4l2_dv_timings_aspect_ratio(t: *const v4l2_dv_timings) -> v4l2_fract;
}

pub type v4l2_check_dv_timings_fnc = unsafe extern "C" fn(
    t: *const v4l2_dv_timings,
    handle: *mut core::ffi::c_void,
) -> bool;

pub unsafe fn can_reduce_fps(bt: *mut v4l2_bt_timings) -> bool {
    if ((*bt).standards & V4L2_DV_BT_STD_CVT) != 0 && (*bt).vsync == 8 {
        return true;
    }
    if ((*bt).standards & V4L2_DV_BT_STD_CEA861) != 0
        && ((*bt).flags & V4L2_DV_FL_CAN_REDUCE_FPS) != 0
    {
        return true;
    }
    false
}

#[repr(C)]
pub struct v4l2_hdmi_colorimetry {
    pub colorspace: v4l2_colorspace,
    pub ycbcr_enc: v4l2_ycbcr_encoding,
    pub quantization: v4l2_quantization,
    pub xfer_func: v4l2_xfer_func,
}

pub enum hdmi_avi_infoframe {}
pub enum hdmi_vendor_infoframe {}

extern "C" {
    pub fn v4l2_hdmi_rx_colorimetry(
        avi: *const hdmi_avi_infoframe,
        hdmi: *const hdmi_vendor_infoframe,
        height: u32,
    ) -> v4l2_hdmi_colorimetry;

    pub fn v4l2_num_edid_blocks(edid: *const u8, max_blocks: u32) -> u32;
    pub fn v4l2_get_edid_phys_addr(edid: *const u8, size: u32, offset: *mut u32) -> u16;
    pub fn v4l2_set_edid_phys_addr(edid: *mut u8, size: u32, phys_addr: u16);
    pub fn v4l2_phys_addr_for_input(phys_addr: u16, input: u8) -> u16;
    pub fn v4l2_phys_addr_validate(phys_addr: u16, parent: *mut u16, port: *mut u16) -> i32;
}

pub const V4L2_DEBUGFS_IF_MAX_LEN: u32 = 35;
pub const V4L2_DEBUGFS_IF_AVI: u32 = 1 << 0;
pub const V4L2_DEBUGFS_IF_AUDIO: u32 = 1 << 1;
pub const V4L2_DEBUGFS_IF_SPD: u32 = 1 << 2;
pub const V4L2_DEBUGFS_IF_HDMI: u32 = 1 << 3;
pub const V4L2_DEBUGFS_IF_DRM: u32 = 1 << 4;

pub type v4l2_debugfs_if_read_t = unsafe extern "C" fn(
    type_: u32,
    priv_: *mut core::ffi::c_void,
    filp: *mut file,
    ubuf: *mut core::ffi::c_char,
    count: usize,
    ppos: *mut i64,
) -> isize;

#[repr(C)]
pub struct v4l2_debugfs_if {
    pub if_dir: *mut dentry,
    pub priv_: *mut core::ffi::c_void,
    pub if_read: v4l2_debugfs_if_read_t,
}

#[cfg(CONFIG_DEBUG_FS)]
extern "C" {
    pub fn v4l2_debugfs_if_alloc(
        root: *mut dentry,
        if_types: u32,
        priv_: *mut core::ffi::c_void,
        if_read: v4l2_debugfs_if_read_t,
    ) -> *mut v4l2_debugfs_if;
    pub fn v4l2_debugfs_if_free(infoframes: *mut v4l2_debugfs_if);
}

#[cfg(not(CONFIG_DEBUG_FS))]
pub unsafe fn v4l2_debugfs_if_alloc(
    _root: *mut dentry,
    _if_types: u32,
    _priv_: *mut core::ffi::c_void,
    _if_read: v4l2_debugfs_if_read_t,
) -> *mut v4l2_debugfs_if {
    core::ptr::null_mut()
}

#[cfg(not(CONFIG_DEBUG_FS))]
pub unsafe fn v4l2_debugfs_if_free(_infoframes: *mut v4l2_debugfs_if) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
