/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (c) 2013 Samsung Electronics Co., Ltd.
 * Author: Andrzej Hajda <a.hajda@samsung.com>
 *
 * Device Tree binding constants for Exynos5250 clock controller.
 */

/* core clocks */
pub const CLK_FIN_PLL: u32 = 1;
pub const CLK_FOUT_APLL: u32 = 2;
pub const CLK_FOUT_MPLL: u32 = 3;
pub const CLK_FOUT_BPLL: u32 = 4;
pub const CLK_FOUT_GPLL: u32 = 5;
pub const CLK_FOUT_CPLL: u32 = 6;
pub const CLK_FOUT_EPLL: u32 = 7;
pub const CLK_FOUT_VPLL: u32 = 8;
pub const CLK_ARM_CLK: u32 = 9;
pub const CLK_DIV_ARM2: u32 = 10;

/* gate for special clocks (sclk) */
pub const CLK_SCLK_CAM_BAYER: u32 = 128;
pub const CLK_SCLK_CAM0: u32 = 129;
pub const CLK_SCLK_CAM1: u32 = 130;
pub const CLK_SCLK_GSCL_WA: u32 = 131;
pub const CLK_SCLK_GSCL_WB: u32 = 132;
pub const CLK_SCLK_FIMD1: u32 = 133;
pub const CLK_SCLK_MIPI1: u32 = 134;
pub const CLK_SCLK_DP: u32 = 135;
pub const CLK_SCLK_HDMI: u32 = 136;
pub const CLK_SCLK_PIXEL: u32 = 137;
pub const CLK_SCLK_AUDIO0: u32 = 138;
pub const CLK_SCLK_MMC0: u32 = 139;
pub const CLK_SCLK_MMC1: u32 = 140;
pub const CLK_SCLK_MMC2: u32 = 141;
pub const CLK_SCLK_MMC3: u32 = 142;
pub const CLK_SCLK_SATA: u32 = 143;
pub const CLK_SCLK_USB3: u32 = 144;
pub const CLK_SCLK_JPEG: u32 = 145;
pub const CLK_SCLK_UART0: u32 = 146;
pub const CLK_SCLK_UART1: u32 = 147;
pub const CLK_SCLK_UART2: u32 = 148;
pub const CLK_SCLK_UART3: u32 = 149;
pub const CLK_SCLK_PWM: u32 = 150;
pub const CLK_SCLK_AUDIO1: u32 = 151;
pub const CLK_SCLK_AUDIO2: u32 = 152;
pub const CLK_SCLK_SPDIF: u32 = 153;
pub const CLK_SCLK_SPI0: u32 = 154;
pub const CLK_SCLK_SPI1: u32 = 155;
pub const CLK_SCLK_SPI2: u32 = 156;
pub const CLK_DIV_I2S1: u32 = 157;
pub const CLK_DIV_I2S2: u32 = 158;
pub const CLK_SCLK_HDMIPHY: u32 = 159;
pub const CLK_DIV_PCM0: u32 = 160;

