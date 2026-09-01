/* SPDX-License-Identifier: GPL-2.0 */
/*
 * mt8192-afe-clk.h  --  Mediatek 8192 afe clock ctrl definition
 *
 * Copyright (c) 2020 MediaTek Inc.
 * Author: Shane Chien <shane.chien@mediatek.com>
 */

pub const AP_PLL_CON3: u32 = 0x0014;
pub const APLL1_CON0: u32 = 0x0318;
pub const APLL1_CON1: u32 = 0x031c;
pub const APLL1_CON2: u32 = 0x0320;
pub const APLL1_CON4: u32 = 0x0328;
pub const APLL1_TUNER_CON0: u32 = 0x0040;

pub const APLL2_CON0: u32 = 0x032c;
pub const APLL2_CON1: u32 = 0x0330;
pub const APLL2_CON2: u32 = 0x0334;
pub const APLL2_CON4: u32 = 0x033c;
pub const APLL2_TUNER_CON0: u32 = 0x0044;

pub const CLK_CFG_7: u32 = 0x0080;
pub const CLK_CFG_8: u32 = 0x0090;
pub const CLK_CFG_11: u32 = 0x00c0;
pub const CLK_CFG_12: u32 = 0x00d0;
pub const CLK_CFG_13: u32 = 0x00e0;
pub const CLK_CFG_15: u32 = 0x0100;

pub const CLK_AUDDIV_0: u32 = 0x0320;
pub const CLK_AUDDIV_2: u32 = 0x0328;
pub const CLK_AUDDIV_3: u32 = 0x0334;
pub const CLK_AUDDIV_4: u32 = 0x0338;
pub const CKSYS_AUD_TOP_CFG: u32 = 0x032c;
pub const CKSYS_AUD_TOP_MON: u32 = 0x0330;

pub const PERI_BUS_DCM_CTRL: u32 = 0x0074;
pub const MODULE_SW_CG_1_STA: u32 = 0x0094;
pub const MODULE_SW_CG_2_STA: u32 = 0x00ac;

