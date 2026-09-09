/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *  Atheros AR71XX/AR724X/AR913X common definitions
 *
 *  Copyright (C) 2008-2011 Gabor Juhos <juhosg@openwrt.org>
 *  Copyright (C) 2008 Imre Kaloz <kaloz@openwrt.org>
 *
 *  Parts of this file are based on Atheros' 2.6.15 BSP
 */

// Dependency corresponding to <linux/types.h> is supplied externally.

pub const ATH79_MEM_SIZE_MIN: usize = 2 * 1024 * 1024;
pub const ATH79_MEM_SIZE_MAX: usize = 256 * 1024 * 1024;

unsafe extern "C" {
    pub fn ath79_ddr_ctrl_init();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
