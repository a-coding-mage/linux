/* SPDX-License-Identifier: GPL-2.0-only */

/*
 * OMAP2/3 PRCM base and module definitions
 *
 * Copyright (C) 2007-2009, 2011 Texas Instruments, Inc.
 * Copyright (C) 2007-2009 Nokia Corporation
 *
 * Written by Paul Walmsley
 */

/* Module offsets from both CM_BASE & PRM_BASE */

/*
 * Offsets that are the same on 24xx and 34xx
 *
 * Technically, in terms of the TRM, OCP_MOD is 34xx only; PLL_MOD is
 * CCR_MOD on 3430; and GFX_MOD only exists < 3430ES2.
 */
pub const OCP_MOD: u32 = 0x000;
pub const MPU_MOD: u32 = 0x100;
pub const CORE_MOD: u32 = 0x200;
pub const GFX_MOD: u32 = 0x300;
pub const WKUP_MOD: u32 = 0x400;
pub const PLL_MOD: u32 = 0x500;


/* Chip-specific module offsets */
pub const OMAP24XX_GR_MOD: u32 = OCP_MOD;
pub const OMAP24XX_DSP_MOD: u32 = 0x800;

pub const OMAP2430_MDM_MOD: u32 = 0xc00;

/* IVA2 module is < base on 3430 */
pub const OMAP3430_IVA2_MOD: i32 = -0x800;
pub const OMAP3430ES2_SGX_MOD: u32 = GFX_MOD;
pub const OMAP3430_CCR_MOD: u32 = PLL_MOD;
pub const OMAP3430_DSS_MOD: u32 = 0x600;
pub const OMAP3430_CAM_MOD: u32 = 0x700;
pub const OMAP3430_PER_MOD: u32 = 0x800;
pub const OMAP3430_EMU_MOD: u32 = 0x900;
pub const OMAP3430_GR_MOD: u32 = 0xa00;
pub const OMAP3430_NEON_MOD: u32 = 0xb00;
pub const OMAP3430ES2_USBHOST_MOD: u32 = 0xc00;

/*
 * TI81XX PRM module offsets
 */
pub const TI814X_PRM_DSP_MOD: u32 = 0x0a00;
pub const TI814X_PRM_HDVICP_MOD: u32 = 0x0c00;
pub const TI814X_PRM_ISP_MOD: u32 = 0x0d00;
pub const TI814X_PRM_HDVPSS_MOD: u32 = 0x0e00;
pub const TI814X_PRM_GFX_MOD: u32 = 0x0f00;

pub const TI81XX_PRM_DEVICE_MOD: u32 = 0x0000;
pub const TI816X_PRM_ACTIVE_MOD: u32 = 0x0a00;
pub const TI81XX_PRM_DEFAULT_MOD: u32 = 0x0b00;
pub const TI816X_PRM_IVAHD0_MOD: u32 = 0x0c00;
pub const TI816X_PRM_IVAHD1_MOD: u32 = 0x0d00;
pub const TI816X_PRM_IVAHD2_MOD: u32 = 0x0e00;
pub const TI816X_PRM_SGX_MOD: u32 = 0x0f00;
pub const TI81XX_PRM_ALWON_MOD: u32 = 0x1800;

/* 24XX register bits shared between CM & PRM registers */

