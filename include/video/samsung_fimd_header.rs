/* SPDX-License-Identifier: GPL-2.0-only */
/* include/video/samsung_fimd.h
 *
 * Copyright 2008 Openmoko, Inc.
 * Copyright 2008 Simtec Electronics
 *      http://armlinux.simtec.co.uk/
 *      Ben Dooks <ben@simtec.co.uk>
 *
 * S3C Platform - new-style fimd and framebuffer register definitions
 *
 * This is the register set for the fimd and new style framebuffer interface
 * found from the S3C2443 onwards into the S3C2416, S3C2450, the
 * S3C64XX series such as the S3C6400 and S3C6410, and Exynos series.
*/

/* VIDCON0 */

pub const VIDCON0: u32 = 0x00;
macro_rules! VIDCON0_DSI_EN				 { (1 << 30:expr) => {  }; }
macro_rules! VIDCON0_INTERLACE			 { (1 << 29:expr) => {  }; }
macro_rules! VIDCON0_VIDOUT_MASK			 { (0x7 << 26:expr) => {  }; }
pub const VIDCON0_VIDOUT_SHIFT: u32 = 26;
macro_rules! VIDCON0_VIDOUT_RGB			 { (0x0 << 26:expr) => {  }; }
macro_rules! VIDCON0_VIDOUT_TV			 { (0x1 << 26:expr) => {  }; }
macro_rules! VIDCON0_VIDOUT_I80_LDI0			 { (0x2 << 26:expr) => {  }; }
macro_rules! VIDCON0_VIDOUT_I80_LDI1			 { (0x3 << 26:expr) => {  }; }
macro_rules! VIDCON0_VIDOUT_WB_RGB			 { (0x4 << 26:expr) => {  }; }
macro_rules! VIDCON0_VIDOUT_WB_I80_LDI0		 { (0x6 << 26:expr) => {  }; }
macro_rules! VIDCON0_VIDOUT_WB_I80_LDI1		 { (0x7 << 26:expr) => {  }; }

macro_rules! VIDCON0_L1_DATA_MASK			 { (0x7 << 23:expr) => {  }; }
pub const VIDCON0_L1_DATA_SHIFT: u32 = 23;
macro_rules! VIDCON0_L1_DATA_16BPP			 { (0x0 << 23:expr) => {  }; }
macro_rules! VIDCON0_L1_DATA_18BPP16			 { (0x1 << 23:expr) => {  }; }
macro_rules! VIDCON0_L1_DATA_18BPP9			 { (0x2 << 23:expr) => {  }; }
macro_rules! VIDCON0_L1_DATA_24BPP			 { (0x3 << 23:expr) => {  }; }
macro_rules! VIDCON0_L1_DATA_18BPP			 { (0x4 << 23:expr) => {  }; }
macro_rules! VIDCON0_L1_DATA_16BPP8			 { (0x5 << 23:expr) => {  }; }

macro_rules! VIDCON0_L0_DATA_MASK			 { (0x7 << 20:expr) => {  }; }
pub const VIDCON0_L0_DATA_SHIFT: u32 = 20;
macro_rules! VIDCON0_L0_DATA_16BPP			 { (0x0 << 20:expr) => {  }; }
macro_rules! VIDCON0_L0_DATA_18BPP16			 { (0x1 << 20:expr) => {  }; }
macro_rules! VIDCON0_L0_DATA_18BPP9			 { (0x2 << 20:expr) => {  }; }
macro_rules! VIDCON0_L0_DATA_24BPP			 { (0x3 << 20:expr) => {  }; }
macro_rules! VIDCON0_L0_DATA_18BPP			 { (0x4 << 20:expr) => {  }; }
macro_rules! VIDCON0_L0_DATA_16BPP8			 { (0x5 << 20:expr) => {  }; }

macro_rules! VIDCON0_PNRMODE_MASK			 { (0x3 << 17:expr) => {  }; }
pub const VIDCON0_PNRMODE_SHIFT: u32 = 17;
macro_rules! VIDCON0_PNRMODE_RGB			 { (0x0 << 17:expr) => {  }; }
macro_rules! VIDCON0_PNRMODE_BGR			 { (0x1 << 17:expr) => {  }; }
macro_rules! VIDCON0_PNRMODE_SERIAL_RGB		 { (0x2 << 17:expr) => {  }; }
macro_rules! VIDCON0_PNRMODE_SERIAL_BGR		 { (0x3 << 17:expr) => {  }; }

macro_rules! VIDCON0_CLKVALUP			 { (1 << 16:expr) => {  }; }
macro_rules! VIDCON0_CLKVAL_F_MASK			 { (0xff << 6:expr) => {  }; }
pub const VIDCON0_CLKVAL_F_SHIFT: u32 = 6;
pub const VIDCON0_CLKVAL_F_LIMIT: u32 = 0xff;
macro_rules! VIDCON0_CLKVAL_F { (_x((_x) << 6):expr) => { ((_x) << 6) }; }
macro_rules! VIDCON0_VLCKFREE			 { (1 << 5:expr) => {  }; }
macro_rules! VIDCON0_CLKDIR				 { (1 << 4:expr) => {  }; }

macro_rules! VIDCON0_CLKSEL_MASK			 { (0x3 << 2:expr) => {  }; }
pub const VIDCON0_CLKSEL_SHIFT: u32 = 2;
macro_rules! VIDCON0_CLKSEL_HCLK			 { (0x0 << 2:expr) => {  }; }
macro_rules! VIDCON0_CLKSEL_LCD			 { (0x1 << 2:expr) => {  }; }
macro_rules! VIDCON0_CLKSEL_27M			 { (0x3 << 2:expr) => {  }; }