/* CLK_AUDDIV_0 */
pub const APLL12_DIV0_PDN_SFT: u32 = 0;
pub const APLL12_DIV0_PDN_MASK: u32 = 0x1;
pub const APLL12_DIV0_PDN_MASK_SFT: u32 = 0x1 << 0;
pub const APLL12_DIV1_PDN_SFT: u32 = 1;
pub const APLL12_DIV1_PDN_MASK: u32 = 0x1;
pub const APLL12_DIV1_PDN_MASK_SFT: u32 = 0x1 << 1;
pub const APLL12_DIV2_PDN_SFT: u32 = 2;
pub const APLL12_DIV2_PDN_MASK: u32 = 0x1;
pub const APLL12_DIV2_PDN_MASK_SFT: u32 = 0x1 << 2;
pub const APLL12_DIV3_PDN_SFT: u32 = 3;
pub const APLL12_DIV3_PDN_MASK: u32 = 0x1;
pub const APLL12_DIV3_PDN_MASK_SFT: u32 = 0x1 << 3;
pub const APLL12_DIV4_PDN_SFT: u32 = 4;
pub const APLL12_DIV4_PDN_MASK: u32 = 0x1;
pub const APLL12_DIV4_PDN_MASK_SFT: u32 = 0x1 << 4;
pub const APLL12_DIVB_PDN_SFT: u32 = 5;
pub const APLL12_DIVB_PDN_MASK: u32 = 0x1;
pub const APLL12_DIVB_PDN_MASK_SFT: u32 = 0x1 << 5;
pub const APLL12_DIV5_PDN_SFT: u32 = 6;
pub const APLL12_DIV5_PDN_MASK: u32 = 0x1;
pub const APLL12_DIV5_PDN_MASK_SFT: u32 = 0x1 << 6;
pub const APLL12_DIV6_PDN_SFT: u32 = 7;
pub const APLL12_DIV6_PDN_MASK: u32 = 0x1;
pub const APLL12_DIV6_PDN_MASK_SFT: u32 = 0x1 << 7;
pub const APLL12_DIV7_PDN_SFT: u32 = 8;
pub const APLL12_DIV7_PDN_MASK: u32 = 0x1;
pub const APLL12_DIV7_PDN_MASK_SFT: u32 = 0x1 << 8;
pub const APLL12_DIV8_PDN_SFT: u32 = 9;
pub const APLL12_DIV8_PDN_MASK: u32 = 0x1;
pub const APLL12_DIV8_PDN_MASK_SFT: u32 = 0x1 << 9;
pub const APLL12_DIV9_PDN_SFT: u32 = 10;
pub const APLL12_DIV9_PDN_MASK: u32 = 0x1;
pub const APLL12_DIV9_PDN_MASK_SFT: u32 = 0x1 << 10;
pub const APLL_I2S0_MCK_SEL_SFT: u32 = 16;
pub const APLL_I2S0_MCK_SEL_MASK: u32 = 0x1;
pub const APLL_I2S0_MCK_SEL_MASK_SFT: u32 = 0x1 << 16;
pub const APLL_I2S1_MCK_SEL_SFT: u32 = 17;
pub const APLL_I2S1_MCK_SEL_MASK: u32 = 0x1;
pub const APLL_I2S1_MCK_SEL_MASK_SFT: u32 = 0x1 << 17;
pub const APLL_I2S2_MCK_SEL_SFT: u32 = 18;
pub const APLL_I2S2_MCK_SEL_MASK: u32 = 0x1;
pub const APLL_I2S2_MCK_SEL_MASK_SFT: u32 = 0x1 << 18;
pub const APLL_I2S3_MCK_SEL_SFT: u32 = 19;
pub const APLL_I2S3_MCK_SEL_MASK: u32 = 0x1;
pub const APLL_I2S3_MCK_SEL_MASK_SFT: u32 = 0x1 << 19;
pub const APLL_I2S4_MCK_SEL_SFT: u32 = 20;
pub const APLL_I2S4_MCK_SEL_MASK: u32 = 0x1;
pub const APLL_I2S4_MCK_SEL_MASK_SFT: u32 = 0x1 << 20;
pub const APLL_I2S5_MCK_SEL_SFT: u32 = 21;
pub const APLL_I2S5_MCK_SEL_MASK: u32 = 0x1;
pub const APLL_I2S5_MCK_SEL_MASK_SFT: u32 = 0x1 << 21;
pub const APLL_I2S6_MCK_SEL_SFT: u32 = 22;
pub const APLL_I2S6_MCK_SEL_MASK: u32 = 0x1;
pub const APLL_I2S6_MCK_SEL_MASK_SFT: u32 = 0x1 << 22;
pub const APLL_I2S7_MCK_SEL_SFT: u32 = 23;
pub const APLL_I2S7_MCK_SEL_MASK: u32 = 0x1;
pub const APLL_I2S7_MCK_SEL_MASK_SFT: u32 = 0x1 << 23;
pub const APLL_I2S8_MCK_SEL_SFT: u32 = 24;
pub const APLL_I2S8_MCK_SEL_MASK: u32 = 0x1;
pub const APLL_I2S8_MCK_SEL_MASK_SFT: u32 = 0x1 << 24;
pub const APLL_I2S9_MCK_SEL_SFT: u32 = 25;
pub const APLL_I2S9_MCK_SEL_MASK: u32 = 0x1;
pub const APLL_I2S9_MCK_SEL_MASK_SFT: u32 = 0x1 << 25;

/* CLK_AUDDIV_2 */
pub const APLL12_CK_DIV0_SFT: u32 = 0;
pub const APLL12_CK_DIV0_MASK: u32 = 0xff;
pub const APLL12_CK_DIV0_MASK_SFT: u32 = 0xff << 0;
pub const APLL12_CK_DIV1_SFT: u32 = 8;
pub const APLL12_CK_DIV1_MASK: u32 = 0xff;
pub const APLL12_CK_DIV1_MASK_SFT: u32 = 0xff << 8;
pub const APLL12_CK_DIV2_SFT: u32 = 16;
pub const APLL12_CK_DIV2_MASK: u32 = 0xff;
pub const APLL12_CK_DIV2_MASK_SFT: u32 = 0xff << 16;
pub const APLL12_CK_DIV3_SFT: u32 = 24;
pub const APLL12_CK_DIV3_MASK: u32 = 0xff;
pub const APLL12_CK_DIV3_MASK_SFT: u32 = 0xff << 24;

