/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (c) 2014 Samsung Electronics Co., Ltd.
 * Author: Rahul Sharma <rahul.sharma@samsung.com>
 *
 * Common Clock Framework support for Exynos5260 SoC.
 */


/*
*Registers for CMU_AUD
*/
pub const MUX_SEL_AUD: u32 = 0x0200;
pub const MUX_ENABLE_AUD: u32 = 0x0300;
pub const MUX_STAT_AUD: u32 = 0x0400;
pub const MUX_IGNORE_AUD: u32 = 0x0500;
pub const DIV_AUD0: u32 = 0x0600;
pub const DIV_AUD1: u32 = 0x0604;
pub const DIV_STAT_AUD0: u32 = 0x0700;
pub const DIV_STAT_AUD1: u32 = 0x0704;
pub const EN_ACLK_AUD: u32 = 0x0800;
pub const EN_PCLK_AUD: u32 = 0x0900;
pub const EN_SCLK_AUD: u32 = 0x0a00;
pub const EN_IP_AUD: u32 = 0x0b00;

/*
*Registers for CMU_DISP
*/
pub const MUX_SEL_DISP0: u32 = 0x0200;
pub const MUX_SEL_DISP1: u32 = 0x0204;
pub const MUX_SEL_DISP2: u32 = 0x0208;
pub const MUX_SEL_DISP3: u32 = 0x020C;
pub const MUX_SEL_DISP4: u32 = 0x0210;
pub const MUX_ENABLE_DISP0: u32 = 0x0300;
pub const MUX_ENABLE_DISP1: u32 = 0x0304;
pub const MUX_ENABLE_DISP2: u32 = 0x0308;
pub const MUX_ENABLE_DISP3: u32 = 0x030c;
pub const MUX_ENABLE_DISP4: u32 = 0x0310;
pub const MUX_STAT_DISP0: u32 = 0x0400;
pub const MUX_STAT_DISP1: u32 = 0x0404;
pub const MUX_STAT_DISP2: u32 = 0x0408;
pub const MUX_STAT_DISP3: u32 = 0x040c;
pub const MUX_STAT_DISP4: u32 = 0x0410;
pub const MUX_IGNORE_DISP0: u32 = 0x0500;
pub const MUX_IGNORE_DISP1: u32 = 0x0504;
pub const MUX_IGNORE_DISP2: u32 = 0x0508;
pub const MUX_IGNORE_DISP3: u32 = 0x050c;
pub const MUX_IGNORE_DISP4: u32 = 0x0510;
pub const DIV_DISP: u32 = 0x0600;
pub const DIV_STAT_DISP: u32 = 0x0700;
pub const EN_ACLK_DISP: u32 = 0x0800;
pub const EN_PCLK_DISP: u32 = 0x0900;
pub const EN_SCLK_DISP0: u32 = 0x0a00;
pub const EN_SCLK_DISP1: u32 = 0x0a04;
pub const EN_IP_DISP: u32 = 0x0b00;
pub const EN_IP_DISP_BUS: u32 = 0x0b04;


/*
*Registers for CMU_EGL
*/
pub const EGL_PLL_LOCK: u32 = 0x0000;
pub const EGL_DPLL_LOCK: u32 = 0x0004;
pub const EGL_PLL_CON0: u32 = 0x0100;
pub const EGL_PLL_CON1: u32 = 0x0104;
pub const EGL_PLL_FREQ_DET: u32 = 0x010c;
pub const EGL_DPLL_CON0: u32 = 0x0110;
pub const EGL_DPLL_CON1: u32 = 0x0114;
pub const EGL_DPLL_FREQ_DET: u32 = 0x011c;
pub const MUX_SEL_EGL: u32 = 0x0200;
pub const MUX_ENABLE_EGL: u32 = 0x0300;
pub const MUX_STAT_EGL: u32 = 0x0400;
pub const DIV_EGL: u32 = 0x0600;
pub const DIV_EGL_PLL_FDET: u32 = 0x0604;
pub const DIV_STAT_EGL: u32 = 0x0700;
pub const DIV_STAT_EGL_PLL_FDET: u32 = 0x0704;
pub const EN_ACLK_EGL: u32 = 0x0800;
pub const EN_PCLK_EGL: u32 = 0x0900;
pub const EN_SCLK_EGL: u32 = 0x0a00;
pub const EN_IP_EGL: u32 = 0x0b00;
pub const CLKOUT_CMU_EGL: u32 = 0x0c00;
pub const CLKOUT_CMU_EGL_DIV_STAT: u32 = 0x0c04;
pub const ARMCLK_STOPCTRL: u32 = 0x1000;
pub const EAGLE_EMA_CTRL: u32 = 0x1008;
pub const EAGLE_EMA_STATUS: u32 = 0x100c;
pub const PWR_CTRL: u32 = 0x1020;
pub const PWR_CTRL2: u32 = 0x1024;
pub const CLKSTOP_CTRL: u32 = 0x1028;
pub const INTR_SPREAD_EN: u32 = 0x1080;
pub const INTR_SPREAD_USE_STANDBYWFI: u32 = 0x1084;
pub const INTR_SPREAD_BLOCKING_DURATION: u32 = 0x1088;
pub const CMU_EGL_SPARE0: u32 = 0x2000;
pub const CMU_EGL_SPARE1: u32 = 0x2004;
pub const CMU_EGL_SPARE2: u32 = 0x2008;
pub const CMU_EGL_SPARE3: u32 = 0x200c;
pub const CMU_EGL_SPARE4: u32 = 0x2010;

