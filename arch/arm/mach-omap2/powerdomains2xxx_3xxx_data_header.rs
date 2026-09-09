/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * OMAP2/3 common powerdomains - prototypes
 *
 * Copyright (C) 2008 Texas Instruments, Inc.
 * Copyright (C) 2008-2010 Nokia Corporation
 *
 * Paul Walmsley
 */

// Dependency provided by the translated powerdomain header.

extern "C" {
    pub static mut gfx_omap2_pwrdm: powerdomain;
    pub static mut wkup_omap2_pwrdm: powerdomain;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