/* CLK_AUDDIV_3 */
pub const APLL12_CK_DIV4_SFT: u32 = 0;
pub const APLL12_CK_DIV4_MASK: u32 = 0xff;
pub const APLL12_CK_DIV4_MASK_SFT: u32 = 0xff << 0;
pub const APLL12_CK_DIVB_SFT: u32 = 8;
pub const APLL12_CK_DIVB_MASK: u32 = 0xff;
pub const APLL12_CK_DIVB_MASK_SFT: u32 = 0xff << 8;
pub const APLL12_CK_DIV5_SFT: u32 = 16;
pub const APLL12_CK_DIV5_MASK: u32 = 0xff;
pub const APLL12_CK_DIV5_MASK_SFT: u32 = 0xff << 16;
pub const APLL12_CK_DIV6_SFT: u32 = 24;
pub const APLL12_CK_DIV6_MASK: u32 = 0xff;
pub const APLL12_CK_DIV6_MASK_SFT: u32 = 0xff << 24;

/* CLK_AUDDIV_4 */
pub const APLL12_CK_DIV7_SFT: u32 = 0;
pub const APLL12_CK_DIV7_MASK: u32 = 0xff;
pub const APLL12_CK_DIV7_MASK_SFT: u32 = 0xff << 0;
pub const APLL12_CK_DIV8_SFT: u32 = 8;
pub const APLL12_CK_DIV8_MASK: u32 = 0xff;
pub const APLL12_CK_DIV8_MASK_SFT: u32 = 0xff << 0;
pub const APLL12_CK_DIV9_SFT: u32 = 16;
pub const APLL12_CK_DIV9_MASK: u32 = 0xff;
pub const APLL12_CK_DIV9_MASK_SFT: u32 = 0xff << 0;

/* AUD_TOP_CFG */
pub const AUD_TOP_CFG_SFT: u32 = 0;
pub const AUD_TOP_CFG_MASK: u32 = 0xffffffff;
pub const AUD_TOP_CFG_MASK_SFT: u32 = 0xffffffff << 0;

/* AUD_TOP_MON */
pub const AUD_TOP_MON_SFT: u32 = 0;
pub const AUD_TOP_MON_MASK: u32 = 0xffffffff;
pub const AUD_TOP_MON_MASK_SFT: u32 = 0xffffffff << 0;

/* CLK_AUDDIV_3 */
pub const APLL12_CK_DIV5_MSB_SFT: u32 = 0;
pub const APLL12_CK_DIV5_MSB_MASK: u32 = 0xf;
pub const APLL12_CK_DIV5_MSB_MASK_SFT: u32 = 0xf << 0;
pub const RESERVED0_SFT: u32 = 4;
pub const RESERVED0_MASK: u32 = 0xfffffff;
pub const RESERVED0_MASK_SFT: u32 = 0xfffffff << 4;

/* APLL */
pub const APLL1_W_NAME: &[u8; 6] = b"APLL1\0";
pub const APLL2_W_NAME: &[u8; 6] = b"APLL2\0";

pub const MT8192_APLL1: i32 = 0;
pub const MT8192_APLL2: i32 = 1;

