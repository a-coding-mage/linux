/* SPDX-License-Identifier: GPL-2.0 */
/*
 * cs35l45.h - CS35L45 ALSA SoC audio driver
 *
 * Copyright 2019-2022 Cirrus Logic, Inc.
 *
 * Author: James Schulman <james.schulman@cirrus.com>
 *
 */

// C header dependencies: linux/pm_runtime.h, linux/regmap.h,
// linux/regulator/consumer.h, dt-bindings/sound/cs35l45.h, "wm_adsp.h".

use core::ffi::{c_char, c_void};

const fn BIT(n: u32) -> u32 {
    1u32 << n
}

const fn GENMASK(h: u32, l: u32) -> u32 {
    if h == 31 {
        u32::MAX << l
    } else {
        ((1u32 << (h + 1)) - 1) & !((1u32 << l) - 1)
    }
}

pub const CS35L45_DEVID: u32 = 0x00000000;
pub const CS35L45_REVID: u32 = 0x00000004;
pub const CS35L45_RELID: u32 = 0x0000000C;
pub const CS35L45_OTPID: u32 = 0x00000010;
pub const CS35L45_SFT_RESET: u32 = 0x00000020;
pub const CS35L45_GLOBAL_ENABLES: u32 = 0x00002014;
pub const CS35L45_BLOCK_ENABLES: u32 = 0x00002018;
pub const CS35L45_BLOCK_ENABLES2: u32 = 0x0000201C;
pub const CS35L45_ERROR_RELEASE: u32 = 0x00002034;
pub const CS35L45_SYNC_GPIO1: u32 = 0x00002430;
pub const CS35L45_INTB_GPIO2_MCLK_REF: u32 = 0x00002434;
pub const CS35L45_GPIO3: u32 = 0x00002438;
pub const CS35L45_PWRMGT_CTL: u32 = 0x00002900;
pub const CS35L45_WAKESRC_CTL: u32 = 0x00002904;
pub const CS35L45_WKI2C_CTL: u32 = 0x00002908;
pub const CS35L45_PWRMGT_STS: u32 = 0x0000290C;
pub const CS35L45_REFCLK_INPUT: u32 = 0x00002C04;
pub const CS35L45_GLOBAL_SAMPLE_RATE: u32 = 0x00002C0C;
pub const CS35L45_BOOST_CCM_CFG: u32 = 0x00003808;
pub const CS35L45_BOOST_DCM_CFG: u32 = 0x0000380C;
pub const CS35L45_BOOST_OV_CFG: u32 = 0x0000382C;
pub const CS35L45_ASP_ENABLES1: u32 = 0x00004800;
pub const CS35L45_ASP_CONTROL1: u32 = 0x00004804;
pub const CS35L45_ASP_CONTROL2: u32 = 0x00004808;
pub const CS35L45_ASP_CONTROL3: u32 = 0x0000480C;
pub const CS35L45_ASP_FRAME_CONTROL1: u32 = 0x00004810;
pub const CS35L45_ASP_FRAME_CONTROL2: u32 = 0x00004814;
pub const CS35L45_ASP_FRAME_CONTROL5: u32 = 0x00004820;
pub const CS35L45_ASP_DATA_CONTROL1: u32 = 0x00004830;
pub const CS35L45_ASP_DATA_CONTROL5: u32 = 0x00004840;
pub const CS35L45_DACPCM1_INPUT: u32 = 0x00004C00;
pub const CS35L45_ASPTX1_INPUT: u32 = 0x00004C20;
pub const CS35L45_ASPTX2_INPUT: u32 = 0x00004C24;
pub const CS35L45_ASPTX3_INPUT: u32 = 0x00004C28;
pub const CS35L45_ASPTX4_INPUT: u32 = 0x00004C2C;
pub const CS35L45_ASPTX5_INPUT: u32 = 0x00004C30;
pub const CS35L45_DSP1RX1_INPUT: u32 = 0x00004C40;
pub const CS35L45_DSP1RX2_INPUT: u32 = 0x00004C44;
pub const CS35L45_DSP1RX3_INPUT: u32 = 0x00004C48;
pub const CS35L45_DSP1RX4_INPUT: u32 = 0x00004C4C;
pub const CS35L45_DSP1RX5_INPUT: u32 = 0x00004C50;
pub const CS35L45_DSP1RX6_INPUT: u32 = 0x00004C54;
pub const CS35L45_DSP1RX7_INPUT: u32 = 0x00004C58;
pub const CS35L45_DSP1RX8_INPUT: u32 = 0x00004C5C;
pub const CS35L45_HVLV_CONFIG: u32 = 0x00006400;
pub const CS35L45_LDPM_CONFIG: u32 = 0x00006404;
pub const CS35L45_AMP_PCM_CONTROL: u32 = 0x00007000;
pub const CS35L45_AMP_PCM_HPF_TST: u32 = 0x00007004;
pub const CS35L45_AMP_GAIN: u32 = 0x00007800;
pub const CS35L45_IRQ1_CFG: u32 = 0x0000E000;
pub const CS35L45_IRQ1_STATUS: u32 = 0x0000E004;
pub const CS35L45_IRQ1_EINT_1: u32 = 0x0000E010;
pub const CS35L45_IRQ1_EINT_2: u32 = 0x0000E014;
pub const CS35L45_IRQ1_EINT_3: u32 = 0x0000E018;
pub const CS35L45_IRQ1_EINT_4: u32 = 0x0000E01C;
pub const CS35L45_IRQ1_EINT_5: u32 = 0x0000E020;
pub const CS35L45_IRQ1_EINT_7: u32 = 0x0000E028;
pub const CS35L45_IRQ1_EINT_8: u32 = 0x0000E02C;
pub const CS35L45_IRQ1_EINT_18: u32 = 0x0000E054;
pub const CS35L45_IRQ1_STS_1: u32 = 0x0000E090;
pub const CS35L45_IRQ1_STS_2: u32 = 0x0000E094;
pub const CS35L45_IRQ1_STS_3: u32 = 0x0000E098;
pub const CS35L45_IRQ1_STS_4: u32 = 0x0000E09C;
pub const CS35L45_IRQ1_STS_5: u32 = 0x0000E0A0;
pub const CS35L45_IRQ1_STS_7: u32 = 0x0000E0A8;
pub const CS35L45_IRQ1_STS_8: u32 = 0x0000E0AC;
pub const CS35L45_IRQ1_STS_18: u32 = 0x0000E0D4;
pub const CS35L45_IRQ1_MASK_1: u32 = 0x0000E110;
pub const CS35L45_IRQ1_MASK_2: u32 = 0x0000E114;
pub const CS35L45_IRQ1_MASK_3: u32 = 0x0000E118;
pub const CS35L45_IRQ1_MASK_4: u32 = 0x0000E11C;
pub const CS35L45_IRQ1_MASK_5: u32 = 0x0000E120;
pub const CS35L45_IRQ1_MASK_6: u32 = 0x0000E124;
pub const CS35L45_IRQ1_MASK_7: u32 = 0x0000E128;
pub const CS35L45_IRQ1_MASK_8: u32 = 0x0000E12C;
pub const CS35L45_IRQ1_MASK_9: u32 = 0x0000E130;
pub const CS35L45_IRQ1_MASK_10: u32 = 0x0000E134;
pub const CS35L45_IRQ1_MASK_11: u32 = 0x0000E138;
pub const CS35L45_IRQ1_MASK_12: u32 = 0x0000E13C;
pub const CS35L45_IRQ1_MASK_13: u32 = 0x0000E140;
pub const CS35L45_IRQ1_MASK_14: u32 = 0x0000E144;
pub const CS35L45_IRQ1_MASK_15: u32 = 0x0000E148;
pub const CS35L45_IRQ1_MASK_16: u32 = 0x0000E14C;
pub const CS35L45_IRQ1_MASK_17: u32 = 0x0000E150;
pub const CS35L45_IRQ1_MASK_18: u32 = 0x0000E154;
pub const CS35L45_GPIO_STATUS1: u32 = 0x0000F000;
pub const CS35L45_GPIO1_CTRL1: u32 = 0x0000F008;
pub const CS35L45_GPIO2_CTRL1: u32 = 0x0000F00C;
pub const CS35L45_GPIO3_CTRL1: u32 = 0x0000F010;
pub const CS35L45_DSP_MBOX_1: u32 = 0x00011000;
pub const CS35L45_DSP_MBOX_2: u32 = 0x00011004;
pub const CS35L45_DSP_VIRT1_MBOX_1: u32 = 0x00011020;
pub const CS35L45_DSP_VIRT1_MBOX_2: u32 = 0x00011024;
pub const CS35L45_DSP_VIRT1_MBOX_3: u32 = 0x00011028;
pub const CS35L45_DSP_VIRT1_MBOX_4: u32 = 0x0001102C;
pub const CS35L45_DSP_VIRT2_MBOX_1: u32 = 0x00011040;
pub const CS35L45_DSP_VIRT2_MBOX_2: u32 = 0x00011044;
pub const CS35L45_DSP_VIRT2_MBOX_3: u32 = 0x00011048;
pub const CS35L45_DSP_VIRT2_MBOX_4: u32 = 0x0001104C;
pub const CS35L45_DSP1_XMEM_PACK_0: u32 = 0x02000000;
pub const CS35L45_DSP1_XMEM_PACK_4607: u32 = 0x020047FC;
pub const CS35L45_DSP1_XMEM_UNPACK32_0: u32 = 0x02400000;
pub const CS35L45_DSP1_XMEM_UNPACK32_3071: u32 = 0x02402FFC;
pub const CS35L45_DSP1_SYS_ID: u32 = 0x025E0000;
pub const CS35L45_DSP1_XMEM_UNPACK24_0: u32 = 0x02800000;
pub const CS35L45_DSP1_XMEM_UNPACK24_6143: u32 = 0x02805FFC;
pub const CS35L45_DSP1_CLOCK_FREQ: u32 = 0x02B80000;
pub const CS35L45_DSP1_RX1_RATE: u32 = 0x02B80080;
pub const CS35L45_DSP1_RX2_RATE: u32 = 0x02B80088;
pub const CS35L45_DSP1_RX3_RATE: u32 = 0x02B80090;
pub const CS35L45_DSP1_RX4_RATE: u32 = 0x02B80098;
pub const CS35L45_DSP1_RX5_RATE: u32 = 0x02B800A0;
pub const CS35L45_DSP1_RX6_RATE: u32 = 0x02B800A8;
pub const CS35L45_DSP1_RX7_RATE: u32 = 0x02B800B0;
pub const CS35L45_DSP1_RX8_RATE: u32 = 0x02B800B8;
pub const CS35L45_DSP1_TX1_RATE: u32 = 0x02B80280;
pub const CS35L45_DSP1_TX2_RATE: u32 = 0x02B80288;
pub const CS35L45_DSP1_TX3_RATE: u32 = 0x02B80290;
pub const CS35L45_DSP1_TX4_RATE: u32 = 0x02B80298;
pub const CS35L45_DSP1_TX5_RATE: u32 = 0x02B802A0;
pub const CS35L45_DSP1_TX6_RATE: u32 = 0x02B802A8;
pub const CS35L45_DSP1_TX7_RATE: u32 = 0x02B802B0;
pub const CS35L45_DSP1_TX8_RATE: u32 = 0x02B802B8;
pub const CS35L45_DSP1_SCRATCH1: u32 = 0x02B805C0;
pub const CS35L45_DSP1_SCRATCH2: u32 = 0x02B805C8;
pub const CS35L45_DSP1_SCRATCH3: u32 = 0x02B805D0;
pub const CS35L45_DSP1_SCRATCH4: u32 = 0x02B805D8;
pub const CS35L45_DSP1_CCM_CORE_CONTROL: u32 = 0x02BC1000;
pub const CS35L45_DSP1_YMEM_PACK_0: u32 = 0x02C00000;
pub const CS35L45_DSP1_YMEM_PACK_1532: u32 = 0x02C017F0;
pub const CS35L45_DSP1_YMEM_UNPACK32_0: u32 = 0x03000000;
pub const CS35L45_DSP1_YMEM_UNPACK32_1022: u32 = 0x03000FF8;
pub const CS35L45_DSP1_YMEM_UNPACK24_0: u32 = 0x03400000;
pub const CS35L45_DSP1_YMEM_UNPACK24_2043: u32 = 0x03401FEC;
pub const CS35L45_DSP1_PMEM_0: u32 = 0x03800000;
pub const CS35L45_DSP1_PMEM_3834: u32 = 0x03803BE8;
pub const CS35L45_LASTREG: u32 = 0x03C6EFE8;

