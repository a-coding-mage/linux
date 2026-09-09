/* SPDX-License-Identifier: MIT */
/*
 * Copyright 2025 Advanced Micro Devices, Inc.
 *
 * Authors: AMD
 */

// Dependency equivalent of: #include "os_types.h"

#[repr(C)]
pub struct dml2_context {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dc {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ip_params_st {
    _private: [u8; 0],
}

#[repr(C)]
pub struct soc_bounding_box_st {
    _private: [u8; 0],
}

#[repr(C)]
pub struct soc_states_st {
    _private: [u8; 0],
}

extern "C" {
    pub fn initialize_dml2_ip_params(
        dml2: *mut dml2_context,
        in_dc: *const dc,
        out: *mut ip_params_st,
    );

    pub fn initialize_dml2_soc_bbox(
        dml2: *mut dml2_context,
        in_dc: *const dc,
        out: *mut soc_bounding_box_st,
    );

    pub fn initialize_dml2_soc_states(
        dml2: *mut dml2_context,
        in_dc: *const dc,
        in_bbox: *const soc_bounding_box_st,
        out: *mut soc_states_st,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
