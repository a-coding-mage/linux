// SPDX-License-Identifier: MIT
//
// Copyright 2025 Advanced Micro Devices, Inc.

// Dependencies supplied by the corresponding translated headers:
// core_types.h, dc.h, clk_mgr.h, soc_and_ip_translator.h,
// dml2_0/dml21/inc/dml_top_soc_parameter_types.h

extern "C" {
    pub fn dcn401_construct_soc_and_ip_translator(
        soc_and_ip_translator: *mut soc_and_ip_translator,
    );

    /* Functions that can be re-used by higher DCN revisions of this component */
    pub fn dcn401_get_soc_bb(
        soc_bb: *mut dml2_soc_bb,
        dc: *const dc,
        config: *const dml2_configuration_options,
    );

    pub fn dcn401_update_soc_bb_with_values_from_clk_mgr(
        soc_bb: *mut dml2_soc_bb,
        dc: *const dc,
        config: *const dml2_configuration_options,
    );

    pub fn dcn401_update_soc_bb_with_values_from_vbios(
        soc_bb: *mut dml2_soc_bb,
        dc: *const dc,
    );

    pub fn dcn401_update_soc_bb_with_values_from_software_policy(
        soc_bb: *mut dml2_soc_bb,
        dc: *const dc,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
