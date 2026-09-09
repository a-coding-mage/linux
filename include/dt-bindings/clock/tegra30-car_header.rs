/* SPDX-License-Identifier: GPL-2.0 */
/*
 * This header provides constants for binding nvidia,tegra30-car.
 *
 * The first 130 clocks are numbered to match the bits in the CAR's CLK_OUT_ENB
 * registers. These IDs often match those in the CAR's RST_DEVICES registers,
 * but not in all cases. Some bits in CLK_OUT_ENB affect multiple clocks. In
 * this case, those clocks are assigned IDs above 160 in order to highlight
 * this issue. Implementations that interpret these clock IDs as bit values
 * within the CLK_OUT_ENB or RST_DEVICES registers should be careful to
 * explicitly handle these special cases.
 *
 * The balance of the clocks controlled by the CAR are assigned IDs of 160 and
 * above.
 */

pub const TEGRA30_CLK_CPU: u32 = 0;
/* 1 */ /* 2 */ /* 3 */
pub const TEGRA30_CLK_RTC: u32 = 4;
pub const TEGRA30_CLK_TIMER: u32 = 5;
pub const TEGRA30_CLK_UARTA: u32 = 6;
/* 7 (register bit affects uartb and vfir) */
pub const TEGRA30_CLK_GPIO: u32 = 8;
pub const TEGRA30_CLK_SDMMC2: u32 = 9;
/* 10 (register bit affects spdif_in and spdif_out) */
pub const TEGRA30_CLK_I2S1: u32 = 11;
pub const TEGRA30_CLK_I2C1: u32 = 12;
pub const TEGRA30_CLK_NDFLASH: u32 = 13;
pub const TEGRA30_CLK_SDMMC1: u32 = 14;
pub const TEGRA30_CLK_SDMMC4: u32 = 15;
/* 16 */
pub const TEGRA30_CLK_PWM: u32 = 17;
pub const TEGRA30_CLK_I2S2: u32 = 18;
pub const TEGRA30_CLK_EPP: u32 = 19;
/* 20 (register bit affects vi and vi_sensor) */
pub const TEGRA30_CLK_GR2D: u32 = 21;
pub const TEGRA30_CLK_USBD: u32 = 22;
pub const TEGRA30_CLK_ISP: u32 = 23;
pub const TEGRA30_CLK_GR3D: u32 = 24;
/* 25 */
pub const TEGRA30_CLK_DISP2: u32 = 26;
pub const TEGRA30_CLK_DISP1: u32 = 27;
pub const TEGRA30_CLK_HOST1X: u32 = 28;
pub const TEGRA30_CLK_VCP: u32 = 29;
pub const TEGRA30_CLK_I2S0: u32 = 30;
pub const TEGRA30_CLK_COP_CACHE: u32 = 31;

pub const TEGRA30_CLK_MC: u32 = 32;
pub const TEGRA30_CLK_AHBDMA: u32 = 33;
pub const TEGRA30_CLK_APBDMA: u32 = 34;
/* 35 */
pub const TEGRA30_CLK_KBC: u32 = 36;
pub const TEGRA30_CLK_STATMON: u32 = 37;
pub const TEGRA30_CLK_PMC: u32 = 38;
/* 39 (register bit affects fuse and fuse_burn) */
pub const TEGRA30_CLK_KFUSE: u32 = 40;
pub const TEGRA30_CLK_SBC1: u32 = 41;
pub const TEGRA30_CLK_NOR: u32 = 42;
/* 43 */
pub const TEGRA30_CLK_SBC2: u32 = 44;
/* 45 */
pub const TEGRA30_CLK_SBC3: u32 = 46;
pub const TEGRA30_CLK_I2C5: u32 = 47;
pub const TEGRA30_CLK_DSIA: u32 = 48;
/* 49 (register bit affects cve and tvo) */
pub const TEGRA30_CLK_MIPI: u32 = 50;
pub const TEGRA30_CLK_HDMI: u32 = 51;
pub const TEGRA30_CLK_CSI: u32 = 52;
pub const TEGRA30_CLK_TVDAC: u32 = 53;
pub const TEGRA30_CLK_I2C2: u32 = 54;
pub const TEGRA30_CLK_UARTC: u32 = 55;
/* 56 */
pub const TEGRA30_CLK_EMC: u32 = 57;
pub const TEGRA30_CLK_USB2: u32 = 58;
pub const TEGRA30_CLK_USB3: u32 = 59;
pub const TEGRA30_CLK_MPE: u32 = 60;
pub const TEGRA30_CLK_VDE: u32 = 61;
pub const TEGRA30_CLK_BSEA: u32 = 62;
pub const TEGRA30_CLK_BSEV: u32 = 63;

