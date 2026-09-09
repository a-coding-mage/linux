/* SPDX-License-Identifier: GPL-2.0 */
/*
 * This header provides constants for binding nvidia,tegra20-car.
 *
 * The first 96 clocks are numbered to match the bits in the CAR's CLK_OUT_ENB
 * registers. These IDs often match those in the CAR's RST_DEVICES registers,
 * but not in all cases. Some bits in CLK_OUT_ENB affect multiple clocks. In
 * this case, those clocks are assigned IDs above 95 in order to highlight this
 * issue. Implementations that interpret these clock IDs as bit values
 * within the CLK_OUT_ENB or RST_DEVICES registers should be careful to
 * explicitly handle these special cases.
 *
 * The balance of the clocks controlled by the CAR are assigned IDs of 96 and
 * above.
 */

pub const TEGRA20_CLK_CPU: u32 = 0;
/* 1 */
/* 2 */
pub const TEGRA20_CLK_AC97: u32 = 3;
pub const TEGRA20_CLK_RTC: u32 = 4;
pub const TEGRA20_CLK_TIMER: u32 = 5;
pub const TEGRA20_CLK_UARTA: u32 = 6;
/* 7 (register bit affects uart2 and vfir) */
pub const TEGRA20_CLK_GPIO: u32 = 8;
pub const TEGRA20_CLK_SDMMC2: u32 = 9;
/* 10 (register bit affects spdif_in and spdif_out) */
pub const TEGRA20_CLK_I2S1: u32 = 11;
pub const TEGRA20_CLK_I2C1: u32 = 12;
pub const TEGRA20_CLK_NDFLASH: u32 = 13;
pub const TEGRA20_CLK_SDMMC1: u32 = 14;
pub const TEGRA20_CLK_SDMMC4: u32 = 15;
pub const TEGRA20_CLK_TWC: u32 = 16;
pub const TEGRA20_CLK_PWM: u32 = 17;
pub const TEGRA20_CLK_I2S2: u32 = 18;
pub const TEGRA20_CLK_EPP: u32 = 19;
/* 20 (register bit affects vi and vi_sensor) */
pub const TEGRA20_CLK_GR2D: u32 = 21;
pub const TEGRA20_CLK_USBD: u32 = 22;
pub const TEGRA20_CLK_ISP: u32 = 23;
pub const TEGRA20_CLK_GR3D: u32 = 24;
pub const TEGRA20_CLK_IDE: u32 = 25;
pub const TEGRA20_CLK_DISP2: u32 = 26;
pub const TEGRA20_CLK_DISP1: u32 = 27;
pub const TEGRA20_CLK_HOST1X: u32 = 28;
pub const TEGRA20_CLK_VCP: u32 = 29;
/* 30 */
pub const TEGRA20_CLK_CACHE2: u32 = 31;

pub const TEGRA20_CLK_MC: u32 = 32;
pub const TEGRA20_CLK_AHBDMA: u32 = 33;
pub const TEGRA20_CLK_APBDMA: u32 = 34;
/* 35 */
pub const TEGRA20_CLK_KBC: u32 = 36;
pub const TEGRA20_CLK_STAT_MON: u32 = 37;
pub const TEGRA20_CLK_PMC: u32 = 38;
pub const TEGRA20_CLK_FUSE: u32 = 39;
pub const TEGRA20_CLK_KFUSE: u32 = 40;
pub const TEGRA20_CLK_SBC1: u32 = 41;
pub const TEGRA20_CLK_NOR: u32 = 42;
pub const TEGRA20_CLK_SPI: u32 = 43;
pub const TEGRA20_CLK_SBC2: u32 = 44;
pub const TEGRA20_CLK_XIO: u32 = 45;
pub const TEGRA20_CLK_SBC3: u32 = 46;
pub const TEGRA20_CLK_DVC: u32 = 47;
pub const TEGRA20_CLK_DSI: u32 = 48;
/* 49 (register bit affects tvo and cve) */
pub const TEGRA20_CLK_MIPI: u32 = 50;
pub const TEGRA20_CLK_HDMI: u32 = 51;
pub const TEGRA20_CLK_CSI: u32 = 52;
pub const TEGRA20_CLK_TVDAC: u32 = 53;
pub const TEGRA20_CLK_I2C2: u32 = 54;
pub const TEGRA20_CLK_UARTC: u32 = 55;
/* 56 */
pub const TEGRA20_CLK_EMC: u32 = 57;
pub const TEGRA20_CLK_USB2: u32 = 58;
pub const TEGRA20_CLK_USB3: u32 = 59;
pub const TEGRA20_CLK_MPE: u32 = 60;
pub const TEGRA20_CLK_VDE: u32 = 61;
pub const TEGRA20_CLK_BSEA: u32 = 62;
pub const TEGRA20_CLK_BSEV: u32 = 63;

