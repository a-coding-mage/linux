/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * OMAP Traffic Controller
 *
 * Copyright (C) 2004 Nokia Corporation
 * Author: Imre Deak <imre.deak@nokia.com>
 */

pub const TCMIF_BASE: u32 = 0xfffe_cc00;
pub const OMAP_TC_OCPT1_PRIOR: u32 = TCMIF_BASE + 0x00;
pub const OMAP_TC_EMIFS_PRIOR: u32 = TCMIF_BASE + 0x04;
pub const OMAP_TC_EMIFF_PRIOR: u32 = TCMIF_BASE + 0x08;
pub const EMIFS_CONFIG: u32 = TCMIF_BASE + 0x0c;
pub const EMIFS_CS0_CONFIG: u32 = TCMIF_BASE + 0x10;
pub const EMIFS_CS1_CONFIG: u32 = TCMIF_BASE + 0x14;
pub const EMIFS_CS2_CONFIG: u32 = TCMIF_BASE + 0x18;
pub const EMIFS_CS3_CONFIG: u32 = TCMIF_BASE + 0x1c;
pub const EMIFF_SDRAM_CONFIG: u32 = TCMIF_BASE + 0x20;
pub const EMIFF_MRS: u32 = TCMIF_BASE + 0x24;
pub const TC_TIMEOUT1: u32 = TCMIF_BASE + 0x28;
pub const TC_TIMEOUT2: u32 = TCMIF_BASE + 0x2c;
pub const TC_TIMEOUT3: u32 = TCMIF_BASE + 0x30;
pub const TC_ENDIANISM: u32 = TCMIF_BASE + 0x34;
pub const EMIFF_SDRAM_CONFIG_2: u32 = TCMIF_BASE + 0x3c;
pub const EMIF_CFG_DYNAMIC_WS: u32 = TCMIF_BASE + 0x40;
pub const EMIFS_ACS0: u32 = TCMIF_BASE + 0x50;
pub const EMIFS_ACS1: u32 = TCMIF_BASE + 0x54;
pub const EMIFS_ACS2: u32 = TCMIF_BASE + 0x58;
pub const EMIFS_ACS3: u32 = TCMIF_BASE + 0x5c;
pub const OMAP_TC_OCPT2_PRIOR: u32 = TCMIF_BASE + 0xd0;

/* external EMIFS chipselect regions */
pub const OMAP_CS0_PHYS: u32 = 0x0000_0000;
pub const OMAP_CS0_SIZE: u32 = SZ_64M;

pub const OMAP_CS1_PHYS: u32 = 0x0400_0000;
pub const OMAP_CS1_SIZE: u32 = SZ_64M;

pub const OMAP_CS1A_PHYS: u32 = OMAP_CS1_PHYS;
pub const OMAP_CS1A_SIZE: u32 = SZ_32M;

pub const OMAP_CS1B_PHYS: u32 = OMAP_CS1A_PHYS + OMAP_CS1A_SIZE;
pub const OMAP_CS1B_SIZE: u32 = SZ_32M;

pub const OMAP_CS2_PHYS: u32 = 0x0800_0000;
pub const OMAP_CS2_SIZE: u32 = SZ_64M;

pub const OMAP_CS2A_PHYS: u32 = OMAP_CS2_PHYS;
pub const OMAP_CS2A_SIZE: u32 = SZ_32M;

pub const OMAP_CS2B_PHYS: u32 = OMAP_CS2A_PHYS + OMAP_CS2A_SIZE;
pub const OMAP_CS2B_SIZE: u32 = SZ_32M;

pub const OMAP_CS3_PHYS: u32 = 0x0c00_0000;
pub const OMAP_CS3_SIZE: u32 = SZ_64M;

/* EMIF Slow Interface Configuration Register */
pub const OMAP_EMIFS_CONFIG_FR: u32 = 1 << 4;
pub const OMAP_EMIFS_CONFIG_PDE: u32 = 1 << 3;
pub const OMAP_EMIFS_CONFIG_PWD_EN: u32 = 1 << 2;
pub const OMAP_EMIFS_CONFIG_BM: u32 = 1 << 1;
pub const OMAP_EMIFS_CONFIG_WP: u32 = 1 << 0;

pub const fn EMIFS_CCS(n: u32) -> u32 {
    EMIFS_CS0_CONFIG + (4 * n)
}

pub const fn EMIFS_ACS(n: u32) -> u32 {
    EMIFS_ACS0 + (4 * n)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