/* SFT_RESET */
pub const CS35L45_SOFT_RESET_TRIGGER: u32 = 0x5A000000;

/* GLOBAL_ENABLES */
pub const CS35L45_GLOBAL_EN_SHIFT: u32 = 0;
pub const CS35L45_GLOBAL_EN_MASK: u32 = BIT(0);

/* BLOCK_ENABLES */
pub const CS35L45_IMON_EN_SHIFT: u32 = 13;
pub const CS35L45_VMON_EN_SHIFT: u32 = 12;
pub const CS35L45_TEMPMON_EN_SHIFT: u32 = 10;
pub const CS35L45_VDD_BSTMON_EN_SHIFT: u32 = 9;
pub const CS35L45_VDD_BATTMON_EN_SHIFT: u32 = 8;
pub const CS35L45_BST_EN_SHIFT: u32 = 4;
pub const CS35L45_BST_EN_MASK: u32 = GENMASK(5, 4);
pub const CS35L45_RCV_EN_SHIFT: u32 = 2;
pub const CS35L45_RCV_EN_MASK: u32 = BIT(2);
pub const CS35L45_AMP_EN_SHIFT: u32 = 0;
pub const CS35L45_AMP_EN_MASK: u32 = BIT(0);

pub const CS35L45_BST_DISABLE_FET_OFF: u32 = 0x00;
pub const CS35L45_BST_DISABLE_FET_ON: u32 = 0x01;
pub const CS35L45_BST_ENABLE: u32 = 0x02;

