/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright 2016 Freescale Semiconductor, Inc.
 * Copyright 2017 NXP
 */

// Translated from the C header; include guards are not applicable in Rust.

/* ANAMIX PLL clocks */
/* FRAC PLLs */
/* ARM PLL */
pub const IMX8MQ_CLK_DUMMY: i32 = 0;
pub const IMX8MQ_CLK_32K: i32 = 1;
pub const IMX8MQ_CLK_25M: i32 = 2;
pub const IMX8MQ_CLK_27M: i32 = 3;
pub const IMX8MQ_CLK_EXT1: i32 = 4;
pub const IMX8MQ_CLK_EXT2: i32 = 5;
pub const IMX8MQ_CLK_EXT3: i32 = 6;
pub const IMX8MQ_CLK_EXT4: i32 = 7;
pub const IMX8MQ_ARM_PLL_REF_SEL: i32 = 8;
pub const IMX8MQ_ARM_PLL_REF_DIV: i32 = 9;
pub const IMX8MQ_ARM_PLL: i32 = 10;
pub const IMX8MQ_ARM_PLL_BYPASS: i32 = 11;
pub const IMX8MQ_ARM_PLL_OUT: i32 = 12;
/* GPU PLL */
pub const IMX8MQ_GPU_PLL_REF_SEL: i32 = 13;
pub const IMX8MQ_GPU_PLL_REF_DIV: i32 = 14;
pub const IMX8MQ_GPU_PLL: i32 = 15;
pub const IMX8MQ_GPU_PLL_BYPASS: i32 = 16;
pub const IMX8MQ_GPU_PLL_OUT: i32 = 17;
/* VPU PLL */
pub const IMX8MQ_VPU_PLL_REF_SEL: i32 = 18;
pub const IMX8MQ_VPU_PLL_REF_DIV: i32 = 19;
pub const IMX8MQ_VPU_PLL: i32 = 20;
pub const IMX8MQ_VPU_PLL_BYPASS: i32 = 21;
pub const IMX8MQ_VPU_PLL_OUT: i32 = 22;
/* AUDIO PLL1 */
pub const IMX8MQ_AUDIO_PLL1_REF_SEL: i32 = 23;
pub const IMX8MQ_AUDIO_PLL1_REF_DIV: i32 = 24;
pub const IMX8MQ_AUDIO_PLL1: i32 = 25;
pub const IMX8MQ_AUDIO_PLL1_BYPASS: i32 = 26;
pub const IMX8MQ_AUDIO_PLL1_OUT: i32 = 27;
/* AUDIO PLL2 */
pub const IMX8MQ_AUDIO_PLL2_REF_SEL: i32 = 28;
pub const IMX8MQ_AUDIO_PLL2_REF_DIV: i32 = 29;
pub const IMX8MQ_AUDIO_PLL2: i32 = 30;
pub const IMX8MQ_AUDIO_PLL2_BYPASS: i32 = 31;
pub const IMX8MQ_AUDIO_PLL2_OUT: i32 = 32;
/* VIDEO PLL1 */
pub const IMX8MQ_VIDEO_PLL1_REF_SEL: i32 = 33;
pub const IMX8MQ_VIDEO_PLL1_REF_DIV: i32 = 34;
pub const IMX8MQ_VIDEO_PLL1: i32 = 35;
pub const IMX8MQ_VIDEO_PLL1_BYPASS: i32 = 36;
pub const IMX8MQ_VIDEO_PLL1_OUT: i32 = 37;
/* SYS1 PLL */
pub const IMX8MQ_SYS1_PLL1_REF_SEL: i32 = 38;
pub const IMX8MQ_SYS1_PLL1_REF_DIV: i32 = 39;
pub const IMX8MQ_SYS1_PLL1: i32 = 40;
pub const IMX8MQ_SYS1_PLL1_OUT: i32 = 41;
pub const IMX8MQ_SYS1_PLL1_OUT_DIV: i32 = 42;
pub const IMX8MQ_SYS1_PLL2: i32 = 43;
pub const IMX8MQ_SYS1_PLL2_DIV: i32 = 44;
pub const IMX8MQ_SYS1_PLL2_OUT: i32 = 45;
/* SYS2 PLL */
pub const IMX8MQ_SYS2_PLL1_REF_SEL: i32 = 46;
pub const IMX8MQ_SYS2_PLL1_REF_DIV: i32 = 47;
pub const IMX8MQ_SYS2_PLL1: i32 = 48;
pub const IMX8MQ_SYS2_PLL1_OUT: i32 = 49;
pub const IMX8MQ_SYS2_PLL1_OUT_DIV: i32 = 50;
pub const IMX8MQ_SYS2_PLL2: i32 = 51;
pub const IMX8MQ_SYS2_PLL2_DIV: i32 = 52;
pub const IMX8MQ_SYS2_PLL2_OUT: i32 = 53;
/* SYS3 PLL */
pub const IMX8MQ_SYS3_PLL1_REF_SEL: i32 = 54;
pub const IMX8MQ_SYS3_PLL1_REF_DIV: i32 = 55;
pub const IMX8MQ_SYS3_PLL1: i32 = 56;
pub const IMX8MQ_SYS3_PLL1_OUT: i32 = 57;
pub const IMX8MQ_SYS3_PLL1_OUT_DIV: i32 = 58;
pub const IMX8MQ_SYS3_PLL2: i32 = 59;
pub const IMX8MQ_SYS3_PLL2_DIV: i32 = 60;
pub const IMX8MQ_SYS3_PLL2_OUT: i32 = 61;
/* DRAM PLL */
pub const IMX8MQ_DRAM_PLL1_REF_SEL: i32 = 62;
pub const IMX8MQ_DRAM_PLL1_REF_DIV: i32 = 63;
pub const IMX8MQ_DRAM_PLL1: i32 = 64;
pub const IMX8MQ_DRAM_PLL1_OUT: i32 = 65;
pub const IMX8MQ_DRAM_PLL1_OUT_DIV: i32 = 66;
pub const IMX8MQ_DRAM_PLL2: i32 = 67;
pub const IMX8MQ_DRAM_PLL2_DIV: i32 = 68;
pub const IMX8MQ_DRAM_PLL2_OUT: i32 = 69;
/* SYS PLL DIV */
pub const IMX8MQ_SYS1_PLL_40M: i32 = 70;
pub const IMX8MQ_SYS1_PLL_80M: i32 = 71;
pub const IMX8MQ_SYS1_PLL_100M: i32 = 72;
pub const IMX8MQ_SYS1_PLL_133M: i32 = 73;
pub const IMX8MQ_SYS1_PLL_160M: i32 = 74;
pub const IMX8MQ_SYS1_PLL_200M: i32 = 75;
pub const IMX8MQ_SYS1_PLL_266M: i32 = 76;
pub const IMX8MQ_SYS1_PLL_400M: i32 = 77;
pub const IMX8MQ_SYS1_PLL_800M: i32 = 78;
pub const IMX8MQ_SYS2_PLL_50M: i32 = 79;
pub const IMX8MQ_SYS2_PLL_100M: i32 = 80;
pub const IMX8MQ_SYS2_PLL_125M: i32 = 81;
pub const IMX8MQ_SYS2_PLL_166M: i32 = 82;
pub const IMX8MQ_SYS2_PLL_200M: i32 = 83;
pub const IMX8MQ_SYS2_PLL_250M: i32 = 84;
pub const IMX8MQ_SYS2_PLL_333M: i32 = 85;
pub const IMX8MQ_SYS2_PLL_500M: i32 = 86;
pub const IMX8MQ_SYS2_PLL_1000M: i32 = 87;
/* CCM ROOT clocks */
/* A53 */
pub const IMX8MQ_CLK_A53_SRC: i32 = 88;
pub const IMX8MQ_CLK_A53_CG: i32 = 89;
pub const IMX8MQ_CLK_A53_DIV: i32 = 90;
/* M4 */
pub const IMX8MQ_CLK_M4_SRC: i32 = 91;
pub const IMX8MQ_CLK_M4_CG: i32 = 92;
pub const IMX8MQ_CLK_M4_DIV: i32 = 93;
/* VPU */
pub const IMX8MQ_CLK_VPU_SRC: i32 = 94;
pub const IMX8MQ_CLK_VPU_CG: i32 = 95;
pub const IMX8MQ_CLK_VPU_DIV: i32 = 96;
/* GPU CORE */
pub const IMX8MQ_CLK_GPU_CORE_SRC: i32 = 97;
pub const IMX8MQ_CLK_GPU_CORE_CG: i32 = 98;
pub const IMX8MQ_CLK_GPU_CORE_DIV: i32 = 99;
/* GPU SHADER */
pub const IMX8MQ_CLK_GPU_SHADER_SRC: i32 = 100;
pub const IMX8MQ_CLK_GPU_SHADER_CG: i32 = 101;
pub const IMX8MQ_CLK_GPU_SHADER_DIV: i32 = 102;
/* BUS TYPE */
/* MAIN AXI */
pub const IMX8MQ_CLK_MAIN_AXI: i32 = 103;
/* ENET AXI */
pub const IMX8MQ_CLK_ENET_AXI: i32 = 104;
/* NAND_USDHC_BUS */
pub const IMX8MQ_CLK_NAND_USDHC_BUS: i32 = 105;
/* VPU BUS */
pub const IMX8MQ_CLK_VPU_BUS: i32 = 106;
/* DISP_AXI */
pub const IMX8MQ_CLK_DISP_AXI: i32 = 107;
/* DISP APB */
pub const IMX8MQ_CLK_DISP_APB: i32 = 108;
/* DISP RTRM */
pub const IMX8MQ_CLK_DISP_RTRM: i32 = 109;
/* USB_BUS */
pub const IMX8MQ_CLK_USB_BUS: i32 = 110;
/* GPU_AXI */
pub const IMX8MQ_CLK_GPU_AXI: i32 = 111;
/* GPU_AHB */
pub const IMX8MQ_CLK_GPU_AHB: i32 = 112;
/* NOC */
pub const IMX8MQ_CLK_NOC: i32 = 113;
/* NOC_APB: C header value 115 intentionally leaves 114 unused. */
pub const IMX8MQ_CLK_NOC_APB: i32 = 115;
/* AHB */
pub const IMX8MQ_CLK_AHB: i32 = 116;
/* AUDIO AHB */
pub const IMX8MQ_CLK_AUDIO_AHB: i32 = 117;
/* DRAM_ALT */
pub const IMX8MQ_CLK_DRAM_ALT: i32 = 118;
/* DRAM APB */
pub const IMX8MQ_CLK_DRAM_APB: i32 = 119;
/* VPU_G1 */
pub const IMX8MQ_CLK_VPU_G1: i32 = 120;
/* VPU_G2 */
pub const IMX8MQ_CLK_VPU_G2: i32 = 121;
/* DISP_DTRC */
pub const IMX8MQ_CLK_DISP_DTRC: i32 = 122;
/* DISP_DC8000 */
pub const IMX8MQ_CLK_DISP_DC8000: i32 = 123;
/* PCIE_CTRL */
pub const IMX8MQ_CLK_PCIE1_CTRL: i32 = 124;
/* PCIE_PHY */
pub const IMX8MQ_CLK_PCIE1_PHY: i32 = 125;
/* PCIE_AUX */
pub const IMX8MQ_CLK_PCIE1_AUX: i32 = 126;
/* DC_PIXEL */
pub const IMX8MQ_CLK_DC_PIXEL: i32 = 127;
/* LCDIF_PIXEL */
pub const IMX8MQ_CLK_LCDIF_PIXEL: i32 = 128;
/* SAI1~6 */
pub const IMX8MQ_CLK_SAI1: i32 = 129;
pub const IMX8MQ_CLK_SAI2: i32 = 130;
pub const IMX8MQ_CLK_SAI3: i32 = 131;
pub const IMX8MQ_CLK_SAI4: i32 = 132;
pub const IMX8MQ_CLK_SAI5: i32 = 133;
pub const IMX8MQ_CLK_SAI6: i32 = 134;
/* SPDIF1 */
pub const IMX8MQ_CLK_SPDIF1: i32 = 135;
/* SPDIF2 */
pub const IMX8MQ_CLK_SPDIF2: i32 = 136;
/* ENET_REF */
pub const IMX8MQ_CLK_ENET_REF: i32 = 137;
/* ENET_TIMER */
pub const IMX8MQ_CLK_ENET_TIMER: i32 = 138;
/* ENET_PHY */
pub const IMX8MQ_CLK_ENET_PHY_REF: i32 = 139;
/* NAND */
pub const IMX8MQ_CLK_NAND: i32 = 140;
/* QSPI */
pub const IMX8MQ_CLK_QSPI: i32 = 141;
/* USDHC1 */
pub const IMX8MQ_CLK_USDHC1: i32 = 142;
/* USDHC2 */
pub const IMX8MQ_CLK_USDHC2: i32 = 143;
/* I2C1 */
pub const IMX8MQ_CLK_I2C1: i32 = 144;
/* I2C2 */
pub const IMX8MQ_CLK_I2C2: i32 = 145;
/* I2C3 */
pub const IMX8MQ_CLK_I2C3: i32 = 146;
/* I2C4 */
pub const IMX8MQ_CLK_I2C4: i32 = 147;
/* UART1 */
pub const IMX8MQ_CLK_UART1: i32 = 148;
/* UART2 */
pub const IMX8MQ_CLK_UART2: i32 = 149;
/* UART3 */
pub const IMX8MQ_CLK_UART3: i32 = 150;
/* UART4 */
pub const IMX8MQ_CLK_UART4: i32 = 151;
/* USB_CORE_REF */
pub const IMX8MQ_CLK_USB_CORE_REF: i32 = 152;
/* USB_PHY_REF */
pub const IMX8MQ_CLK_USB_PHY_REF: i32 = 153;
/* ECSPI1 */
pub const IMX8MQ_CLK_ECSPI1: i32 = 154;
/* ECSPI2 */
pub const IMX8MQ_CLK_ECSPI2: i32 = 155;
/* PWM1 */
pub const IMX8MQ_CLK_PWM1: i32 = 156;
/* PWM2 */
pub const IMX8MQ_CLK_PWM2: i32 = 157;
/* PWM3 */
pub const IMX8MQ_CLK_PWM3: i32 = 158;
/* PWM4 */
pub const IMX8MQ_CLK_PWM4: i32 = 159;
/* GPT1 */
pub const IMX8MQ_CLK_GPT1: i32 = 160;
/* WDOG */
pub const IMX8MQ_CLK_WDOG: i32 = 161;
/* WRCLK */
pub const IMX8MQ_CLK_WRCLK: i32 = 162;
/* DSI_CORE */
pub const IMX8MQ_CLK_DSI_CORE: i32 = 163;
/* DSI_PHY */
pub const IMX8MQ_CLK_DSI_PHY_REF: i32 = 164;
/* DSI_DBI */
pub const IMX8MQ_CLK_DSI_DBI: i32 = 165;
/*DSI_ESC */
pub const IMX8MQ_CLK_DSI_ESC: i32 = 166;
/* CSI1_CORE */
pub const IMX8MQ_CLK_CSI1_CORE: i32 = 167;
/* CSI1_PHY */
pub const IMX8MQ_CLK_CSI1_PHY_REF: i32 = 168;
/* CSI_ESC */
pub const IMX8MQ_CLK_CSI1_ESC: i32 = 169;
/* CSI2_CORE */
pub const IMX8MQ_CLK_CSI2_CORE: i32 = 170;
/* CSI2_PHY */
pub const IMX8MQ_CLK_CSI2_PHY_REF: i32 = 171;
/* CSI2_ESC */
pub const IMX8MQ_CLK_CSI2_ESC: i32 = 172;
/* PCIE2_CTRL */
pub const IMX8MQ_CLK_PCIE2_CTRL: i32 = 173;
/* PCIE2_PHY */
pub const IMX8MQ_CLK_PCIE2_PHY: i32 = 174;
/* PCIE2_AUX */
pub const IMX8MQ_CLK_PCIE2_AUX: i32 = 175;
/* ECSPI3 */
pub const IMX8MQ_CLK_ECSPI3: i32 = 176;
/* CCGR clocks */
pub const IMX8MQ_CLK_A53_ROOT: i32 = 177;
pub const IMX8MQ_CLK_DRAM_ROOT: i32 = 178;
pub const IMX8MQ_CLK_ECSPI1_ROOT: i32 = 179;
pub const IMX8MQ_CLK_ECSPI2_ROOT: i32 = 180;
pub const IMX8MQ_CLK_ECSPI3_ROOT: i32 = 181;
pub const IMX8MQ_CLK_ENET1_ROOT: i32 = 182;
pub const IMX8MQ_CLK_GPT1_ROOT: i32 = 183;
pub const IMX8MQ_CLK_I2C1_ROOT: i32 = 184;
pub const IMX8MQ_CLK_I2C2_ROOT: i32 = 185;
pub const IMX8MQ_CLK_I2C3_ROOT: i32 = 186;
pub const IMX8MQ_CLK_I2C4_ROOT: i32 = 187;
pub const IMX8MQ_CLK_M4_ROOT: i32 = 188;
pub const IMX8MQ_CLK_PCIE1_ROOT: i32 = 189;
pub const IMX8MQ_CLK_PCIE2_ROOT: i32 = 190;
pub const IMX8MQ_CLK_PWM1_ROOT: i32 = 191;
pub const IMX8MQ_CLK_PWM2_ROOT: i32 = 192;
pub const IMX8MQ_CLK_PWM3_ROOT: i32 = 193;
pub const IMX8MQ_CLK_PWM4_ROOT: i32 = 194;
pub const IMX8MQ_CLK_QSPI_ROOT: i32 = 195;
pub const IMX8MQ_CLK_SAI1_ROOT: i32 = 196;
pub const IMX8MQ_CLK_SAI2_ROOT: i32 = 197;
pub const IMX8MQ_CLK_SAI3_ROOT: i32 = 198;
pub const IMX8MQ_CLK_SAI4_ROOT: i32 = 199;
pub const IMX8MQ_CLK_SAI5_ROOT: i32 = 200;
pub const IMX8MQ_CLK_SAI6_ROOT: i32 = 201;
pub const IMX8MQ_CLK_UART1_ROOT: i32 = 202;
pub const IMX8MQ_CLK_UART2_ROOT: i32 = 203;
pub const IMX8MQ_CLK_UART3_ROOT: i32 = 204;
pub const IMX8MQ_CLK_UART4_ROOT: i32 = 205;
pub const IMX8MQ_CLK_USB1_CTRL_ROOT: i32 = 206;
pub const IMX8MQ_CLK_USB2_CTRL_ROOT: i32 = 207;
pub const IMX8MQ_CLK_USB1_PHY_ROOT: i32 = 208;
pub const IMX8MQ_CLK_USB2_PHY_ROOT: i32 = 209;
pub const IMX8MQ_CLK_USDHC1_ROOT: i32 = 210;
pub const IMX8MQ_CLK_USDHC2_ROOT: i32 = 211;
pub const IMX8MQ_CLK_WDOG1_ROOT: i32 = 212;
pub const IMX8MQ_CLK_WDOG2_ROOT: i32 = 213;
pub const IMX8MQ_CLK_WDOG3_ROOT: i32 = 214;
pub const IMX8MQ_CLK_GPU_ROOT: i32 = 215;
pub const IMX8MQ_CLK_HEVC_ROOT: i32 = 216;
pub const IMX8MQ_CLK_AVC_ROOT: i32 = 217;
pub const IMX8MQ_CLK_VP9_ROOT: i32 = 218;
pub const IMX8MQ_CLK_HEVC_INTER_ROOT: i32 = 219;
pub const IMX8MQ_CLK_DISP_ROOT: i32 = 220;
pub const IMX8MQ_CLK_HDMI_ROOT: i32 = 221;
pub const IMX8MQ_CLK_HDMI_PHY_ROOT: i32 = 222;
pub const IMX8MQ_CLK_VPU_DEC_ROOT: i32 = 223;
pub const IMX8MQ_CLK_CSI1_ROOT: i32 = 224;
pub const IMX8MQ_CLK_CSI2_ROOT: i32 = 225;
pub const IMX8MQ_CLK_RAWNAND_ROOT: i32 = 226;
pub const IMX8MQ_CLK_SDMA1_ROOT: i32 = 227;
pub const IMX8MQ_CLK_SDMA2_ROOT: i32 = 228;
pub const IMX8MQ_CLK_VPU_G1_ROOT: i32 = 229;
pub const IMX8MQ_CLK_VPU_G2_ROOT: i32 = 230;
/* SCCG PLL GATE */
pub const IMX8MQ_SYS1_PLL_OUT: i32 = 231;
pub const IMX8MQ_SYS2_PLL_OUT: i32 = 232;
pub const IMX8MQ_SYS3_PLL_OUT: i32 = 233;
pub const IMX8MQ_DRAM_PLL_OUT: i32 = 234;
pub const IMX8MQ_GPT_3M_CLK: i32 = 235;
pub const IMX8MQ_CLK_IPG_ROOT: i32 = 236;
pub const IMX8MQ_CLK_IPG_AUDIO_ROOT: i32 = 237;
pub const IMX8MQ_CLK_SAI1_IPG: i32 = 238;
pub const IMX8MQ_CLK_SAI2_IPG: i32 = 239;
pub const IMX8MQ_CLK_SAI3_IPG: i32 = 240;
pub const IMX8MQ_CLK_SAI4_IPG: i32 = 241;
pub const IMX8MQ_CLK_SAI5_IPG: i32 = 242;
pub const IMX8MQ_CLK_SAI6_IPG: i32 = 243;
/* DSI AHB/IPG clocks */
/* rxesc clock */
pub const IMX8MQ_CLK_DSI_AHB: i32 = 244;
/* txesc clock */
pub const IMX8MQ_CLK_DSI_IPG_DIV: i32 = 245;
pub const IMX8MQ_CLK_TMU_ROOT: i32 = 246;
/* Display root clocks */
pub const IMX8MQ_CLK_DISP_AXI_ROOT: i32 = 247;
pub const IMX8MQ_CLK_DISP_APB_ROOT: i32 = 248;
pub const IMX8MQ_CLK_DISP_RTRM_ROOT: i32 = 249;
pub const IMX8MQ_CLK_OCOTP_ROOT: i32 = 250;
pub const IMX8MQ_CLK_DRAM_ALT_ROOT: i32 = 251;
pub const IMX8MQ_CLK_DRAM_CORE: i32 = 252;
pub const IMX8MQ_CLK_MU_ROOT: i32 = 253;
pub const IMX8MQ_VIDEO2_PLL_OUT: i32 = 254;
pub const IMX8MQ_CLK_CLKO2: i32 = 255;
pub const IMX8MQ_CLK_NAND_USDHC_BUS_RAWNAND_CLK: i32 = 256;
pub const IMX8MQ_CLK_CLKO1: i32 = 257;
pub const IMX8MQ_CLK_ARM: i32 = 258;
pub const IMX8MQ_CLK_GPIO1_ROOT: i32 = 259;
pub const IMX8MQ_CLK_GPIO2_ROOT: i32 = 260;
pub const IMX8MQ_CLK_GPIO3_ROOT: i32 = 261;
pub const IMX8MQ_CLK_GPIO4_ROOT: i32 = 262;
pub const IMX8MQ_CLK_GPIO5_ROOT: i32 = 263;
pub const IMX8MQ_CLK_SNVS_ROOT: i32 = 264;
pub const IMX8MQ_CLK_GIC: i32 = 265;
pub const IMX8MQ_VIDEO2_PLL1_REF_SEL: i32 = 266;
pub const IMX8MQ_CLK_GPU_CORE: i32 = 285;
pub const IMX8MQ_CLK_GPU_SHADER: i32 = 286;
pub const IMX8MQ_CLK_M4_CORE: i32 = 287;
pub const IMX8MQ_CLK_VPU_CORE: i32 = 288;
pub const IMX8MQ_CLK_A53_CORE: i32 = 289;
pub const IMX8MQ_CLK_MON_AUDIO_PLL1_DIV: i32 = 290;
pub const IMX8MQ_CLK_MON_AUDIO_PLL2_DIV: i32 = 291;
pub const IMX8MQ_CLK_MON_VIDEO_PLL1_DIV: i32 = 292;
pub const IMX8MQ_CLK_MON_GPU_PLL_DIV: i32 = 293;
pub const IMX8MQ_CLK_MON_VPU_PLL_DIV: i32 = 294;
pub const IMX8MQ_CLK_MON_ARM_PLL_DIV: i32 = 295;
pub const IMX8MQ_CLK_MON_SYS_PLL1_DIV: i32 = 296;
pub const IMX8MQ_CLK_MON_SYS_PLL2_DIV: i32 = 297;
pub const IMX8MQ_CLK_MON_SYS_PLL3_DIV: i32 = 298;
pub const IMX8MQ_CLK_MON_DRAM_PLL_DIV: i32 = 299;
pub const IMX8MQ_CLK_MON_VIDEO_PLL2_DIV: i32 = 300;
pub const IMX8MQ_CLK_MON_SEL: i32 = 301;
pub const IMX8MQ_CLK_MON_CLK2_OUT: i32 = 302;
pub const IMX8MQ_CLK_END: i32 = 303;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
