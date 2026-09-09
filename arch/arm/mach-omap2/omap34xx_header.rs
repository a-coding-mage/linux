/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * This file contains the processor specific definitions of the TI OMAP34XX.
 *
 * Copyright (C) 2007 Texas Instruments.
 * Copyright (C) 2007 Nokia Corporation.
 */

/*
 * Please place only base defines here and put the rest in device
 * specific headers.
 */

pub const L4_34XX_BASE: u32 = 0x48000000;
pub const L4_WK_34XX_BASE: u32 = 0x48300000;
pub const L4_PER_34XX_BASE: u32 = 0x49000000;
pub const L4_EMU_34XX_BASE: u32 = 0x54000000;
pub const L3_34XX_BASE: u32 = 0x68000000;

pub const L4_WK_AM33XX_BASE: u32 = 0x44C00000;

pub const OMAP3430_32KSYNCT_BASE: u32 = 0x48320000;
pub const OMAP3430_CM_BASE: u32 = 0x48004800;
pub const OMAP3430_PRM_BASE: u32 = 0x48306800;
pub const OMAP343X_SMS_BASE: u32 = 0x6C000000;
pub const OMAP343X_SDRC_BASE: u32 = 0x6D000000;
pub const OMAP34XX_GPMC_BASE: u32 = 0x6E000000;
pub const OMAP343X_SCM_BASE: u32 = 0x48002000;
pub const OMAP343X_CTRL_BASE: u32 = OMAP343X_SCM_BASE;

pub const OMAP34XX_IC_BASE: u32 = 0x48200000;

pub const OMAP3430_ISP_BASE: u32 = L4_34XX_BASE + 0xBC000;
pub const OMAP3430_ISP_MMU_BASE: u32 = OMAP3430_ISP_BASE + 0x1400;
pub const OMAP3430_ISP_BASE2: u32 = OMAP3430_ISP_BASE + 0x1800;

pub const OMAP34XX_HSUSB_OTG_BASE: u32 = L4_34XX_BASE + 0xAB000;
pub const OMAP34XX_USBTLL_BASE: u32 = L4_34XX_BASE + 0x62000;
pub const OMAP34XX_UHH_CONFIG_BASE: u32 = L4_34XX_BASE + 0x64000;
pub const OMAP34XX_OHCI_BASE: u32 = L4_34XX_BASE + 0x64400;
pub const OMAP34XX_EHCI_BASE: u32 = L4_34XX_BASE + 0x64800;
pub const OMAP34XX_SR1_BASE: u32 = 0x480C9000;
pub const OMAP34XX_SR2_BASE: u32 = 0x480CB000;

pub const OMAP34XX_MAILBOX_BASE: u32 = L4_34XX_BASE + 0x94000;

/* Security */
pub const OMAP34XX_SEC_BASE: u32 = L4_34XX_BASE + 0xA0000;
pub const OMAP34XX_SEC_SHA1MD5_BASE: u32 = OMAP34XX_SEC_BASE + 0x23000;
pub const OMAP34XX_SEC_AES_BASE: u32 = OMAP34XX_SEC_BASE + 0x25000;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
