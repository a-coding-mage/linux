/* SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause) */
/*
 * Copyright (C) 2020 Intel Corporation.
 * Lei Chuanhua <Chuanhua.lei@intel.com>
 * Zhu Yixin <Yixin.zhu@intel.com>
 */

/* PLL clocks */
pub const LGM_CLK_OSC: i32 = 1;
pub const LGM_CLK_PLLPP: i32 = 2;
pub const LGM_CLK_PLL2: i32 = 3;
pub const LGM_CLK_PLL0CZ: i32 = 4;
pub const LGM_CLK_PLL0B: i32 = 5;
pub const LGM_CLK_PLL1: i32 = 6;
pub const LGM_CLK_LJPLL3: i32 = 7;
pub const LGM_CLK_LJPLL4: i32 = 8;
pub const LGM_CLK_PLL0CM0: i32 = 9;
pub const LGM_CLK_PLL0CM1: i32 = 10;

/* clocks from PLLs */

/* ROPLL clocks */
pub const LGM_CLK_PP_HW: i32 = 15;
pub const LGM_CLK_PP_UC: i32 = 16;
pub const LGM_CLK_PP_FXD: i32 = 17;
pub const LGM_CLK_PP_TBM: i32 = 18;

/* PLL2 clocks */
pub const LGM_CLK_DDR: i32 = 20;

/* PLL0CZ */
pub const LGM_CLK_CM: i32 = 25;
pub const LGM_CLK_IC: i32 = 26;
pub const LGM_CLK_SDXC3: i32 = 27;

/* PLL0B */
pub const LGM_CLK_NGI: i32 = 30;
pub const LGM_CLK_NOC4: i32 = 31;
pub const LGM_CLK_SW: i32 = 32;
pub const LGM_CLK_QSPI: i32 = 33;
pub const LGM_CLK_CQEM: i32 = LGM_CLK_SW;
pub const LGM_CLK_EMMC5: i32 = LGM_CLK_NOC4;

/* PLL1 */
pub const LGM_CLK_CT: i32 = 35;
pub const LGM_CLK_DSP: i32 = 36;
pub const LGM_CLK_VIF: i32 = 37;

/* LJPLL3 */
pub const LGM_CLK_CML: i32 = 40;
pub const LGM_CLK_SERDES: i32 = 41;
pub const LGM_CLK_POOL: i32 = 42;
pub const LGM_CLK_PTP: i32 = 43;

/* LJPLL4 */
pub const LGM_CLK_PCIE: i32 = 45;
pub const LGM_CLK_SATA: i32 = LGM_CLK_PCIE;

/* PLL0CM0 */
pub const LGM_CLK_CPU0: i32 = 50;

/* PLL0CM1 */
pub const LGM_CLK_CPU1: i32 = 55;

/* Miscellaneous clocks */
pub const LGM_CLK_EMMC4: i32 = 60;
pub const LGM_CLK_SDXC2: i32 = 61;
pub const LGM_CLK_EMMC: i32 = 62;
pub const LGM_CLK_SDXC: i32 = 63;
pub const LGM_CLK_SLIC: i32 = 64;
pub const LGM_CLK_DCL: i32 = 65;
pub const LGM_CLK_DOCSIS: i32 = 66;
pub const LGM_CLK_PCM: i32 = 67;
pub const LGM_CLK_DDR_PHY: i32 = 68;
pub const LGM_CLK_PONDEF: i32 = 69;
pub const LGM_CLK_PL25M: i32 = 70;
pub const LGM_CLK_PL10M: i32 = 71;
pub const LGM_CLK_PL1544K: i32 = 72;
pub const LGM_CLK_PL2048K: i32 = 73;
pub const LGM_CLK_PL8K: i32 = 74;
pub const LGM_CLK_PON_NTR: i32 = 75;
pub const LGM_CLK_SYNC0: i32 = 76;
pub const LGM_CLK_SYNC1: i32 = 77;
pub const LGM_CLK_PROGDIV: i32 = 78;
pub const LGM_CLK_OD0: i32 = 79;
pub const LGM_CLK_OD1: i32 = 80;
pub const LGM_CLK_CBPHY0: i32 = 81;
pub const LGM_CLK_CBPHY1: i32 = 82;
pub const LGM_CLK_CBPHY2: i32 = 83;
pub const LGM_CLK_CBPHY3: i32 = 84;