macro_rules! VIDCON0_ENVID				 { (1 << 1:expr) => {  }; }
macro_rules! VIDCON0_ENVID_F				 { (1 << 0:expr) => {  }; }

pub const VIDCON1: u32 = 0x04;
macro_rules! VIDCON1_LINECNT_MASK			 { (0x7ff << 16:expr) => {  }; }
pub const VIDCON1_LINECNT_SHIFT: u32 = 16;
macro_rules! VIDCON1_LINECNT_GET { (_v(((_v) >> 16) & 0x7ff):expr) => { (((_v) >> 16) & 0x7ff) }; }
macro_rules! VIDCON1_FSTATUS_EVEN			 { (1 << 15:expr) => {  }; }
macro_rules! VIDCON1_VSTATUS_MASK			 { (0x3 << 13:expr) => {  }; }
pub const VIDCON1_VSTATUS_SHIFT: u32 = 13;
macro_rules! VIDCON1_VSTATUS_VSYNC			 { (0x0 << 13:expr) => {  }; }
macro_rules! VIDCON1_VSTATUS_BACKPORCH		 { (0x1 << 13:expr) => {  }; }
macro_rules! VIDCON1_VSTATUS_ACTIVE			 { (0x2 << 13:expr) => {  }; }
macro_rules! VIDCON1_VSTATUS_FRONTPORCH		 { (0x3 << 13:expr) => {  }; }
macro_rules! VIDCON1_VCLK_MASK			 { (0x3 << 9:expr) => {  }; }
macro_rules! VIDCON1_VCLK_HOLD			 { (0x0 << 9:expr) => {  }; }
macro_rules! VIDCON1_VCLK_RUN			 { (0x1 << 9:expr) => {  }; }

macro_rules! VIDCON1_INV_VCLK			 { (1 << 7:expr) => {  }; }
macro_rules! VIDCON1_INV_HSYNC			 { (1 << 6:expr) => {  }; }
macro_rules! VIDCON1_INV_VSYNC			 { (1 << 5:expr) => {  }; }
macro_rules! VIDCON1_INV_VDEN			 { (1 << 4:expr) => {  }; }

/* VIDCON2 */

pub const VIDCON2: u32 = 0x08;
macro_rules! VIDCON2_EN601				 { (1 << 23:expr) => {  }; }
macro_rules! VIDCON2_TVFMTSEL_SW			 { (1 << 14:expr) => {  }; }

macro_rules! VIDCON2_TVFMTSEL1_MASK			 { (0x3 << 12:expr) => {  }; }
pub const VIDCON2_TVFMTSEL1_SHIFT: u32 = 12;
macro_rules! VIDCON2_TVFMTSEL1_RGB			 { (0x0 << 12:expr) => {  }; }
macro_rules! VIDCON2_TVFMTSEL1_YUV422		 { (0x1 << 12:expr) => {  }; }
macro_rules! VIDCON2_TVFMTSEL1_YUV444		 { (0x2 << 12:expr) => {  }; }

macro_rules! VIDCON2_ORGYCbCr			 { (1 << 8:expr) => {  }; }
macro_rules! VIDCON2_YUVORDCrCb			 { (1 << 7:expr) => {  }; }

/* PRTCON (S3C6410)
 * Might not be present in the S3C6410 documentation,
 * but tests prove it's there almost for sure; shouldn't hurt in any case.
 */
pub const PRTCON: u32 = 0x0c;
macro_rules! PRTCON_PROTECT				 { (1 << 11:expr) => {  }; }

/* VIDTCON0 */

pub const VIDTCON0: u32 = 0x10;
macro_rules! VIDTCON0_VBPDE_MASK			 { (0xff << 24:expr) => {  }; }
pub const VIDTCON0_VBPDE_SHIFT: u32 = 24;
pub const VIDTCON0_VBPDE_LIMIT: u32 = 0xff;
macro_rules! VIDTCON0_VBPDE { (_x((_x) << 24):expr) => { ((_x) << 24) }; }

macro_rules! VIDTCON0_VBPD_MASK			 { (0xff << 16:expr) => {  }; }
pub const VIDTCON0_VBPD_SHIFT: u32 = 16;
pub const VIDTCON0_VBPD_LIMIT: u32 = 0xff;
macro_rules! VIDTCON0_VBPD { (_x((_x) << 16):expr) => { ((_x) << 16) }; }

macro_rules! VIDTCON0_VFPD_MASK			 { (0xff << 8:expr) => {  }; }
pub const VIDTCON0_VFPD_SHIFT: u32 = 8;
pub const VIDTCON0_VFPD_LIMIT: u32 = 0xff;
macro_rules! VIDTCON0_VFPD { (_x((_x) << 8):expr) => { ((_x) << 8) }; }

macro_rules! VIDTCON0_VSPW_MASK			 { (0xff << 0:expr) => {  }; }
pub const VIDTCON0_VSPW_SHIFT: u32 = 0;
pub const VIDTCON0_VSPW_LIMIT: u32 = 0xff;
macro_rules! VIDTCON0_VSPW { (_x((_x) << 0):expr) => { ((_x) << 0) }; }

/* VIDTCON1 */

