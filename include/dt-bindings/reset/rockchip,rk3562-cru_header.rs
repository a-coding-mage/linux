/* SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause) */
/*
 * Copyright (c) 2024-2025 Rockchip Electronics Co. Ltd.
 *
 * Author: Elaine Zhang <zhangqing@rock-chips.com>
 */

/* Name=SOFTRST_CON01,Offset=0x404 */
pub const SRST_A_TOP_BIU: u32 = 0;
pub const SRST_A_TOP_VIO_BIU: u32 = 1;
pub const SRST_REF_PVTPLL_LOGIC: u32 = 2;
/* Name=SOFTRST_CON03,Offset=0x40C */
pub const SRST_NCOREPORESET0: u32 = 3;
pub const SRST_NCOREPORESET1: u32 = 4;
pub const SRST_NCOREPORESET2: u32 = 5;
pub const SRST_NCOREPORESET3: u32 = 6;
pub const SRST_NCORESET0: u32 = 7;
pub const SRST_NCORESET1: u32 = 8;
pub const SRST_NCORESET2: u32 = 9;
pub const SRST_NCORESET3: u32 = 10;
pub const SRST_NL2RESET: u32 = 11;
/* Name=SOFTRST_CON04,Offset=0x410 */
pub const SRST_DAP: u32 = 12;
pub const SRST_P_DBG_DAPLITE: u32 = 13;
pub const SRST_REF_PVTPLL_CORE: u32 = 14;
/* Name=SOFTRST_CON05,Offset=0x414 */
pub const SRST_A_CORE_BIU: u32 = 15;
pub const SRST_P_CORE_BIU: u32 = 16;
pub const SRST_H_CORE_BIU: u32 = 17;
/* Name=SOFTRST_CON06,Offset=0x418 */
pub const SRST_A_NPU_BIU: u32 = 18;
pub const SRST_H_NPU_BIU: u32 = 19;
pub const SRST_A_RKNN: u32 = 20;
pub const SRST_H_RKNN: u32 = 21;
pub const SRST_REF_PVTPLL_NPU: u32 = 22;
/* Name=SOFTRST_CON08,Offset=0x420 */
pub const SRST_A_GPU_BIU: u32 = 23;
pub const SRST_GPU: u32 = 24;
pub const SRST_REF_PVTPLL_GPU: u32 = 25;
pub const SRST_GPU_BRG_BIU: u32 = 26;
/* Name=SOFTRST_CON09,Offset=0x424 */
pub const SRST_RKVENC_CORE: u32 = 27;
pub const SRST_A_VEPU_BIU: u32 = 28;
pub const SRST_H_VEPU_BIU: u32 = 29;
pub const SRST_A_RKVENC: u32 = 30;
pub const SRST_H_RKVENC: u32 = 31;
/* Name=SOFTRST_CON10,Offset=0x428 */
pub const SRST_RKVDEC_HEVC_CA: u32 = 32;
pub const SRST_A_VDPU_BIU: u32 = 33;
pub const SRST_H_VDPU_BIU: u32 = 34;
pub const SRST_A_RKVDEC: u32 = 35;
pub const SRST_H_RKVDEC: u32 = 36;
/* Name=SOFTRST_CON11,Offset=0x42C */
pub const SRST_A_VI_BIU: u32 = 37;
pub const SRST_H_VI_BIU: u32 = 38;
pub const SRST_P_VI_BIU: u32 = 39;
pub const SRST_ISP: u32 = 40;
pub const SRST_A_VICAP: u32 = 41;
pub const SRST_H_VICAP: u32 = 42;
pub const SRST_D_VICAP: u32 = 43;
pub const SRST_I0_VICAP: u32 = 44;
pub const SRST_I1_VICAP: u32 = 45;
pub const SRST_I2_VICAP: u32 = 46;
pub const SRST_I3_VICAP: u32 = 47;
/* Name=SOFTRST_CON12,Offset=0x430 */
pub const SRST_P_CSIHOST0: u32 = 48;
pub const SRST_P_CSIHOST1: u32 = 49;
pub const SRST_P_CSIHOST2: u32 = 50;
pub const SRST_P_CSIHOST3: u32 = 51;
pub const SRST_P_CSIPHY0: u32 = 52;
pub const SRST_P_CSIPHY1: u32 = 53;
/* Name=SOFTRST_CON13,Offset=0x434 */
pub const SRST_A_VO_BIU: u32 = 54;
pub const SRST_H_VO_BIU: u32 = 55;
pub const SRST_A_VOP: u32 = 56;
pub const SRST_H_VOP: u32 = 57;
pub const SRST_D_VOP: u32 = 58;
pub const SRST_D_VOP1: u32 = 59;
/* Name=SOFTRST_CON14,Offset=0x438 */
pub const SRST_A_RGA_BIU: u32 = 60;
pub const SRST_H_RGA_BIU: u32 = 61;
pub const SRST_A_RGA: u32 = 62;
pub const SRST_H_RGA: u32 = 63;
pub const SRST_RGA_CORE: u32 = 64;
pub const SRST_A_JDEC: u32 = 65;
pub const SRST_H_JDEC: u32 = 66;
/* Name=SOFTRST_CON15,Offset=0x43C */
pub const SRST_B_EBK_BIU: u32 = 67;
pub const SRST_P_EBK_BIU: u32 = 68;
pub const SRST_AHB2AXI_EBC: u32 = 69;
pub const SRST_H_EBC: u32 = 70;
pub const SRST_D_EBC: u32 = 71;
pub const SRST_H_EINK: u32 = 72;
pub const SRST_P_EINK: u32 = 73;
/* Name=SOFTRST_CON16,Offset=0x440 */
pub const SRST_P_PHP_BIU: u32 = 74;
pub const SRST_A_PHP_BIU: u32 = 75;
pub const SRST_P_PCIE20: u32 = 76;
pub const SRST_PCIE20_POWERUP: u32 = 77;
pub const SRST_USB3OTG: u32 = 78;
/* Name=SOFTRST_CON17,Offset=0x444 */
pub const SRST_PIPEPHY: u32 = 79;
/* Name=SOFTRST_CON18,Offset=0x448 */
pub const SRST_A_BUS_BIU: u32 = 80;
pub const SRST_H_BUS_BIU: u32 = 81;
pub const SRST_P_BUS_BIU: u32 = 82;
/* Name=SOFTRST_CON19,Offset=0x44C */
pub const SRST_P_I2C1: u32 = 83;
pub const SRST_P_I2C2: u32 = 84;
pub const SRST_P_I2C3: u32 = 85;
pub const SRST_P_I2C4: u32 = 86;
pub const SRST_P_I2C5: u32 = 87;
pub const SRST_I2C1: u32 = 88;
pub const SRST_I2C2: u32 = 89;
pub const SRST_I2C3: u32 = 90;
pub const SRST_I2C4: u32 = 91;
pub const SRST_I2C5: u32 = 92;
/* Name=SOFTRST_CON20,Offset=0x450 */
pub const SRST_BUS_GPIO3: u32 = 93;
pub const SRST_BUS_GPIO4: u32 = 94;
/* Name=SOFTRST_CON21,Offset=0x454 */
pub const SRST_P_TIMER: u32 = 95;
pub const SRST_TIMER0: u32 = 96;
pub const SRST_TIMER1: u32 = 97;
pub const SRST_TIMER2: u32 = 98;
pub const SRST_TIMER3: u32 = 99;
pub const SRST_TIMER4: u32 = 100;
pub const SRST_TIMER5: u32 = 101;
pub const SRST_P_STIMER: u32 = 102;
pub const SRST_STIMER0: u32 = 103;
pub const SRST_STIMER1: u32 = 104;
/* Name=SOFTRST_CON22,Offset=0x458 */
pub const SRST_P_WDTNS: u32 = 105;
pub const SRST_WDTNS: u32 = 106;
pub const SRST_P_GRF: u32 = 107;
pub const SRST_P_SGRF: u32 = 108;
pub const SRST_P_MAILBOX: u32 = 109;
pub const SRST_P_INTC: u32 = 110;
pub const SRST_A_BUS_GIC400: u32 = 111;
pub const SRST_A_BUS_GIC400_DEBUG: u32 = 112;
/* Name=SOFTRST_CON23,Offset=0x45C */
pub const SRST_A_BUS_SPINLOCK: u32 = 113;
pub const SRST_A_DCF: u32 = 114;
pub const SRST_P_DCF: u32 = 115;
pub const SRST_F_BUS_CM0_CORE: u32 = 116;
pub const SRST_T_BUS_CM0_JTAG: u32 = 117;
pub const SRST_H_ICACHE: u32 = 118;
pub const SRST_H_DCACHE: u32 = 119;
/* Name=SOFTRST_CON24,Offset=0x460 */
pub const SRST_P_TSADC: u32 = 120;
pub const SRST_TSADC: u32 = 121;
pub const SRST_TSADCPHY: u32 = 122;
pub const SRST_P_DFT2APB: u32 = 123;
/* Name=SOFTRST_CON25,Offset=0x464 */
pub const SRST_A_GMAC: u32 = 124;
pub const SRST_P_APB2ASB_VCCIO156: u32 = 125;
pub const SRST_P_DSIPHY: u32 = 126;
pub const SRST_P_DSITX: u32 = 127;
pub const SRST_P_CPU_EMA_DET: u32 = 128;
pub const SRST_P_HASH: u32 = 129;
pub const SRST_P_TOPCRU: u32 = 130;
/* Name=SOFTRST_CON26,Offset=0x468 */
pub const SRST_P_ASB2APB_VCCIO156: u32 = 131;
pub const SRST_P_IOC_VCCIO156: u32 = 132;
pub const SRST_P_GPIO3_VCCIO156: u32 = 133;
pub const SRST_P_GPIO4_VCCIO156: u32 = 134;
pub const SRST_P_SARADC_VCCIO156: u32 = 135;
pub const SRST_SARADC_VCCIO156: u32 = 136;
pub const SRST_SARADC_VCCIO156_PHY: u32 = 137;
/* Name=SOFTRST_CON27,Offset=0x46c */
pub const SRST_A_MAC100: u32 = 138;

