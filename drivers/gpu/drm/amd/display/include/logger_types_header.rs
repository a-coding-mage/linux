/*
 * Copyright 2012-15 Advanced Micro Devices, Inc.
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
 *
 * Authors: AMD
 */

// Dependency supplied by the original os_types.h and the kernel logging API.

macro_rules! DC_LOG_ERROR { ($($arg:tt)*) => { drm_err(DC_LOGGER.dev, $($arg)*); }; }
macro_rules! DC_LOG_WARNING { ($($arg:tt)*) => { drm_warn(DC_LOGGER.dev, $($arg)*); }; }
macro_rules! DC_LOG_DEBUG { ($($arg:tt)*) => { drm_dbg(DC_LOGGER.dev, $($arg)*); }; }
macro_rules! DC_LOG_DC { ($($arg:tt)*) => { drm_dbg(DC_LOGGER.dev, $($arg)*); }; }
macro_rules! DC_LOG_INFO { ($($arg:tt)*) => { drm_info(DC_LOGGER.dev, $($arg)*); }; }
macro_rules! DC_LOG_SURFACE { ($($arg:tt)*) => { pr_debug!("[SURFACE]:" $($arg)*); }; }
macro_rules! DC_LOG_HW_HOTPLUG { ($($arg:tt)*) => { drm_dbg(DC_LOGGER.dev, $($arg)*); }; }
macro_rules! DC_LOG_HW_LINK_TRAINING { ($($arg:tt)*) => { pr_debug!("[HW_LINK_TRAINING]:" $($arg)*); }; }
macro_rules! DC_LOG_HW_RESUME_S3 { ($($arg:tt)*) => { drm_dbg(DC_LOGGER.dev, $($arg)*); }; }
macro_rules! DC_LOG_HW_AUDIO { ($($arg:tt)*) => { pr_debug!("[HW_AUDIO]:" $($arg)*); }; }
macro_rules! DC_LOG_HW_HPD_IRQ { ($($arg:tt)*) => { drm_dbg_dp(DC_LOGGER.dev, $($arg)*); }; }
macro_rules! DC_LOG_MST { ($($arg:tt)*) => { drm_dbg_dp(DC_LOGGER.dev, $($arg)*); }; }
macro_rules! DC_LOG_SCALER { ($($arg:tt)*) => { pr_debug!("[SCALER]:" $($arg)*); }; }
macro_rules! DC_LOG_BIOS { ($($arg:tt)*) => { pr_debug!("[BIOS]:" $($arg)*); }; }
macro_rules! DC_LOG_BANDWIDTH_CALCS { ($($arg:tt)*) => { pr_debug!("[BANDWIDTH_CALCS]:" $($arg)*); }; }
macro_rules! DC_LOG_BANDWIDTH_VALIDATION { ($($arg:tt)*) => { drm_dbg(DC_LOGGER.dev, $($arg)*); }; }
macro_rules! DC_LOG_SYNC { ($($arg:tt)*) => { drm_dbg(DC_LOGGER.dev, $($arg)*); }; }
macro_rules! DC_LOG_BACKLIGHT { ($($arg:tt)*) => { drm_dbg_dp(DC_LOGGER.dev, $($arg)*); }; }
macro_rules! DC_LOG_DETECTION_EDID_PARSER { ($($arg:tt)*) => { drm_dbg(DC_LOGGER.dev, $($arg)*); }; }
macro_rules! DC_LOG_DETECTION_DP_CAPS { ($($arg:tt)*) => { drm_dbg_dp(DC_LOGGER.dev, $($arg)*); }; }
macro_rules! DC_LOG_DML { ($($arg:tt)*) => { pr_debug!("[DML]:" $($arg)*); }; }
macro_rules! DC_LOG_EVENT_MODE_SET { ($($arg:tt)*) => { drm_dbg_kms(DC_LOGGER.dev, $($arg)*); }; }
macro_rules! DC_LOG_EVENT_DETECTION { ($($arg:tt)*) => { drm_dbg(DC_LOGGER.dev, $($arg)*); }; }
macro_rules! DC_LOG_EVENT_LINK_TRAINING { ($($arg:tt)*) => { drm_dbg_dp(DC_LOGGER.dev, $($arg)*); }; }
macro_rules! DC_LOG_EVENT_LINK_LOSS { ($($arg:tt)*) => { drm_dbg_dp(DC_LOGGER.dev, $($arg)*); }; }
macro_rules! DC_LOG_IF_TRACE { ($($arg:tt)*) => { pr_debug!("[IF_TRACE]:" $($arg)*); }; }
macro_rules! DC_LOG_PERF_TRACE { ($($arg:tt)*) => { drm_dbg(DC_LOGGER.dev, $($arg)*); }; }
macro_rules! DC_LOG_RETIMER_REDRIVER { ($($arg:tt)*) => { drm_dbg(DC_LOGGER.dev, $($arg)*); }; }
macro_rules! DC_LOG_GAMMA { ($($arg:tt)*) => { pr_debug!("[GAMMA]:" $($arg)*); }; }
macro_rules! DC_LOG_ALL_GAMMA { ($($arg:tt)*) => { pr_debug!("[GAMMA]:" $($arg)*); }; }
macro_rules! DC_LOG_ALL_TF_CHANNELS { ($($arg:tt)*) => { pr_debug!("[GAMMA]:" $($arg)*); }; }
macro_rules! DC_LOG_DSC { ($($arg:tt)*) => { drm_dbg_dp(DC_LOGGER.dev, $($arg)*); }; }
macro_rules! DC_LOG_SMU { ($($arg:tt)*) => { pr_debug!("[SMU_MSG]:" $($arg)*); }; }
macro_rules! DC_LOG_HDMI_FRL { ($($arg:tt)*) => { drm_dbg(DC_LOGGER.dev, $($arg)*); }; }
macro_rules! DC_LOG_HDMI_FRL_LTP { ($($arg:tt)*) => { drm_dbg(DC_LOGGER.dev, $($arg)*); }; }
macro_rules! DC_LOG_DWB { ($($arg:tt)*) => { drm_dbg(DC_LOGGER.dev, $($arg)*); }; }
macro_rules! DC_LOG_DP2 { ($($arg:tt)*) => { drm_dbg_dp(DC_LOGGER.dev, $($arg)*); }; }
macro_rules! DC_LOG_AUTO_DPM_TEST { ($($arg:tt)*) => { pr_debug!("[AutoDPMTest]: " $($arg)*); }; }
macro_rules! DC_LOG_IPS { ($($arg:tt)*) => { pr_debug!("[IPS]: " $($arg)*); }; }
macro_rules! DC_LOG_MALL { ($($arg:tt)*) => { pr_debug!("[MALL]:" $($arg)*); }; }
macro_rules! DC_LOG_REGISTER_READ { ($($arg:tt)*) => { pr_debug!("[REGISTER_READ]: " $($arg)*); }; }
macro_rules! DC_LOG_REGISTER_WRITE { ($($arg:tt)*) => { pr_debug!("[REGISTER_WRITE]: " $($arg)*); }; }

#[repr(C)]
pub struct dc_log_buffer_ctx {
    pub buf: *mut core::ffi::c_char,
    pub pos: usize,
    pub size: usize,
}

#[repr(C)]
pub struct dal_logger {
    pub dev: *mut drm_device,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