/*
*Registers for CMU_FSYS
*/

pub const MUX_SEL_FSYS0: u32 = 0x0200;
pub const MUX_SEL_FSYS1: u32 = 0x0204;
pub const MUX_ENABLE_FSYS0: u32 = 0x0300;
pub const MUX_ENABLE_FSYS1: u32 = 0x0304;
pub const MUX_STAT_FSYS0: u32 = 0x0400;
pub const MUX_STAT_FSYS1: u32 = 0x0404;
pub const MUX_IGNORE_FSYS0: u32 = 0x0500;
pub const MUX_IGNORE_FSYS1: u32 = 0x0504;
pub const EN_ACLK_FSYS: u32 = 0x0800;
pub const EN_ACLK_FSYS_SECURE_RTIC: u32 = 0x0804;
pub const EN_ACLK_FSYS_SECURE_SMMU_RTIC: u32 = 0x0808;
pub const EN_PCLK_FSYS: u32 = 0x0900;
pub const EN_SCLK_FSYS: u32 = 0x0a00;
pub const EN_IP_FSYS: u32 = 0x0b00;
pub const EN_IP_FSYS_SECURE_RTIC: u32 = 0x0b04;
pub const EN_IP_FSYS_SECURE_SMMU_RTIC: u32 = 0x0b08;

/*
*Registers for CMU_G2D
*/

pub const MUX_SEL_G2D: u32 = 0x0200;
pub const MUX_ENABLE_G2D: u32 = 0x0300;
pub const MUX_STAT_G2D: u32 = 0x0400;
pub const DIV_G2D: u32 = 0x0600;
pub const DIV_STAT_G2D: u32 = 0x0700;
pub const EN_ACLK_G2D: u32 = 0x0800;
pub const EN_ACLK_G2D_SECURE_SSS: u32 = 0x0804;
pub const EN_ACLK_G2D_SECURE_SLIM_SSS: u32 = 0x0808;
pub const EN_ACLK_G2D_SECURE_SMMU_SLIM_SSS: u32 = 0x080c;
pub const EN_ACLK_G2D_SECURE_SMMU_SSS: u32 = 0x0810;
pub const EN_ACLK_G2D_SECURE_SMMU_MDMA: u32 = 0x0814;
pub const EN_ACLK_G2D_SECURE_SMMU_G2D: u32 = 0x0818;
pub const EN_PCLK_G2D: u32 = 0x0900;
pub const EN_PCLK_G2D_SECURE_SMMU_SLIM_SSS: u32 = 0x0904;
pub const EN_PCLK_G2D_SECURE_SMMU_SSS: u32 = 0x0908;
pub const EN_PCLK_G2D_SECURE_SMMU_MDMA: u32 = 0x090c;
pub const EN_PCLK_G2D_SECURE_SMMU_G2D: u32 = 0x0910;
pub const EN_IP_G2D: u32 = 0x0b00;
pub const EN_IP_G2D_SECURE_SSS: u32 = 0x0b04;
pub const EN_IP_G2D_SECURE_SLIM_SSS: u32 = 0x0b08;
pub const EN_IP_G2D_SECURE_SMMU_SLIM_SSS: u32 = 0x0b0c;
pub const EN_IP_G2D_SECURE_SMMU_SSS: u32 = 0x0b10;
pub const EN_IP_G2D_SECURE_SMMU_MDMA: u32 = 0x0b14;
pub const EN_IP_G2D_SECURE_SMMU_G2D: u32 = 0x0b18;

