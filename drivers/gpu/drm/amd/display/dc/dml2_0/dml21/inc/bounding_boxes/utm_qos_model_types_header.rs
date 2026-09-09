// SPDX-License-Identifier: MIT
//
// Copyright 2025 Advanced Micro Devices, Inc.

// C header dependencies (including uint32_t and uint8_t) are supplied externally.

#[repr(C)]
pub struct utm_soc_operating_point {
    pub uclk_khz: u32,
    pub fclk_khz: u32,
}

#[repr(C)]
pub struct utm_qos_model_socbb {
    pub fabric_datapath_to_dcn_data_return_bytes: u8,
    pub dram_channel_width_bytes: u8,
    pub dram_channel_count: u8,
    pub dram_transactions_per_clock: u8,
    pub fabric_derate_percent_nominal: u8,
    pub fabric_derate_percent_urgent: u8,
    pub dram_derate_percent_nominal: u8,
    pub dram_derate_percent_urgent: u8,
    pub lsdma_fabric_derate_percent: u8,
    pub lsdma_dram_derate_percent: u8,
    pub fabric_datapath_to_lsdma_data_return_bytes: u8,
}

pub const MAX_UTM_SOP_COUNT: usize = 20;

#[repr(C)]
pub enum utm_qos_model_version {
    utm_qos_model_version_v1,
    utm_qos_model_version_v2,
    utm_qos_model_version_v3,
}

pub struct utm_qos_model_dchub_v1;
pub struct utm_qos_model_dchub_v2;
pub struct utm_qos_model_dchub_v3;
pub struct utm_qos_model_lsdma;

#[repr(C)]
pub union utm_qos_model__bindgen_ty_1 {
    pub dchub_v1: *const utm_qos_model_dchub_v1,
    pub dchub_v2: *const utm_qos_model_dchub_v2,
    pub dchub_v3: *const utm_qos_model_dchub_v3,
}

#[repr(C)]
pub struct utm_qos_model {
    pub version: i32,
    pub sops: [utm_soc_operating_point; MAX_UTM_SOP_COUNT],
    pub dchub: utm_qos_model__bindgen_ty_1,
    pub lsdma: *const utm_qos_model_lsdma,
    pub socbb: utm_qos_model_socbb,
    pub sop_count: u8,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
