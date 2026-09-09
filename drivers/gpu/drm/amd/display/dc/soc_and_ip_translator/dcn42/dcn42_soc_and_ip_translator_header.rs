// SPDX-License-Identifier: MIT
//
// Copyright 2025 Advanced Micro Devices, Inc.

// Dependencies supplied by the surrounding translation unit:
// core_types.h, dc.h, clk_mgr.h, dml_top_soc_parameter_types.h,
// and soc_and_ip_translator.h.

extern "C" {
    pub fn dcn42_construct_soc_and_ip_translator(
        soc_and_ip_translator: *mut soc_and_ip_translator,
    );

    pub fn dcn42_get_soc_bb(
        soc_bb: *mut dml2_soc_bb,
        dc: *const dc,
        config: *const dml2_configuration_options,
    );

    pub fn dcn42_apply_soc_bb_updates(
        soc_bb: *mut dml2_soc_bb,
        dc: *const dc,
        config: *const dml2_configuration_options,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