/* Gate clocks */
/* Gate CLK0 */
pub const LGM_GCLK_C55: i32 = 100;
pub const LGM_GCLK_QSPI: i32 = 101;
pub const LGM_GCLK_EIP197: i32 = 102;
pub const LGM_GCLK_VAULT: i32 = 103;
pub const LGM_GCLK_TOE: i32 = 104;
pub const LGM_GCLK_SDXC: i32 = 105;
pub const LGM_GCLK_EMMC: i32 = 106;
pub const LGM_GCLK_SPI_DBG: i32 = 107;
pub const LGM_GCLK_DMA3: i32 = 108;

/* Gate CLK1 */
pub const LGM_GCLK_DMA0: i32 = 120;
pub const LGM_GCLK_LEDC0: i32 = 121;
pub const LGM_GCLK_LEDC1: i32 = 122;
pub const LGM_GCLK_I2S0: i32 = 123;
pub const LGM_GCLK_I2S1: i32 = 124;
pub const LGM_GCLK_EBU: i32 = 125;
pub const LGM_GCLK_PWM: i32 = 126;
pub const LGM_GCLK_I2C0: i32 = 127;
pub const LGM_GCLK_I2C1: i32 = 128;
pub const LGM_GCLK_I2C2: i32 = 129;
pub const LGM_GCLK_I2C3: i32 = 130;
pub const LGM_GCLK_SSC0: i32 = 131;
pub const LGM_GCLK_SSC1: i32 = 132;
pub const LGM_GCLK_SSC2: i32 = 133;
pub const LGM_GCLK_SSC3: i32 = 134;
pub const LGM_GCLK_GPTC0: i32 = 135;
pub const LGM_GCLK_GPTC1: i32 = 136;
pub const LGM_GCLK_GPTC2: i32 = 137;
pub const LGM_GCLK_GPTC3: i32 = 138;
pub const LGM_GCLK_ASC0: i32 = 139;
pub const LGM_GCLK_ASC1: i32 = 140;
pub const LGM_GCLK_ASC2: i32 = 141;
pub const LGM_GCLK_ASC3: i32 = 142;
pub const LGM_GCLK_PCM0: i32 = 143;
pub const LGM_GCLK_PCM1: i32 = 144;
pub const LGM_GCLK_PCM2: i32 = 145;

/* Gate CLK2 */
pub const LGM_GCLK_PCIE10: i32 = 150;
pub const LGM_GCLK_PCIE11: i32 = 151;
pub const LGM_GCLK_PCIE30: i32 = 152;
pub const LGM_GCLK_PCIE31: i32 = 153;
pub const LGM_GCLK_PCIE20: i32 = 154;
pub const LGM_GCLK_PCIE21: i32 = 155;
pub const LGM_GCLK_PCIE40: i32 = 156;
pub const LGM_GCLK_PCIE41: i32 = 157;
pub const LGM_GCLK_XPCS0: i32 = 158;
pub const LGM_GCLK_XPCS1: i32 = 159;
pub const LGM_GCLK_XPCS2: i32 = 160;
pub const LGM_GCLK_XPCS3: i32 = 161;
pub const LGM_GCLK_SATA0: i32 = 162;
pub const LGM_GCLK_SATA1: i32 = 163;
pub const LGM_GCLK_SATA2: i32 = 164;
pub const LGM_GCLK_SATA3: i32 = 165;

/* Gate CLK3 */
pub const LGM_GCLK_ARCEM4: i32 = 170;
pub const LGM_GCLK_IDMAR1: i32 = 171;
pub const LGM_GCLK_IDMAT0: i32 = 172;
pub const LGM_GCLK_IDMAT1: i32 = 173;
pub const LGM_GCLK_IDMAT2: i32 = 174;
pub const LGM_GCLK_PPV4: i32 = 175;
pub const LGM_GCLK_GSWIPO: i32 = 176;
pub const LGM_GCLK_CQEM: i32 = 177;
pub const LGM_GCLK_XPCS5: i32 = 178;
pub const LGM_GCLK_USB1: i32 = 179;
pub const LGM_GCLK_USB2: i32 = 180;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