pub const VIDTCON1: u32 = 0x14;
macro_rules! VIDTCON1_VFPDE_MASK			 { (0xff << 24:expr) => {  }; }
pub const VIDTCON1_VFPDE_SHIFT: u32 = 24;
pub const VIDTCON1_VFPDE_LIMIT: u32 = 0xff;
macro_rules! VIDTCON1_VFPDE { (_x((_x) << 24):expr) => { ((_x) << 24) }; }

macro_rules! VIDTCON1_HBPD_MASK			 { (0xff << 16:expr) => {  }; }
pub const VIDTCON1_HBPD_SHIFT: u32 = 16;
pub const VIDTCON1_HBPD_LIMIT: u32 = 0xff;
macro_rules! VIDTCON1_HBPD { (_x((_x) << 16):expr) => { ((_x) << 16) }; }

macro_rules! VIDTCON1_HFPD_MASK			 { (0xff << 8:expr) => {  }; }
pub const VIDTCON1_HFPD_SHIFT: u32 = 8;
pub const VIDTCON1_HFPD_LIMIT: u32 = 0xff;
macro_rules! VIDTCON1_HFPD { (_x((_x) << 8):expr) => { ((_x) << 8) }; }

macro_rules! VIDTCON1_HSPW_MASK			 { (0xff << 0:expr) => {  }; }
pub const VIDTCON1_HSPW_SHIFT: u32 = 0;
pub const VIDTCON1_HSPW_LIMIT: u32 = 0xff;
macro_rules! VIDTCON1_HSPW { (_x((_x) << 0):expr) => { ((_x) << 0) }; }

pub const VIDTCON2: u32 = 0x18;
macro_rules! VIDTCON2_LINEVAL_E { (_x((((_x) & 0x800) >> 11) << 23):expr) => { ((((_x) & 0x800) >> 11) << 23) }; }
macro_rules! VIDTCON2_LINEVAL_MASK			 { (0x7ff << 11:expr) => {  }; }
pub const VIDTCON2_LINEVAL_SHIFT: u32 = 11;
pub const VIDTCON2_LINEVAL_LIMIT: u32 = 0x7ff;
macro_rules! VIDTCON2_LINEVAL { (_x(((_x) & 0x7ff) << 11):expr) => { (((_x) & 0x7ff) << 11) }; }

macro_rules! VIDTCON2_HOZVAL_E { (_x((((_x) & 0x800) >> 11) << 22):expr) => { ((((_x) & 0x800) >> 11) << 22) }; }
macro_rules! VIDTCON2_HOZVAL_MASK			 { (0x7ff << 0:expr) => {  }; }
pub const VIDTCON2_HOZVAL_SHIFT: u32 = 0;
pub const VIDTCON2_HOZVAL_LIMIT: u32 = 0x7ff;
macro_rules! VIDTCON2_HOZVAL { (_x(((_x) & 0x7ff) << 0):expr) => { (((_x) & 0x7ff) << 0) }; }

/* WINCONx */

macro_rules! WINCON { (_win(0x20 + ((_win) * 4)):expr) => { (0x20 + ((_win) * 4)) }; }
macro_rules! WINCONx_CSCCON_EQ601			 { (0x0 << 28:expr) => {  }; }
macro_rules! WINCONx_CSCCON_EQ709			 { (0x1 << 28:expr) => {  }; }
macro_rules! WINCONx_CSCWIDTH_MASK			 { (0x3 << 26:expr) => {  }; }
pub const WINCONx_CSCWIDTH_SHIFT: u32 = 26;
macro_rules! WINCONx_CSCWIDTH_WIDE			 { (0x0 << 26:expr) => {  }; }
macro_rules! WINCONx_CSCWIDTH_NARROW			 { (0x3 << 26:expr) => {  }; }
macro_rules! WINCONx_ENLOCAL				 { (1 << 22:expr) => {  }; }
macro_rules! WINCONx_BUFSTATUS			 { (1 << 21:expr) => {  }; }
macro_rules! WINCONx_BUFSEL				 { (1 << 20:expr) => {  }; }
macro_rules! WINCONx_BUFAUTOEN			 { (1 << 19:expr) => {  }; }
macro_rules! WINCONx_BITSWP				 { (1 << 18:expr) => {  }; }
macro_rules! WINCONx_BYTSWP				 { (1 << 17:expr) => {  }; }
macro_rules! WINCONx_HAWSWP				 { (1 << 16:expr) => {  }; }
macro_rules! WINCONx_WSWP				 { (1 << 15:expr) => {  }; }
macro_rules! WINCONx_YCbCr				 { (1 << 13:expr) => {  }; }
macro_rules! WINCONx_BURSTLEN_MASK			 { (0x3 << 9:expr) => {  }; }
pub const WINCONx_BURSTLEN_SHIFT: u32 = 9;
macro_rules! WINCONx_BURSTLEN_16WORD			 { (0x0 << 9:expr) => {  }; }
macro_rules! WINCONx_BURSTLEN_8WORD			 { (0x1 << 9:expr) => {  }; }
macro_rules! WINCONx_BURSTLEN_4WORD			 { (0x2 << 9:expr) => {  }; }
macro_rules! WINCONx_ENWIN				 { (1 << 0:expr) => {  }; }
macro_rules! WINCONx_BLEND_MODE_MASK			 { (0xc2:expr) => {  }; }