/* CM_FCLKEN1_CORE, CM_ICLKEN1_CORE, PM_WKEN1_CORE shared bits */
pub const OMAP2420_EN_MMC_SHIFT: u32 = 26;
pub const OMAP2420_EN_MMC_MASK: u32 = (1u32 << 26);
pub const OMAP24XX_EN_UART2_SHIFT: u32 = 22;
pub const OMAP24XX_EN_UART2_MASK: u32 = (1u32 << 22);
pub const OMAP24XX_EN_UART1_SHIFT: u32 = 21;
pub const OMAP24XX_EN_UART1_MASK: u32 = (1u32 << 21);
pub const OMAP24XX_EN_MCSPI2_SHIFT: u32 = 18;
pub const OMAP24XX_EN_MCSPI2_MASK: u32 = (1u32 << 18);
pub const OMAP24XX_EN_MCSPI1_SHIFT: u32 = 17;
pub const OMAP24XX_EN_MCSPI1_MASK: u32 = (1u32 << 17);
pub const OMAP24XX_EN_MCBSP2_SHIFT: u32 = 16;
pub const OMAP24XX_EN_MCBSP2_MASK: u32 = (1u32 << 16);
pub const OMAP24XX_EN_MCBSP1_SHIFT: u32 = 15;
pub const OMAP24XX_EN_MCBSP1_MASK: u32 = (1u32 << 15);
pub const OMAP24XX_EN_GPT12_SHIFT: u32 = 14;
pub const OMAP24XX_EN_GPT12_MASK: u32 = (1u32 << 14);
pub const OMAP24XX_EN_GPT11_SHIFT: u32 = 13;
pub const OMAP24XX_EN_GPT11_MASK: u32 = (1u32 << 13);
pub const OMAP24XX_EN_GPT10_SHIFT: u32 = 12;
pub const OMAP24XX_EN_GPT10_MASK: u32 = (1u32 << 12);
pub const OMAP24XX_EN_GPT9_SHIFT: u32 = 11;
pub const OMAP24XX_EN_GPT9_MASK: u32 = (1u32 << 11);
pub const OMAP24XX_EN_GPT8_SHIFT: u32 = 10;
pub const OMAP24XX_EN_GPT8_MASK: u32 = (1u32 << 10);
pub const OMAP24XX_EN_GPT7_SHIFT: u32 = 9;
pub const OMAP24XX_EN_GPT7_MASK: u32 = (1u32 << 9);
pub const OMAP24XX_EN_GPT6_SHIFT: u32 = 8;
pub const OMAP24XX_EN_GPT6_MASK: u32 = (1u32 << 8);
pub const OMAP24XX_EN_GPT5_SHIFT: u32 = 7;
pub const OMAP24XX_EN_GPT5_MASK: u32 = (1u32 << 7);
pub const OMAP24XX_EN_GPT4_SHIFT: u32 = 6;
pub const OMAP24XX_EN_GPT4_MASK: u32 = (1u32 << 6);
pub const OMAP24XX_EN_GPT3_SHIFT: u32 = 5;
pub const OMAP24XX_EN_GPT3_MASK: u32 = (1u32 << 5);
pub const OMAP24XX_EN_GPT2_SHIFT: u32 = 4;
pub const OMAP24XX_EN_GPT2_MASK: u32 = (1u32 << 4);
pub const OMAP2420_EN_VLYNQ_SHIFT: u32 = 3;
pub const OMAP2420_EN_VLYNQ_MASK: u32 = (1u32 << 3);

/* CM_FCLKEN2_CORE, CM_ICLKEN2_CORE, PM_WKEN2_CORE shared bits */
pub const OMAP2430_EN_GPIO5_SHIFT: u32 = 10;
pub const OMAP2430_EN_GPIO5_MASK: u32 = (1u32 << 10);
pub const OMAP2430_EN_MCSPI3_SHIFT: u32 = 9;
pub const OMAP2430_EN_MCSPI3_MASK: u32 = (1u32 << 9);
pub const OMAP2430_EN_MMCHS2_SHIFT: u32 = 8;
pub const OMAP2430_EN_MMCHS2_MASK: u32 = (1u32 << 8);
pub const OMAP2430_EN_MMCHS1_SHIFT: u32 = 7;
pub const OMAP2430_EN_MMCHS1_MASK: u32 = (1u32 << 7);
pub const OMAP24XX_EN_UART3_SHIFT: u32 = 2;
pub const OMAP24XX_EN_UART3_MASK: u32 = (1u32 << 2);
pub const OMAP24XX_EN_USB_SHIFT: u32 = 0;
pub const OMAP24XX_EN_USB_MASK: u32 = (1u32 << 0);

/* CM_ICLKEN2_CORE, PM_WKEN2_CORE shared bits */
pub const OMAP2430_EN_MDM_INTC_SHIFT: u32 = 11;
pub const OMAP2430_EN_MDM_INTC_MASK: u32 = (1u32 << 11);
pub const OMAP2430_EN_USBHS_SHIFT: u32 = 6;
pub const OMAP2430_EN_USBHS_MASK: u32 = (1u32 << 6);
pub const OMAP24XX_EN_GPMC_SHIFT: u32 = 1;
pub const OMAP24XX_EN_GPMC_MASK: u32 = (1u32 << 1);

