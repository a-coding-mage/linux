/* SPDX-License-Identifier: MIT */
/*
 * Copyright 2019 Advanced Micro Devices, Inc.
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

// C includes: mod_hdcp.h, hdcp.h, dc.h, dm_cp_psp.h

/* Minimal declarations needed by this header. */
pub const AMDGPU_DM_MAX_DISPLAY_COUNT: usize = 32;

pub enum amdgpu_dm_connector {}
pub enum mod_hdcp {}
pub enum mod_hdcp_link {}
pub enum mod_hdcp_display {}
pub enum cp_psp {}
pub enum amdgpu_device {}
pub enum psp_context {}
pub enum file {}
pub enum kobject {}
pub enum bin_attribute {}
pub enum mod_hdcp_atomic_op_i2c {}
pub enum mod_hdcp_atomic_op_aux {}
pub enum work_struct {}
pub enum delayed_work {}
pub enum mutex {}
pub enum mod_hdcp_output {}
pub enum dc {}
pub enum dc_link {}
pub enum cp_psp_stream_config {}
pub enum mod_hdcp_link_adjustment {}
pub enum mod_hdcp_display_adjustment {}

pub type uint8_t = u8;
pub type uint32_t = u32;
pub type u8 = u8;
pub type bool_t = bool;
pub type unsigned_int = u32;
pub type ssize_t = isize;
pub type loff_t = i64;
pub type size_t = usize;

#[repr(C)]
pub struct hdcp_workqueue {
    pub cpirq_work: work_struct,
    pub property_update_work: work_struct,
    pub callback_dwork: delayed_work,
    pub watchdog_timer_dwork: delayed_work,
    pub property_validate_dwork: delayed_work,
    pub aconnector: [*mut amdgpu_dm_connector; AMDGPU_DM_MAX_DISPLAY_COUNT],
    pub mutex: mutex,
    pub hdcp: mod_hdcp,
    pub output: mod_hdcp_output,
    pub display: mod_hdcp_display,
    pub link: mod_hdcp_link,
    pub encryption_status: [mod_hdcp_encryption_status; AMDGPU_DM_MAX_DISPLAY_COUNT],
    /* when display is unplugged from mst hub, connctor will be destroyed within dm_dp_mst_connector_destroy. */
    /* Save hdcp properties into hdcp_work within amdgpu_dm_atomic_commit_tail. */
    /* un-desired, desired, enabled */
    pub content_protection: [unsigned_int; AMDGPU_DM_MAX_DISPLAY_COUNT],
    /* hdcp1.x, hdcp2.x */
    pub hdcp_content_type: [unsigned_int; AMDGPU_DM_MAX_DISPLAY_COUNT],
    pub max_link: u8,
    pub srm: *mut u8,
    pub srm_temp: *mut u8,
    pub srm_version: u32,
    pub srm_size: u32,
    pub attr: bin_attribute,
}

pub enum mod_hdcp_encryption_status {}

unsafe extern "C" {
    pub fn hdcp_update_display(work: *mut hdcp_workqueue, link_index: unsigned_int,
        aconnector: *mut amdgpu_dm_connector, content_type: u8, enable_encryption: bool);
    pub fn hdcp_reset_display(work: *mut hdcp_workqueue, link_index: unsigned_int);
    pub fn hdcp_handle_cpirq(work: *mut hdcp_workqueue, link_index: unsigned_int);
    pub fn hdcp_destroy(kobj: *mut kobject, work: *mut hdcp_workqueue);
    pub fn hdcp_create_workqueue(adev: *mut amdgpu_device, cp_psp: *mut cp_psp, dc: *mut dc) -> *mut hdcp_workqueue;

    // These declarations are present when CONFIG_DRM_AMD_DC_KUNIT_TEST is enabled.
    pub fn process_output(hdcp_work: *mut hdcp_workqueue);
    pub fn hdcp_get_content_protection_from_status(hdcp_content_type: unsigned_int,
        encryption_status: mod_hdcp_encryption_status, content_protection: *mut unsigned_int) -> bool;
    pub fn hdcp_get_link_display_adjustments(enable_encryption: bool, content_type: u8,
        fused_io_supported: bool, hdcp_lc_force_fw_enable: bool, hdcp_lc_enable_sw_fallback: bool,
        link_adjust: *mut mod_hdcp_link_adjustment, display_adjust: *mut mod_hdcp_display_adjustment);
    pub fn hdcp_update_display_encryption_control(hdcp_work: *mut hdcp_workqueue,
        hdcp_w: *mut hdcp_workqueue, conn_index: unsigned_int, enable_encryption: bool);
    pub fn event_property_update(work: *mut work_struct);
    pub fn event_property_validate(work: *mut work_struct);
    pub fn event_callback(work: *mut work_struct);
    pub fn event_watchdog_timer(work: *mut work_struct);
    pub fn event_cpirq(work: *mut work_struct);
    pub fn link_lock(work: *mut hdcp_workqueue, lock: bool);
    pub fn hdcp_remove_display(hdcp_work: *mut hdcp_workqueue, link_index: unsigned_int,
        aconnector: *mut amdgpu_dm_connector);
    pub fn psp_get_srm(psp: *mut psp_context, srm_version: *mut u32, srm_size: *mut u32) -> *mut u8;
    pub fn psp_set_srm(psp: *mut psp_context, srm: *mut u8, srm_size: u32, srm_version: *mut u32) -> i32;
    pub fn enable_assr(handle: *mut core::ffi::c_void, link: *mut dc_link) -> bool;
    pub fn update_config(handle: *mut core::ffi::c_void, config: *mut cp_psp_stream_config);
    pub fn srm_data_write(filp: *mut file, kobj: *mut kobject, bin_attr: *const bin_attribute,
        buffer: *mut i8, pos: loff_t, count: size_t) -> ssize_t;
    pub fn srm_data_read(filp: *mut file, kobj: *mut kobject, bin_attr: *const bin_attribute,
        buffer: *mut i8, pos: loff_t, count: size_t) -> ssize_t;
    pub fn lp_write_i2c(handle: *mut core::ffi::c_void, address: u32, data: *const u8, size: u32) -> bool;
    pub fn lp_read_i2c(handle: *mut core::ffi::c_void, address: u32, offset: u8, data: *mut u8, size: u32) -> bool;
    pub fn lp_write_dpcd(handle: *mut core::ffi::c_void, address: u32, data: *const u8, size: u32) -> bool;
    pub fn lp_read_dpcd(handle: *mut core::ffi::c_void, address: u32, data: *mut u8, size: u32) -> bool;
    pub fn lp_atomic_write_poll_read_i2c(handle: *mut core::ffi::c_void,
        write: *const mod_hdcp_atomic_op_i2c, poll: *const mod_hdcp_atomic_op_i2c,
        read: *mut mod_hdcp_atomic_op_i2c, poll_timeout_us: u32, poll_mask_msb: u8) -> bool;
    pub fn lp_atomic_write_poll_read_aux(handle: *mut core::ffi::c_void,
        write: *const mod_hdcp_atomic_op_aux, poll: *const mod_hdcp_atomic_op_aux,
        read: *mut mod_hdcp_atomic_op_aux, poll_timeout_us: u32, poll_mask_msb: u8) -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