/* Name=PMU0SOFTRST_CON00,Offset=0x10200 */
pub const SRST_P_PMU0_CRU: u32 = 139;
pub const SRST_P_PMU0_PMU: u32 = 140;
pub const SRST_PMU0_PMU: u32 = 141;
pub const SRST_P_PMU0_HP_TIMER: u32 = 142;
pub const SRST_PMU0_HP_TIMER: u32 = 143;
pub const SRST_PMU0_32K_HP_TIMER: u32 = 144;
pub const SRST_P_PMU0_PVTM: u32 = 145;
pub const SRST_PMU0_PVTM: u32 = 146;
pub const SRST_P_IOC_PMUIO: u32 = 147;
pub const SRST_P_PMU0_GPIO0: u32 = 148;
pub const SRST_PMU0_GPIO0: u32 = 149;
pub const SRST_P_PMU0_GRF: u32 = 150;
pub const SRST_P_PMU0_SGRF: u32 = 151;
/* Name=PMU0SOFTRST_CON01,Offset=0x10204 */
pub const SRST_DDR_FAIL_SAFE: u32 = 152;
pub const SRST_P_PMU0_SCRKEYGEN: u32 = 153;
/* Name=PMU0SOFTRST_CON02,Offset=0x10208 */
pub const SRST_P_PMU0_I2C0: u32 = 154;
pub const SRST_PMU0_I2C0: u32 = 155;