/* CM_IDLEST1_CORE, PM_WKST1_CORE shared bits */
pub const OMAP2420_ST_MMC_SHIFT: u32 = 26;
pub const OMAP2420_ST_MMC_MASK: u32 = (1u32 << 26);
pub const OMAP24XX_ST_UART2_SHIFT: u32 = 22;
pub const OMAP24XX_ST_UART2_MASK: u32 = (1u32 << 22);
pub const OMAP24XX_ST_UART1_SHIFT: u32 = 21;
pub const OMAP24XX_ST_UART1_MASK: u32 = (1u32 << 21);
pub const OMAP24XX_ST_MCSPI2_SHIFT: u32 = 18;
pub const OMAP24XX_ST_MCSPI2_MASK: u32 = (1u32 << 18);
pub const OMAP24XX_ST_MCSPI1_SHIFT: u32 = 17;
pub const OMAP24XX_ST_MCSPI1_MASK: u32 = (1u32 << 17);
pub const OMAP24XX_ST_MCBSP2_SHIFT: u32 = 16;
pub const OMAP24XX_ST_MCBSP2_MASK: u32 = (1u32 << 16);
pub const OMAP24XX_ST_MCBSP1_SHIFT: u32 = 15;
pub const OMAP24XX_ST_MCBSP1_MASK: u32 = (1u32 << 15);
pub const OMAP24XX_ST_GPT12_SHIFT: u32 = 14;
pub const OMAP24XX_ST_GPT12_MASK: u32 = (1u32 << 14);
pub const OMAP24XX_ST_GPT11_SHIFT: u32 = 13;
pub const OMAP24XX_ST_GPT11_MASK: u32 = (1u32 << 13);
pub const OMAP24XX_ST_GPT10_SHIFT: u32 = 12;
pub const OMAP24XX_ST_GPT10_MASK: u32 = (1u32 << 12);
pub const OMAP24XX_ST_GPT9_SHIFT: u32 = 11;
pub const OMAP24XX_ST_GPT9_MASK: u32 = (1u32 << 11);
pub const OMAP24XX_ST_GPT8_SHIFT: u32 = 10;
pub const OMAP24XX_ST_GPT8_MASK: u32 = (1u32 << 10);
pub const OMAP24XX_ST_GPT7_SHIFT: u32 = 9;
pub const OMAP24XX_ST_GPT7_MASK: u32 = (1u32 << 9);
pub const OMAP24XX_ST_GPT6_SHIFT: u32 = 8;
pub const OMAP24XX_ST_GPT6_MASK: u32 = (1u32 << 8);
pub const OMAP24XX_ST_GPT5_SHIFT: u32 = 7;
pub const OMAP24XX_ST_GPT5_MASK: u32 = (1u32 << 7);
pub const OMAP24XX_ST_GPT4_SHIFT: u32 = 6;
pub const OMAP24XX_ST_GPT4_MASK: u32 = (1u32 << 6);
pub const OMAP24XX_ST_GPT3_SHIFT: u32 = 5;
pub const OMAP24XX_ST_GPT3_MASK: u32 = (1u32 << 5);
pub const OMAP24XX_ST_GPT2_SHIFT: u32 = 4;
pub const OMAP24XX_ST_GPT2_MASK: u32 = (1u32 << 4);
pub const OMAP2420_ST_VLYNQ_SHIFT: u32 = 3;
pub const OMAP2420_ST_VLYNQ_MASK: u32 = (1u32 << 3);

/* CM_IDLEST2_CORE, PM_WKST2_CORE shared bits */
pub const OMAP2430_ST_MDM_INTC_SHIFT: u32 = 11;
pub const OMAP2430_ST_MDM_INTC_MASK: u32 = (1u32 << 11);
pub const OMAP2430_ST_GPIO5_SHIFT: u32 = 10;
pub const OMAP2430_ST_GPIO5_MASK: u32 = (1u32 << 10);
pub const OMAP2430_ST_MCSPI3_SHIFT: u32 = 9;
pub const OMAP2430_ST_MCSPI3_MASK: u32 = (1u32 << 9);
pub const OMAP2430_ST_MMCHS2_SHIFT: u32 = 8;
pub const OMAP2430_ST_MMCHS2_MASK: u32 = (1u32 << 8);
pub const OMAP2430_ST_MMCHS1_SHIFT: u32 = 7;
pub const OMAP2430_ST_MMCHS1_MASK: u32 = (1u32 << 7);
pub const OMAP2430_ST_USBHS_SHIFT: u32 = 6;
pub const OMAP2430_ST_USBHS_MASK: u32 = (1u32 << 6);
pub const OMAP24XX_ST_UART3_SHIFT: u32 = 2;
pub const OMAP24XX_ST_UART3_MASK: u32 = (1u32 << 2);
pub const OMAP24XX_ST_USB_SHIFT: u32 = 0;
pub const OMAP24XX_ST_USB_MASK: u32 = (1u32 << 0);