macro_rules! WINCON0_BPPMODE_MASK			 { (0xf << 2:expr) => {  }; }
pub const WINCON0_BPPMODE_SHIFT: u32 = 2;
macro_rules! WINCON0_BPPMODE_1BPP			 { (0x0 << 2:expr) => {  }; }
macro_rules! WINCON0_BPPMODE_2BPP			 { (0x1 << 2:expr) => {  }; }
macro_rules! WINCON0_BPPMODE_4BPP			 { (0x2 << 2:expr) => {  }; }
macro_rules! WINCON0_BPPMODE_8BPP_PALETTE		 { (0x3 << 2:expr) => {  }; }
macro_rules! WINCON0_BPPMODE_16BPP_565		 { (0x5 << 2:expr) => {  }; }
macro_rules! WINCON0_BPPMODE_16BPP_1555		 { (0x7 << 2:expr) => {  }; }
macro_rules! WINCON0_BPPMODE_18BPP_666		 { (0x8 << 2:expr) => {  }; }
macro_rules! WINCON0_BPPMODE_24BPP_888		 { (0xb << 2:expr) => {  }; }

macro_rules! WINCON1_LOCALSEL_CAMIF			 { (1 << 23:expr) => {  }; }
macro_rules! WINCON1_ALPHA_MUL			 { (1 << 7:expr) => {  }; }
macro_rules! WINCON1_BLD_PIX				 { (1 << 6:expr) => {  }; }
macro_rules! WINCON1_BPPMODE_MASK			 { (0xf << 2:expr) => {  }; }
pub const WINCON1_BPPMODE_SHIFT: u32 = 2;
macro_rules! WINCON1_BPPMODE_1BPP			 { (0x0 << 2:expr) => {  }; }
macro_rules! WINCON1_BPPMODE_2BPP			 { (0x1 << 2:expr) => {  }; }
macro_rules! WINCON1_BPPMODE_4BPP			 { (0x2 << 2:expr) => {  }; }
macro_rules! WINCON1_BPPMODE_8BPP_PALETTE		 { (0x3 << 2:expr) => {  }; }
macro_rules! WINCON1_BPPMODE_8BPP_1232		 { (0x4 << 2:expr) => {  }; }
macro_rules! WINCON1_BPPMODE_16BPP_565		 { (0x5 << 2:expr) => {  }; }
macro_rules! WINCON1_BPPMODE_16BPP_A1555		 { (0x6 << 2:expr) => {  }; }
macro_rules! WINCON1_BPPMODE_16BPP_I1555		 { (0x7 << 2:expr) => {  }; }
macro_rules! WINCON1_BPPMODE_18BPP_666		 { (0x8 << 2:expr) => {  }; }
macro_rules! WINCON1_BPPMODE_18BPP_A1665		 { (0x9 << 2:expr) => {  }; }
macro_rules! WINCON1_BPPMODE_19BPP_A1666		 { (0xa << 2:expr) => {  }; }
macro_rules! WINCON1_BPPMODE_24BPP_888		 { (0xb << 2:expr) => {  }; }
macro_rules! WINCON1_BPPMODE_24BPP_A1887		 { (0xc << 2:expr) => {  }; }
macro_rules! WINCON1_BPPMODE_25BPP_A1888		 { (0xd << 2:expr) => {  }; }
macro_rules! WINCON1_BPPMODE_28BPP_A4888		 { (0xd << 2:expr) => {  }; }
macro_rules! WINCON1_ALPHA_SEL			 { (1 << 1:expr) => {  }; }

/* S5PV210 */
pub const SHADOWCON: u32 = 0x34;
macro_rules! SHADOWCON_WINx_PROTECT { (_win(1 << (10 + (_win))):expr) => { (1 << (10 + (_win))) }; }
/* DMA channels (all windows) */
macro_rules! SHADOWCON_CHx_ENABLE { (_win(1 << (_win)):expr) => { (1 << (_win)) }; }
/* Local input channels (windows 0-2) */
macro_rules! SHADOWCON_CHx_LOCAL_ENABLE { (_win(1 << (5 + (_win))):expr) => { (1 << (5 + (_win))) }; }

/* VIDOSDx */

pub const VIDOSD_BASE: u32 = 0x40;
macro_rules! VIDOSDxA_TOPLEFT_X_E { (_x((((_x) & 0x800) >> 11) << 23):expr) => { ((((_x) & 0x800) >> 11) << 23) }; }
macro_rules! VIDOSDxA_TOPLEFT_X_MASK			 { (0x7ff << 11:expr) => {  }; }
pub const VIDOSDxA_TOPLEFT_X_SHIFT: u32 = 11;
pub const VIDOSDxA_TOPLEFT_X_LIMIT: u32 = 0x7ff;
macro_rules! VIDOSDxA_TOPLEFT_X { (_x(((_x) & 0x7ff) << 11):expr) => { (((_x) & 0x7ff) << 11) }; }

macro_rules! VIDOSDxA_TOPLEFT_Y_E { (_x((((_x) & 0x800) >> 11) << 22):expr) => { ((((_x) & 0x800) >> 11) << 22) }; }
macro_rules! VIDOSDxA_TOPLEFT_Y_MASK			 { (0x7ff << 0:expr) => {  }; }
pub const VIDOSDxA_TOPLEFT_Y_SHIFT: u32 = 0;
pub const VIDOSDxA_TOPLEFT_Y_LIMIT: u32 = 0x7ff;
macro_rules! VIDOSDxA_TOPLEFT_Y { (_x(((_x) & 0x7ff) << 0):expr) => { (((_x) & 0x7ff) << 0) }; }

