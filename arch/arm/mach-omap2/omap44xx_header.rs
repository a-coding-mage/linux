/* SPDX-License-Identifier: GPL-2.0-only */
/*:
 * Address mappings and base address for OMAP4 interconnects
 * and peripherals.
 *
 * Copyright (C) 2009 Texas Instruments
 *
 * Author: Santosh Shilimkar <santosh.shilimkar@ti.com>
 */

/*
 * Please place only base defines here and put the rest in device
 * specific headers.
 */
pub const L4_44XX_BASE: u32 = 0x4a000000;
pub const L4_WK_44XX_BASE: u32 = 0x4a300000;
pub const L4_PER_44XX_BASE: u32 = 0x48000000;
pub const L4_EMU_44XX_BASE: u32 = 0x54000000;
pub const L3_44XX_BASE: u32 = 0x44000000;
pub const OMAP44XX_EMIF1_BASE: u32 = 0x4c000000;
pub const OMAP44XX_EMIF2_BASE: u32 = 0x4d000000;
pub const OMAP44XX_DMM_BASE: u32 = 0x4e000000;
pub const OMAP4430_32KSYNCT_BASE: u32 = 0x4a304000;
pub const OMAP4430_CM1_BASE: u32 = 0x4a004000;
pub const OMAP4430_CM_BASE: u32 = OMAP4430_CM1_BASE;
pub const OMAP4430_CM2_BASE: u32 = 0x4a008000;
pub const OMAP4430_PRM_BASE: u32 = 0x4a306000;
pub const OMAP4430_PRCM_MPU_BASE: u32 = 0x48243000;
pub const OMAP44XX_GPMC_BASE: u32 = 0x50000000;
pub const OMAP443X_SCM_BASE: u32 = 0x4a002000;
pub const OMAP443X_CTRL_BASE: u32 = 0x4a100000;
pub const OMAP44XX_IC_BASE: u32 = 0x48200000;
pub const OMAP44XX_IVA_INTC_BASE: u32 = 0x40000000;
pub const IRQ_SIR_IRQ: u32 = 0x0040;
pub const OMAP44XX_GIC_DIST_BASE: u32 = 0x48241000;
pub const OMAP44XX_GIC_CPU_BASE: u32 = 0x48240100;
pub const OMAP44XX_IRQ_GIC_START: u32 = 32;
pub const OMAP44XX_LOCAL_TWD_BASE: u32 = 0x48240600;
pub const OMAP44XX_L2CACHE_BASE: u32 = 0x48242000;
pub const OMAP44XX_WKUPGEN_BASE: u32 = 0x48281000;
pub const OMAP44XX_MCPDM_BASE: u32 = 0x40132000;
pub const OMAP44XX_SAR_RAM_BASE: u32 = 0x4a326000;

pub const OMAP44XX_MAILBOX_BASE: u32 = L4_44XX_BASE + 0xF4000;
pub const OMAP44XX_HSUSB_OTG_BASE: u32 = L4_44XX_BASE + 0xAB000;

pub const OMAP4_MMU1_BASE: u32 = 0x55082000;
pub const OMAP4_MMU2_BASE: u32 = 0x4A066000;

pub const OMAP44XX_USBTLL_BASE: u32 = L4_44XX_BASE + 0x62000;
pub const OMAP44XX_UHH_CONFIG_BASE: u32 = L4_44XX_BASE + 0x64000;
pub const OMAP44XX_HSUSB_OHCI_BASE: u32 = L4_44XX_BASE + 0x64800;
pub const OMAP44XX_HSUSB_EHCI_BASE: u32 = L4_44XX_BASE + 0x64C00;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