/* CM_FCLKEN_WKUP, CM_ICLKEN_WKUP, PM_WKEN_WKUP shared bits */
pub const OMAP24XX_EN_GPIOS_SHIFT: u32 = 2;
pub const OMAP24XX_EN_GPIOS_MASK: u32 = (1u32 << 2);
pub const OMAP24XX_EN_GPT1_SHIFT: u32 = 0;
pub const OMAP24XX_EN_GPT1_MASK: u32 = (1u32 << 0);

/* PM_WKST_WKUP, CM_IDLEST_WKUP shared bits */
pub const OMAP24XX_ST_GPIOS_SHIFT: u32 = 2;
pub const OMAP24XX_ST_GPIOS_MASK: u32 = (1u32 << 2);
pub const OMAP24XX_ST_32KSYNC_SHIFT: u32 = 1;
pub const OMAP24XX_ST_32KSYNC_MASK: u32 = (1u32 << 1);
pub const OMAP24XX_ST_GPT1_SHIFT: u32 = 0;
pub const OMAP24XX_ST_GPT1_MASK: u32 = (1u32 << 0);

/* CM_IDLEST_MDM and PM_WKST_MDM shared bits */
pub const OMAP2430_ST_MDM_SHIFT: u32 = 0;
pub const OMAP2430_ST_MDM_MASK: u32 = (1u32 << 0);


/* 3430 register bits shared between CM & PRM registers */

/* CM_REVISION, PRM_REVISION shared bits */
pub const OMAP3430_REV_SHIFT: u32 = 0;
pub const OMAP3430_REV_MASK: u32 = (0xff << 0);

/* CM_SYSCONFIG, PRM_SYSCONFIG shared bits */
pub const OMAP3430_AUTOIDLE_MASK: u32 = (1u32 << 0);

/* CM_FCLKEN1_CORE, CM_ICLKEN1_CORE, PM_WKEN1_CORE shared bits */
pub const OMAP3430_EN_MMC3_MASK: u32 = (1u32 << 30);
pub const OMAP3430_EN_MMC3_SHIFT: u32 = 30;
pub const OMAP3430_EN_MMC2_MASK: u32 = (1u32 << 25);
pub const OMAP3430_EN_MMC2_SHIFT: u32 = 25;
pub const OMAP3430_EN_MMC1_MASK: u32 = (1u32 << 24);
pub const OMAP3430_EN_MMC1_SHIFT: u32 = 24;
pub const AM35XX_EN_UART4_MASK: u32 = (1u32 << 23);
pub const AM35XX_EN_UART4_SHIFT: u32 = 23;
pub const OMAP3430_EN_MCSPI4_MASK: u32 = (1u32 << 21);
pub const OMAP3430_EN_MCSPI4_SHIFT: u32 = 21;
pub const OMAP3430_EN_MCSPI3_MASK: u32 = (1u32 << 20);
pub const OMAP3430_EN_MCSPI3_SHIFT: u32 = 20;
pub const OMAP3430_EN_MCSPI2_MASK: u32 = (1u32 << 19);
pub const OMAP3430_EN_MCSPI2_SHIFT: u32 = 19;
pub const OMAP3430_EN_MCSPI1_MASK: u32 = (1u32 << 18);
pub const OMAP3430_EN_MCSPI1_SHIFT: u32 = 18;
pub const OMAP3430_EN_I2C3_MASK: u32 = (1u32 << 17);
pub const OMAP3430_EN_I2C3_SHIFT: u32 = 17;
pub const OMAP3430_EN_I2C2_MASK: u32 = (1u32 << 16);
pub const OMAP3430_EN_I2C2_SHIFT: u32 = 16;
pub const OMAP3430_EN_I2C1_MASK: u32 = (1u32 << 15);
pub const OMAP3430_EN_I2C1_SHIFT: u32 = 15;
pub const OMAP3430_EN_UART2_MASK: u32 = (1u32 << 14);
pub const OMAP3430_EN_UART2_SHIFT: u32 = 14;
pub const OMAP3430_EN_UART1_MASK: u32 = (1u32 << 13);
pub const OMAP3430_EN_UART1_SHIFT: u32 = 13;
pub const OMAP3430_EN_GPT11_MASK: u32 = (1u32 << 12);
pub const OMAP3430_EN_GPT11_SHIFT: u32 = 12;
pub const OMAP3430_EN_GPT10_MASK: u32 = (1u32 << 11);
pub const OMAP3430_EN_GPT10_SHIFT: u32 = 11;
pub const OMAP3430_EN_MCBSP5_MASK: u32 = (1u32 << 10);
pub const OMAP3430_EN_MCBSP5_SHIFT: u32 = 10;
pub const OMAP3430_EN_MCBSP1_MASK: u32 = (1u32 << 9);
pub const OMAP3430_EN_MCBSP1_SHIFT: u32 = 9;
pub const OMAP3430_EN_FSHOSTUSB_MASK: u32 = (1u32 << 5);
pub const OMAP3430_EN_FSHOSTUSB_SHIFT: u32 = 5;
pub const OMAP3430_EN_D2D_MASK: u32 = (1u32 << 3);
pub const OMAP3430_EN_D2D_SHIFT: u32 = 3;

