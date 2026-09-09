/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2013 Freescale Semiconductor, Inc.
 */

use core::ffi::{c_int, c_uint};

extern "C" {
    pub fn mxs_saif_clkmux_select(clkmux: c_uint) -> c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