/* Name=PMU1SOFTRST_CON00,Offset=0x18200 */
pub const SRST_P_PMU1_CRU: u32 = 156;
pub const SRST_H_PMU1_MEM: u32 = 157;
pub const SRST_H_PMU1_BIU: u32 = 158;
pub const SRST_P_PMU1_BIU: u32 = 159;
pub const SRST_P_PMU1_UART0: u32 = 160;
pub const SRST_S_PMU1_UART0: u32 = 161;
/* Name=PMU1SOFTRST_CON01,Offset=0x18204 */
pub const SRST_P_PMU1_SPI0: u32 = 162;
pub const SRST_PMU1_SPI0: u32 = 163;
pub const SRST_P_PMU1_PWM0: u32 = 164;
pub const SRST_PMU1_PWM0: u32 = 165;
/* Name=PMU1SOFTRST_CON02,Offset=0x18208 */
pub const SRST_F_PMU1_CM0_CORE: u32 = 166;
pub const SRST_T_PMU1_CM0_JTAG: u32 = 167;
pub const SRST_P_PMU1_WDTNS: u32 = 168;
pub const SRST_PMU1_WDTNS: u32 = 169;
pub const SRST_PMU1_MAILBOX: u32 = 170;

/* Name=DDRSOFTRST_CON00,Offset=0x20200 */
pub const SRST_MSCH_BRG_BIU: u32 = 171;
pub const SRST_P_MSCH_BIU: u32 = 172;
pub const SRST_P_DDR_HWLP: u32 = 173;
pub const SRST_P_DDR_PHY: u32 = 290;
pub const SRST_P_DDR_DFICTL: u32 = 174;
pub const SRST_P_DDR_DMA2DDR: u32 = 175;
/* Name=DDRSOFTRST_CON01,Offset=0x20204 */
pub const SRST_P_DDR_MON: u32 = 176;
pub const SRST_TM_DDR_MON: u32 = 177;
pub const SRST_P_DDR_GRF: u32 = 178;
pub const SRST_P_DDR_CRU: u32 = 179;
pub const SRST_P_SUBDDR_CRU: u32 = 180;