/* CM_ICLKEN1_CORE, PM_WKEN1_CORE shared bits */
pub const OMAP3430_EN_HSOTGUSB_MASK: u32 = (1u32 << 4);
pub const OMAP3430_EN_HSOTGUSB_SHIFT: u32 = 4;

/* PM_WKST1_CORE, CM_IDLEST1_CORE shared bits */
pub const OMAP3430_ST_MMC3_SHIFT: u32 = 30;
pub const OMAP3430_ST_MMC3_MASK: u32 = (1u32 << 30);
pub const OMAP3430_ST_MMC2_SHIFT: u32 = 25;
pub const OMAP3430_ST_MMC2_MASK: u32 = (1u32 << 25);
pub const OMAP3430_ST_MMC1_SHIFT: u32 = 24;
pub const OMAP3430_ST_MMC1_MASK: u32 = (1u32 << 24);
pub const OMAP3430_ST_MCSPI4_SHIFT: u32 = 21;
pub const OMAP3430_ST_MCSPI4_MASK: u32 = (1u32 << 21);
pub const OMAP3430_ST_MCSPI3_SHIFT: u32 = 20;
pub const OMAP3430_ST_MCSPI3_MASK: u32 = (1u32 << 20);
pub const OMAP3430_ST_MCSPI2_SHIFT: u32 = 19;
pub const OMAP3430_ST_MCSPI2_MASK: u32 = (1u32 << 19);
pub const OMAP3430_ST_MCSPI1_SHIFT: u32 = 18;
pub const OMAP3430_ST_MCSPI1_MASK: u32 = (1u32 << 18);
pub const OMAP3430_ST_I2C3_SHIFT: u32 = 17;
pub const OMAP3430_ST_I2C3_MASK: u32 = (1u32 << 17);
pub const OMAP3430_ST_I2C2_SHIFT: u32 = 16;
pub const OMAP3430_ST_I2C2_MASK: u32 = (1u32 << 16);
pub const OMAP3430_ST_I2C1_SHIFT: u32 = 15;
pub const OMAP3430_ST_I2C1_MASK: u32 = (1u32 << 15);
pub const OMAP3430_ST_UART2_SHIFT: u32 = 14;
pub const OMAP3430_ST_UART2_MASK: u32 = (1u32 << 14);
pub const OMAP3430_ST_UART1_SHIFT: u32 = 13;
pub const OMAP3430_ST_UART1_MASK: u32 = (1u32 << 13);
pub const OMAP3430_ST_GPT11_SHIFT: u32 = 12;
pub const OMAP3430_ST_GPT11_MASK: u32 = (1u32 << 12);
pub const OMAP3430_ST_GPT10_SHIFT: u32 = 11;
pub const OMAP3430_ST_GPT10_MASK: u32 = (1u32 << 11);
pub const OMAP3430_ST_MCBSP5_SHIFT: u32 = 10;
pub const OMAP3430_ST_MCBSP5_MASK: u32 = (1u32 << 10);
pub const OMAP3430_ST_MCBSP1_SHIFT: u32 = 9;
pub const OMAP3430_ST_MCBSP1_MASK: u32 = (1u32 << 9);
pub const OMAP3430ES1_ST_FSHOSTUSB_SHIFT: u32 = 5;
pub const OMAP3430ES1_ST_FSHOSTUSB_MASK: u32 = (1u32 << 5);
pub const OMAP3430ES1_ST_HSOTGUSB_SHIFT: u32 = 4;
pub const OMAP3430ES1_ST_HSOTGUSB_MASK: u32 = (1u32 << 4);
pub const OMAP3430ES2_ST_HSOTGUSB_IDLE_SHIFT: u32 = 5;
pub const OMAP3430ES2_ST_HSOTGUSB_IDLE_MASK: u32 = (1u32 << 5);
pub const OMAP3430ES2_ST_HSOTGUSB_STDBY_SHIFT: u32 = 4;
pub const OMAP3430ES2_ST_HSOTGUSB_STDBY_MASK: u32 = (1u32 << 4);
pub const OMAP3430_ST_D2D_SHIFT: u32 = 3;
pub const OMAP3430_ST_D2D_MASK: u32 = (1u32 << 3);

