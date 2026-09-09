/* SPDX-License-Identifier: GPL-2.0 */
/*
 * This header provides constants for binding nvidia,tegra210-car.
 *
 * The first 224 clocks are numbered to match the bits in the CAR's CLK_OUT_ENB
 * registers. These IDs often match those in the CAR's RST_DEVICES registers,
 * but not in all cases. Some bits in CLK_OUT_ENB affect multiple clocks. In
 * this case, those clocks are assigned IDs above 224 in order to highlight
 * this issue. Implementations that interpret these clock IDs as bit values
 * within the CLK_OUT_ENB or RST_DEVICES registers should be careful to
 * explicitly handle these special cases.
 *
 * The balance of the clocks controlled by the CAR are assigned IDs of 224 and
 * above.
 */

// Translated from the C header; preprocessor header guards are not applicable in Rust.

/* 0 */
/* 1 */
/* 2 */
pub const TEGRA210_CLK_ISPB: u32 = 3;
pub const TEGRA210_CLK_RTC: u32 = 4;
pub const TEGRA210_CLK_TIMER: u32 = 5;
pub const TEGRA210_CLK_UARTA: u32 = 6;
/* 7 (register bit affects uartb and vfir) */
pub const TEGRA210_CLK_GPIO: u32 = 8;
pub const TEGRA210_CLK_SDMMC2: u32 = 9;
/* 10 (register bit affects spdif_in and spdif_out) */
pub const TEGRA210_CLK_I2S1: u32 = 11;
pub const TEGRA210_CLK_I2C1: u32 = 12;
/* 13 */
pub const TEGRA210_CLK_SDMMC1: u32 = 14;
pub const TEGRA210_CLK_SDMMC4: u32 = 15;
/* 16 */
pub const TEGRA210_CLK_PWM: u32 = 17;
pub const TEGRA210_CLK_I2S2: u32 = 18;
/* 19 */
/* 20 (register bit affects vi and vi_sensor) */
/* 21 */
pub const TEGRA210_CLK_USBD: u32 = 22;
pub const TEGRA210_CLK_ISPA: u32 = 23;
/* 24 */
/* 25 */
pub const TEGRA210_CLK_DISP2: u32 = 26;
pub const TEGRA210_CLK_DISP1: u32 = 27;
pub const TEGRA210_CLK_HOST1X: u32 = 28;
/* 29 */
pub const TEGRA210_CLK_I2S0: u32 = 30;
/* 31 */

pub const TEGRA210_CLK_MC: u32 = 32;
pub const TEGRA210_CLK_AHBDMA: u32 = 33;
pub const TEGRA210_CLK_APBDMA: u32 = 34;
/* 35 */
/* 36 */
/* 37 */
pub const TEGRA210_CLK_PMC: u32 = 38;
/* 39 (register bit affects fuse and fuse_burn) */
pub const TEGRA210_CLK_KFUSE: u32 = 40;
pub const TEGRA210_CLK_SBC1: u32 = 41;
/* 42 */
/* 43 */
pub const TEGRA210_CLK_SBC2: u32 = 44;
/* 45 */
pub const TEGRA210_CLK_SBC3: u32 = 46;
pub const TEGRA210_CLK_I2C5: u32 = 47;
pub const TEGRA210_CLK_DSIA: u32 = 48;
/* 49 */
/* 50 */
/* 51 */
pub const TEGRA210_CLK_CSI: u32 = 52;
/* 53 */
pub const TEGRA210_CLK_I2C2: u32 = 54;
pub const TEGRA210_CLK_UARTC: u32 = 55;
pub const TEGRA210_CLK_MIPI_CAL: u32 = 56;
pub const TEGRA210_CLK_EMC: u32 = 57;
pub const TEGRA210_CLK_USB2: u32 = 58;
/* 59 */
/* 60 */
/* 61 */
/* 62 */
pub const TEGRA210_CLK_BSEV: u32 = 63;

pub const TEGRA210_CLK_UARTD: u32 = 65;
pub const TEGRA210_CLK_I2C3: u32 = 67;
pub const TEGRA210_CLK_SBC4: u32 = 68;
pub const TEGRA210_CLK_SDMMC3: u32 = 69;
pub const TEGRA210_CLK_PCIE: u32 = 70;
pub const TEGRA210_CLK_OWR: u32 = 71;
pub const TEGRA210_CLK_AFI: u32 = 72;
pub const TEGRA210_CLK_CSITE: u32 = 73;
pub const TEGRA210_CLK_LA: u32 = 76;
pub const TEGRA210_CLK_SOC_THERM: u32 = 78;
pub const TEGRA210_CLK_DTV: u32 = 79;
pub const TEGRA210_CLK_I2CSLOW: u32 = 81;
pub const TEGRA210_CLK_DSIB: u32 = 82;
pub const TEGRA210_CLK_TSEC: u32 = 83;
pub const TEGRA210_CLK_XUSB_HOST: u32 = 89;
pub const TEGRA210_CLK_CSUS: u32 = 92;