/* gate clocks */
pub const CLK_GSCL0: u32 = 256;
pub const CLK_GSCL1: u32 = 257;
pub const CLK_GSCL2: u32 = 258;
pub const CLK_GSCL3: u32 = 259;
pub const CLK_GSCL_WA: u32 = 260;
pub const CLK_GSCL_WB: u32 = 261;
pub const CLK_SMMU_GSCL0: u32 = 262;
pub const CLK_SMMU_GSCL1: u32 = 263;
pub const CLK_SMMU_GSCL2: u32 = 264;
pub const CLK_SMMU_GSCL3: u32 = 265;
pub const CLK_MFC: u32 = 266;
pub const CLK_SMMU_MFCL: u32 = 267;
pub const CLK_SMMU_MFCR: u32 = 268;
pub const CLK_ROTATOR: u32 = 269;
pub const CLK_JPEG: u32 = 270;
pub const CLK_MDMA1: u32 = 271;
pub const CLK_SMMU_ROTATOR: u32 = 272;
pub const CLK_SMMU_JPEG: u32 = 273;
pub const CLK_SMMU_MDMA1: u32 = 274;
pub const CLK_PDMA0: u32 = 275;
pub const CLK_PDMA1: u32 = 276;
pub const CLK_SATA: u32 = 277;
pub const CLK_USBOTG: u32 = 278;
pub const CLK_MIPI_HSI: u32 = 279;
pub const CLK_SDMMC0: u32 = 280;
pub const CLK_SDMMC1: u32 = 281;
pub const CLK_SDMMC2: u32 = 282;
pub const CLK_SDMMC3: u32 = 283;
pub const CLK_SROMC: u32 = 284;
pub const CLK_USB2: u32 = 285;
pub const CLK_USB3: u32 = 286;
pub const CLK_SATA_PHYCTRL: u32 = 287;
pub const CLK_SATA_PHYI2C: u32 = 288;
pub const CLK_UART0: u32 = 289;
pub const CLK_UART1: u32 = 290;
pub const CLK_UART2: u32 = 291;
pub const CLK_UART3: u32 = 292;
pub const CLK_UART4: u32 = 293;
pub const CLK_I2C0: u32 = 294;
pub const CLK_I2C1: u32 = 295;
pub const CLK_I2C2: u32 = 296;
pub const CLK_I2C3: u32 = 297;
pub const CLK_I2C4: u32 = 298;
pub const CLK_I2C5: u32 = 299;
pub const CLK_I2C6: u32 = 300;
pub const CLK_I2C7: u32 = 301;
pub const CLK_I2C_HDMI: u32 = 302;
pub const CLK_ADC: u32 = 303;
pub const CLK_SPI0: u32 = 304;
pub const CLK_SPI1: u32 = 305;
pub const CLK_SPI2: u32 = 306;
pub const CLK_I2S1: u32 = 307;
pub const CLK_I2S2: u32 = 308;
pub const CLK_PCM1: u32 = 309;
pub const CLK_PCM2: u32 = 310;
pub const CLK_PWM: u32 = 311;
pub const CLK_SPDIF: u32 = 312;
pub const CLK_AC97: u32 = 313;
pub const CLK_HSI2C0: u32 = 314;
pub const CLK_HSI2C1: u32 = 315;
pub const CLK_HSI2C2: u32 = 316;
pub const CLK_HSI2C3: u32 = 317;
pub const CLK_CHIPID: u32 = 318;
pub const CLK_SYSREG: u32 = 319;
pub const CLK_PMU: u32 = 320;
pub const CLK_CMU_TOP: u32 = 321;
pub const CLK_CMU_CORE: u32 = 322;
pub const CLK_CMU_MEM: u32 = 323;
pub const CLK_TZPC0: u32 = 324;
pub const CLK_TZPC1: u32 = 325;
pub const CLK_TZPC2: u32 = 326;
pub const CLK_TZPC3: u32 = 327;
pub const CLK_TZPC4: u32 = 328;
pub const CLK_TZPC5: u32 = 329;
pub const CLK_TZPC6: u32 = 330;
pub const CLK_TZPC7: u32 = 331;
pub const CLK_TZPC8: u32 = 332;
pub const CLK_TZPC9: u32 = 333;
pub const CLK_HDMI_CEC: u32 = 334;
pub const CLK_MCT: u32 = 335;
pub const CLK_WDT: u32 = 336;
pub const CLK_RTC: u32 = 337;
pub const CLK_TMU: u32 = 338;
pub const CLK_FIMD1: u32 = 339;
pub const CLK_MIE1: u32 = 340;
pub const CLK_DSIM0: u32 = 341;
pub const CLK_DP: u32 = 342;
pub const CLK_MIXER: u32 = 343;
pub const CLK_HDMI: u32 = 344;
pub const CLK_G2D: u32 = 345;
pub const CLK_MDMA0: u32 = 346;
pub const CLK_SMMU_MDMA0: u32 = 347;
pub const CLK_SSS: u32 = 348;
pub const CLK_G3D: u32 = 349;
pub const CLK_SMMU_TV: u32 = 350;
pub const CLK_SMMU_FIMD1: u32 = 351;
pub const CLK_SMMU_2D: u32 = 352;
pub const CLK_SMMU_FIMC_ISP: u32 = 353;
pub const CLK_SMMU_FIMC_DRC: u32 = 354;
pub const CLK_SMMU_FIMC_SCC: u32 = 355;
pub const CLK_SMMU_FIMC_SCP: u32 = 356;
pub const CLK_SMMU_FIMC_FD: u32 = 357;
pub const CLK_SMMU_FIMC_MCU: u32 = 358;
pub const CLK_SMMU_FIMC_ODC: u32 = 359;
pub const CLK_SMMU_FIMC_DIS0: u32 = 360;
pub const CLK_SMMU_FIMC_DIS1: u32 = 361;
pub const CLK_SMMU_FIMC_3DNR: u32 = 362;
pub const CLK_SMMU_FIMC_LITE0: u32 = 363;
pub const CLK_SMMU_FIMC_LITE1: u32 = 364;
pub const CLK_CAMIF_TOP: u32 = 365;

/* mux clocks */
pub const CLK_MOUT_HDMI: u32 = 1024;
pub const CLK_MOUT_GPLL: u32 = 1025;
pub const CLK_MOUT_ACLK200_DISP1_SUB: u32 = 1026;
pub const CLK_MOUT_ACLK300_DISP1_SUB: u32 = 1027;
pub const CLK_MOUT_APLL: u32 = 1028;
pub const CLK_MOUT_MPLL: u32 = 1029;
pub const CLK_MOUT_VPLLSRC: u32 = 1030;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