macro_rules! VIDOSDxB_BOTRIGHT_X_E { (_x((((_x) & 0x800) >> 11) << 23):expr) => { ((((_x) & 0x800) >> 11) << 23) }; }
macro_rules! VIDOSDxB_BOTRIGHT_X_MASK		 { (0x7ff << 11:expr) => {  }; }
pub const VIDOSDxB_BOTRIGHT_X_SHIFT: u32 = 11;
pub const VIDOSDxB_BOTRIGHT_X_LIMIT: u32 = 0x7ff;
macro_rules! VIDOSDxB_BOTRIGHT_X { (_x(((_x) & 0x7ff) << 11):expr) => { (((_x) & 0x7ff) << 11) }; }

macro_rules! VIDOSDxB_BOTRIGHT_Y_E { (_x((((_x) & 0x800) >> 11) << 22):expr) => { ((((_x) & 0x800) >> 11) << 22) }; }
macro_rules! VIDOSDxB_BOTRIGHT_Y_MASK		 { (0x7ff << 0:expr) => {  }; }
pub const VIDOSDxB_BOTRIGHT_Y_SHIFT: u32 = 0;
pub const VIDOSDxB_BOTRIGHT_Y_LIMIT: u32 = 0x7ff;
macro_rules! VIDOSDxB_BOTRIGHT_Y { (_x(((_x) & 0x7ff) << 0):expr) => { (((_x) & 0x7ff) << 0) }; }

/* For VIDOSD[1..4]C */
macro_rules! VIDISD14C_ALPHA0_R { (_x((_x) << 20):expr) => { ((_x) << 20) }; }
macro_rules! VIDISD14C_ALPHA0_G_MASK			 { (0xf << 16:expr) => {  }; }
pub const VIDISD14C_ALPHA0_G_SHIFT: u32 = 16;
pub const VIDISD14C_ALPHA0_G_LIMIT: u32 = 0xf;
macro_rules! VIDISD14C_ALPHA0_G { (_x((_x) << 16):expr) => { ((_x) << 16) }; }
macro_rules! VIDISD14C_ALPHA0_B_MASK			 { (0xf << 12:expr) => {  }; }
pub const VIDISD14C_ALPHA0_B_SHIFT: u32 = 12;
pub const VIDISD14C_ALPHA0_B_LIMIT: u32 = 0xf;
macro_rules! VIDISD14C_ALPHA0_B { (_x((_x) << 12):expr) => { ((_x) << 12) }; }
macro_rules! VIDISD14C_ALPHA1_R_MASK			 { (0xf << 8:expr) => {  }; }
pub const VIDISD14C_ALPHA1_R_SHIFT: u32 = 8;
pub const VIDISD14C_ALPHA1_R_LIMIT: u32 = 0xf;
macro_rules! VIDISD14C_ALPHA1_R { (_x((_x) << 8):expr) => { ((_x) << 8) }; }
macro_rules! VIDISD14C_ALPHA1_G_MASK			 { (0xf << 4:expr) => {  }; }
pub const VIDISD14C_ALPHA1_G_SHIFT: u32 = 4;
pub const VIDISD14C_ALPHA1_G_LIMIT: u32 = 0xf;
macro_rules! VIDISD14C_ALPHA1_G { (_x((_x) << 4):expr) => { ((_x) << 4) }; }
macro_rules! VIDISD14C_ALPHA1_B_MASK			 { (0xf << 0:expr) => {  }; }
pub const VIDISD14C_ALPHA1_B_SHIFT: u32 = 0;
pub const VIDISD14C_ALPHA1_B_LIMIT: u32 = 0xf;
macro_rules! VIDISD14C_ALPHA1_B { (_x((_x) << 0):expr) => { ((_x) << 0) }; }

pub const VIDW_ALPHA: u32 = 0x021c;
macro_rules! VIDW_ALPHA_R { (_x((_x) << 16):expr) => { ((_x) << 16) }; }
macro_rules! VIDW_ALPHA_G { (_x((_x) << 8):expr) => { ((_x) << 8) }; }
macro_rules! VIDW_ALPHA_B { (_x((_x) << 0):expr) => { ((_x) << 0) }; }

/* Video buffer addresses */
macro_rules! VIDW_BUF_START { (_buff(0xA0 + ((_buff) * 8)):expr) => { (0xA0 + ((_buff) * 8)) }; }
macro_rules! VIDW_BUF_START_S { (_buff(0x40A0 + ((_buff) * 8)):expr) => { (0x40A0 + ((_buff) * 8)) }; }
macro_rules! VIDW_BUF_START1 { (_buff(0xA4 + ((_buff) * 8)):expr) => { (0xA4 + ((_buff) * 8)) }; }
macro_rules! VIDW_BUF_END { (_buff(0xD0 + ((_buff) * 8)):expr) => { (0xD0 + ((_buff) * 8)) }; }
macro_rules! VIDW_BUF_END1 { (_buff(0xD4 + ((_buff) * 8)):expr) => { (0xD4 + ((_buff) * 8)) }; }
macro_rules! VIDW_BUF_SIZE { (_buff(0x100 + ((_buff) * 4)):expr) => { (0x100 + ((_buff) * 4)) }; }