/* CM_FCLKEN_WKUP, CM_ICLKEN_WKUP, PM_WKEN_WKUP shared bits */
pub const OMAP3430_EN_GPIO1_MASK: u32 = (1u32 << 3);
pub const OMAP3430_EN_GPIO1_SHIFT: u32 = 3;
pub const OMAP3430_EN_GPT12_MASK: u32 = (1u32 << 1);
pub const OMAP3430_EN_GPT12_SHIFT: u32 = 1;
pub const OMAP3430_EN_GPT1_MASK: u32 = (1u32 << 0);
pub const OMAP3430_EN_GPT1_SHIFT: u32 = 0;

/* CM_FCLKEN_WKUP, PM_WKEN_WKUP shared bits */
pub const OMAP3430_EN_SR2_MASK: u32 = (1u32 << 7);
pub const OMAP3430_EN_SR2_SHIFT: u32 = 7;
pub const OMAP3430_EN_SR1_MASK: u32 = (1u32 << 6);
pub const OMAP3430_EN_SR1_SHIFT: u32 = 6;

/* CM_ICLKEN_WKUP, PM_WKEN_WKUP shared bits */
pub const OMAP3430_EN_GPT12_MASK: u32 = (1u32 << 1);
pub const OMAP3430_EN_GPT12_SHIFT: u32 = 1;

/* CM_IDLEST_WKUP, PM_WKST_WKUP shared bits */
pub const OMAP3430_ST_SR2_SHIFT: u32 = 7;
pub const OMAP3430_ST_SR2_MASK: u32 = (1u32 << 7);
pub const OMAP3430_ST_SR1_SHIFT: u32 = 6;
pub const OMAP3430_ST_SR1_MASK: u32 = (1u32 << 6);
pub const OMAP3430_ST_GPIO1_SHIFT: u32 = 3;
pub const OMAP3430_ST_GPIO1_MASK: u32 = (1u32 << 3);
pub const OMAP3430_ST_32KSYNC_SHIFT: u32 = 2;
pub const OMAP3430_ST_32KSYNC_MASK: u32 = (1u32 << 2);
pub const OMAP3430_ST_GPT12_SHIFT: u32 = 1;
pub const OMAP3430_ST_GPT12_MASK: u32 = (1u32 << 1);
pub const OMAP3430_ST_GPT1_SHIFT: u32 = 0;
pub const OMAP3430_ST_GPT1_MASK: u32 = (1u32 << 0);

/*
 * CM_SLEEPDEP_GFX, CM_SLEEPDEP_DSS, CM_SLEEPDEP_CAM,
 * CM_SLEEPDEP_PER, PM_WKDEP_IVA2, PM_WKDEP_GFX,
 * PM_WKDEP_DSS, PM_WKDEP_CAM, PM_WKDEP_PER, PM_WKDEP_NEON shared bits
 */
pub const OMAP3430_EN_MPU_MASK: u32 = (1u32 << 1);
pub const OMAP3430_EN_MPU_SHIFT: u32 = 1;

/* CM_FCLKEN_PER, CM_ICLKEN_PER, PM_WKEN_PER shared bits */

