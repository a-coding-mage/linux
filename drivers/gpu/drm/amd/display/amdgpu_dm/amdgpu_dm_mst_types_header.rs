/* SPDX-License-Identifier: MIT */
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
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 *
 * Authors: AMD
 */

pub const DP_BRANCH_DEVICE_ID_90CC24: u32 = 0x90CC24;

pub const SYNAPTICS_RC_COMMAND: u32 = 0x4B2;
pub const SYNAPTICS_RC_RESULT: u32 = 0x4B3;
pub const SYNAPTICS_RC_LENGTH: u32 = 0x4B8;
pub const SYNAPTICS_RC_OFFSET: u32 = 0x4BC;
pub const SYNAPTICS_RC_DATA: u32 = 0x4C0;

pub const DP_BRANCH_VENDOR_SPECIFIC_START: u32 = 0x50C;

/**
 * Panamera MST Hub detection
 * Offset DPCD 050Eh == 0x5A indicates cascaded MST hub case
 * Check from beginning of branch device vendor specific field (050Ch)
 */
pub unsafe fn is_synaptics_panamera(branch_dev_name: *const u8) -> i32 {
    if ((*branch_dev_name.add(4) as i32) & 0xF0) == 0x50 { 1 } else { 0 }
}

pub const BRANCH_HW_REVISION_PANAMERA_A2: u8 = 0x10;
pub const SYNAPTICS_CASCADED_HUB_ID: u8 = 0x5A;

pub unsafe fn is_synaptics_cascaded_panamera(
    dev_name: *const u8,
    data: *const u8,
) -> i32 {
    if is_synaptics_panamera(dev_name) != 0
        && *data.add(2) as i32 == SYNAPTICS_CASCADED_HUB_ID as i32
    {
        1
    } else {
        0
    }
}

pub const PBN_FEC_OVERHEAD_MULTIPLIER_8B_10B: i32 = 1031;
pub const PBN_FEC_OVERHEAD_MULTIPLIER_128B_132B: i32 = 1000;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum mst_msg_ready_type {
    NONE_MSG_RDY_EVENT = 0,
    DOWN_REP_MSG_RDY_EVENT = 1,
    UP_REQ_MSG_RDY_EVENT = 2,
    DOWN_OR_UP_MSG_RDY_EVENT = 3,
}

#[repr(C)] pub struct amdgpu_device { _unused: [u8; 0] }
#[repr(C)] pub struct amdgpu_display_manager { _unused: [u8; 0] }
#[repr(C)] pub struct amdgpu_dm_connector { _unused: [u8; 0] }
#[repr(C)] pub struct aux_payload { _unused: [u8; 0] }
#[repr(C)] pub struct dc_state { _unused: [u8; 0] }
#[repr(C)] pub struct dc_stream_state { _unused: [u8; 0] }
#[repr(C)] pub struct dc_link { _unused: [u8; 0] }
#[repr(C)] pub struct dm_atomic_state { _unused: [u8; 0] }
#[repr(C)] pub struct drm_atomic_commit { _unused: [u8; 0] }
#[repr(C)] pub struct drm_dp_mst_topology_mgr { _unused: [u8; 0] }
#[repr(C)] pub struct drm_dp_aux { _unused: [u8; 0] }
#[repr(C)] pub struct drm_dp_aux_msg { _unused: [u8; 0] }
#[repr(C)] pub struct drm_connector { _unused: [u8; 0] }
#[repr(C)] pub struct drm_modeset_acquire_ctx { _unused: [u8; 0] }
#[repr(C)] pub struct drm_encoder { _unused: [u8; 0] }
#[repr(C)] pub enum aux_return_code_type { _Unused = 0 }

extern "C" {
    pub fn dm_mst_get_pbn_divider(link: *mut dc_link) -> u32;
    pub fn amdgpu_dm_initialize_dp_connector(
        dm: *mut amdgpu_display_manager,
        aconnector: *mut amdgpu_dm_connector,
        link_index: i32,
    );
    pub fn dm_dp_create_fake_mst_encoders(adev: *mut amdgpu_device);
    pub fn dm_handle_mst_sideband_msg_ready_event(
        mgr: *mut drm_dp_mst_topology_mgr,
        msg_rdy_type: mst_msg_ready_type,
    );
}

#[repr(C)]
pub struct dsc_mst_fairness_vars {
    pub pbn: i32,
    pub dsc_enabled: bool,
    pub bpp_x16: i32,
    pub aconnector: *mut amdgpu_dm_connector,
}

extern "C" {
    pub fn compute_mst_dsc_configs_for_state(
        state: *mut drm_atomic_commit,
        dc_state: *mut dc_state,
        vars: *mut dsc_mst_fairness_vars,
    ) -> i32;
    pub fn needs_dsc_aux_workaround(link: *mut dc_link) -> bool;
    pub fn pre_validate_dsc(
        state: *mut drm_atomic_commit,
        dm_state_ptr: *mut *mut dm_atomic_state,
        vars: *mut dsc_mst_fairness_vars,
    ) -> i32;
    pub fn dm_dp_mst_is_port_support_mode(
        aconnector: *mut amdgpu_dm_connector,
        stream: *mut dc_stream_state,
    ) -> i32; // enum dc_status
}

// The following declarations are available only when CONFIG_DRM_AMD_DC_KUNIT_TEST is enabled.
#[cfg(CONFIG_DRM_AMD_DC_KUNIT_TEST)]
extern "C" {
    pub fn amdgpu_dm_mst_reset_mst_connector_setting(aconnector: *mut amdgpu_dm_connector);
    pub fn retrieve_downstream_port_device(aconnector: *mut amdgpu_dm_connector) -> bool;
    pub fn retrieve_branch_specific_data(aconnector: *mut amdgpu_dm_connector) -> bool;
    pub fn dm_dp_aux_transfer_result(result: isize, operation_result: aux_return_code_type) -> isize;
    pub fn dm_dp_aux_fill_payload_flags(request: u8, payload: *mut aux_payload);
    pub fn dm_dp_aux_transfer(aux: *mut drm_dp_aux, msg: *mut drm_dp_aux_msg) -> isize;
    pub fn dm_mst_msg_ready_mask(msg_rdy_type: mst_msg_ready_type) -> u8;
    pub fn dm_mst_select_esi_dpcd(dpcd_rev: u8, dpcd_addr: *mut i32, dpcd_bytes_to_read: *mut u8);
    pub fn dm_handle_mst_down_rep_msg_ready(mgr: *mut drm_dp_mst_topology_mgr);
    pub fn dm_mst_atomic_best_encoder(connector: *mut drm_connector, state: *mut drm_atomic_commit) -> *mut drm_encoder;
    pub fn dm_dp_mst_atomic_check(connector: *mut drm_connector, state: *mut drm_atomic_commit) -> i32;
    pub fn dm_dp_mst_detect(connector: *mut drm_connector, ctx: *mut drm_modeset_acquire_ctx, force: bool) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