/* BLOCK_ENABLES2 */
pub const CS35L45_ASP_EN_SHIFT: u32 = 27;
pub const CS35L45_AMP_DRE_EN_SHIFT: u32 = 20;
pub const CS35L45_AMP_DRE_EN_MASK: u32 = BIT(20);
pub const CS35L45_MEM_RDY_SHIFT: u32 = 1;
pub const CS35L45_MEM_RDY_MASK: u32 = BIT(1);

/* ERROR_RELEASE */
pub const CS35L45_GLOBAL_ERR_RLS_MASK: u32 = BIT(11);

/* CCM_CORE */
pub const CS35L45_CCM_CORE_RESET_SHIFT: u32 = 9;
pub const CS35L45_CCM_CORE_RESET_MASK: u32 = BIT(9);
pub const CS35L45_CCM_PM_REMAP_SHIFT: u32 = 7;
pub const CS35L45_CCM_PM_REMAP_MASK: u32 = BIT(7);
pub const CS35L45_CCM_CORE_EN_SHIFT: u32 = 0;
pub const CS35L45_CCM_CORE_EN_MASK: u32 = BIT(0);

/* REFCLK_INPUT */
pub const CS35L45_PLL_FORCE_EN_SHIFT: u32 = 16;
pub const CS35L45_PLL_FORCE_EN_MASK: u32 = BIT(16);
pub const CS35L45_PLL_OPEN_LOOP_SHIFT: u32 = 11;
pub const CS35L45_PLL_OPEN_LOOP_MASK: u32 = BIT(11);
pub const CS35L45_PLL_REFCLK_FREQ_SHIFT: u32 = 5;
pub const CS35L45_PLL_REFCLK_FREQ_MASK: u32 = GENMASK(10, 5);
pub const CS35L45_PLL_REFCLK_EN_SHIFT: u32 = 4;
pub const CS35L45_PLL_REFCLK_EN_MASK: u32 = BIT(4);
pub const CS35L45_PLL_REFCLK_SEL_SHIFT: u32 = 0;
pub const CS35L45_PLL_REFCLK_SEL_MASK: u32 = GENMASK(2, 0);

