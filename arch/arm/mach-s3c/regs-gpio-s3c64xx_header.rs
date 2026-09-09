/* SPDX-License-Identifier: GPL-2.0 */
/* linux/arch/arm/plat-s3c64xx/include/mach/regs-gpio.h
 *
 * Copyright 2008 Openmoko, Inc.
 * Copyright 2008 Simtec Electronics
 *      Ben Dooks <ben@simtec.co.uk>
 *      http://armlinux.simtec.co.uk/
 *
 * S3C64XX - GPIO register definitions
 */

// S3C64XX_VA_GPIO is supplied by the platform dependencies.
macro_rules! S3C64XX_GPIOREG { ($reg:expr) => { S3C64XX_VA_GPIO + ($reg) }; }

pub const S3C64XX_GPA_BASE: usize = S3C64XX_GPIOREG!(0x0000usize);
pub const S3C64XX_GPB_BASE: usize = S3C64XX_GPIOREG!(0x0020usize);
pub const S3C64XX_GPC_BASE: usize = S3C64XX_GPIOREG!(0x0040usize);
pub const S3C64XX_GPD_BASE: usize = S3C64XX_GPIOREG!(0x0060usize);
pub const S3C64XX_GPE_BASE: usize = S3C64XX_GPIOREG!(0x0080usize);
pub const S3C64XX_GPF_BASE: usize = S3C64XX_GPIOREG!(0x00A0usize);
pub const S3C64XX_GPG_BASE: usize = S3C64XX_GPIOREG!(0x00C0usize);
pub const S3C64XX_GPH_BASE: usize = S3C64XX_GPIOREG!(0x00E0usize);
pub const S3C64XX_GPI_BASE: usize = S3C64XX_GPIOREG!(0x0100usize);
pub const S3C64XX_GPJ_BASE: usize = S3C64XX_GPIOREG!(0x0120usize);
pub const S3C64XX_GPK_BASE: usize = S3C64XX_GPIOREG!(0x0800usize);
pub const S3C64XX_GPL_BASE: usize = S3C64XX_GPIOREG!(0x0810usize);
pub const S3C64XX_GPM_BASE: usize = S3C64XX_GPIOREG!(0x0820usize);
pub const S3C64XX_GPN_BASE: usize = S3C64XX_GPIOREG!(0x0830usize);
pub const S3C64XX_GPO_BASE: usize = S3C64XX_GPIOREG!(0x0140usize);
pub const S3C64XX_GPP_BASE: usize = S3C64XX_GPIOREG!(0x0160usize);
pub const S3C64XX_GPQ_BASE: usize = S3C64XX_GPIOREG!(0x0180usize);

/* SPCON */
pub const S3C64XX_SPCON: usize = S3C64XX_GPIOREG!(0x1A0usize);

macro_rules! s3c64xx_const { ($name:ident, $value:expr) => { pub const $name: usize = $value; }; }

