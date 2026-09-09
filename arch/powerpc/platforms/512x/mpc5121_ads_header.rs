/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (C) 2008 Freescale Semiconductor, Inc. All rights reserved.
 *
 * Prototypes for ADS5121 specific code
 */

// #ifndef __MPC512ADS_H__
// #define __MPC512ADS_H__

// The C __init annotation is preserved as conditional/source intent.
unsafe extern "C" {
    pub fn mpc5121_ads_cpld_map();
    pub fn mpc5121_ads_cpld_pic_init();
}

// #endif /* __MPC512ADS_H__ */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