pub const CS35L45_PLL_REFCLK_SEL_BCLK: u32 = 0x0;

/* GLOBAL_SAMPLE_RATE */
pub const CS35L45_GLOBAL_FS_SHIFT: u32 = 0;
pub const CS35L45_GLOBAL_FS_MASK: u32 = GENMASK(4, 0);

pub const CS35L45_48P0_KHZ: u32 = 0x03;
pub const CS35L45_96P0_KHZ: u32 = 0x04;
pub const CS35L45_44P100_KHZ: u32 = 0x0B;
pub const CS35L45_88P200_KHZ: u32 = 0x0C;

/* ASP_ENABLES_1 */
pub const CS35L45_ASP_RX2_EN_SHIFT: u32 = 17;
pub const CS35L45_ASP_RX1_EN_SHIFT: u32 = 16;
pub const CS35L45_ASP_TX5_EN_SHIFT: u32 = 4;
pub const CS35L45_ASP_TX4_EN_SHIFT: u32 = 3;
pub const CS35L45_ASP_TX3_EN_SHIFT: u32 = 2;
pub const CS35L45_ASP_TX2_EN_SHIFT: u32 = 1;
pub const CS35L45_ASP_TX1_EN_SHIFT: u32 = 0;

/* ASP_CONTROL2 */
pub const CS35L45_ASP_WIDTH_RX_SHIFT: u32 = 24;
pub const CS35L45_ASP_WIDTH_RX_MASK: u32 = GENMASK(31, 24);
pub const CS35L45_ASP_WIDTH_TX_SHIFT: u32 = 16;
pub const CS35L45_ASP_WIDTH_TX_MASK: u32 = GENMASK(23, 16);
pub const CS35L45_ASP_FMT_SHIFT: u32 = 8;
pub const CS35L45_ASP_FMT_MASK: u32 = GENMASK(10, 8);
pub const CS35L45_ASP_BCLK_INV_SHIFT: u32 = 6;
pub const CS35L45_ASP_BCLK_INV_MASK: u32 = BIT(6);
pub const CS35L45_ASP_FSYNC_INV_SHIFT: u32 = 2;
pub const CS35L45_ASP_FSYNC_INV_MASK: u32 = BIT(2);