s3c64xx_const!(S3C64XX_SPCON_DRVCON_CAM_MASK, 0x3usize << 30);
s3c64xx_const!(S3C64XX_SPCON_DRVCON_CAM_SHIFT, 30usize);
s3c64xx_const!(S3C64XX_SPCON_DRVCON_CAM_2mA, 0x0usize << 30);
s3c64xx_const!(S3C64XX_SPCON_DRVCON_CAM_4mA, 0x1usize << 30);
s3c64xx_const!(S3C64XX_SPCON_DRVCON_CAM_7mA, 0x2usize << 30);
s3c64xx_const!(S3C64XX_SPCON_DRVCON_CAM_9mA, 0x3usize << 30);
s3c64xx_const!(S3C64XX_SPCON_DRVCON_HSSPI_MASK, 0x3usize << 28);
s3c64xx_const!(S3C64XX_SPCON_DRVCON_HSSPI_SHIFT, 28usize);
s3c64xx_const!(S3C64XX_SPCON_DRVCON_HSSPI_2mA, 0x0usize << 28);
s3c64xx_const!(S3C64XX_SPCON_DRVCON_HSSPI_4mA, 0x1usize << 28);
s3c64xx_const!(S3C64XX_SPCON_DRVCON_HSSPI_7mA, 0x2usize << 28);
s3c64xx_const!(S3C64XX_SPCON_DRVCON_HSSPI_9mA, 0x3usize << 28);
s3c64xx_const!(S3C64XX_SPCON_DRVCON_HSMMC_MASK, 0x3usize << 26);
s3c64xx_const!(S3C64XX_SPCON_DRVCON_HSMMC_SHIFT, 26usize);
s3c64xx_const!(S3C64XX_SPCON_DRVCON_HSMMC_2mA, 0x0usize << 26);
s3c64xx_const!(S3C64XX_SPCON_DRVCON_HSMMC_4mA, 0x1usize << 26);
s3c64xx_const!(S3C64XX_SPCON_DRVCON_HSMMC_7mA, 0x2usize << 26);
s3c64xx_const!(S3C64XX_SPCON_DRVCON_HSMMC_9mA, 0x3usize << 26);
s3c64xx_const!(S3C64XX_SPCON_DRVCON_LCD_MASK, 0x3usize << 24);
s3c64xx_const!(S3C64XX_SPCON_DRVCON_LCD_SHIFT, 24usize);
s3c64xx_const!(S3C64XX_SPCON_DRVCON_LCD_2mA, 0x0usize << 24);
s3c64xx_const!(S3C64XX_SPCON_DRVCON_LCD_4mA, 0x1usize << 24);
s3c64xx_const!(S3C64XX_SPCON_DRVCON_LCD_7mA, 0x2usize << 24);
s3c64xx_const!(S3C64XX_SPCON_DRVCON_LCD_9mA, 0x3usize << 24);
s3c64xx_const!(S3C64XX_SPCON_DRVCON_MODEM_MASK, 0x3usize << 22);
s3c64xx_const!(S3C64XX_SPCON_DRVCON_MODEM_SHIFT, 22usize);
s3c64xx_const!(S3C64XX_SPCON_DRVCON_MODEM_2mA, 0x0usize << 22);
s3c64xx_const!(S3C64XX_SPCON_DRVCON_MODEM_4mA, 0x1usize << 22);
s3c64xx_const!(S3C64XX_SPCON_DRVCON_MODEM_7mA, 0x2usize << 22);
s3c64xx_const!(S3C64XX_SPCON_DRVCON_MODEM_9mA, 0x3usize << 22);
s3c64xx_const!(S3C64XX_SPCON_nRSTOUT_OEN, 1usize << 21);
s3c64xx_const!(S3C64XX_SPCON_DRVCON_SPICLK1_MASK, 0x3usize << 18);
s3c64xx_const!(S3C64XX_SPCON_DRVCON_SPICLK1_SHIFT, 18usize);
s3c64xx_const!(S3C64XX_SPCON_DRVCON_SPICLK1_2mA, 0x0usize << 18);
s3c64xx_const!(S3C64XX_SPCON_DRVCON_SPICLK1_4mA, 0x1usize << 18);
s3c64xx_const!(S3C64XX_SPCON_DRVCON_SPICLK1_7mA, 0x2usize << 18);
s3c64xx_const!(S3C64XX_SPCON_DRVCON_SPICLK1_9mA, 0x3usize << 18);
s3c64xx_const!(S3C64XX_SPCON_MEM1_DQS_PUD_MASK, 0x3usize << 16);
s3c64xx_const!(S3C64XX_SPCON_MEM1_DQS_PUD_SHIFT, 16usize);
s3c64xx_const!(S3C64XX_SPCON_MEM1_DQS_PUD_DISABLED, 0x0usize << 16);
s3c64xx_const!(S3C64XX_SPCON_MEM1_DQS_PUD_DOWN, 0x1usize << 16);
s3c64xx_const!(S3C64XX_SPCON_MEM1_DQS_PUD_UP, 0x2usize << 16);
s3c64xx_const!(S3C64XX_SPCON_MEM1_D_PUD1_MASK, 0x3usize << 14);
s3c64xx_const!(S3C64XX_SPCON_MEM1_D_PUD1_SHIFT, 14usize);
s3c64xx_const!(S3C64XX_SPCON_MEM1_D_PUD1_DISABLED, 0x0usize << 14);
s3c64xx_const!(S3C64XX_SPCON_MEM1_D_PUD1_DOWN, 0x1usize << 14);
s3c64xx_const!(S3C64XX_SPCON_MEM1_D_PUD1_UP, 0x2usize << 14);
s3c64xx_const!(S3C64XX_SPCON_MEM1_D_PUD0_MASK, 0x3usize << 12);
s3c64xx_const!(S3C64XX_SPCON_MEM1_D_PUD0_SHIFT, 12usize);
s3c64xx_const!(S3C64XX_SPCON_MEM1_D_PUD0_DISABLED, 0x0usize << 12);
s3c64xx_const!(S3C64XX_SPCON_MEM1_D_PUD0_DOWN, 0x1usize << 12);
s3c64xx_const!(S3C64XX_SPCON_MEM1_D_PUD0_UP, 0x2usize << 12);
s3c64xx_const!(S3C64XX_SPCON_MEM0_D_PUD_MASK, 0x3usize << 8);
s3c64xx_const!(S3C64XX_SPCON_MEM0_D_PUD_SHIFT, 8usize);
s3c64xx_const!(S3C64XX_SPCON_MEM0_D_PUD_DISABLED, 0x0usize << 8);
s3c64xx_const!(S3C64XX_SPCON_MEM0_D_PUD_DOWN, 0x1usize << 8);
s3c64xx_const!(S3C64XX_SPCON_MEM0_D_PUD_UP, 0x2usize << 8);
s3c64xx_const!(S3C64XX_SPCON_USBH_DMPD, 1usize << 7);
s3c64xx_const!(S3C64XX_SPCON_USBH_DPPD, 1usize << 6);
s3c64xx_const!(S3C64XX_SPCON_USBH_PUSW2, 1usize << 5);
s3c64xx_const!(S3C64XX_SPCON_USBH_PUSW1, 1usize << 4);
s3c64xx_const!(S3C64XX_SPCON_USBH_SUSPND, 1usize << 3);
s3c64xx_const!(S3C64XX_SPCON_LCD_SEL_MASK, 0x3usize << 0);
s3c64xx_const!(S3C64XX_SPCON_LCD_SEL_SHIFT, 0usize);
s3c64xx_const!(S3C64XX_SPCON_LCD_SEL_HOST, 0x0usize << 0);
s3c64xx_const!(S3C64XX_SPCON_LCD_SEL_RGB, 0x1usize << 0);
s3c64xx_const!(S3C64XX_SPCON_LCD_SEL_606_656, 0x2usize << 0);