pub const OMAP3630_EN_UART4_MASK: u32 = (1u32 << 18);
pub const OMAP3630_EN_UART4_SHIFT: u32 = 18;
pub const OMAP3430_EN_GPIO6_MASK: u32 = (1u32 << 17);
pub const OMAP3430_EN_GPIO6_SHIFT: u32 = 17;
pub const OMAP3430_EN_GPIO5_MASK: u32 = (1u32 << 16);
pub const OMAP3430_EN_GPIO5_SHIFT: u32 = 16;
pub const OMAP3430_EN_GPIO4_MASK: u32 = (1u32 << 15);
pub const OMAP3430_EN_GPIO4_SHIFT: u32 = 15;
pub const OMAP3430_EN_GPIO3_MASK: u32 = (1u32 << 14);
pub const OMAP3430_EN_GPIO3_SHIFT: u32 = 14;
pub const OMAP3430_EN_GPIO2_MASK: u32 = (1u32 << 13);
pub const OMAP3430_EN_GPIO2_SHIFT: u32 = 13;
pub const OMAP3430_EN_UART3_MASK: u32 = (1u32 << 11);
pub const OMAP3430_EN_UART3_SHIFT: u32 = 11;
pub const OMAP3430_EN_GPT9_MASK: u32 = (1u32 << 10);
pub const OMAP3430_EN_GPT9_SHIFT: u32 = 10;
pub const OMAP3430_EN_GPT8_MASK: u32 = (1u32 << 9);
pub const OMAP3430_EN_GPT8_SHIFT: u32 = 9;
pub const OMAP3430_EN_GPT7_MASK: u32 = (1u32 << 8);
pub const OMAP3430_EN_GPT7_SHIFT: u32 = 8;
pub const OMAP3430_EN_GPT6_MASK: u32 = (1u32 << 7);
pub const OMAP3430_EN_GPT6_SHIFT: u32 = 7;
pub const OMAP3430_EN_GPT5_MASK: u32 = (1u32 << 6);
pub const OMAP3430_EN_GPT5_SHIFT: u32 = 6;
pub const OMAP3430_EN_GPT4_MASK: u32 = (1u32 << 5);
pub const OMAP3430_EN_GPT4_SHIFT: u32 = 5;
pub const OMAP3430_EN_GPT3_MASK: u32 = (1u32 << 4);
pub const OMAP3430_EN_GPT3_SHIFT: u32 = 4;
pub const OMAP3430_EN_GPT2_MASK: u32 = (1u32 << 3);
pub const OMAP3430_EN_GPT2_SHIFT: u32 = 3;

/* CM_FCLKEN_PER, CM_ICLKEN_PER, PM_WKEN_PER, PM_WKST_PER shared bits */
/* XXX Possible TI documentation bug: should the PM_WKST_PER EN_* bits
 * be ST_* bits instead? */
pub const OMAP3430_EN_MCBSP4_MASK: u32 = (1u32 << 2);
pub const OMAP3430_EN_MCBSP4_SHIFT: u32 = 2;
pub const OMAP3430_EN_MCBSP3_MASK: u32 = (1u32 << 1);
pub const OMAP3430_EN_MCBSP3_SHIFT: u32 = 1;
pub const OMAP3430_EN_MCBSP2_MASK: u32 = (1u32 << 0);
pub const OMAP3430_EN_MCBSP2_SHIFT: u32 = 0;