pub const TEGRA210_CLK_MSELECT: u32 = 99;
pub const TEGRA210_CLK_TSENSOR: u32 = 100;
pub const TEGRA210_CLK_I2S3: u32 = 101;
pub const TEGRA210_CLK_I2S4: u32 = 102;
pub const TEGRA210_CLK_I2C4: u32 = 103;
pub const TEGRA210_CLK_D_AUDIO: u32 = 106;
pub const TEGRA210_CLK_APB2APE: u32 = 107;
pub const TEGRA210_CLK_HDA2CODEC_2X: u32 = 111;
pub const TEGRA210_CLK_SPDIF_2X: u32 = 118;
pub const TEGRA210_CLK_ACTMON: u32 = 119;
pub const TEGRA210_CLK_EXTERN1: u32 = 120;
pub const TEGRA210_CLK_EXTERN2: u32 = 121;
pub const TEGRA210_CLK_EXTERN3: u32 = 122;
pub const TEGRA210_CLK_SATA_OOB: u32 = 123;
pub const TEGRA210_CLK_SATA: u32 = 124;
pub const TEGRA210_CLK_HDA: u32 = 125;
pub const TEGRA210_CLK_HDA2HDMI: u32 = 128;
pub const TEGRA210_CLK_CEC: u32 = 136;
/* (bit affects xusb_falcon_src, xusb_fs_src, xusb_host_src and xusb_ss_src) */
pub const TEGRA210_CLK_XUSB_GATE: u32 = 143;
pub const TEGRA210_CLK_CILAB: u32 = 144;
pub const TEGRA210_CLK_CILCD: u32 = 145;
pub const TEGRA210_CLK_CILE: u32 = 146;
pub const TEGRA210_CLK_DSIALP: u32 = 147;
pub const TEGRA210_CLK_DSIBLP: u32 = 148;
pub const TEGRA210_CLK_ENTROPY: u32 = 149;
pub const TEGRA210_CLK_DP2: u32 = 152;
pub const TEGRA210_CLK_XUSB_SS: u32 = 156;
pub const TEGRA210_CLK_DMIC1: u32 = 161;
pub const TEGRA210_CLK_DMIC2: u32 = 162;
pub const TEGRA210_CLK_I2C6: u32 = 166;
pub const TEGRA210_CLK_VIM2_CLK: u32 = 171;
pub const TEGRA210_CLK_MIPIBIF: u32 = 173;
pub const TEGRA210_CLK_CLK72MHZ: u32 = 177;
pub const TEGRA210_CLK_VIC03: u32 = 178;
pub const TEGRA210_CLK_DPAUX: u32 = 181;
pub const TEGRA210_CLK_SOR0: u32 = 182;
pub const TEGRA210_CLK_SOR1: u32 = 183;
pub const TEGRA210_CLK_GPU: u32 = 184;
pub const TEGRA210_CLK_DBGAPB: u32 = 185;
pub const TEGRA210_CLK_PLL_P_OUT_ADSP: u32 = 187;
/* 188 ((bit affects pll_a_out_adsp and pll_a_out0_out_adsp) */
pub const TEGRA210_CLK_PLL_G_REF: u32 = 189;
pub const TEGRA210_CLK_SDMMC_LEGACY: u32 = 193;
pub const TEGRA210_CLK_NVDEC: u32 = 194;
pub const TEGRA210_CLK_NVJPG: u32 = 195;
pub const TEGRA210_CLK_DMIC3: u32 = 197;
pub const TEGRA210_CLK_APE: u32 = 198;
pub const TEGRA210_CLK_ADSP: u32 = 199;
pub const TEGRA210_CLK_MAUD: u32 = 202;
pub const TEGRA210_CLK_TSECB: u32 = 206;
pub const TEGRA210_CLK_DPAUX1: u32 = 207;
pub const TEGRA210_CLK_VI_I2C: u32 = 208;
pub const TEGRA210_CLK_HSIC_TRK: u32 = 209;
pub const TEGRA210_CLK_USB2_TRK: u32 = 210;
pub const TEGRA210_CLK_QSPI: u32 = 211;
pub const TEGRA210_CLK_UARTAPE: u32 = 212;
pub const TEGRA210_CLK_ADSP_NEON: u32 = 218;
pub const TEGRA210_CLK_NVENC: u32 = 219;
pub const TEGRA210_CLK_IQC2: u32 = 220;
pub const TEGRA210_CLK_IQC1: u32 = 221;
pub const TEGRA210_CLK_SOR_SAFE: u32 = 222;
pub const TEGRA210_CLK_PLL_P_OUT_CPU: u32 = 223;

