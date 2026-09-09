/*
 * SPDX-License-Identifier: MIT
 *
 * Copyright 2026 Advanced Micro Devices, Inc.
 */

// Dependency intent preserved from the C header:
// #include "dc.h"
// #include "hw_sequencer_private.h"

unsafe extern "C" {
    pub fn dcn42b_init_pipes(dc: *mut crate::dc, context: *mut crate::dc_state);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