/* CM_IDLEST_PER, PM_WKST_PER shared bits */
pub const OMAP3630_ST_UART4_SHIFT: u32 = 18;
pub const OMAP3630_ST_UART4_MASK: u32 = (1u32 << 18);
pub const OMAP3430_ST_GPIO6_SHIFT: u32 = 17;
pub const OMAP3430_ST_GPIO6_MASK: u32 = (1u32 << 17);
pub const OMAP3430_ST_GPIO5_SHIFT: u32 = 16;
pub const OMAP3430_ST_GPIO5_MASK: u32 = (1u32 << 16);
pub const OMAP3430_ST_GPIO4_SHIFT: u32 = 15;
pub const OMAP3430_ST_GPIO4_MASK: u32 = (1u32 << 15);
pub const OMAP3430_ST_GPIO3_SHIFT: u32 = 14;
pub const OMAP3430_ST_GPIO3_MASK: u32 = (1u32 << 14);
pub const OMAP3430_ST_GPIO2_SHIFT: u32 = 13;
pub const OMAP3430_ST_GPIO2_MASK: u32 = (1u32 << 13);
pub const OMAP3430_ST_UART3_SHIFT: u32 = 11;
pub const OMAP3430_ST_UART3_MASK: u32 = (1u32 << 11);
pub const OMAP3430_ST_GPT9_SHIFT: u32 = 10;
pub const OMAP3430_ST_GPT9_MASK: u32 = (1u32 << 10);
pub const OMAP3430_ST_GPT8_SHIFT: u32 = 9;
pub const OMAP3430_ST_GPT8_MASK: u32 = (1u32 << 9);
pub const OMAP3430_ST_GPT7_SHIFT: u32 = 8;
pub const OMAP3430_ST_GPT7_MASK: u32 = (1u32 << 8);
pub const OMAP3430_ST_GPT6_SHIFT: u32 = 7;
pub const OMAP3430_ST_GPT6_MASK: u32 = (1u32 << 7);
pub const OMAP3430_ST_GPT5_SHIFT: u32 = 6;
pub const OMAP3430_ST_GPT5_MASK: u32 = (1u32 << 6);
pub const OMAP3430_ST_GPT4_SHIFT: u32 = 5;
pub const OMAP3430_ST_GPT4_MASK: u32 = (1u32 << 5);
pub const OMAP3430_ST_GPT3_SHIFT: u32 = 4;
pub const OMAP3430_ST_GPT3_MASK: u32 = (1u32 << 4);
pub const OMAP3430_ST_GPT2_SHIFT: u32 = 3;
pub const OMAP3430_ST_GPT2_MASK: u32 = (1u32 << 3);

/* CM_SLEEPDEP_PER, PM_WKDEP_IVA2, PM_WKDEP_MPU, PM_WKDEP_PER shared bits */
pub const OMAP3430_EN_CORE_SHIFT: u32 = 0;
pub const OMAP3430_EN_CORE_MASK: u32 = (1u32 << 0);



/*
 * Maximum time(us) it takes to output the signal WUCLKOUT of the last
 * pad of the I/O ring after asserting WUCLKIN high.  Tero measured
 * the actual time at 7 to 8 microseconds on OMAP3 and 2 to 4
 * microseconds on OMAP4, so this timeout may be too high.
 */
pub const MAX_IOPAD_LATCH_TIME: u32 = 100;

#[macro_export]
macro_rules! omap_test_timeout {
    ($cond:expr, $timeout:expr, $index:ident) => {{
        for $index in 0..$timeout {
            if $cond { break; }
            unsafe { udelay(1); }
        }
    }};
}

extern "C" {
    fn udelay(usecs: u32);
}


#[repr(C)]
pub struct omap_prcm_irq {
    pub name: *const core::ffi::c_char,
    pub offset: u32,
    pub priority: bool,
}

#[repr(C)]
pub struct omap_prcm_irq_setup {
    pub ack: u16,
    pub mask: u16,
    pub pm_ctrl: u16,
    pub nr_regs: u8,
    pub nr_irqs: u8,
    pub irqs: *const omap_prcm_irq,
    pub irq: i32,
    pub read_pending_irqs: Option<unsafe extern "C" fn(*mut usize)>,
    pub ocp_barrier: Option<unsafe extern "C" fn()>,
    pub save_and_clear_irqen: Option<unsafe extern "C" fn(*mut u32)>,
    pub restore_irqen: Option<unsafe extern "C" fn(*mut u32)>,
    pub reconfigure_io_chain: Option<unsafe extern "C" fn()>,
    pub saved_mask: *mut u32,
    pub priority_mask: *mut u32,
    pub base_irq: i32,
    pub suspended: bool,
    pub suspend_save_flag: bool,
}

#[repr(C)]
pub struct omap_domain_base {
    pub pa: u32,
    pub va: *mut core::ffi::c_void,
    pub offset: i16,
}

#[repr(C)]
pub struct omap_prcm_init_data {
    pub index: i32,
    pub mem: *mut core::ffi::c_void,
    pub phys: u32,
    pub offset: i16,
    pub flags: u16,
    pub device_inst_offset: i32,
    pub init: Option<unsafe extern "C" fn(*const omap_prcm_init_data) -> i32>,
    pub np: *mut device_node,
}

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

extern "C" {
    pub fn omap_prcm_register_chain_handler(irq_setup: *mut omap_prcm_irq_setup) -> i32;
    pub fn omap_prcm_event_to_irq(event: *const core::ffi::c_char) -> i32;
    pub fn omap_prcm_irq_prepare();
    pub fn omap_prcm_irq_complete();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
