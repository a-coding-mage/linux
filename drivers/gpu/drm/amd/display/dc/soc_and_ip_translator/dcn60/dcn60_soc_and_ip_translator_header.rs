// SPDX-License-Identifier: MIT
//
// Copyright 2025 Advanced Micro Devices, Inc.

// Original C header guard: _DCN60_SOC_AND_IP_TRANSLATOR_H_
// Dependencies supplied by the surrounding translation unit:
// core_types.h, dc.h, clk_mgr.h, dml_top_soc_parameter_types.h,
// and soc_and_ip_translator.h.

/// Opaque declaration of the C `struct soc_and_ip_translator`.
#[repr(C)]
pub struct soc_and_ip_translator {
    _private: [u8; 0],
}

extern "C" {
    pub fn dcn60_construct_soc_and_ip_translator(
        soc_and_ip_translator: *mut soc_and_ip_translator,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
