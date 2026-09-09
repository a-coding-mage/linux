/* SPDX-License-Identifier: MIT
 * Copyright (C) 2018 Intel Corp.
 *
 * Authors:
 * Manasi Navare <manasi.d.navare@intel.com>
 */

// Dependency declarations supplied by the corresponding DRM headers.
#[allow(non_camel_case_types)]
pub enum dp_sdp_header {}
#[allow(non_camel_case_types)]
pub enum drm_dsc_picture_parameter_set {}
#[allow(non_camel_case_types)]
pub enum drm_dsc_config {}
#[allow(non_camel_case_types)]
pub enum drm_printer {}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[allow(non_camel_case_types)]
pub enum drm_dsc_params_type {
    DRM_DSC_1_2_444,
    // legacy params from DSC 1.1
    DRM_DSC_1_1_PRE_SCR,
    DRM_DSC_1_2_422,
    DRM_DSC_1_2_420,
}

unsafe extern "C" {
    pub fn drm_dsc_dp_pps_header_init(pps_header: *mut dp_sdp_header);
    pub fn drm_dsc_dp_rc_buffer_size(
        rc_buffer_block_size: u8,
        rc_buffer_size: u8,
    ) -> i32;
    pub fn drm_dsc_pps_payload_pack(
        pps_sdp: *mut drm_dsc_picture_parameter_set,
        dsc_cfg: *const drm_dsc_config,
    );
    pub fn drm_dsc_set_const_params(vdsc_cfg: *mut drm_dsc_config);
    pub fn drm_dsc_set_rc_buf_thresh(vdsc_cfg: *mut drm_dsc_config);
    pub fn drm_dsc_setup_rc_params(
        vdsc_cfg: *mut drm_dsc_config,
        type_: drm_dsc_params_type,
    ) -> i32;
    pub fn drm_dsc_compute_rc_parameters(vdsc_cfg: *mut drm_dsc_config) -> i32;
    pub fn drm_dsc_initial_scale_value(dsc: *const drm_dsc_config) -> u8;
    pub fn drm_dsc_flatness_det_thresh(dsc: *const drm_dsc_config) -> u32;
    pub fn drm_dsc_get_bpp_int(vdsc_cfg: *const drm_dsc_config) -> u32;
    pub fn drm_dsc_dump_config(
        p: *mut drm_printer,
        indent: i32,
        cfg: *const drm_dsc_config,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