pub const CLK_AFE: i32 = 0;
pub const CLK_TML: i32 = 1;
pub const CLK_APLL22M: i32 = 2;
pub const CLK_APLL24M: i32 = 3;
pub const CLK_APLL1_TUNER: i32 = 4;
pub const CLK_APLL2_TUNER: i32 = 5;
pub const CLK_NLE: i32 = 6;
pub const CLK_INFRA_SYS_AUDIO: i32 = 7;
pub const CLK_INFRA_AUDIO_26M: i32 = 8;
pub const CLK_MUX_AUDIO: i32 = 9;
pub const CLK_MUX_AUDIOINTBUS: i32 = 10;
pub const CLK_TOP_MAINPLL_D4_D4: i32 = 11;
/* apll related mux */
pub const CLK_TOP_MUX_AUD_1: i32 = 12;
pub const CLK_TOP_APLL1_CK: i32 = 13;
pub const CLK_TOP_MUX_AUD_2: i32 = 14;
pub const CLK_TOP_APLL2_CK: i32 = 15;
pub const CLK_TOP_MUX_AUD_ENG1: i32 = 16;
pub const CLK_TOP_APLL1_D4: i32 = 17;
pub const CLK_TOP_MUX_AUD_ENG2: i32 = 18;
pub const CLK_TOP_APLL2_D4: i32 = 19;
pub const CLK_TOP_MUX_AUDIO_H: i32 = 20;
pub const CLK_TOP_I2S0_M_SEL: i32 = 21;
pub const CLK_TOP_I2S1_M_SEL: i32 = 22;
pub const CLK_TOP_I2S2_M_SEL: i32 = 23;
pub const CLK_TOP_I2S3_M_SEL: i32 = 24;
pub const CLK_TOP_I2S4_M_SEL: i32 = 25;
pub const CLK_TOP_I2S5_M_SEL: i32 = 26;
pub const CLK_TOP_I2S6_M_SEL: i32 = 27;
pub const CLK_TOP_I2S7_M_SEL: i32 = 28;
pub const CLK_TOP_I2S8_M_SEL: i32 = 29;
pub const CLK_TOP_I2S9_M_SEL: i32 = 30;
pub const CLK_TOP_APLL12_DIV0: i32 = 31;
pub const CLK_TOP_APLL12_DIV1: i32 = 32;
pub const CLK_TOP_APLL12_DIV2: i32 = 33;
pub const CLK_TOP_APLL12_DIV3: i32 = 34;
pub const CLK_TOP_APLL12_DIV4: i32 = 35;
pub const CLK_TOP_APLL12_DIVB: i32 = 36;
pub const CLK_TOP_APLL12_DIV5: i32 = 37;
pub const CLK_TOP_APLL12_DIV6: i32 = 38;
pub const CLK_TOP_APLL12_DIV7: i32 = 39;
pub const CLK_TOP_APLL12_DIV8: i32 = 40;
pub const CLK_TOP_APLL12_DIV9: i32 = 41;
pub const CLK_CLK26M: i32 = 42;
pub const CLK_NUM: i32 = 43;

#[repr(C)]
pub struct mtk_base_afe {
    _unused: [u8; 0],
}

unsafe extern "C" {
    pub fn mt8192_init_clock(afe: *mut mtk_base_afe) -> core::ffi::c_int;
    pub fn mt8192_afe_enable_clock(afe: *mut mtk_base_afe) -> core::ffi::c_int;
    pub fn mt8192_afe_disable_clock(afe: *mut mtk_base_afe);

    pub fn mt8192_apll1_enable(afe: *mut mtk_base_afe) -> core::ffi::c_int;
    pub fn mt8192_apll1_disable(afe: *mut mtk_base_afe);

    pub fn mt8192_apll2_enable(afe: *mut mtk_base_afe) -> core::ffi::c_int;
    pub fn mt8192_apll2_disable(afe: *mut mtk_base_afe);

    pub fn mt8192_get_apll_rate(afe: *mut mtk_base_afe, apll: core::ffi::c_int) -> core::ffi::c_int;
    pub fn mt8192_get_apll_by_rate(afe: *mut mtk_base_afe, rate: core::ffi::c_int) -> core::ffi::c_int;
    pub fn mt8192_get_apll_by_name(
        afe: *mut mtk_base_afe,
        name: *const core::ffi::c_char,
    ) -> core::ffi::c_int;

    /* these will be replaced by using CCF */
    pub fn mt8192_mck_enable(
        afe: *mut mtk_base_afe,
        mck_id: core::ffi::c_int,
        rate: core::ffi::c_int,
    ) -> core::ffi::c_int;
    pub fn mt8192_mck_disable(afe: *mut mtk_base_afe, mck_id: core::ffi::c_int);

    pub fn mt8192_set_audio_int_bus_parent(
        afe: *mut mtk_base_afe,
        clk_id: core::ffi::c_int,
    ) -> core::ffi::c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
