/*
 * Copyright (c) 2017 Andreas Färber
 *
 * SPDX-License-Identifier: GPL-2.0+
 */

// Translated from the C header guard SOC_ACTIONS_OWL_SPS_H.

extern "C" {
    pub fn owl_sps_set_pg(
        base: *mut core::ffi::c_void,
        pwr_mask: u32,
        ack_mask: u32,
        enable: bool,
    ) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