macro_rules! VIDW_BUF_SIZE_OFFSET_E { (_x((((_x) & 0x2000) >> 13) << 27):expr) => { ((((_x) & 0x2000) >> 13) << 27) }; }
macro_rules! VIDW_BUF_SIZE_OFFSET_MASK		 { (0x1fff << 13:expr) => {  }; }
pub const VIDW_BUF_SIZE_OFFSET_SHIFT: u32 = 13;
pub const VIDW_BUF_SIZE_OFFSET_LIMIT: u32 = 0x1fff;
macro_rules! VIDW_BUF_SIZE_OFFSET { (_x(((_x) & 0x1fff) << 13):expr) => { (((_x) & 0x1fff) << 13) }; }

macro_rules! VIDW_BUF_SIZE_PAGEWIDTH_E { (_x((((_x) & 0x2000) >> 13) << 26):expr) => { ((((_x) & 0x2000) >> 13) << 26) }; }
macro_rules! VIDW_BUF_SIZE_PAGEWIDTH_MASK		 { (0x1fff << 0:expr) => {  }; }
pub const VIDW_BUF_SIZE_PAGEWIDTH_SHIFT: u32 = 0;
pub const VIDW_BUF_SIZE_PAGEWIDTH_LIMIT: u32 = 0x1fff;
macro_rules! VIDW_BUF_SIZE_PAGEWIDTH { (_x(((_x) & 0x1fff) << 0):expr) => { (((_x) & 0x1fff) << 0) }; }

/* Interrupt controls and status */

pub const VIDINTCON0: u32 = 0x130;
macro_rules! VIDINTCON0_FIFOINTERVAL_MASK		 { (0x3f << 20:expr) => {  }; }
pub const VIDINTCON0_FIFOINTERVAL_SHIFT: u32 = 20;
pub const VIDINTCON0_FIFOINTERVAL_LIMIT: u32 = 0x3f;
macro_rules! VIDINTCON0_FIFOINTERVAL { (_x((_x) << 20):expr) => { ((_x) << 20) }; }

macro_rules! VIDINTCON0_INT_SYSMAINCON		 { (1 << 19:expr) => {  }; }
macro_rules! VIDINTCON0_INT_SYSSUBCON		 { (1 << 18:expr) => {  }; }
macro_rules! VIDINTCON0_INT_I80IFDONE		 { (1 << 17:expr) => {  }; }

macro_rules! VIDINTCON0_FRAMESEL0_MASK		 { (0x3 << 15:expr) => {  }; }
pub const VIDINTCON0_FRAMESEL0_SHIFT: u32 = 15;
macro_rules! VIDINTCON0_FRAMESEL0_BACKPORCH		 { (0x0 << 15:expr) => {  }; }
macro_rules! VIDINTCON0_FRAMESEL0_VSYNC		 { (0x1 << 15:expr) => {  }; }
macro_rules! VIDINTCON0_FRAMESEL0_ACTIVE		 { (0x2 << 15:expr) => {  }; }
macro_rules! VIDINTCON0_FRAMESEL0_FRONTPORCH		 { (0x3 << 15:expr) => {  }; }

macro_rules! VIDINTCON0_FRAMESEL1			 { (1 << 13:expr) => {  }; }
macro_rules! VIDINTCON0_FRAMESEL1_MASK		 { (0x3 << 13:expr) => {  }; }
macro_rules! VIDINTCON0_FRAMESEL1_NONE		 { (0x0 << 13:expr) => {  }; }
macro_rules! VIDINTCON0_FRAMESEL1_BACKPORCH		 { (0x1 << 13:expr) => {  }; }
macro_rules! VIDINTCON0_FRAMESEL1_VSYNC		 { (0x2 << 13:expr) => {  }; }
macro_rules! VIDINTCON0_FRAMESEL1_FRONTPORCH		 { (0x3 << 13:expr) => {  }; }

macro_rules! VIDINTCON0_INT_FRAME			 { (1 << 12:expr) => {  }; }
macro_rules! VIDINTCON0_FIFIOSEL_MASK		 { (0x7f << 5:expr) => {  }; }
pub const VIDINTCON0_FIFIOSEL_SHIFT: u32 = 5;
macro_rules! VIDINTCON0_FIFIOSEL_WINDOW0		 { (0x1 << 5:expr) => {  }; }
macro_rules! VIDINTCON0_FIFIOSEL_WINDOW1		 { (0x2 << 5:expr) => {  }; }
macro_rules! VIDINTCON0_FIFIOSEL_WINDOW2		 { (0x10 << 5:expr) => {  }; }
macro_rules! VIDINTCON0_FIFIOSEL_WINDOW3		 { (0x20 << 5:expr) => {  }; }
macro_rules! VIDINTCON0_FIFIOSEL_WINDOW4		 { (0x40 << 5:expr) => {  }; }

macro_rules! VIDINTCON0_FIFOLEVEL_MASK		 { (0x7 << 2:expr) => {  }; }
pub const VIDINTCON0_FIFOLEVEL_SHIFT: u32 = 2;
macro_rules! VIDINTCON0_FIFOLEVEL_TO25PC		 { (0x0 << 2:expr) => {  }; }
macro_rules! VIDINTCON0_FIFOLEVEL_TO50PC		 { (0x1 << 2:expr) => {  }; }
macro_rules! VIDINTCON0_FIFOLEVEL_TO75PC		 { (0x2 << 2:expr) => {  }; }
macro_rules! VIDINTCON0_FIFOLEVEL_EMPTY		 { (0x3 << 2:expr) => {  }; }
macro_rules! VIDINTCON0_FIFOLEVEL_FULL		 { (0x4 << 2:expr) => {  }; }