/*
*Registers for CMU_G3D
*/

pub const G3D_PLL_LOCK: u32 = 0x0000;
pub const G3D_PLL_CON0: u32 = 0x0100;
pub const G3D_PLL_CON1: u32 = 0x0104;
pub const G3D_PLL_FDET: u32 = 0x010c;
pub const MUX_SEL_G3D: u32 = 0x0200;
pub const MUX_EN_G3D: u32 = 0x0300;
pub const MUX_STAT_G3D: u32 = 0x0400;
pub const MUX_IGNORE_G3D: u32 = 0x0500;
pub const DIV_G3D: u32 = 0x0600;
pub const DIV_G3D_PLL_FDET: u32 = 0x0604;
pub const DIV_STAT_G3D: u32 = 0x0700;
pub const DIV_STAT_G3D_PLL_FDET: u32 = 0x0704;
pub const EN_ACLK_G3D: u32 = 0x0800;
pub const EN_PCLK_G3D: u32 = 0x0900;
pub const EN_SCLK_G3D: u32 = 0x0a00;
pub const EN_IP_G3D: u32 = 0x0b00;
pub const CLKOUT_CMU_G3D: u32 = 0x0c00;
pub const CLKOUT_CMU_G3D_DIV_STAT: u32 = 0x0c04;
pub const G3DCLK_STOPCTRL: u32 = 0x1000;
pub const G3D_EMA_CTRL: u32 = 0x1008;
pub const G3D_EMA_STATUS: u32 = 0x100c;

/*
*Registers for CMU_GSCL
*/

pub const MUX_SEL_GSCL: u32 = 0x0200;
pub const MUX_EN_GSCL: u32 = 0x0300;
pub const MUX_STAT_GSCL: u32 = 0x0400;
pub const MUX_IGNORE_GSCL: u32 = 0x0500;
pub const DIV_GSCL: u32 = 0x0600;
pub const DIV_STAT_GSCL: u32 = 0x0700;
pub const EN_ACLK_GSCL: u32 = 0x0800;
pub const EN_ACLK_GSCL_FIMC: u32 = 0x0804;
pub const EN_ACLK_GSCL_SECURE_SMMU_GSCL0: u32 = 0x0808;
pub const EN_ACLK_GSCL_SECURE_SMMU_GSCL1: u32 = 0x080c;
pub const EN_ACLK_GSCL_SECURE_SMMU_MSCL0: u32 = 0x0810;
pub const EN_ACLK_GSCL_SECURE_SMMU_MSCL1: u32 = 0x0814;
pub const EN_PCLK_GSCL: u32 = 0x0900;
pub const EN_PCLK_GSCL_FIMC: u32 = 0x0904;
pub const EN_PCLK_GSCL_SECURE_SMMU_GSCL0: u32 = 0x0908;
pub const EN_PCLK_GSCL_SECURE_SMMU_GSCL1: u32 = 0x090c;
pub const EN_PCLK_GSCL_SECURE_SMMU_MSCL0: u32 = 0x0910;
pub const EN_PCLK_GSCL_SECURE_SMMU_MSCL1: u32 = 0x0914;
pub const EN_SCLK_GSCL: u32 = 0x0a00;
pub const EN_SCLK_GSCL_FIMC: u32 = 0x0a04;
pub const EN_IP_GSCL: u32 = 0x0b00;
pub const EN_IP_GSCL_FIMC: u32 = 0x0b04;
pub const EN_IP_GSCL_SECURE_SMMU_GSCL0: u32 = 0x0b08;
pub const EN_IP_GSCL_SECURE_SMMU_GSCL1: u32 = 0x0b0c;
pub const EN_IP_GSCL_SECURE_SMMU_MSCL0: u32 = 0x0b10;
pub const EN_IP_GSCL_SECURE_SMMU_MSCL1: u32 = 0x0b14;

