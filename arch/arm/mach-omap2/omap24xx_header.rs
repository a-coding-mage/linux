/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * This file contains the processor specific definitions
 * of the TI OMAP24XX.
 *
 * Copyright (C) 2007 Texas Instruments.
 * Copyright (C) 2007 Nokia Corporation.
 */

/*
 * Please place only base defines here and put the rest in device
 * specific headers. Note also that some of these defines are needed
 * for omap1 to compile without adding ifdefs.
 */

pub const L4_24XX_BASE: u32 = 0x48000000;
pub const L4_WK_243X_BASE: u32 = 0x49000000;
pub const L3_24XX_BASE: u32 = 0x68000000;

/* interrupt controller */
pub const OMAP24XX_IC_BASE: u32 = L4_24XX_BASE + 0xfe000;
pub const OMAP24XX_IVA_INTC_BASE: u32 = 0x40000000;

pub const OMAP242X_CTRL_BASE: u32 = L4_24XX_BASE;
pub const OMAP2420_32KSYNCT_BASE: u32 = L4_24XX_BASE + 0x4000;
pub const OMAP2420_PRCM_BASE: u32 = L4_24XX_BASE + 0x8000;
pub const OMAP2420_CM_BASE: u32 = L4_24XX_BASE + 0x8000;
pub const OMAP2420_PRM_BASE: u32 = OMAP2420_CM_BASE;
pub const OMAP2420_SDRC_BASE: u32 = L3_24XX_BASE + 0x9000;
pub const OMAP2420_SMS_BASE: u32 = 0x68008000;
pub const OMAP2420_GPMC_BASE: u32 = 0x6800a000;

pub const OMAP2430_32KSYNCT_BASE: u32 = L4_WK_243X_BASE + 0x20000;
pub const OMAP2430_PRCM_BASE: u32 = L4_WK_243X_BASE + 0x6000;
pub const OMAP2430_CM_BASE: u32 = L4_WK_243X_BASE + 0x6000;
pub const OMAP2430_PRM_BASE: u32 = OMAP2430_CM_BASE;

pub const OMAP243X_SMS_BASE: u32 = 0x6C000000;
pub const OMAP243X_SDRC_BASE: u32 = 0x6D000000;
pub const OMAP243X_GPMC_BASE: u32 = 0x6E000000;
pub const OMAP243X_SCM_BASE: u32 = L4_WK_243X_BASE + 0x2000;
pub const OMAP243X_CTRL_BASE: u32 = OMAP243X_SCM_BASE;
pub const OMAP243X_HS_BASE: u32 = L4_24XX_BASE + 0x000ac000;

/* DSP SS */
pub const OMAP2420_DSP_BASE: u32 = 0x58000000;
pub const OMAP2420_DSP_MEM_BASE: u32 = OMAP2420_DSP_BASE + 0x0;
pub const OMAP2420_DSP_IPI_BASE: u32 = OMAP2420_DSP_BASE + 0x1000000;
pub const OMAP2420_DSP_MMU_BASE: u32 = OMAP2420_DSP_BASE + 0x2000000;

pub const OMAP243X_DSP_BASE: u32 = 0x5C000000;
pub const OMAP243X_DSP_MEM_BASE: u32 = OMAP243X_DSP_BASE + 0x0;
pub const OMAP243X_DSP_MMU_BASE: u32 = OMAP243X_DSP_BASE + 0x1000000;

/* Mailbox */
pub const OMAP24XX_MAILBOX_BASE: u32 = L4_24XX_BASE + 0x94000;

/* Camera */
pub const OMAP24XX_CAMERA_BASE: u32 = L4_24XX_BASE + 0x52000;

/* Security */
pub const OMAP24XX_SEC_BASE: u32 = L4_24XX_BASE + 0xA0000;
pub const OMAP24XX_SEC_RNG_BASE: u32 = OMAP24XX_SEC_BASE + 0x0000;
pub const OMAP24XX_SEC_DES_BASE: u32 = OMAP24XX_SEC_BASE + 0x2000;
pub const OMAP24XX_SEC_SHA1MD5_BASE: u32 = OMAP24XX_SEC_BASE + 0x4000;
pub const OMAP24XX_SEC_AES_BASE: u32 = OMAP24XX_SEC_BASE + 0x6000;
pub const OMAP24XX_SEC_PKA_BASE: u32 = OMAP24XX_SEC_BASE + 0x8000;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