pub const TEGRA30_CLK_SPEEDO: u32 = 64;
pub const TEGRA30_CLK_UARTD: u32 = 65;
pub const TEGRA30_CLK_UARTE: u32 = 66;
pub const TEGRA30_CLK_I2C3: u32 = 67;
pub const TEGRA30_CLK_SBC4: u32 = 68;
pub const TEGRA30_CLK_SDMMC3: u32 = 69;
pub const TEGRA30_CLK_PCIE: u32 = 70;
pub const TEGRA30_CLK_OWR: u32 = 71;
pub const TEGRA30_CLK_AFI: u32 = 72;
pub const TEGRA30_CLK_CSITE: u32 = 73;
/* 74 */
pub const TEGRA30_CLK_AVPUCQ: u32 = 75;
pub const TEGRA30_CLK_LA: u32 = 76;
/* 77 */ /* 78 */
pub const TEGRA30_CLK_DTV: u32 = 79;
pub const TEGRA30_CLK_NDSPEED: u32 = 80;
pub const TEGRA30_CLK_I2CSLOW: u32 = 81;
pub const TEGRA30_CLK_DSIB: u32 = 82;
/* 83 */
pub const TEGRA30_CLK_IRAMA: u32 = 84;
pub const TEGRA30_CLK_IRAMB: u32 = 85;
pub const TEGRA30_CLK_IRAMC: u32 = 86;
pub const TEGRA30_CLK_IRAMD: u32 = 87;
pub const TEGRA30_CLK_CRAM2: u32 = 88;
/* 89 */
pub const TEGRA30_CLK_AUDIO_2X: u32 = 90; /* a/k/a audio_2x_sync_clk */
/* 91 */
pub const TEGRA30_CLK_CSUS: u32 = 92;
pub const TEGRA30_CLK_CDEV2: u32 = 93;
pub const TEGRA30_CLK_CDEV1: u32 = 94;
/* 95 */

pub const TEGRA30_CLK_CPU_G: u32 = 96;
pub const TEGRA30_CLK_CPU_LP: u32 = 97;
pub const TEGRA30_CLK_GR3D2: u32 = 98;
pub const TEGRA30_CLK_MSELECT: u32 = 99;
pub const TEGRA30_CLK_TSENSOR: u32 = 100;
pub const TEGRA30_CLK_I2S3: u32 = 101;
pub const TEGRA30_CLK_I2S4: u32 = 102;
pub const TEGRA30_CLK_I2C4: u32 = 103;
pub const TEGRA30_CLK_SBC5: u32 = 104;
pub const TEGRA30_CLK_SBC6: u32 = 105;
pub const TEGRA30_CLK_D_AUDIO: u32 = 106;
pub const TEGRA30_CLK_APBIF: u32 = 107;
pub const TEGRA30_CLK_DAM0: u32 = 108;
pub const TEGRA30_CLK_DAM1: u32 = 109;
pub const TEGRA30_CLK_DAM2: u32 = 110;
pub const TEGRA30_CLK_HDA2CODEC_2X: u32 = 111;
pub const TEGRA30_CLK_ATOMICS: u32 = 112;
pub const TEGRA30_CLK_AUDIO0_2X: u32 = 113;
pub const TEGRA30_CLK_AUDIO1_2X: u32 = 114;
pub const TEGRA30_CLK_AUDIO2_2X: u32 = 115;
pub const TEGRA30_CLK_AUDIO3_2X: u32 = 116;
pub const TEGRA30_CLK_AUDIO4_2X: u32 = 117;
pub const TEGRA30_CLK_SPDIF_2X: u32 = 118;
pub const TEGRA30_CLK_ACTMON: u32 = 119;
pub const TEGRA30_CLK_EXTERN1: u32 = 120;
pub const TEGRA30_CLK_EXTERN2: u32 = 121;
pub const TEGRA30_CLK_EXTERN3: u32 = 122;
pub const TEGRA30_CLK_SATA_OOB: u32 = 123;
pub const TEGRA30_CLK_SATA: u32 = 124;
pub const TEGRA30_CLK_HDA: u32 = 125;
/* 126 */
pub const TEGRA30_CLK_SE: u32 = 127;

pub const TEGRA30_CLK_HDA2HDMI: u32 = 128;
pub const TEGRA30_CLK_SATA_COLD: u32 = 129;
/* 130 through 159 are unassigned. */