/*
*Registers for CMU_ISP
*/
pub const MUX_SEL_ISP0: u32 = 0x0200;
pub const MUX_SEL_ISP1: u32 = 0x0204;
pub const MUX_ENABLE_ISP0: u32 = 0x0300;
pub const MUX_ENABLE_ISP1: u32 = 0x0304;
pub const MUX_STAT_ISP0: u32 = 0x0400;
pub const MUX_STAT_ISP1: u32 = 0x0404;
pub const MUX_IGNORE_ISP0: u32 = 0x0500;
pub const MUX_IGNORE_ISP1: u32 = 0x0504;
pub const DIV_ISP: u32 = 0x0600;
pub const DIV_STAT_ISP: u32 = 0x0700;
pub const EN_ACLK_ISP0: u32 = 0x0800;
pub const EN_ACLK_ISP1: u32 = 0x0804;
pub const EN_PCLK_ISP0: u32 = 0x0900;
pub const EN_PCLK_ISP1: u32 = 0x0904;
pub const EN_SCLK_ISP: u32 = 0x0a00;
pub const EN_IP_ISP0: u32 = 0x0b00;
pub const EN_IP_ISP1: u32 = 0x0b04;

/*
*Registers for CMU_KFC
*/
pub const KFC_PLL_LOCK: u32 = 0x0000;
pub const KFC_PLL_CON0: u32 = 0x0100;
pub const KFC_PLL_CON1: u32 = 0x0104;
pub const KFC_PLL_FDET: u32 = 0x010c;
pub const MUX_SEL_KFC0: u32 = 0x0200;
pub const MUX_SEL_KFC2: u32 = 0x0208;
pub const MUX_ENABLE_KFC0: u32 = 0x0300;
pub const MUX_ENABLE_KFC2: u32 = 0x0308;
pub const MUX_STAT_KFC0: u32 = 0x0400;
pub const MUX_STAT_KFC2: u32 = 0x0408;
pub const DIV_KFC: u32 = 0x0600;
pub const DIV_KFC_PLL_FDET: u32 = 0x0604;
pub const DIV_STAT_KFC: u32 = 0x0700;
pub const DIV_STAT_KFC_PLL_FDET: u32 = 0x0704;
pub const EN_ACLK_KFC: u32 = 0x0800;
pub const EN_PCLK_KFC: u32 = 0x0900;
pub const EN_SCLK_KFC: u32 = 0x0a00;
pub const EN_IP_KFC: u32 = 0x0b00;
pub const CLKOUT_CMU_KFC: u32 = 0x0c00;
pub const CLKOUT_CMU_KFC_DIV_STAT: u32 = 0x0c04;
pub const ARMCLK_STOPCTRL_KFC: u32 = 0x1000;
pub const ARM_EMA_CTRL: u32 = 0x1008;
pub const ARM_EMA_STATUS: u32 = 0x100c;
pub const PWR_CTRL_KFC: u32 = 0x1020;
pub const PWR_CTRL2_KFC: u32 = 0x1024;
pub const CLKSTOP_CTRL_KFC: u32 = 0x1028;
pub const INTR_SPREAD_ENABLE_KFC: u32 = 0x1080;
pub const INTR_SPREAD_USE_STANDBYWFI_KFC: u32 = 0x1084;
pub const INTR_SPREAD_BLOCKING_DURATION_KFC: u32 = 0x1088;
pub const CMU_KFC_SPARE0: u32 = 0x2000;
pub const CMU_KFC_SPARE1: u32 = 0x2004;
pub const CMU_KFC_SPARE2: u32 = 0x2008;
pub const CMU_KFC_SPARE3: u32 = 0x200c;
pub const CMU_KFC_SPARE4: u32 = 0x2010;

/*
*Registers for CMU_MFC
*/
pub const MUX_SEL_MFC: u32 = 0x0200;
pub const MUX_ENABLE_MFC: u32 = 0x0300;
pub const MUX_STAT_MFC: u32 = 0x0400;
pub const DIV_MFC: u32 = 0x0600;
pub const DIV_STAT_MFC: u32 = 0x0700;
pub const EN_ACLK_MFC: u32 = 0x0800;
pub const EN_ACLK_SECURE_SMMU2_MFC: u32 = 0x0804;
pub const EN_PCLK_MFC: u32 = 0x0900;
pub const EN_PCLK_SECURE_SMMU2_MFC: u32 = 0x0904;
pub const EN_IP_MFC: u32 = 0x0b00;
pub const EN_IP_MFC_SECURE_SMMU2_MFC: u32 = 0x0b04;

