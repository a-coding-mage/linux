// SPDX-License-Identifier: MIT
//
// Copyright 2026 Advanced Micro Devices, Inc.

// Dependencies supplied by the corresponding C headers:
// "core_types.h"
// "dc.h"
// "clk_mgr.h"
// "dml_top_soc_parameter_types.h"
// "soc_and_ip_translator.h"

#[repr(C)]
pub struct soc_and_ip_translator {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dml2_soc_bb {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dc {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dml2_configuration_options {
    _private: [u8; 0],
}

extern "C" {
    pub fn dcn42b_construct_soc_and_ip_translator(
        soc_and_ip_translator: *mut soc_and_ip_translator,
    );

    pub fn dcn42b_get_soc_bb(
        soc_bb: *mut dml2_soc_bb,
        dc: *const dc,
        config: *const dml2_configuration_options,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