pub const TEGRA30_CLK_UARTB: u32 = 160;
pub const TEGRA30_CLK_VFIR: u32 = 161;
pub const TEGRA30_CLK_SPDIF_IN: u32 = 162;
pub const TEGRA30_CLK_SPDIF_OUT: u32 = 163;
pub const TEGRA30_CLK_VI: u32 = 164;
pub const TEGRA30_CLK_VI_SENSOR: u32 = 165;
pub const TEGRA30_CLK_FUSE: u32 = 166;
pub const TEGRA30_CLK_FUSE_BURN: u32 = 167;
pub const TEGRA30_CLK_CVE: u32 = 168;
pub const TEGRA30_CLK_TVO: u32 = 169;
pub const TEGRA30_CLK_CLK_32K: u32 = 170;
pub const TEGRA30_CLK_CLK_M: u32 = 171;
pub const TEGRA30_CLK_CLK_M_DIV2: u32 = 172;
pub const TEGRA30_CLK_CLK_M_DIV4: u32 = 173;
pub const TEGRA30_CLK_OSC_DIV2: u32 = 172;
pub const TEGRA30_CLK_OSC_DIV4: u32 = 173;
pub const TEGRA30_CLK_PLL_REF: u32 = 174;
pub const TEGRA30_CLK_PLL_C: u32 = 175;
pub const TEGRA30_CLK_PLL_C_OUT1: u32 = 176;
pub const TEGRA30_CLK_PLL_M: u32 = 177;
pub const TEGRA30_CLK_PLL_M_OUT1: u32 = 178;
pub const TEGRA30_CLK_PLL_P: u32 = 179;
pub const TEGRA30_CLK_PLL_P_OUT1: u32 = 180;
pub const TEGRA30_CLK_PLL_P_OUT2: u32 = 181;
pub const TEGRA30_CLK_PLL_P_OUT3: u32 = 182;
pub const TEGRA30_CLK_PLL_P_OUT4: u32 = 183;
pub const TEGRA30_CLK_PLL_A: u32 = 184;
pub const TEGRA30_CLK_PLL_A_OUT0: u32 = 185;
pub const TEGRA30_CLK_PLL_D: u32 = 186;
pub const TEGRA30_CLK_PLL_D_OUT0: u32 = 187;
pub const TEGRA30_CLK_PLL_D2: u32 = 188;
pub const TEGRA30_CLK_PLL_D2_OUT0: u32 = 189;
pub const TEGRA30_CLK_PLL_U: u32 = 190;
pub const TEGRA30_CLK_PLL_X: u32 = 191;

pub const TEGRA30_CLK_PLL_X_OUT0: u32 = 192;
pub const TEGRA30_CLK_PLL_E: u32 = 193;
pub const TEGRA30_CLK_SPDIF_IN_SYNC: u32 = 194;
pub const TEGRA30_CLK_I2S0_SYNC: u32 = 195;
pub const TEGRA30_CLK_I2S1_SYNC: u32 = 196;
pub const TEGRA30_CLK_I2S2_SYNC: u32 = 197;
pub const TEGRA30_CLK_I2S3_SYNC: u32 = 198;
pub const TEGRA30_CLK_I2S4_SYNC: u32 = 199;
pub const TEGRA30_CLK_VIMCLK_SYNC: u32 = 200;
pub const TEGRA30_CLK_AUDIO0: u32 = 201;
pub const TEGRA30_CLK_AUDIO1: u32 = 202;
pub const TEGRA30_CLK_AUDIO2: u32 = 203;
pub const TEGRA30_CLK_AUDIO3: u32 = 204;
pub const TEGRA30_CLK_AUDIO4: u32 = 205;
pub const TEGRA30_CLK_SPDIF: u32 = 206;
/* 207 through 209 are unassigned. */
pub const TEGRA30_CLK_SCLK: u32 = 210;
/* 211 */
pub const TEGRA30_CLK_CCLK_G: u32 = 212;
pub const TEGRA30_CLK_CCLK_LP: u32 = 213;
pub const TEGRA30_CLK_TWD: u32 = 214;
pub const TEGRA30_CLK_CML0: u32 = 215;
pub const TEGRA30_CLK_CML1: u32 = 216;
pub const TEGRA30_CLK_HCLK: u32 = 217;
pub const TEGRA30_CLK_PCLK: u32 = 218;
/* 219 */
pub const TEGRA30_CLK_OSC: u32 = 220;
/* 221 through 223 are unassigned. */

/* 288 through 302 are unassigned. */
pub const TEGRA30_CLK_AUDIO0_MUX: u32 = 303;
pub const TEGRA30_CLK_AUDIO1_MUX: u32 = 304;
pub const TEGRA30_CLK_AUDIO2_MUX: u32 = 305;
pub const TEGRA30_CLK_AUDIO3_MUX: u32 = 306;
pub const TEGRA30_CLK_AUDIO4_MUX: u32 = 307;
pub const TEGRA30_CLK_SPDIF_MUX: u32 = 308;
pub const TEGRA30_CLK_CSIA_PAD: u32 = 309;
pub const TEGRA30_CLK_CSIB_PAD: u32 = 310;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