/* External interrupt registers */
pub const S3C64XX_EINT12CON: usize = S3C64XX_GPIOREG!(0x200usize);
pub const S3C64XX_EINT34CON: usize = S3C64XX_GPIOREG!(0x204usize);
pub const S3C64XX_EINT56CON: usize = S3C64XX_GPIOREG!(0x208usize);
pub const S3C64XX_EINT78CON: usize = S3C64XX_GPIOREG!(0x20Cusize);
pub const S3C64XX_EINT9CON: usize = S3C64XX_GPIOREG!(0x210usize);
pub const S3C64XX_EINT12FLTCON: usize = S3C64XX_GPIOREG!(0x220usize);
pub const S3C64XX_EINT34FLTCON: usize = S3C64XX_GPIOREG!(0x224usize);
pub const S3C64XX_EINT56FLTCON: usize = S3C64XX_GPIOREG!(0x228usize);
pub const S3C64XX_EINT78FLTCON: usize = S3C64XX_GPIOREG!(0x22Cusize);
pub const S3C64XX_EINT9FLTCON: usize = S3C64XX_GPIOREG!(0x230usize);
pub const S3C64XX_EINT12MASK: usize = S3C64XX_GPIOREG!(0x240usize);
pub const S3C64XX_EINT34MASK: usize = S3C64XX_GPIOREG!(0x244usize);
pub const S3C64XX_EINT56MASK: usize = S3C64XX_GPIOREG!(0x248usize);
pub const S3C64XX_EINT78MASK: usize = S3C64XX_GPIOREG!(0x24Cusize);
pub const S3C64XX_EINT9MASK: usize = S3C64XX_GPIOREG!(0x250usize);
pub const S3C64XX_EINT12PEND: usize = S3C64XX_GPIOREG!(0x260usize);
pub const S3C64XX_EINT34PEND: usize = S3C64XX_GPIOREG!(0x264usize);
pub const S3C64XX_EINT56PEND: usize = S3C64XX_GPIOREG!(0x268usize);
pub const S3C64XX_EINT78PEND: usize = S3C64XX_GPIOREG!(0x26Cusize);
pub const S3C64XX_EINT9PEND: usize = S3C64XX_GPIOREG!(0x270usize);
pub const S3C64XX_PRIORITY: usize = S3C64XX_GPIOREG!(0x280usize);
macro_rules! S3C64XX_PRIORITY_ARB { ($x:expr) => { 1usize << ($x) }; }
pub const S3C64XX_SERVICE: usize = S3C64XX_GPIOREG!(0x284usize);
pub const S3C64XX_SERVICEPEND: usize = S3C64XX_GPIOREG!(0x288usize);
pub const S3C64XX_EINT0CON0: usize = S3C64XX_GPIOREG!(0x900usize);
pub const S3C64XX_EINT0CON1: usize = S3C64XX_GPIOREG!(0x904usize);
pub const S3C64XX_EINT0FLTCON0: usize = S3C64XX_GPIOREG!(0x910usize);
pub const S3C64XX_EINT0FLTCON1: usize = S3C64XX_GPIOREG!(0x914usize);
pub const S3C64XX_EINT0FLTCON2: usize = S3C64XX_GPIOREG!(0x918usize);
pub const S3C64XX_EINT0FLTCON3: usize = S3C64XX_GPIOREG!(0x91Cusize);
pub const S3C64XX_EINT0MASK: usize = S3C64XX_GPIOREG!(0x920usize);
pub const S3C64XX_EINT0PEND: usize = S3C64XX_GPIOREG!(0x924usize);