macro_rules! VIDINTCON0_INT_FIFO_MASK		 { (0x3 << 0:expr) => {  }; }
pub const VIDINTCON0_INT_FIFO_SHIFT: u32 = 0;
macro_rules! VIDINTCON0_INT_ENABLE			 { (1 << 0:expr) => {  }; }

pub const VIDINTCON1: u32 = 0x134;
macro_rules! VIDINTCON1_INT_I80			 { (1 << 2:expr) => {  }; }
macro_rules! VIDINTCON1_INT_FRAME			 { (1 << 1:expr) => {  }; }
macro_rules! VIDINTCON1_INT_FIFO			 { (1 << 0:expr) => {  }; }

/* Window colour-key control registers */
pub const WKEYCON: u32 = 0x140;

pub const WKEYCON0: u32 = 0x00;
pub const WKEYCON1: u32 = 0x04;

macro_rules! WxKEYCON0_KEYBL_EN			 { (1 << 26:expr) => {  }; }
macro_rules! WxKEYCON0_KEYEN_F			 { (1 << 25:expr) => {  }; }
macro_rules! WxKEYCON0_DIRCON			 { (1 << 24:expr) => {  }; }
macro_rules! WxKEYCON0_COMPKEY_MASK			 { (0xffffff << 0:expr) => {  }; }
pub const WxKEYCON0_COMPKEY_SHIFT: u32 = 0;
pub const WxKEYCON0_COMPKEY_LIMIT: u32 = 0xffffff;
macro_rules! WxKEYCON0_COMPKEY { (_x((_x) << 0):expr) => { ((_x) << 0) }; }
macro_rules! WxKEYCON1_COLVAL_MASK			 { (0xffffff << 0:expr) => {  }; }
pub const WxKEYCON1_COLVAL_SHIFT: u32 = 0;
pub const WxKEYCON1_COLVAL_LIMIT: u32 = 0xffffff;
macro_rules! WxKEYCON1_COLVAL { (_x((_x) << 0):expr) => { ((_x) << 0) }; }

/* Dithering control */
pub const DITHMODE: u32 = 0x170;
macro_rules! DITHMODE_R_POS_MASK			 { (0x3 << 5:expr) => {  }; }
pub const DITHMODE_R_POS_SHIFT: u32 = 5;
macro_rules! DITHMODE_R_POS_8BIT			 { (0x0 << 5:expr) => {  }; }
macro_rules! DITHMODE_R_POS_6BIT			 { (0x1 << 5:expr) => {  }; }
macro_rules! DITHMODE_R_POS_5BIT			 { (0x2 << 5:expr) => {  }; }
macro_rules! DITHMODE_G_POS_MASK			 { (0x3 << 3:expr) => {  }; }
pub const DITHMODE_G_POS_SHIFT: u32 = 3;
macro_rules! DITHMODE_G_POS_8BIT			 { (0x0 << 3:expr) => {  }; }
macro_rules! DITHMODE_G_POS_6BIT			 { (0x1 << 3:expr) => {  }; }
macro_rules! DITHMODE_G_POS_5BIT			 { (0x2 << 3:expr) => {  }; }
macro_rules! DITHMODE_B_POS_MASK			 { (0x3 << 1:expr) => {  }; }
pub const DITHMODE_B_POS_SHIFT: u32 = 1;
macro_rules! DITHMODE_B_POS_8BIT			 { (0x0 << 1:expr) => {  }; }
macro_rules! DITHMODE_B_POS_6BIT			 { (0x1 << 1:expr) => {  }; }
macro_rules! DITHMODE_B_POS_5BIT			 { (0x2 << 1:expr) => {  }; }
macro_rules! DITHMODE_DITH_EN			 { (1 << 0:expr) => {  }; }

/* Window blanking (MAP) */
macro_rules! WINxMAP { (_win(0x180 + ((_win) * 4)):expr) => { (0x180 + ((_win) * 4)) }; }
macro_rules! WINxMAP_MAP				 { (1 << 24:expr) => {  }; }
macro_rules! WINxMAP_MAP_COLOUR_MASK			 { (0xffffff << 0:expr) => {  }; }
pub const WINxMAP_MAP_COLOUR_SHIFT: u32 = 0;
pub const WINxMAP_MAP_COLOUR_LIMIT: u32 = 0xffffff;
macro_rules! WINxMAP_MAP_COLOUR { (_x((_x) << 0):expr) => { ((_x) << 0) }; }