/*
*Registers for CMU_MIF
*/
pub const MEM_PLL_LOCK: u32 = 0x0000;
pub const BUS_PLL_LOCK: u32 = 0x0004;
pub const MEDIA_PLL_LOCK: u32 = 0x0008;
pub const MEM_PLL_CON0: u32 = 0x0100;
pub const MEM_PLL_CON1: u32 = 0x0104;
pub const MEM_PLL_FDET: u32 = 0x010c;
pub const BUS_PLL_CON0: u32 = 0x0110;
pub const BUS_PLL_CON1: u32 = 0x0114;
pub const BUS_PLL_FDET: u32 = 0x011c;
pub const MEDIA_PLL_CON0: u32 = 0x0120;
pub const MEDIA_PLL_CON1: u32 = 0x0124;
pub const MEDIA_PLL_FDET: u32 = 0x012c;
pub const MUX_SEL_MIF: u32 = 0x0200;
pub const MUX_ENABLE_MIF: u32 = 0x0300;
pub const MUX_STAT_MIF: u32 = 0x0400;
pub const MUX_IGNORE_MIF: u32 = 0x0500;
pub const DIV_MIF: u32 = 0x0600;
pub const DIV_MIF_PLL_FDET: u32 = 0x0604;
pub const DIV_STAT_MIF: u32 = 0x0700;
pub const DIV_STAT_MIF_PLL_FDET: u32 = 0x0704;
pub const EN_ACLK_MIF: u32 = 0x0800;
pub const EN_ACLK_MIF_SECURE_DREX1_TZ: u32 = 0x0804;
pub const EN_ACLK_MIF_SECURE_DREX0_TZ: u32 = 0x0808;
pub const EN_ACLK_MIF_SECURE_INTMEM: u32 = 0x080c;
pub const EN_PCLK_MIF: u32 = 0x0900;
pub const EN_PCLK_MIF_SECURE_MONOCNT: u32 = 0x0904;
pub const EN_PCLK_MIF_SECURE_RTC_APBIF: u32 = 0x0908;
pub const EN_PCLK_MIF_SECURE_DREX1_TZ: u32 = 0x090c;
pub const EN_PCLK_MIF_SECURE_DREX0_TZ: u32 = 0x0910;
pub const EN_SCLK_MIF: u32 = 0x0a00;
pub const EN_IP_MIF: u32 = 0x0b00;
pub const EN_IP_MIF_SECURE_MONOCNT: u32 = 0x0b04;
pub const EN_IP_MIF_SECURE_RTC_APBIF: u32 = 0x0b08;
pub const EN_IP_MIF_SECURE_DREX1_TZ: u32 = 0x0b0c;
pub const EN_IP_MIF_SECURE_DREX0_TZ: u32 = 0x0b10;
pub const EN_IP_MIF_SECURE_INTEMEM: u32 = 0x0b14;
pub const CLKOUT_CMU_MIF_DIV_STAT: u32 = 0x0c04;
pub const DREX_FREQ_CTRL: u32 = 0x1000;
pub const PAUSE: u32 = 0x1004;
pub const DDRPHY_LOCK_CTRL: u32 = 0x1008;
pub const CLKOUT_CMU_MIF: u32 = 0xcb00;