pub const CS35l45_ASP_FMT_DSP_A: u32 = 0;
pub const CS35L45_ASP_FMT_I2S: u32 = 2;

/* ASP_CONTROL3 */
pub const CS35L45_ASP_DOUT_HIZ_CTRL_SHIFT: u32 = 0;
pub const CS35L45_ASP_DOUT_HIZ_CTRL_MASK: u32 = GENMASK(1, 0);

/* ASP_FRAME_CONTROL1 */
pub const CS35L45_ASP_TX4_SLOT_SHIFT: u32 = 24;
pub const CS35L45_ASP_TX4_SLOT_MASK: u32 = GENMASK(29, 24);
pub const CS35L45_ASP_TX3_SLOT_SHIFT: u32 = 16;
pub const CS35L45_ASP_TX3_SLOT_MASK: u32 = GENMASK(21, 16);
pub const CS35L45_ASP_TX2_SLOT_SHIFT: u32 = 8;
pub const CS35L45_ASP_TX2_SLOT_MASK: u32 = GENMASK(13, 8);
pub const CS35L45_ASP_TX1_SLOT_SHIFT: u32 = 0;
pub const CS35L45_ASP_TX1_SLOT_MASK: u32 = GENMASK(5, 0);

pub const CS35L45_ASP_TX_ALL_SLOTS: u32 = CS35L45_ASP_TX4_SLOT_MASK
    | CS35L45_ASP_TX3_SLOT_MASK
    | CS35L45_ASP_TX2_SLOT_MASK
    | CS35L45_ASP_TX1_SLOT_MASK;

/* ASP_FRAME_CONTROL5 */
pub const CS35L45_ASP_RX2_SLOT_SHIFT: u32 = 8;
pub const CS35L45_ASP_RX2_SLOT_MASK: u32 = GENMASK(13, 8);
pub const CS35L45_ASP_RX1_SLOT_SHIFT: u32 = 0;
pub const CS35L45_ASP_RX1_SLOT_MASK: u32 = GENMASK(5, 0);

pub const CS35L45_ASP_RX_ALL_SLOTS: u32 =
    CS35L45_ASP_RX2_SLOT_MASK | CS35L45_ASP_RX1_SLOT_MASK;

/* ASP_DATA_CONTROL1 */
/* ASP_DATA_CONTROL5 */
pub const CS35L45_ASP_WL_SHIFT: u32 = 0;
pub const CS35L45_ASP_WL_MASK: u32 = GENMASK(5, 0);

/* HVLV_CONFIG */
pub const CS35L45_FORCE_LV_OPERATION: u32 = 0x01;
pub const CS35L45_FORCE_HV_OPERATION: u32 = 0x02;
pub const CS35L45_HVLV_OPERATION: u32 = 0x03;
pub const CS35L45_HVLV_MODE_SHIFT: u32 = 0;
pub const CS35L45_HVLV_MODE_MASK: u32 = GENMASK(1, 0);

/* AMP_PCM_CONTROL */
pub const CS35L45_AMP_VOL_PCM_SHIFT: u32 = 0;
pub const CS35L45_AMP_VOL_PCM_WIDTH: u32 = 11;