/* Winodw palette control */
pub const WPALCON: u32 = 0x1A0;
macro_rules! WPALCON_PAL_UPDATE			 { (1 << 9:expr) => {  }; }
macro_rules! WPALCON_W4PAL_16BPP_A555		 { (1 << 8:expr) => {  }; }
macro_rules! WPALCON_W3PAL_16BPP_A555		 { (1 << 7:expr) => {  }; }
macro_rules! WPALCON_W2PAL_16BPP_A555		 { (1 << 6:expr) => {  }; }
macro_rules! WPALCON_W1PAL_MASK			 { (0x7 << 3:expr) => {  }; }
pub const WPALCON_W1PAL_SHIFT: u32 = 3;
macro_rules! WPALCON_W1PAL_25BPP_A888		 { (0x0 << 3:expr) => {  }; }
macro_rules! WPALCON_W1PAL_24BPP			 { (0x1 << 3:expr) => {  }; }
macro_rules! WPALCON_W1PAL_19BPP_A666		 { (0x2 << 3:expr) => {  }; }
macro_rules! WPALCON_W1PAL_18BPP_A665		 { (0x3 << 3:expr) => {  }; }
macro_rules! WPALCON_W1PAL_18BPP			 { (0x4 << 3:expr) => {  }; }
macro_rules! WPALCON_W1PAL_16BPP_A555		 { (0x5 << 3:expr) => {  }; }
macro_rules! WPALCON_W1PAL_16BPP_565			 { (0x6 << 3:expr) => {  }; }
macro_rules! WPALCON_W0PAL_MASK			 { (0x7 << 0:expr) => {  }; }
pub const WPALCON_W0PAL_SHIFT: u32 = 0;
macro_rules! WPALCON_W0PAL_25BPP_A888		 { (0x0 << 0:expr) => {  }; }
macro_rules! WPALCON_W0PAL_24BPP			 { (0x1 << 0:expr) => {  }; }
macro_rules! WPALCON_W0PAL_19BPP_A666		 { (0x2 << 0:expr) => {  }; }
macro_rules! WPALCON_W0PAL_18BPP_A665		 { (0x3 << 0:expr) => {  }; }
macro_rules! WPALCON_W0PAL_18BPP			 { (0x4 << 0:expr) => {  }; }
macro_rules! WPALCON_W0PAL_16BPP_A555		 { (0x5 << 0:expr) => {  }; }
macro_rules! WPALCON_W0PAL_16BPP_565			 { (0x6 << 0:expr) => {  }; }

/* Blending equation control */
macro_rules! BLENDEQx { (_win(0x244 + ((_win - 1) * 4)):expr) => { (0x244 + ((_win - 1) * 4)) }; }
pub const BLENDEQ_ZERO: u32 = 0x0;
pub const BLENDEQ_ONE: u32 = 0x1;
pub const BLENDEQ_ALPHA_A: u32 = 0x2;
pub const BLENDEQ_ONE_MINUS_ALPHA_A: u32 = 0x3;
pub const BLENDEQ_ALPHA0: u32 = 0x6;
macro_rules! BLENDEQ_B_FUNC_F { (_x(_x << 6):expr) => { (_x << 6) }; }
macro_rules! BLENDEQ_A_FUNC_F { (_x(_x << 0):expr) => { (_x << 0) }; }
pub const BLENDCON: u32 = 0x260;
macro_rules! BLENDCON_NEW_MASK			 { (1 << 0:expr) => {  }; }
macro_rules! BLENDCON_NEW_8BIT_ALPHA_VALUE		 { (1 << 0:expr) => {  }; }
macro_rules! BLENDCON_NEW_4BIT_ALPHA_VALUE		 { (0 << 0:expr) => {  }; }

/* Display port clock control */
pub const DP_MIE_CLKCON: u32 = 0x27c;
pub const DP_MIE_CLK_DISABLE: u32 = 0x0;
pub const DP_MIE_CLK_DP_ENABLE: u32 = 0x2;
pub const DP_MIE_CLK_MIE_ENABLE: u32 = 0x3;

/* Notes on per-window bpp settings
 *
 * Value	Win0	 Win1	  Win2	   Win3	    Win 4
 * 0000		1(P)	 1(P)	  1(P)	   1(P)	    1(P)
 * 0001		2(P)	 2(P)     2(P)	   2(P)	    2(P)
 * 0010		4(P)	 4(P)     4(P)	   4(P)     -none-
 * 0011		8(P)	 8(P)     -none-   -none-   -none-
 * 0100		-none-	 8(A232)  8(A232)  -none-   -none-
 * 0101		16(565)	 16(565)  16(565)  16(565)   16(565)
 * 0110		-none-	 16(A555) 16(A555) 16(A555)  16(A555)
 * 0111		16(I555) 16(I565) 16(I555) 16(I555)  16(I555)
 * 1000		18(666)	 18(666)  18(666)  18(666)   18(666)
 * 1001		-none-	 18(A665) 18(A665) 18(A665)  16(A665)
 * 1010		-none-	 19(A666) 19(A666) 19(A666)  19(A666)
 * 1011		24(888)	 24(888)  24(888)  24(888)   24(888)
 * 1100		-none-	 24(A887) 24(A887) 24(A887)  24(A887)
 * 1101		-none-	 25(A888) 25(A888) 25(A888)  25(A888)
 * 1110		-none-	 -none-	  -none-   -none-    -none-
 * 1111		-none-	 -none-   -none-   -none-    -none-
*/

macro_rules! WIN_RGB_ORDER { (_win(0x2020 + ((_win) * 4)):expr) => { (0x2020 + ((_win) * 4)) }; }
macro_rules! WIN_RGB_ORDER_FORWARD			 { (0 << 11:expr) => {  }; }
macro_rules! WIN_RGB_ORDER_REVERSE			 { (1 << 11:expr) => {  }; }

/* FIMD Version 8 register offset definitions */
pub const FIMD_V8_VIDTCON0: u32 = 0x20010;
pub const FIMD_V8_VIDTCON1: u32 = 0x20014;
pub const FIMD_V8_VIDTCON2: u32 = 0x20018;
pub const FIMD_V8_VIDTCON3: u32 = 0x2001C;
pub const FIMD_V8_VIDCON1: u32 = 0x20004;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