/*
*Registers for CMU_PERI
*/
pub const MUX_SEL_PERI: u32 = 0x0200;
pub const MUX_SEL_PERI1: u32 = 0x0204;
pub const MUX_ENABLE_PERI: u32 = 0x0300;
pub const MUX_ENABLE_PERI1: u32 = 0x0304;
pub const MUX_STAT_PERI: u32 = 0x0400;
pub const MUX_STAT_PERI1: u32 = 0x0404;
pub const MUX_IGNORE_PERI: u32 = 0x0500;
pub const MUX_IGNORE_PERI1: u32 = 0x0504;
pub const DIV_PERI: u32 = 0x0600;
pub const DIV_STAT_PERI: u32 = 0x0700;
pub const EN_PCLK_PERI0: u32 = 0x0800;
pub const EN_PCLK_PERI1: u32 = 0x0804;
pub const EN_PCLK_PERI2: u32 = 0x0808;
pub const EN_PCLK_PERI3: u32 = 0x080c;
pub const EN_PCLK_PERI_SECURE_CHIPID: u32 = 0x0810;
pub const EN_PCLK_PERI_SECURE_PROVKEY0: u32 = 0x0814;
pub const EN_PCLK_PERI_SECURE_PROVKEY1: u32 = 0x0818;
pub const EN_PCLK_PERI_SECURE_SECKEY: u32 = 0x081c;
pub const EN_PCLK_PERI_SECURE_ANTIRBKCNT: u32 = 0x0820;
pub const EN_PCLK_PERI_SECURE_TOP_RTC: u32 = 0x0824;
pub const EN_PCLK_PERI_SECURE_TZPC: u32 = 0x0828;
pub const EN_SCLK_PERI: u32 = 0x0a00;
pub const EN_SCLK_PERI_SECURE_TOP_RTC: u32 = 0x0a04;
pub const EN_IP_PERI0: u32 = 0x0b00;
pub const EN_IP_PERI1: u32 = 0x0b04;
pub const EN_IP_PERI2: u32 = 0x0b08;
pub const EN_IP_PERI_SECURE_CHIPID: u32 = 0x0b0c;
pub const EN_IP_PERI_SECURE_PROVKEY0: u32 = 0x0b10;
pub const EN_IP_PERI_SECURE_PROVKEY1: u32 = 0x0b14;
pub const EN_IP_PERI_SECURE_SECKEY: u32 = 0x0b18;
pub const EN_IP_PERI_SECURE_ANTIRBKCNT: u32 = 0x0b1c;
pub const EN_IP_PERI_SECURE_TOP_RTC: u32 = 0x0b20;
pub const EN_IP_PERI_SECURE_TZPC: u32 = 0x0b24;

