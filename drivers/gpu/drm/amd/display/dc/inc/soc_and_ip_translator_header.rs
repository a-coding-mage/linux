// SPDX-License-Identifier: MIT
//
// Copyright 2025 Advanced Micro Devices, Inc.

// Translated from soc_and_ip_translator.h.
// Dependencies supplied by dc.h and dml_top_soc_parameter_types.h remain external.

#[repr(C)]
pub struct soc_and_ip_translator_funcs {
    pub get_soc_bb: Option<
        unsafe extern "C" fn(
            soc_bb: *mut dml2_soc_bb,
            dc: *const dc,
            config: *const dml2_configuration_options,
        ),
    >,
    pub get_ip_caps:
        Option<unsafe extern "C" fn(dml_ip_caps: *mut dml2_ip_capabilities)>,
}

#[repr(C)]
pub struct soc_and_ip_translator {
    pub translator_funcs: *const soc_and_ip_translator_funcs,
}

extern "C" {
    pub fn dc_create_soc_and_ip_translator(
        dc_version: dce_version,
    ) -> *mut soc_and_ip_translator;
    pub fn dc_destroy_soc_and_ip_translator(
        soc_and_ip_translator: *mut *mut soc_and_ip_translator,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