/* AMP_PCM_HPF_TST */
pub const CS35l45_HPF_DEFAULT: u32 = 0x00000000;
pub const CS35L45_HPF_44P1: u32 = 0x000108BD;
pub const CS35L45_HPF_88P2: u32 = 0x0001045F;

/* AMP_GAIN_PCM */
pub const CS35L45_AMP_GAIN_PCM_10DBV: u32 = 0x00;
pub const CS35L45_AMP_GAIN_PCM_13DBV: u32 = 0x01;
pub const CS35L45_AMP_GAIN_PCM_16DBV: u32 = 0x02;
pub const CS35L45_AMP_GAIN_PCM_19DBV: u32 = 0x03;

pub const CS35L45_AMP_GAIN_PCM_SHIFT: u32 = 8;
pub const CS35L45_AMP_GAIN_PCM_MASK: u32 = GENMASK(9, 8);

/* IRQ1_EINT_4 */
pub const CS35L45_OTP_BOOT_DONE_STS_MASK: u32 = BIT(1);
pub const CS35L45_OTP_BUSY_MASK: u32 = BIT(0);

/* GPIOX_CTRL1 */
pub const CS35L45_GPIO_DIR_SHIFT: u32 = 31;
pub const CS35L45_GPIO_DIR_MASK: u32 = BIT(31);
pub const CS35L45_GPIO_LVL_SHIFT: u32 = 15;
pub const CS35L45_GPIO_LVL_MASK: u32 = BIT(15);
pub const CS35L45_GPIO_OP_CFG_SHIFT: u32 = 14;
pub const CS35L45_GPIO_OP_CFG_MASK: u32 = BIT(14);
pub const CS35L45_GPIO_POL_SHIFT: u32 = 12;
pub const CS35L45_GPIO_POL_MASK: u32 = BIT(12);

/* SYNC_GPIO1, INTB_GPIO2_MCLK_REF, GPIO3 */
pub const CS35L45_GPIO_CTRL_SHIFT: u32 = 20;
pub const CS35L45_GPIO_CTRL_MASK: u32 = GENMASK(22, 20);
pub const CS35L45_GPIO_INVERT_SHIFT: u32 = 19;
pub const CS35L45_GPIO_INVERT_MASK: u32 = BIT(19);

/* CS35L45_IRQ1_EINT_1 */
pub const CS35L45_BST_UVP_ERR_SHIFT: u32 = 7;
pub const CS35L45_BST_UVP_ERR_MASK: u32 = BIT(7);
pub const CS35L45_BST_SHORT_ERR_SHIFT: u32 = 8;
pub const CS35L45_BST_SHORT_ERR_MASK: u32 = BIT(8);
pub const CS35L45_TEMP_ERR_SHIFT: u32 = 17;
pub const CS35L45_TEMP_ERR_MASK: u32 = BIT(17);
pub const CS35L45_MSM_GLOBAL_EN_ASSERT_SHIFT: u32 = 22;
pub const CS35L45_MSM_GLOBAL_EN_ASSERT_MASK: u32 = BIT(22);
pub const CS35L45_UVLO_VDDBATT_ERR_SHIFT: u32 = 29;
pub const CS35L45_UVLO_VDDBATT_ERR_MASK: u32 = BIT(29);
pub const CS35L45_AMP_SHORT_ERR_SHIFT: u32 = 31;
pub const CS35L45_AMP_SHORT_ERR_MASK: u32 = BIT(31);

/* CS35L45_IRQ1_EINT_2 */
pub const CS35L45_DSP_WDT_EXPIRE_SHIFT: u32 = 4;
pub const CS35L45_DSP_WDT_EXPIRE_MASK: u32 = BIT(4);
pub const CS35L45_DSP_VIRT2_MBOX_SHIFT: u32 = 21;
pub const CS35L45_DSP_VIRT2_MBOX_MASK: u32 = BIT(21);

/* CS35L45_IRQ1_EINT_3 */
pub const CS35L45_PLL_LOCK_FLAG_SHIFT: u32 = 1;
pub const CS35L45_PLL_LOCK_FLAG_MASK: u32 = BIT(1);
pub const CS35L45_PLL_UNLOCK_FLAG_RISE_SHIFT: u32 = 4;
pub const CS35L45_PLL_UNLOCK_FLAG_RISE_MASK: u32 = BIT(4);
pub const CS35L45_AMP_CAL_ERR_SHIFT: u32 = 25;
pub const CS35L45_AMP_CAL_ERR_MASK: u32 = BIT(25);

