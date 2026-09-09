/*
 * Copyright © 2016 Intel Corporation
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 */

// Dependency declarations from <linux/types.h> are supplied externally.

/* Optional for type 1 DVI adaptors; mandatory for type 1 HDMI and type 2 adaptors. */
pub const DP_DUAL_MODE_HDMI_ID: u8 = 0x00;
pub const DP_DUAL_MODE_HDMI_ID_LEN: u8 = 16;
/* Optional for type 1 adaptors; mandatory for type 2 adaptors. */
pub const DP_DUAL_MODE_ADAPTOR_ID: u8 = 0x10;
pub const DP_DUAL_MODE_REV_MASK: u8 = 0x07;
pub const DP_DUAL_MODE_REV_TYPE2: u8 = 0x00;
pub const DP_DUAL_MODE_TYPE_MASK: u8 = 0xf0;
pub const DP_DUAL_MODE_TYPE_TYPE2: u8 = 0xa0;
/* This field is marked reserved in dual mode spec, used in LSPCON. */
pub const DP_DUAL_MODE_TYPE_HAS_DPCD: u8 = 0x08;
pub const DP_DUAL_MODE_IEEE_OUI: u8 = 0x11;
pub const DP_DUAL_IEEE_OUI_LEN: u8 = 3;
pub const DP_DUAL_DEVICE_ID: u8 = 0x14;
pub const DP_DUAL_DEVICE_ID_LEN: u8 = 6;
pub const DP_DUAL_MODE_HARDWARE_REV: u8 = 0x1a;
pub const DP_DUAL_MODE_FIRMWARE_MAJOR_REV: u8 = 0x1b;
pub const DP_DUAL_MODE_FIRMWARE_MINOR_REV: u8 = 0x1c;
pub const DP_DUAL_MODE_MAX_TMDS_CLOCK: u8 = 0x1d;
pub const DP_DUAL_MODE_I2C_SPEED_CAP: u8 = 0x1e;
pub const DP_DUAL_MODE_TMDS_OEN: u8 = 0x20;
pub const DP_DUAL_MODE_TMDS_DISABLE: u8 = 0x01;
pub const DP_DUAL_MODE_HDMI_PIN_CTRL: u8 = 0x21;
pub const DP_DUAL_MODE_CEC_ENABLE: u8 = 0x01;
pub const DP_DUAL_MODE_I2C_SPEED_CTRL: u8 = 0x22;

/* LSPCON specific registers, defined by MCA. */
pub const DP_DUAL_MODE_LSPCON_MODE_CHANGE: u8 = 0x40;
pub const DP_DUAL_MODE_LSPCON_CURRENT_MODE: u8 = 0x41;
pub const DP_DUAL_MODE_LSPCON_MODE_PCON: u8 = 0x1;

#[repr(C)]
pub struct drm_device {
    _private: [u8; 0],
}
#[repr(C)]
pub struct i2c_adapter {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum drm_lspcon_mode {
    DRM_LSPCON_MODE_INVALID,
    DRM_LSPCON_MODE_LS,
    DRM_LSPCON_MODE_PCON,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum drm_dp_dual_mode_type {
    DRM_DP_DUAL_MODE_NONE,
    DRM_DP_DUAL_MODE_UNKNOWN,
    DRM_DP_DUAL_MODE_TYPE1_DVI,
    DRM_DP_DUAL_MODE_TYPE1_HDMI,
    DRM_DP_DUAL_MODE_TYPE2_DVI,
    DRM_DP_DUAL_MODE_TYPE2_HDMI,
    DRM_DP_DUAL_MODE_LSPCON,
}

extern "C" {
    pub fn drm_dp_dual_mode_read(
        adapter: *mut i2c_adapter,
        offset: u8,
        buffer: *mut core::ffi::c_void,
        size: usize,
    ) -> isize;
    pub fn drm_dp_dual_mode_write(
        adapter: *mut i2c_adapter,
        offset: u8,
        buffer: *const core::ffi::c_void,
        size: usize,
    ) -> isize;
    pub fn drm_dp_dual_mode_detect(
        dev: *const drm_device,
        adapter: *mut i2c_adapter,
    ) -> drm_dp_dual_mode_type;
    pub fn drm_dp_dual_mode_max_tmds_clock(
        dev: *const drm_device,
        type_: drm_dp_dual_mode_type,
        adapter: *mut i2c_adapter,
    ) -> i32;
    pub fn drm_dp_dual_mode_get_tmds_output(
        dev: *const drm_device,
        type_: drm_dp_dual_mode_type,
        adapter: *mut i2c_adapter,
        enabled: *mut bool,
    ) -> i32;
    pub fn drm_dp_dual_mode_set_tmds_output(
        dev: *const drm_device,
        type_: drm_dp_dual_mode_type,
        adapter: *mut i2c_adapter,
        enable: bool,
    ) -> i32;
    pub fn drm_dp_get_dual_mode_type_name(type_: drm_dp_dual_mode_type) -> *const core::ffi::c_char;
    pub fn drm_lspcon_get_mode(
        dev: *const drm_device,
        adapter: *mut i2c_adapter,
        current_mode: *mut drm_lspcon_mode,
    ) -> i32;
    pub fn drm_lspcon_set_mode(
        dev: *const drm_device,
        adapter: *mut i2c_adapter,
        reqd_mode: drm_lspcon_mode,
        time_out: i32,
    ) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