/*
*Registers for CMU_TOP
*/
pub const DISP_PLL_LOCK: u32 = 0x0000;
pub const AUD_PLL_LOCK: u32 = 0x0004;
pub const DISP_PLL_CON0: u32 = 0x0100;
pub const DISP_PLL_CON1: u32 = 0x0104;
pub const DISP_PLL_FDET: u32 = 0x0108;
pub const AUD_PLL_CON0: u32 = 0x0110;
pub const AUD_PLL_CON1: u32 = 0x0114;
pub const AUD_PLL_CON2: u32 = 0x0118;
pub const AUD_PLL_FDET: u32 = 0x011c;
pub const MUX_SEL_TOP_PLL0: u32 = 0x0200;
pub const MUX_SEL_TOP_MFC: u32 = 0x0204;
pub const MUX_SEL_TOP_G2D: u32 = 0x0208;
pub const MUX_SEL_TOP_GSCL: u32 = 0x020c;
pub const MUX_SEL_TOP_ISP10: u32 = 0x0214;
pub const MUX_SEL_TOP_ISP11: u32 = 0x0218;
pub const MUX_SEL_TOP_DISP0: u32 = 0x021c;
pub const MUX_SEL_TOP_DISP1: u32 = 0x0220;
pub const MUX_SEL_TOP_BUS: u32 = 0x0224;
pub const MUX_SEL_TOP_PERI0: u32 = 0x0228;
pub const MUX_SEL_TOP_PERI1: u32 = 0x022c;
pub const MUX_SEL_TOP_FSYS: u32 = 0x0230;
pub const MUX_ENABLE_TOP_PLL0: u32 = 0x0300;
pub const MUX_ENABLE_TOP_MFC: u32 = 0x0304;
pub const MUX_ENABLE_TOP_G2D: u32 = 0x0308;
pub const MUX_ENABLE_TOP_GSCL: u32 = 0x030c;
pub const MUX_ENABLE_TOP_ISP10: u32 = 0x0314;
pub const MUX_ENABLE_TOP_ISP11: u32 = 0x0318;
pub const MUX_ENABLE_TOP_DISP0: u32 = 0x031c;
pub const MUX_ENABLE_TOP_DISP1: u32 = 0x0320;
pub const MUX_ENABLE_TOP_BUS: u32 = 0x0324;
pub const MUX_ENABLE_TOP_PERI0: u32 = 0x0328;
pub const MUX_ENABLE_TOP_PERI1: u32 = 0x032c;
pub const MUX_ENABLE_TOP_FSYS: u32 = 0x0330;
pub const MUX_STAT_TOP_PLL0: u32 = 0x0400;
pub const MUX_STAT_TOP_MFC: u32 = 0x0404;
pub const MUX_STAT_TOP_G2D: u32 = 0x0408;
pub const MUX_STAT_TOP_GSCL: u32 = 0x040c;
pub const MUX_STAT_TOP_ISP10: u32 = 0x0414;
pub const MUX_STAT_TOP_ISP11: u32 = 0x0418;
pub const MUX_STAT_TOP_DISP0: u32 = 0x041c;
pub const MUX_STAT_TOP_DISP1: u32 = 0x0420;
pub const MUX_STAT_TOP_BUS: u32 = 0x0424;
pub const MUX_STAT_TOP_PERI0: u32 = 0x0428;
pub const MUX_STAT_TOP_PERI1: u32 = 0x042c;
pub const MUX_STAT_TOP_FSYS: u32 = 0x0430;
pub const MUX_IGNORE_TOP_PLL0: u32 = 0x0500;
pub const MUX_IGNORE_TOP_MFC: u32 = 0x0504;
pub const MUX_IGNORE_TOP_G2D: u32 = 0x0508;
pub const MUX_IGNORE_TOP_GSCL: u32 = 0x050c;
pub const MUX_IGNORE_TOP_ISP10: u32 = 0x0514;
pub const MUX_IGNORE_TOP_ISP11: u32 = 0x0518;
pub const MUX_IGNORE_TOP_DISP0: u32 = 0x051c;
pub const MUX_IGNORE_TOP_DISP1: u32 = 0x0520;
pub const MUX_IGNORE_TOP_BUS: u32 = 0x0524;
pub const MUX_IGNORE_TOP_PERI0: u32 = 0x0528;
pub const MUX_IGNORE_TOP_PERI1: u32 = 0x052c;
pub const MUX_IGNORE_TOP_FSYS: u32 = 0x0530;
pub const DIV_TOP_G2D_MFC: u32 = 0x0600;
pub const DIV_TOP_GSCL_ISP0: u32 = 0x0604;
pub const DIV_TOP_ISP10: u32 = 0x0608;
pub const DIV_TOP_ISP11: u32 = 0x060c;
pub const DIV_TOP_DISP: u32 = 0x0610;
pub const DIV_TOP_BUS: u32 = 0x0614;
pub const DIV_TOP_PERI0: u32 = 0x0618;
pub const DIV_TOP_PERI1: u32 = 0x061c;
pub const DIV_TOP_PERI2: u32 = 0x0620;
pub const DIV_TOP_FSYS0: u32 = 0x0624;
pub const DIV_TOP_FSYS1: u32 = 0x0628;
pub const DIV_TOP_HPM: u32 = 0x062c;
pub const DIV_TOP_PLL_FDET: u32 = 0x0630;
pub const DIV_STAT_TOP_G2D_MFC: u32 = 0x0700;
pub const DIV_STAT_TOP_GSCL_ISP0: u32 = 0x0704;
pub const DIV_STAT_TOP_ISP10: u32 = 0x0708;
pub const DIV_STAT_TOP_ISP11: u32 = 0x070c;
pub const DIV_STAT_TOP_DISP: u32 = 0x0710;
pub const DIV_STAT_TOP_BUS: u32 = 0x0714;
pub const DIV_STAT_TOP_PERI0: u32 = 0x0718;
pub const DIV_STAT_TOP_PERI1: u32 = 0x071c;
pub const DIV_STAT_TOP_PERI2: u32 = 0x0720;
pub const DIV_STAT_TOP_FSYS0: u32 = 0x0724;
pub const DIV_STAT_TOP_FSYS1: u32 = 0x0728;
pub const DIV_STAT_TOP_HPM: u32 = 0x072c;
pub const DIV_STAT_TOP_PLL_FDET: u32 = 0x0730;
pub const EN_ACLK_TOP: u32 = 0x0800;
pub const EN_SCLK_TOP: u32 = 0x0a00;
pub const EN_IP_TOP: u32 = 0x0b00;
pub const CLKOUT_CMU_TOP: u32 = 0x0c00;
pub const CLKOUT_CMU_TOP_DIV_STAT: u32 = 0x0c04;



// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