pub const TEGRA20_CLK_SPEEDO: u32 = 64;
pub const TEGRA20_CLK_UARTD: u32 = 65;
pub const TEGRA20_CLK_UARTE: u32 = 66;
pub const TEGRA20_CLK_I2C3: u32 = 67;
pub const TEGRA20_CLK_SBC4: u32 = 68;
pub const TEGRA20_CLK_SDMMC3: u32 = 69;
pub const TEGRA20_CLK_PEX: u32 = 70;
pub const TEGRA20_CLK_OWR: u32 = 71;
pub const TEGRA20_CLK_AFI: u32 = 72;
pub const TEGRA20_CLK_CSITE: u32 = 73;
/* 74 */
pub const TEGRA20_CLK_AVPUCQ: u32 = 75;
pub const TEGRA20_CLK_LA: u32 = 76;
/* 77 */
/* 78 */
/* 79 */
/* 80 */
/* 81 */
/* 82 */
/* 83 */
pub const TEGRA20_CLK_IRAMA: u32 = 84;
pub const TEGRA20_CLK_IRAMB: u32 = 85;
pub const TEGRA20_CLK_IRAMC: u32 = 86;
pub const TEGRA20_CLK_IRAMD: u32 = 87;
pub const TEGRA20_CLK_CRAM2: u32 = 88;
pub const TEGRA20_CLK_AUDIO_2X: u32 = 89; /* a/k/a audio_2x_sync_clk */
pub const TEGRA20_CLK_CLK_D: u32 = 90;
/* 91 */
pub const TEGRA20_CLK_CSUS: u32 = 92;
pub const TEGRA20_CLK_CDEV2: u32 = 93;
pub const TEGRA20_CLK_CDEV1: u32 = 94;
/* 95 */

pub const TEGRA20_CLK_UARTB: u32 = 96;
pub const TEGRA20_CLK_VFIR: u32 = 97;
pub const TEGRA20_CLK_SPDIF_IN: u32 = 98;
pub const TEGRA20_CLK_SPDIF_OUT: u32 = 99;
pub const TEGRA20_CLK_VI: u32 = 100;
pub const TEGRA20_CLK_VI_SENSOR: u32 = 101;
pub const TEGRA20_CLK_TVO: u32 = 102;
pub const TEGRA20_CLK_CVE: u32 = 103;
pub const TEGRA20_CLK_OSC: u32 = 104;
pub const TEGRA20_CLK_CLK_32K: u32 = 105; /* a/k/a clk_s */
pub const TEGRA20_CLK_CLK_M: u32 = 106;
pub const TEGRA20_CLK_SCLK: u32 = 107;
pub const TEGRA20_CLK_CCLK: u32 = 108;
pub const TEGRA20_CLK_HCLK: u32 = 109;
pub const TEGRA20_CLK_PCLK: u32 = 110;
/* 111 */
pub const TEGRA20_CLK_PLL_A: u32 = 112;
pub const TEGRA20_CLK_PLL_A_OUT0: u32 = 113;
pub const TEGRA20_CLK_PLL_C: u32 = 114;
pub const TEGRA20_CLK_PLL_C_OUT1: u32 = 115;
pub const TEGRA20_CLK_PLL_D: u32 = 116;
pub const TEGRA20_CLK_PLL_D_OUT0: u32 = 117;
pub const TEGRA20_CLK_PLL_E: u32 = 118;
pub const TEGRA20_CLK_PLL_M: u32 = 119;
pub const TEGRA20_CLK_PLL_M_OUT1: u32 = 120;
pub const TEGRA20_CLK_PLL_P: u32 = 121;
pub const TEGRA20_CLK_PLL_P_OUT1: u32 = 122;
pub const TEGRA20_CLK_PLL_P_OUT2: u32 = 123;
pub const TEGRA20_CLK_PLL_P_OUT3: u32 = 124;
pub const TEGRA20_CLK_PLL_P_OUT4: u32 = 125;
pub const TEGRA20_CLK_PLL_S: u32 = 126;
pub const TEGRA20_CLK_PLL_U: u32 = 127;

pub const TEGRA20_CLK_PLL_X: u32 = 128;
pub const TEGRA20_CLK_COP: u32 = 129; /* a/k/a avp */
pub const TEGRA20_CLK_AUDIO: u32 = 130; /* a/k/a audio_sync_clk */
pub const TEGRA20_CLK_PLL_REF: u32 = 131;
pub const TEGRA20_CLK_TWD: u32 = 132;
pub const TEGRA20_CLK_CLK_MAX: u32 = 133;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
