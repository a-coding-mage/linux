/* SPDX-License-Identifier: GPL-2.0 */
/*
 * OMAP2 clock function prototypes and macros
 *
 * Copyright (C) 2005-2010 Texas Instruments, Inc.
 * Copyright (C) 2004-2010 Nokia Corporation
 */

// Dependencies supplied by the surrounding translation unit:
// #include <linux/clk-provider.h>
// #include "clock.h"

unsafe extern "C" {
    pub fn omap2xxx_clk_get_core_rate() -> core::ffi::c_ulong;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