/* GPIO sleep configuration */
pub const S3C64XX_SPCONSLP: usize = S3C64XX_GPIOREG!(0x880usize);
s3c64xx_const!(S3C64XX_SPCONSLP_TDO_PULLDOWN, 1usize << 14);
s3c64xx_const!(S3C64XX_SPCONSLP_CKE1INIT, 1usize << 5);
s3c64xx_const!(S3C64XX_SPCONSLP_RSTOUT_MASK, 0x3usize << 12);
s3c64xx_const!(S3C64XX_SPCONSLP_RSTOUT_OUT0, 0x0usize << 12);
s3c64xx_const!(S3C64XX_SPCONSLP_RSTOUT_OUT1, 0x1usize << 12);
s3c64xx_const!(S3C64XX_SPCONSLP_RSTOUT_HIZ, 0x2usize << 12);
s3c64xx_const!(S3C64XX_SPCONSLP_KPCOL_MASK, 0x3usize << 0);
s3c64xx_const!(S3C64XX_SPCONSLP_KPCOL_OUT0, 0x0usize << 0);
s3c64xx_const!(S3C64XX_SPCONSLP_KPCOL_OUT1, 0x1usize << 0);
s3c64xx_const!(S3C64XX_SPCONSLP_KPCOL_INP, 0x2usize << 0);
pub const S3C64XX_SLPEN: usize = S3C64XX_GPIOREG!(0x930usize);
s3c64xx_const!(S3C64XX_SLPEN_USE_xSLP, 1usize << 0);
s3c64xx_const!(S3C64XX_SLPEN_CFG_BYSLPEN, 1usize << 1);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