/* CS35L45_IRQ1_EINT_18 */
pub const CS35L45_GLOBAL_ERROR_SHIFT: u32 = 15;
pub const CS35L45_GLOBAL_ERROR_MASK: u32 = BIT(15);
pub const CS35L45_UVLO_VDDLV_ERR_SHIFT: u32 = 16;
pub const CS35L45_UVLO_VDDLV_ERR_MASK: u32 = BIT(16);

/* Mixer sources */
pub const CS35L45_PCM_SRC_MASK: u32 = 0x7F;
pub const CS35L45_PCM_SRC_ZERO: u32 = 0x00;
pub const CS35L45_PCM_SRC_ASP_RX1: u32 = 0x08;
pub const CS35L45_PCM_SRC_ASP_RX2: u32 = 0x09;
pub const CS35L45_PCM_SRC_VMON: u32 = 0x18;
pub const CS35L45_PCM_SRC_IMON: u32 = 0x19;
pub const CS35L45_PCM_SRC_ERR_VOL: u32 = 0x20;
pub const CS35L45_PCM_SRC_CLASSH_TGT: u32 = 0x21;
pub const CS35L45_PCM_SRC_VDD_BATTMON: u32 = 0x28;
pub const CS35L45_PCM_SRC_VDD_BSTMON: u32 = 0x29;
pub const CS35L45_PCM_SRC_DSP_TX1: u32 = 0x32;
pub const CS35L45_PCM_SRC_DSP_TX2: u32 = 0x33;
pub const CS35L45_PCM_SRC_TEMPMON: u32 = 0x3A;
pub const CS35L45_PCM_SRC_INTERPOLATOR: u32 = 0x40;
pub const CS35L45_PCM_SRC_IL_TARGET: u32 = 0x48;

pub const CS35L45_RESET_HOLD_US: u32 = 2000;
pub const CS35L45_RESET_US: u32 = 2000;
pub const CS35L45_POST_GLOBAL_EN_US: u32 = 5000;
pub const CS35L45_PRE_GLOBAL_DIS_US: u32 = 3000;

/* WAKESRC_CTL */
pub const CS35L45_WKSRC_SYNC_GPIO1: u32 = BIT(0);
pub const CS35L45_WKSRC_INT_GPIO2: u32 = BIT(1);
pub const CS35L45_WKSRC_GPIO3: u32 = BIT(2);
pub const CS35L45_WKSRC_SPI: u32 = BIT(3);
pub const CS35L45_WKSRC_I2C: u32 = BIT(4);
pub const CS35L45_UPDT_WKCTL_SHIFT: u32 = 15;
pub const CS35L45_UPDT_WKCTL_MASK: u32 = BIT(15);
pub const CS35L45_WKSRC_EN_SHIFT: u32 = 8;
pub const CS35L45_WKSRC_EN_MASK: u32 = GENMASK(12, 8);
pub const CS35L45_WKSRC_POL_SHIFT: u32 = 0;
pub const CS35L45_WKSRC_POL_MASK: u32 = GENMASK(3, 0);

/* WAKEI2C_CTL */
pub const CS35L45_UPDT_WKI2C_SHIFT: u32 = 15;
pub const CS35L45_UPDT_WKI2C_MASK: u32 = BIT(15);
pub const CS35L45_WKI2C_ADDR_SHIFT: u32 = 0;
pub const CS35L45_WKI2C_ADDR_MASK: u32 = GENMASK(6, 0);