pub const TEGRA210_CLK_UARTB: u32 = 224;
pub const TEGRA210_CLK_VFIR: u32 = 225;
pub const TEGRA210_CLK_SPDIF_IN: u32 = 226;
pub const TEGRA210_CLK_SPDIF_OUT: u32 = 227;
pub const TEGRA210_CLK_VI: u32 = 228;
pub const TEGRA210_CLK_VI_SENSOR: u32 = 229;
pub const TEGRA210_CLK_FUSE: u32 = 230;
pub const TEGRA210_CLK_FUSE_BURN: u32 = 231;
pub const TEGRA210_CLK_CLK_32K: u32 = 232;
pub const TEGRA210_CLK_CLK_M: u32 = 233;
pub const TEGRA210_CLK_CLK_M_DIV2: u32 = 234;
pub const TEGRA210_CLK_CLK_M_DIV4: u32 = 235;
pub const TEGRA210_CLK_OSC_DIV2: u32 = 234;
pub const TEGRA210_CLK_OSC_DIV4: u32 = 235;
pub const TEGRA210_CLK_PLL_REF: u32 = 236;
pub const TEGRA210_CLK_PLL_C: u32 = 237;
pub const TEGRA210_CLK_PLL_C_OUT1: u32 = 238;
pub const TEGRA210_CLK_PLL_C2: u32 = 239;
pub const TEGRA210_CLK_PLL_C3: u32 = 240;
pub const TEGRA210_CLK_PLL_M: u32 = 241;
pub const TEGRA210_CLK_PLL_M_OUT1: u32 = 242;
pub const TEGRA210_CLK_PLL_P: u32 = 243;
pub const TEGRA210_CLK_PLL_P_OUT1: u32 = 244;
pub const TEGRA210_CLK_PLL_P_OUT2: u32 = 245;
pub const TEGRA210_CLK_PLL_P_OUT3: u32 = 246;
pub const TEGRA210_CLK_PLL_P_OUT4: u32 = 247;
pub const TEGRA210_CLK_PLL_A: u32 = 248;
pub const TEGRA210_CLK_PLL_A_OUT0: u32 = 249;
pub const TEGRA210_CLK_PLL_D: u32 = 250;
pub const TEGRA210_CLK_PLL_D_OUT0: u32 = 251;
pub const TEGRA210_CLK_PLL_D2: u32 = 252;
pub const TEGRA210_CLK_PLL_D2_OUT0: u32 = 253;
pub const TEGRA210_CLK_PLL_U: u32 = 254;
pub const TEGRA210_CLK_PLL_U_480M: u32 = 255;
pub const TEGRA210_CLK_PLL_U_60M: u32 = 256;
pub const TEGRA210_CLK_PLL_U_48M: u32 = 257;
pub const TEGRA210_CLK_PLL_X: u32 = 259;
pub const TEGRA210_CLK_PLL_X_OUT0: u32 = 260;
pub const TEGRA210_CLK_PLL_RE_VCO: u32 = 261;
pub const TEGRA210_CLK_PLL_RE_OUT: u32 = 262;
pub const TEGRA210_CLK_PLL_E: u32 = 263;
pub const TEGRA210_CLK_SPDIF_IN_SYNC: u32 = 264;
pub const TEGRA210_CLK_I2S0_SYNC: u32 = 265;
pub const TEGRA210_CLK_I2S1_SYNC: u32 = 266;
pub const TEGRA210_CLK_I2S2_SYNC: u32 = 267;
pub const TEGRA210_CLK_I2S3_SYNC: u32 = 268;
pub const TEGRA210_CLK_I2S4_SYNC: u32 = 269;
pub const TEGRA210_CLK_VIMCLK_SYNC: u32 = 270;
pub const TEGRA210_CLK_AUDIO0: u32 = 271;
pub const TEGRA210_CLK_AUDIO1: u32 = 272;
pub const TEGRA210_CLK_AUDIO2: u32 = 273;
pub const TEGRA210_CLK_AUDIO3: u32 = 274;
pub const TEGRA210_CLK_AUDIO4: u32 = 275;
pub const TEGRA210_CLK_SPDIF: u32 = 276;
pub const TEGRA210_CLK_QSPI_PM: u32 = 278;
pub const TEGRA210_CLK_SOR0_LVDS: u32 = 281; /* deprecated */
pub const TEGRA210_CLK_SOR0_OUT: u32 = 281;
pub const TEGRA210_CLK_SOR1_OUT: u32 = 282;
pub const TEGRA210_CLK_XUSB_HOST_SRC: u32 = 284;
pub const TEGRA210_CLK_XUSB_FALCON_SRC: u32 = 285;
pub const TEGRA210_CLK_XUSB_FS_SRC: u32 = 286;
pub const TEGRA210_CLK_XUSB_SS_SRC: u32 = 287;
pub const TEGRA210_CLK_XUSB_DEV_SRC: u32 = 288;
pub const TEGRA210_CLK_XUSB_DEV: u32 = 289;
pub const TEGRA210_CLK_XUSB_HS_SRC: u32 = 290;
pub const TEGRA210_CLK_SCLK: u32 = 291;
pub const TEGRA210_CLK_HCLK: u32 = 292;
pub const TEGRA210_CLK_PCLK: u32 = 293;
pub const TEGRA210_CLK_CCLK_G: u32 = 294;
pub const TEGRA210_CLK_CCLK_LP: u32 = 295;
pub const TEGRA210_CLK_DFLL_REF: u32 = 296;
pub const TEGRA210_CLK_DFLL_SOC: u32 = 297;
pub const TEGRA210_CLK_VI_SENSOR2: u32 = 298;
pub const TEGRA210_CLK_PLL_P_OUT5: u32 = 299;
pub const TEGRA210_CLK_CML0: u32 = 300;
pub const TEGRA210_CLK_CML1: u32 = 301;
pub const TEGRA210_CLK_PLL_C4: u32 = 302;
pub const TEGRA210_CLK_PLL_DP: u32 = 303;
pub const TEGRA210_CLK_PLL_E_MUX: u32 = 304;
pub const TEGRA210_CLK_PLL_MB: u32 = 305;
pub const TEGRA210_CLK_PLL_A1: u32 = 306;
pub const TEGRA210_CLK_PLL_D_DSI_OUT: u32 = 307;
pub const TEGRA210_CLK_PLL_C4_OUT0: u32 = 308;
pub const TEGRA210_CLK_PLL_C4_OUT1: u32 = 309;
pub const TEGRA210_CLK_PLL_C4_OUT2: u32 = 310;
pub const TEGRA210_CLK_PLL_C4_OUT3: u32 = 311;
pub const TEGRA210_CLK_PLL_U_OUT: u32 = 312;
pub const TEGRA210_CLK_PLL_U_OUT1: u32 = 313;
pub const TEGRA210_CLK_PLL_U_OUT2: u32 = 314;
pub const TEGRA210_CLK_USB2_HSIC_TRK: u32 = 315;
pub const TEGRA210_CLK_PLL_P_OUT_HSIO: u32 = 316;
pub const TEGRA210_CLK_PLL_P_OUT_XUSB: u32 = 317;
pub const TEGRA210_CLK_XUSB_SSP_SRC: u32 = 318;
pub const TEGRA210_CLK_PLL_RE_OUT1: u32 = 319;
pub const TEGRA210_CLK_PLL_MB_UD: u32 = 320;
pub const TEGRA210_CLK_PLL_P_UD: u32 = 321;
pub const TEGRA210_CLK_ISP: u32 = 322;
pub const TEGRA210_CLK_PLL_A_OUT_ADSP: u32 = 323;
pub const TEGRA210_CLK_PLL_A_OUT0_OUT_ADSP: u32 = 324;
pub const TEGRA210_CLK_OSC: u32 = 326;
pub const TEGRA210_CLK_CSI_TPG: u32 = 327;
pub const TEGRA210_CLK_AUDIO0_MUX: u32 = 350;
pub const TEGRA210_CLK_AUDIO1_MUX: u32 = 351;
pub const TEGRA210_CLK_AUDIO2_MUX: u32 = 352;
pub const TEGRA210_CLK_AUDIO3_MUX: u32 = 353;
pub const TEGRA210_CLK_AUDIO4_MUX: u32 = 354;
pub const TEGRA210_CLK_SPDIF_MUX: u32 = 355;
pub const TEGRA210_CLK_DSIA_MUX: u32 = 359;
pub const TEGRA210_CLK_DSIB_MUX: u32 = 360;
pub const TEGRA210_CLK_XUSB_SS_DIV2: u32 = 362;
pub const TEGRA210_CLK_PLL_M_UD: u32 = 363;
pub const TEGRA210_CLK_PLL_C_UD: u32 = 364;
pub const TEGRA210_CLK_SCLK_MUX: u32 = 365;
pub const TEGRA210_CLK_ACLK: u32 = 370;
pub const TEGRA210_CLK_DMIC1_SYNC_CLK: u32 = 388;
pub const TEGRA210_CLK_DMIC1_SYNC_CLK_MUX: u32 = 389;
pub const TEGRA210_CLK_DMIC2_SYNC_CLK: u32 = 390;
pub const TEGRA210_CLK_DMIC2_SYNC_CLK_MUX: u32 = 391;
pub const TEGRA210_CLK_DMIC3_SYNC_CLK: u32 = 392;
pub const TEGRA210_CLK_DMIC3_SYNC_CLK_MUX: u32 = 393;
pub const TEGRA210_CLK_CLK_MAX: u32 = 394;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