/* Name=SUBDDRSOFTRST_CON00,Offset=0x28200 */
pub const SRST_MSCH_BIU: u32 = 181;
pub const SRST_DDR_PHY: u32 = 182;
pub const SRST_DDR_DFICTL: u32 = 183;
pub const SRST_DDR_SCRAMBLE: u32 = 184;
pub const SRST_DDR_MON: u32 = 185;
pub const SRST_A_DDR_SPLIT: u32 = 186;
pub const SRST_DDR_DMA2DDR: u32 = 187;

/* Name=PERISOFTRST_CON01,Offset=0x30404 */
pub const SRST_A_PERI_BIU: u32 = 188;
pub const SRST_H_PERI_BIU: u32 = 189;
pub const SRST_P_PERI_BIU: u32 = 190;
pub const SRST_P_PERICRU: u32 = 191;
/* Name=PERISOFTRST_CON02,Offset=0x30408 */
pub const SRST_H_SAI0_8CH: u32 = 192;
pub const SRST_M_SAI0_8CH: u32 = 193;
pub const SRST_H_SAI1_8CH: u32 = 194;
pub const SRST_M_SAI1_8CH: u32 = 195;
pub const SRST_H_SAI2_2CH: u32 = 196;
pub const SRST_M_SAI2_2CH: u32 = 197;
/* Name=PERISOFTRST_CON03,Offset=0x3040C */
pub const SRST_H_DSM: u32 = 198;
pub const SRST_DSM: u32 = 199;
pub const SRST_H_PDM: u32 = 200;
pub const SRST_M_PDM: u32 = 201;
pub const SRST_H_SPDIF: u32 = 202;
pub const SRST_M_SPDIF: u32 = 203;
/* Name=PERISOFTRST_CON04,Offset=0x30410 */
pub const SRST_H_SDMMC0: u32 = 204;
pub const SRST_H_SDMMC1: u32 = 205;
pub const SRST_H_EMMC: u32 = 206;
pub const SRST_A_EMMC: u32 = 207;
pub const SRST_C_EMMC: u32 = 208;
pub const SRST_B_EMMC: u32 = 209;
pub const SRST_T_EMMC: u32 = 210;
pub const SRST_S_SFC: u32 = 211;
pub const SRST_H_SFC: u32 = 212;
/* Name=PERISOFTRST_CON05,Offset=0x30414 */
pub const SRST_H_USB2HOST: u32 = 213;
pub const SRST_H_USB2HOST_ARB: u32 = 214;
pub const SRST_USB2HOST_UTMI: u32 = 215;
/* Name=PERISOFTRST_CON06,Offset=0x30418 */
pub const SRST_P_SPI1: u32 = 216;
pub const SRST_SPI1: u32 = 217;
pub const SRST_P_SPI2: u32 = 218;
pub const SRST_SPI2: u32 = 219;
/* Name=PERISOFTRST_CON07,Offset=0x3041C */
pub const SRST_P_UART1: u32 = 220;
pub const SRST_P_UART2: u32 = 221;
pub const SRST_P_UART3: u32 = 222;
pub const SRST_P_UART4: u32 = 223;
pub const SRST_P_UART5: u32 = 224;
pub const SRST_P_UART6: u32 = 225;
pub const SRST_P_UART7: u32 = 226;
pub const SRST_P_UART8: u32 = 227;
pub const SRST_P_UART9: u32 = 228;
pub const SRST_S_UART1: u32 = 229;
pub const SRST_S_UART2: u32 = 230;
/* Name=PERISOFTRST_CON08,Offset=0x30420 */
pub const SRST_S_UART3: u32 = 231;
pub const SRST_S_UART4: u32 = 232;
pub const SRST_S_UART5: u32 = 233;
pub const SRST_S_UART6: u32 = 234;
pub const SRST_S_UART7: u32 = 235;
/* Name=PERISOFTRST_CON09,Offset=0x30424 */
pub const SRST_S_UART8: u32 = 236;
pub const SRST_S_UART9: u32 = 237;
/* Name=PERISOFTRST_CON10,Offset=0x30428 */
pub const SRST_P_PWM1_PERI: u32 = 238;
pub const SRST_PWM1_PERI: u32 = 239;
pub const SRST_P_PWM2_PERI: u32 = 240;
pub const SRST_PWM2_PERI: u32 = 241;
pub const SRST_P_PWM3_PERI: u32 = 242;
pub const SRST_PWM3_PERI: u32 = 243;
/* Name=PERISOFTRST_CON11,Offset=0x3042C */
pub const SRST_P_CAN0: u32 = 244;
pub const SRST_CAN0: u32 = 245;
pub const SRST_P_CAN1: u32 = 246;
pub const SRST_CAN1: u32 = 247;
/* Name=PERISOFTRST_CON12,Offset=0x30430 */
pub const SRST_A_CRYPTO: u32 = 248;
pub const SRST_H_CRYPTO: u32 = 249;
pub const SRST_P_CRYPTO: u32 = 250;
pub const SRST_CORE_CRYPTO: u32 = 251;
pub const SRST_PKA_CRYPTO: u32 = 252;
pub const SRST_H_KLAD: u32 = 253;
pub const SRST_P_KEY_READER: u32 = 254;
pub const SRST_H_RK_RNG_NS: u32 = 255;
pub const SRST_H_RK_RNG_S: u32 = 256;
pub const SRST_H_TRNG_NS: u32 = 257;
pub const SRST_H_TRNG_S: u32 = 258;
pub const SRST_H_CRYPTO_S: u32 = 259;
/* Name=PERISOFTRST_CON13,Offset=0x30434 */
pub const SRST_P_PERI_WDT: u32 = 260;
pub const SRST_T_PERI_WDT: u32 = 261;
pub const SRST_A_SYSMEM: u32 = 262;
pub const SRST_H_BOOTROM: u32 = 263;
pub const SRST_P_PERI_GRF: u32 = 264;
pub const SRST_A_DMAC: u32 = 265;
pub const SRST_A_RKDMAC: u32 = 267;
/* Name=PERISOFTRST_CON14,Offset=0x30438 */
pub const SRST_P_OTPC_NS: u32 = 268;
pub const SRST_SBPI_OTPC_NS: u32 = 269;
pub const SRST_USER_OTPC_NS: u32 = 270;
pub const SRST_P_OTPC_S: u32 = 271;
pub const SRST_SBPI_OTPC_S: u32 = 272;
pub const SRST_USER_OTPC_S: u32 = 273;
pub const SRST_OTPC_ARB: u32 = 274;
pub const SRST_P_OTPPHY: u32 = 275;
pub const SRST_OTP_NPOR: u32 = 276;
/* Name=PERISOFTRST_CON15,Offset=0x3043C */
pub const SRST_P_USB2PHY: u32 = 277;
pub const SRST_USB2PHY_POR: u32 = 278;
pub const SRST_USB2PHY_OTG: u32 = 279;
pub const SRST_USB2PHY_HOST: u32 = 280;
pub const SRST_P_PIPEPHY: u32 = 281;
/* Name=PERISOFTRST_CON16,Offset=0x30440 */
pub const SRST_P_SARADC: u32 = 282;
pub const SRST_SARADC: u32 = 283;
pub const SRST_SARADC_PHY: u32 = 284;
pub const SRST_P_IOC_VCCIO234: u32 = 285;
/* Name=PERISOFTRST_CON17,Offset=0x30444 */
pub const SRST_P_PERI_GPIO1: u32 = 286;
pub const SRST_P_PERI_GPIO2: u32 = 287;
pub const SRST_PERI_GPIO1: u32 = 288;
pub const SRST_PERI_GPIO2: u32 = 289;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