pub const CS35L45_SPI_MAX_FREQ: u32 = 4000000;

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum cs35l45_cspl_mboxstate {
    CSPL_MBOX_STS_RUNNING = 0,
    CSPL_MBOX_STS_PAUSED = 1,
    CSPL_MBOX_STS_RDY_FOR_REINIT = 2,
    CSPL_MBOX_STS_HIBERNATE = 3,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum cs35l45_cspl_mboxcmd {
    CSPL_MBOX_CMD_NONE = 0,
    CSPL_MBOX_CMD_PAUSE = 1,
    CSPL_MBOX_CMD_RESUME = 2,
    CSPL_MBOX_CMD_REINIT = 3,
    CSPL_MBOX_CMD_STOP_PRE_REINIT = 4,
    CSPL_MBOX_CMD_HIBERNATE = 5,
    CSPL_MBOX_CMD_OUT_OF_HIBERNATE = 6,
    CSPL_MBOX_CMD_UNKNOWN_CMD = -1,
    CSPL_MBOX_CMD_INVALID_SEQUENCE = -2,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum control_bus_type {
    CONTROL_BUS_I2C = 0,
    CONTROL_BUS_SPI = 1,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum amp_mode {
    AMP_MODE_SPK = 0,
    AMP_MODE_RCV = 1,
}

pub const CS35L45_FORMATS: u32 =
    SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_3LE | SNDRV_PCM_FMTBIT_S24_LE;

pub const CS35L45_RATES: u32 =
    SNDRV_PCM_RATE_44100 | SNDRV_PCM_RATE_48000 | SNDRV_PCM_RATE_88200 | SNDRV_PCM_RATE_96000;

/*
 * IRQs
 */
#[repr(C)]
pub struct cs35l45_irq {
    pub irq: i32,
    pub name: *const c_char,
    pub handler: Option<unsafe extern "C" fn(irq: i32, data: *mut c_void) -> irqreturn_t>,
}

// C macro CS35L45_IRQ(_irq, _name, _hand) expands to:
// { .irq = CS35L45_ ## _irq ## _IRQ, .name = _name, .handler = _hand }
// Rust macro_rules! cannot directly reproduce C token pasting without an
// external helper, so the initializer intent is preserved here.

// C macro CS35L45_REG_IRQ(_reg, _irq) expands to an indexed regmap IRQ entry:
// [CS35L45_ ## _irq ## _IRQ] = {
//     .reg_offset = (CS35L45_ ## _reg) - CS35L45_IRQ1_EINT_1,
//     .mask = CS35L45_ ## _irq ## _MASK
// }
// Rust macro_rules! cannot directly reproduce C token pasting without an
// external helper, so the table-entry intent is preserved here.

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum cs35l45_irq_list {
    CS35L45_AMP_SHORT_ERR_IRQ,
    CS35L45_UVLO_VDDBATT_ERR_IRQ,
    CS35L45_BST_SHORT_ERR_IRQ,
    CS35L45_BST_UVP_ERR_IRQ,
    CS35L45_TEMP_ERR_IRQ,
    CS35L45_AMP_CAL_ERR_IRQ,
    CS35L45_UVLO_VDDLV_ERR_IRQ,
    CS35L45_GLOBAL_ERROR_IRQ,
    CS35L45_DSP_WDT_EXPIRE_IRQ,
    CS35L45_PLL_UNLOCK_FLAG_RISE_IRQ,
    CS35L45_PLL_LOCK_FLAG_IRQ,
    CS35L45_DSP_VIRT2_MBOX_IRQ,
    CS35L45_NUM_IRQ,
}

pub const CS35L45_MBOX3_CMD_MASK: u32 = 0xFF;
pub const CS35L45_MBOX3_CMD_SHIFT: u32 = 0;
pub const CS35L45_MBOX3_DATA_MASK: u32 = 0xFFFFFF00;
pub const CS35L45_MBOX3_DATA_SHIFT: u32 = 8;

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum mbox3_events {
    EVENT_SPEAKER_STATUS = 0x66,
    EVENT_BOOT_DONE = 0x67,
}

#[repr(C)]
pub struct cs35l45_private {
    pub dsp: wm_adsp, /* needs to be first member */
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub reset_gpio: *mut gpio_desc,
    pub vdd_batt: *mut regulator,
    pub vdd_a: *mut regulator,
    pub initialized: bool,
    pub sysclk_set: bool,
    pub slot_width: u8,
    pub slot_count: u8,
    pub amplifier_mode: i32,
    pub irq_invert: i32,
    pub irq: i32,
    pub i2c_addr: u32,
    pub bus_type: control_bus_type,
    pub irq_data: *mut regmap_irq_chip_data,
}

extern "C" {
    pub static cs35l45_pm_ops: dev_pm_ops;
    pub static cs35l45_i2c_regmap: regmap_config;
    pub static cs35l45_spi_regmap: regmap_config;
    pub fn cs35l45_apply_patch(cs35l45: *mut cs35l45_private) -> i32;
    pub fn cs35l45_get_clk_freq_id(freq: u32) -> i32;
    pub fn cs35l45_probe(cs35l45: *mut cs35l45_private) -> i32;
    pub fn cs35l45_remove(cs35l45: *mut cs35l45_private);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
