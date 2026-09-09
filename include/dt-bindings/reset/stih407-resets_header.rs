/* SPDX-License-Identifier: GPL-2.0 */
/*
 * This header provides constants for the reset controller
 * based peripheral powerdown requests on the STMicroelectronics
 * STiH407 SoC.
 */

/* Powerdown requests control 0 */
pub const STIH407_EMISS_POWERDOWN: i32 = 0;
pub const STIH407_NAND_POWERDOWN: i32 = 1;

/* Synp GMAC PowerDown */
pub const STIH407_ETH1_POWERDOWN: i32 = 2;

/* Powerdown requests control 1 */
pub const STIH407_USB3_POWERDOWN: i32 = 3;
pub const STIH407_USB2_PORT1_POWERDOWN: i32 = 4;
pub const STIH407_USB2_PORT0_POWERDOWN: i32 = 5;
pub const STIH407_PCIE1_POWERDOWN: i32 = 6;
pub const STIH407_PCIE0_POWERDOWN: i32 = 7;
pub const STIH407_SATA1_POWERDOWN: i32 = 8;
pub const STIH407_SATA0_POWERDOWN: i32 = 9;

/* Reset defines */
pub const STIH407_ETH1_SOFTRESET: i32 = 0;
pub const STIH407_MMC1_SOFTRESET: i32 = 1;
pub const STIH407_PICOPHY_SOFTRESET: i32 = 2;
pub const STIH407_IRB_SOFTRESET: i32 = 3;
pub const STIH407_PCIE0_SOFTRESET: i32 = 4;
pub const STIH407_PCIE1_SOFTRESET: i32 = 5;
pub const STIH407_SATA0_SOFTRESET: i32 = 6;
pub const STIH407_SATA1_SOFTRESET: i32 = 7;
pub const STIH407_MIPHY0_SOFTRESET: i32 = 8;
pub const STIH407_MIPHY1_SOFTRESET: i32 = 9;
pub const STIH407_MIPHY2_SOFTRESET: i32 = 10;
pub const STIH407_SATA0_PWR_SOFTRESET: i32 = 11;
pub const STIH407_SATA1_PWR_SOFTRESET: i32 = 12;
pub const STIH407_DELTA_SOFTRESET: i32 = 13;
pub const STIH407_BLITTER_SOFTRESET: i32 = 14;
pub const STIH407_HDTVOUT_SOFTRESET: i32 = 15;
pub const STIH407_HDQVDP_SOFTRESET: i32 = 16;
pub const STIH407_VDP_AUX_SOFTRESET: i32 = 17;
pub const STIH407_COMPO_SOFTRESET: i32 = 18;
pub const STIH407_HDMI_TX_PHY_SOFTRESET: i32 = 19;
pub const STIH407_JPEG_DEC_SOFTRESET: i32 = 20;
pub const STIH407_VP8_DEC_SOFTRESET: i32 = 21;
pub const STIH407_GPU_SOFTRESET: i32 = 22;
pub const STIH407_HVA_SOFTRESET: i32 = 23;
pub const STIH407_ERAM_HVA_SOFTRESET: i32 = 24;
pub const STIH407_LPM_SOFTRESET: i32 = 25;
pub const STIH407_KEYSCAN_SOFTRESET: i32 = 26;
pub const STIH407_USB2_PORT0_SOFTRESET: i32 = 27;
pub const STIH407_USB2_PORT1_SOFTRESET: i32 = 28;
pub const STIH407_ST231_AUD_SOFTRESET: i32 = 29;
pub const STIH407_ST231_DMU_SOFTRESET: i32 = 30;
pub const STIH407_ST231_GP0_SOFTRESET: i32 = 31;
pub const STIH407_ST231_GP1_SOFTRESET: i32 = 32;

/* Picophy reset defines */
pub const STIH407_PICOPHY0_RESET: i32 = 0;
pub const STIH407_PICOPHY1_RESET: i32 = 1;
pub const STIH407_PICOPHY2_RESET: i32 = 2;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
