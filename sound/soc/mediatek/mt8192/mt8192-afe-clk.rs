// SPDX-License-Identifier: GPL-2.0
//
// mt8192-afe-clk.c  --  Mediatek 8192 afe clock ctrl
//
// Copyright (c) 2020 MediaTek Inc.
// Author: Shane Chien <shane.chien@mediatek.com>
//

use core::ffi::{c_char, c_int, c_long, c_ulong, c_void};

extern "C" {
    fn clk_set_parent(clk: *mut clk, parent: *mut clk) -> c_int;
    fn clk_prepare_enable(clk: *mut clk) -> c_int;
    fn clk_disable_unprepare(clk: *mut clk);
    fn clk_set_rate(clk: *mut clk, rate: c_int) -> c_int;
    fn regmap_update_bits(map: *mut regmap, reg: c_int, mask: c_int, val: c_int) -> c_int;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn devm_kcalloc(
        dev: *mut device,
        n: usize,
        size: usize,
        flags: c_int,
    ) -> *mut c_void;
    fn devm_clk_get(dev: *mut device, id: *const c_char) -> *mut clk;
    fn syscon_regmap_lookup_by_phandle(
        np: *mut device_node,
        property: *const c_char,
    ) -> *mut regmap;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_long;
}

extern "C" {
    static APLL1_W_NAME: *const c_char;
}

extern "C" {
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
}

#[repr(C)]
pub struct clk {
    _private: [u8; 0],
}

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    pub of_node: *mut device_node,
}

#[repr(C)]
pub struct mtk_base_afe {
    pub platform_priv: *mut c_void,
    pub dev: *mut device,
    pub regmap: *mut regmap,
}

#[repr(C)]
pub struct mt8192_afe_private {
    pub clk: *mut *mut clk,
    pub apmixedsys: *mut regmap,
    pub topckgen: *mut regmap,
    pub infracfg: *mut regmap,
}

extern "C" {
    static CLK_NUM: c_int;
    static CLK_AFE: c_int;
    static CLK_TML: c_int;
    static CLK_APLL22M: c_int;
    static CLK_APLL24M: c_int;
    static CLK_APLL1_TUNER: c_int;
    static CLK_APLL2_TUNER: c_int;
    static CLK_NLE: c_int;
    static CLK_INFRA_SYS_AUDIO: c_int;
    static CLK_INFRA_AUDIO_26M: c_int;
    static CLK_MUX_AUDIO: c_int;
    static CLK_MUX_AUDIOINTBUS: c_int;
    static CLK_TOP_MAINPLL_D4_D4: c_int;
    static CLK_TOP_MUX_AUD_1: c_int;
    static CLK_TOP_APLL1_CK: c_int;
    static CLK_TOP_MUX_AUD_2: c_int;
    static CLK_TOP_APLL2_CK: c_int;
    static CLK_TOP_MUX_AUD_ENG1: c_int;
    static CLK_TOP_APLL1_D4: c_int;
    static CLK_TOP_MUX_AUD_ENG2: c_int;
    static CLK_TOP_APLL2_D4: c_int;
    static CLK_TOP_MUX_AUDIO_H: c_int;
    static CLK_TOP_I2S0_M_SEL: c_int;
    static CLK_TOP_I2S1_M_SEL: c_int;
    static CLK_TOP_I2S2_M_SEL: c_int;
    static CLK_TOP_I2S3_M_SEL: c_int;
    static CLK_TOP_I2S4_M_SEL: c_int;
    static CLK_TOP_I2S5_M_SEL: c_int;
    static CLK_TOP_I2S6_M_SEL: c_int;
    static CLK_TOP_I2S7_M_SEL: c_int;
    static CLK_TOP_I2S8_M_SEL: c_int;
    static CLK_TOP_I2S9_M_SEL: c_int;
    static CLK_TOP_APLL12_DIV0: c_int;
    static CLK_TOP_APLL12_DIV1: c_int;
    static CLK_TOP_APLL12_DIV2: c_int;
    static CLK_TOP_APLL12_DIV3: c_int;
    static CLK_TOP_APLL12_DIV4: c_int;
    static CLK_TOP_APLL12_DIVB: c_int;
    static CLK_TOP_APLL12_DIV5: c_int;
    static CLK_TOP_APLL12_DIV6: c_int;
    static CLK_TOP_APLL12_DIV7: c_int;
    static CLK_TOP_APLL12_DIV8: c_int;
    static CLK_TOP_APLL12_DIV9: c_int;
    static CLK_CLK26M: c_int;
    static MT8192_APLL1: c_int;
    static MT8192_APLL2: c_int;
    static MT8192_MCK_NUM: c_int;
    static MT8192_I2S0_MCK: c_int;
    static MT8192_I2S1_MCK: c_int;
    static MT8192_I2S2_MCK: c_int;
    static MT8192_I2S3_MCK: c_int;
    static MT8192_I2S4_MCK: c_int;
    static MT8192_I2S4_BCK: c_int;
    static MT8192_I2S5_MCK: c_int;
    static MT8192_I2S6_MCK: c_int;
    static MT8192_I2S7_MCK: c_int;
    static MT8192_I2S8_MCK: c_int;
    static MT8192_I2S9_MCK: c_int;
    static AFE_APLL1_TUNER_CFG: c_int;
    static AFE_APLL2_TUNER_CFG: c_int;
    static AFE_HD_ENGEN_ENABLE: c_int;
    static AFE_22M_ON_MASK_SFT: c_int;
    static AFE_22M_ON_SFT: c_int;
    static AFE_24M_ON_MASK_SFT: c_int;
    static AFE_24M_ON_SFT: c_int;
    static CLK_AUDDIV_0: c_int;
    static CLK_AUDDIV_2: c_int;
    static CLK_AUDDIV_3: c_int;
    static CLK_AUDDIV_4: c_int;
    static APLL12_DIV0_PDN_MASK_SFT: c_int;
    static APLL12_DIV1_PDN_MASK_SFT: c_int;
    static APLL12_DIV2_PDN_MASK_SFT: c_int;
    static APLL12_DIV3_PDN_MASK_SFT: c_int;
    static APLL12_DIV4_PDN_MASK_SFT: c_int;
    static APLL12_DIVB_PDN_MASK_SFT: c_int;
    static APLL12_DIV5_PDN_MASK_SFT: c_int;
    static APLL12_DIV6_PDN_MASK_SFT: c_int;
    static APLL12_DIV7_PDN_MASK_SFT: c_int;
    static APLL12_DIV8_PDN_MASK_SFT: c_int;
    static APLL12_DIV9_PDN_MASK_SFT: c_int;
    static APLL12_CK_DIV0_MASK_SFT: c_int;
    static APLL12_CK_DIV1_MASK_SFT: c_int;
    static APLL12_CK_DIV2_MASK_SFT: c_int;
    static APLL12_CK_DIV3_MASK_SFT: c_int;
    static APLL12_CK_DIV4_MASK_SFT: c_int;
    static APLL12_CK_DIVB_MASK_SFT: c_int;
    static APLL12_CK_DIV5_MASK_SFT: c_int;
    static APLL12_CK_DIV6_MASK_SFT: c_int;
    static APLL12_CK_DIV7_MASK_SFT: c_int;
    static APLL12_CK_DIV8_MASK_SFT: c_int;
    static APLL12_CK_DIV9_MASK_SFT: c_int;
    static APLL12_CK_DIV0_MASK: c_int;
    static APLL12_CK_DIV1_MASK: c_int;
    static APLL12_CK_DIV2_MASK: c_int;
    static APLL12_CK_DIV3_MASK: c_int;
    static APLL12_CK_DIV4_MASK: c_int;
    static APLL12_CK_DIVB_MASK: c_int;
    static APLL12_CK_DIV5_MASK: c_int;
    static APLL12_CK_DIV6_MASK: c_int;
    static APLL12_CK_DIV7_MASK: c_int;
    static APLL12_CK_DIV8_MASK: c_int;
    static APLL12_CK_DIV9_MASK: c_int;
    static APLL12_CK_DIV0_SFT: c_int;
    static APLL12_CK_DIV1_SFT: c_int;
    static APLL12_CK_DIV2_SFT: c_int;
    static APLL12_CK_DIV3_SFT: c_int;
    static APLL12_CK_DIV4_SFT: c_int;
    static APLL12_CK_DIVB_SFT: c_int;
    static APLL12_CK_DIV5_SFT: c_int;
    static APLL12_CK_DIV6_SFT: c_int;
    static APLL12_CK_DIV7_SFT: c_int;
    static APLL12_CK_DIV8_SFT: c_int;
    static APLL12_CK_DIV9_SFT: c_int;
    static APLL_I2S0_MCK_SEL_MASK_SFT: c_int;
    static APLL_I2S1_MCK_SEL_MASK_SFT: c_int;
    static APLL_I2S2_MCK_SEL_MASK_SFT: c_int;
    static APLL_I2S3_MCK_SEL_MASK_SFT: c_int;
    static APLL_I2S4_MCK_SEL_MASK_SFT: c_int;
    static APLL_I2S5_MCK_SEL_MASK_SFT: c_int;
    static APLL_I2S6_MCK_SEL_MASK_SFT: c_int;
    static APLL_I2S7_MCK_SEL_MASK_SFT: c_int;
    static APLL_I2S8_MCK_SEL_MASK_SFT: c_int;
    static APLL_I2S9_MCK_SEL_MASK_SFT: c_int;
    static APLL_I2S0_MCK_SEL_SFT: c_int;
    static APLL_I2S1_MCK_SEL_SFT: c_int;
    static APLL_I2S2_MCK_SEL_SFT: c_int;
    static APLL_I2S3_MCK_SEL_SFT: c_int;
    static APLL_I2S4_MCK_SEL_SFT: c_int;
    static APLL_I2S5_MCK_SEL_SFT: c_int;
    static APLL_I2S6_MCK_SEL_SFT: c_int;
    static APLL_I2S7_MCK_SEL_SFT: c_int;
    static APLL_I2S8_MCK_SEL_SFT: c_int;
    static APLL_I2S9_MCK_SEL_SFT: c_int;
    static GFP_KERNEL: c_int;
    static ENOMEM: c_int;
}

static mut AUD_CLKS: [*const c_char; 43] = [
    b"aud_afe_clk\0".as_ptr() as *const c_char,
    b"aud_tml_clk\0".as_ptr() as *const c_char,
    b"aud_apll22m_clk\0".as_ptr() as *const c_char,
    b"aud_apll24m_clk\0".as_ptr() as *const c_char,
    b"aud_apll1_tuner_clk\0".as_ptr() as *const c_char,
    b"aud_apll2_tuner_clk\0".as_ptr() as *const c_char,
    b"aud_nle\0".as_ptr() as *const c_char,
    b"aud_infra_clk\0".as_ptr() as *const c_char,
    b"aud_infra_26m_clk\0".as_ptr() as *const c_char,
    b"top_mux_audio\0".as_ptr() as *const c_char,
    b"top_mux_audio_int\0".as_ptr() as *const c_char,
    b"top_mainpll_d4_d4\0".as_ptr() as *const c_char,
    b"top_mux_aud_1\0".as_ptr() as *const c_char,
    b"top_apll1_ck\0".as_ptr() as *const c_char,
    b"top_mux_aud_2\0".as_ptr() as *const c_char,
    b"top_apll2_ck\0".as_ptr() as *const c_char,
    b"top_mux_aud_eng1\0".as_ptr() as *const c_char,
    b"top_apll1_d4\0".as_ptr() as *const c_char,
    b"top_mux_aud_eng2\0".as_ptr() as *const c_char,
    b"top_apll2_d4\0".as_ptr() as *const c_char,
    b"top_mux_audio_h\0".as_ptr() as *const c_char,
    b"top_i2s0_m_sel\0".as_ptr() as *const c_char,
    b"top_i2s1_m_sel\0".as_ptr() as *const c_char,
    b"top_i2s2_m_sel\0".as_ptr() as *const c_char,
    b"top_i2s3_m_sel\0".as_ptr() as *const c_char,
    b"top_i2s4_m_sel\0".as_ptr() as *const c_char,
    b"top_i2s5_m_sel\0".as_ptr() as *const c_char,
    b"top_i2s6_m_sel\0".as_ptr() as *const c_char,
    b"top_i2s7_m_sel\0".as_ptr() as *const c_char,
    b"top_i2s8_m_sel\0".as_ptr() as *const c_char,
    b"top_i2s9_m_sel\0".as_ptr() as *const c_char,
    b"top_apll12_div0\0".as_ptr() as *const c_char,
    b"top_apll12_div1\0".as_ptr() as *const c_char,
    b"top_apll12_div2\0".as_ptr() as *const c_char,
    b"top_apll12_div3\0".as_ptr() as *const c_char,
    b"top_apll12_div4\0".as_ptr() as *const c_char,
    b"top_apll12_divb\0".as_ptr() as *const c_char,
    b"top_apll12_div5\0".as_ptr() as *const c_char,
    b"top_apll12_div6\0".as_ptr() as *const c_char,
    b"top_apll12_div7\0".as_ptr() as *const c_char,
    b"top_apll12_div8\0".as_ptr() as *const c_char,
    b"top_apll12_div9\0".as_ptr() as *const c_char,
    b"top_clk26m_clk\0".as_ptr() as *const c_char,
];

unsafe fn afe_priv(afe: *mut mtk_base_afe) -> *mut mt8192_afe_private {
    (*afe).platform_priv as *mut mt8192_afe_private
}

unsafe fn clk_at(priv_: *mut mt8192_afe_private, id: c_int) -> *mut clk {
    *(*priv_).clk.add(id as usize)
}

unsafe fn aud_clk(id: c_int) -> *const c_char {
    AUD_CLKS[id as usize]
}

#[no_mangle]
pub unsafe extern "C" fn mt8192_set_audio_int_bus_parent(
    afe: *mut mtk_base_afe,
    clk_id: c_int,
) -> c_int {
    let afe_priv = afe_priv(afe);
    let ret = clk_set_parent(
        clk_at(afe_priv, CLK_MUX_AUDIOINTBUS),
        clk_at(afe_priv, clk_id),
    );
    if ret != 0 {
        dev_err(
            (*afe).dev,
            b"%s clk_set_parent %s-%s fail %d\n\0".as_ptr() as *const c_char,
            b"mt8192_set_audio_int_bus_parent\0".as_ptr() as *const c_char,
            aud_clk(CLK_MUX_AUDIOINTBUS),
            aud_clk(clk_id),
            ret,
        );
    }
    ret
}

unsafe fn apll1_mux_setting(afe: *mut mtk_base_afe, enable: bool) -> c_int {
    let afe_priv = afe_priv(afe);
    let mut ret: c_int = 0;

    if enable {
        ret = clk_prepare_enable(clk_at(afe_priv, CLK_TOP_MUX_AUD_1));
        if ret != 0 {
            dev_err((*afe).dev, b"%s clk_prepare_enable %s fail %d\n\0".as_ptr() as *const c_char, b"apll1_mux_setting\0".as_ptr() as *const c_char, aud_clk(CLK_TOP_MUX_AUD_1), ret);
            return ret;
        }
        ret = clk_set_parent(clk_at(afe_priv, CLK_TOP_MUX_AUD_1), clk_at(afe_priv, CLK_TOP_APLL1_CK));
        if ret != 0 {
            dev_err((*afe).dev, b"%s clk_set_parent %s-%s fail %d\n\0".as_ptr() as *const c_char, b"apll1_mux_setting\0".as_ptr() as *const c_char, aud_clk(CLK_TOP_MUX_AUD_1), aud_clk(CLK_TOP_APLL1_CK), ret);
            return ret;
        }

        /* 180.6336 / 4 = 45.1584MHz */
        ret = clk_prepare_enable(clk_at(afe_priv, CLK_TOP_MUX_AUD_ENG1));
        if ret != 0 {
            dev_err((*afe).dev, b"%s clk_prepare_enable %s fail %d\n\0".as_ptr() as *const c_char, b"apll1_mux_setting\0".as_ptr() as *const c_char, aud_clk(CLK_TOP_MUX_AUD_ENG1), ret);
            return ret;
        }
        ret = clk_set_parent(clk_at(afe_priv, CLK_TOP_MUX_AUD_ENG1), clk_at(afe_priv, CLK_TOP_APLL1_D4));
        if ret != 0 {
            dev_err((*afe).dev, b"%s clk_set_parent %s-%s fail %d\n\0".as_ptr() as *const c_char, b"apll1_mux_setting\0".as_ptr() as *const c_char, aud_clk(CLK_TOP_MUX_AUD_ENG1), aud_clk(CLK_TOP_APLL1_D4), ret);
            return ret;
        }
    } else {
        ret = clk_set_parent(clk_at(afe_priv, CLK_TOP_MUX_AUD_ENG1), clk_at(afe_priv, CLK_CLK26M));
        if ret != 0 {
            dev_err((*afe).dev, b"%s clk_set_parent %s-%s fail %d\n\0".as_ptr() as *const c_char, b"apll1_mux_setting\0".as_ptr() as *const c_char, aud_clk(CLK_TOP_MUX_AUD_ENG1), aud_clk(CLK_CLK26M), ret);
            return ret;
        }
        clk_disable_unprepare(clk_at(afe_priv, CLK_TOP_MUX_AUD_ENG1));

        ret = clk_set_parent(clk_at(afe_priv, CLK_TOP_MUX_AUD_1), clk_at(afe_priv, CLK_CLK26M));
        if ret != 0 {
            dev_err((*afe).dev, b"%s clk_set_parent %s-%s fail %d\n\0".as_ptr() as *const c_char, b"apll1_mux_setting\0".as_ptr() as *const c_char, aud_clk(CLK_TOP_MUX_AUD_1), aud_clk(CLK_CLK26M), ret);
            return ret;
        }
        clk_disable_unprepare(clk_at(afe_priv, CLK_TOP_MUX_AUD_1));
    }

    ret
}

unsafe fn apll2_mux_setting(afe: *mut mtk_base_afe, enable: bool) -> c_int {
    let afe_priv = afe_priv(afe);
    let mut ret: c_int = 0;

    if enable {
        ret = clk_prepare_enable(clk_at(afe_priv, CLK_TOP_MUX_AUD_2));
        if ret != 0 {
            dev_err((*afe).dev, b"%s clk_prepare_enable %s fail %d\n\0".as_ptr() as *const c_char, b"apll2_mux_setting\0".as_ptr() as *const c_char, aud_clk(CLK_TOP_MUX_AUD_2), ret);
            return ret;
        }
        ret = clk_set_parent(clk_at(afe_priv, CLK_TOP_MUX_AUD_2), clk_at(afe_priv, CLK_TOP_APLL2_CK));
        if ret != 0 {
            dev_err((*afe).dev, b"%s clk_set_parent %s-%s fail %d\n\0".as_ptr() as *const c_char, b"apll2_mux_setting\0".as_ptr() as *const c_char, aud_clk(CLK_TOP_MUX_AUD_2), aud_clk(CLK_TOP_APLL2_CK), ret);
            return ret;
        }

        /* 196.608 / 4 = 49.152MHz */
        ret = clk_prepare_enable(clk_at(afe_priv, CLK_TOP_MUX_AUD_ENG2));
        if ret != 0 {
            dev_err((*afe).dev, b"%s clk_prepare_enable %s fail %d\n\0".as_ptr() as *const c_char, b"apll2_mux_setting\0".as_ptr() as *const c_char, aud_clk(CLK_TOP_MUX_AUD_ENG2), ret);
            return ret;
        }
        ret = clk_set_parent(clk_at(afe_priv, CLK_TOP_MUX_AUD_ENG2), clk_at(afe_priv, CLK_TOP_APLL2_D4));
        if ret != 0 {
            dev_err((*afe).dev, b"%s clk_set_parent %s-%s fail %d\n\0".as_ptr() as *const c_char, b"apll2_mux_setting\0".as_ptr() as *const c_char, aud_clk(CLK_TOP_MUX_AUD_ENG2), aud_clk(CLK_TOP_APLL2_D4), ret);
            return ret;
        }
    } else {
        ret = clk_set_parent(clk_at(afe_priv, CLK_TOP_MUX_AUD_ENG2), clk_at(afe_priv, CLK_CLK26M));
        if ret != 0 {
            dev_err((*afe).dev, b"%s clk_set_parent %s-%s fail %d\n\0".as_ptr() as *const c_char, b"apll2_mux_setting\0".as_ptr() as *const c_char, aud_clk(CLK_TOP_MUX_AUD_ENG2), aud_clk(CLK_CLK26M), ret);
            return ret;
        }
        clk_disable_unprepare(clk_at(afe_priv, CLK_TOP_MUX_AUD_ENG2));

        ret = clk_set_parent(clk_at(afe_priv, CLK_TOP_MUX_AUD_2), clk_at(afe_priv, CLK_CLK26M));
        if ret != 0 {
            dev_err((*afe).dev, b"%s clk_set_parent %s-%s fail %d\n\0".as_ptr() as *const c_char, b"apll2_mux_setting\0".as_ptr() as *const c_char, aud_clk(CLK_TOP_MUX_AUD_2), aud_clk(CLK_CLK26M), ret);
            return ret;
        }
        clk_disable_unprepare(clk_at(afe_priv, CLK_TOP_MUX_AUD_2));
    }

    ret
}

#[no_mangle]
pub unsafe extern "C" fn mt8192_afe_enable_clock(afe: *mut mtk_base_afe) -> c_int {
    let afe_priv = afe_priv(afe);
    let mut ret: c_int;

    ret = clk_prepare_enable(clk_at(afe_priv, CLK_INFRA_SYS_AUDIO));
    if ret != 0 {
        dev_err((*afe).dev, b"%s clk_prepare_enable %s fail %d\n\0".as_ptr() as *const c_char, b"mt8192_afe_enable_clock\0".as_ptr() as *const c_char, aud_clk(CLK_INFRA_SYS_AUDIO), ret);
        return ret;
    }

    ret = clk_prepare_enable(clk_at(afe_priv, CLK_INFRA_AUDIO_26M));
    if ret != 0 {
        dev_err((*afe).dev, b"%s clk_prepare_enable %s fail %d\n\0".as_ptr() as *const c_char, b"mt8192_afe_enable_clock\0".as_ptr() as *const c_char, aud_clk(CLK_INFRA_AUDIO_26M), ret);
        return ret;
    }

    ret = clk_prepare_enable(clk_at(afe_priv, CLK_MUX_AUDIO));
    if ret != 0 {
        dev_err((*afe).dev, b"%s clk_prepare_enable %s fail %d\n\0".as_ptr() as *const c_char, b"mt8192_afe_enable_clock\0".as_ptr() as *const c_char, aud_clk(CLK_MUX_AUDIO), ret);
        return ret;
    }
    ret = clk_set_parent(clk_at(afe_priv, CLK_MUX_AUDIO), clk_at(afe_priv, CLK_CLK26M));
    if ret != 0 {
        dev_err((*afe).dev, b"%s clk_set_parent %s-%s fail %d\n\0".as_ptr() as *const c_char, b"mt8192_afe_enable_clock\0".as_ptr() as *const c_char, aud_clk(CLK_MUX_AUDIO), aud_clk(CLK_CLK26M), ret);
        return ret;
    }

    ret = clk_prepare_enable(clk_at(afe_priv, CLK_MUX_AUDIOINTBUS));
    if ret != 0 {
        dev_err((*afe).dev, b"%s clk_prepare_enable %s fail %d\n\0".as_ptr() as *const c_char, b"mt8192_afe_enable_clock\0".as_ptr() as *const c_char, aud_clk(CLK_MUX_AUDIOINTBUS), ret);
        return ret;
    }

    ret = mt8192_set_audio_int_bus_parent(afe, CLK_CLK26M);
    if ret != 0 {
        dev_err((*afe).dev, b"%s clk_set_parent %s-%s fail %d\n\0".as_ptr() as *const c_char, b"mt8192_afe_enable_clock\0".as_ptr() as *const c_char, aud_clk(CLK_MUX_AUDIOINTBUS), aud_clk(CLK_CLK26M), ret);
        return ret;
    }

    ret = clk_set_parent(clk_at(afe_priv, CLK_TOP_MUX_AUDIO_H), clk_at(afe_priv, CLK_TOP_APLL2_CK));
    if ret != 0 {
        dev_err((*afe).dev, b"%s clk_set_parent %s-%s fail %d\n\0".as_ptr() as *const c_char, b"mt8192_afe_enable_clock\0".as_ptr() as *const c_char, aud_clk(CLK_TOP_MUX_AUDIO_H), aud_clk(CLK_TOP_APLL2_CK), ret);
        return ret;
    }

    ret = clk_prepare_enable(clk_at(afe_priv, CLK_AFE));
    if ret != 0 {
        dev_err((*afe).dev, b"%s clk_prepare_enable %s fail %d\n\0".as_ptr() as *const c_char, b"mt8192_afe_enable_clock\0".as_ptr() as *const c_char, aud_clk(CLK_AFE), ret);
        return ret;
    }

    ret
}

#[no_mangle]
pub unsafe extern "C" fn mt8192_afe_disable_clock(afe: *mut mtk_base_afe) {
    let afe_priv = afe_priv(afe);

    clk_disable_unprepare(clk_at(afe_priv, CLK_AFE));
    mt8192_set_audio_int_bus_parent(afe, CLK_CLK26M);
    clk_disable_unprepare(clk_at(afe_priv, CLK_MUX_AUDIOINTBUS));
    clk_disable_unprepare(clk_at(afe_priv, CLK_MUX_AUDIO));
    clk_disable_unprepare(clk_at(afe_priv, CLK_INFRA_AUDIO_26M));
    clk_disable_unprepare(clk_at(afe_priv, CLK_INFRA_SYS_AUDIO));
}

#[no_mangle]
pub unsafe extern "C" fn mt8192_apll1_enable(afe: *mut mtk_base_afe) -> c_int {
    let afe_priv = afe_priv(afe);
    let mut ret: c_int;

    /* setting for APLL */
    apll1_mux_setting(afe, true);

    ret = clk_prepare_enable(clk_at(afe_priv, CLK_APLL22M));
    if ret != 0 {
        dev_err((*afe).dev, b"%s clk_prepare_enable %s fail %d\n\0".as_ptr() as *const c_char, b"mt8192_apll1_enable\0".as_ptr() as *const c_char, aud_clk(CLK_APLL22M), ret);
        return ret;
    }

    ret = clk_prepare_enable(clk_at(afe_priv, CLK_APLL1_TUNER));
    if ret != 0 {
        dev_err((*afe).dev, b"%s clk_prepare_enable %s fail %d\n\0".as_ptr() as *const c_char, b"mt8192_apll1_enable\0".as_ptr() as *const c_char, aud_clk(CLK_APLL1_TUNER), ret);
        return ret;
    }

    regmap_update_bits((*afe).regmap, AFE_APLL1_TUNER_CFG, 0x0000FFF7, 0x00000832);
    regmap_update_bits((*afe).regmap, AFE_APLL1_TUNER_CFG, 0x1, 0x1);
    regmap_update_bits((*afe).regmap, AFE_HD_ENGEN_ENABLE, AFE_22M_ON_MASK_SFT, 0x1 << AFE_22M_ON_SFT);

    ret
}

#[no_mangle]
pub unsafe extern "C" fn mt8192_apll1_disable(afe: *mut mtk_base_afe) {
    let afe_priv = afe_priv(afe);

    regmap_update_bits((*afe).regmap, AFE_HD_ENGEN_ENABLE, AFE_22M_ON_MASK_SFT, 0x0 << AFE_22M_ON_SFT);
    regmap_update_bits((*afe).regmap, AFE_APLL1_TUNER_CFG, 0x1, 0x0);
    clk_disable_unprepare(clk_at(afe_priv, CLK_APLL1_TUNER));
    clk_disable_unprepare(clk_at(afe_priv, CLK_APLL22M));
    apll1_mux_setting(afe, false);
}

#[no_mangle]
pub unsafe extern "C" fn mt8192_apll2_enable(afe: *mut mtk_base_afe) -> c_int {
    let afe_priv = afe_priv(afe);
    let mut ret: c_int;

    /* setting for APLL */
    apll2_mux_setting(afe, true);

    ret = clk_prepare_enable(clk_at(afe_priv, CLK_APLL24M));
    if ret != 0 {
        dev_err((*afe).dev, b"%s clk_prepare_enable %s fail %d\n\0".as_ptr() as *const c_char, b"mt8192_apll2_enable\0".as_ptr() as *const c_char, aud_clk(CLK_APLL24M), ret);
        return ret;
    }

    ret = clk_prepare_enable(clk_at(afe_priv, CLK_APLL2_TUNER));
    if ret != 0 {
        dev_err((*afe).dev, b"%s clk_prepare_enable %s fail %d\n\0".as_ptr() as *const c_char, b"mt8192_apll2_enable\0".as_ptr() as *const c_char, aud_clk(CLK_APLL2_TUNER), ret);
        return ret;
    }

    regmap_update_bits((*afe).regmap, AFE_APLL2_TUNER_CFG, 0x0000FFF7, 0x00000634);
    regmap_update_bits((*afe).regmap, AFE_APLL2_TUNER_CFG, 0x1, 0x1);
    regmap_update_bits((*afe).regmap, AFE_HD_ENGEN_ENABLE, AFE_24M_ON_MASK_SFT, 0x1 << AFE_24M_ON_SFT);

    ret
}

#[no_mangle]
pub unsafe extern "C" fn mt8192_apll2_disable(afe: *mut mtk_base_afe) {
    let afe_priv = afe_priv(afe);

    regmap_update_bits((*afe).regmap, AFE_HD_ENGEN_ENABLE, AFE_24M_ON_MASK_SFT, 0x0 << AFE_24M_ON_SFT);
    regmap_update_bits((*afe).regmap, AFE_APLL2_TUNER_CFG, 0x1, 0x0);
    clk_disable_unprepare(clk_at(afe_priv, CLK_APLL2_TUNER));
    clk_disable_unprepare(clk_at(afe_priv, CLK_APLL24M));
    apll2_mux_setting(afe, false);
}

#[no_mangle]
pub unsafe extern "C" fn mt8192_get_apll_rate(_afe: *mut mtk_base_afe, apll: c_int) -> c_int {
    if apll == MT8192_APLL1 {
        180633600
    } else {
        196608000
    }
}

#[no_mangle]
pub unsafe extern "C" fn mt8192_get_apll_by_rate(_afe: *mut mtk_base_afe, rate: c_int) -> c_int {
    if rate % 8000 == 0 {
        MT8192_APLL2
    } else {
        MT8192_APLL1
    }
}

#[no_mangle]
pub unsafe extern "C" fn mt8192_get_apll_by_name(
    _afe: *mut mtk_base_afe,
    name: *const c_char,
) -> c_int {
    if strcmp(name, APLL1_W_NAME) == 0 {
        MT8192_APLL1
    } else {
        MT8192_APLL2
    }
}

/* mck */
#[repr(C)]
struct mt8192_mck_div {
    m_sel_id: c_int,
    div_clk_id: c_int,
    /* below will be deprecated */
    div_pdn_reg: c_int,
    div_pdn_mask_sft: c_int,
    div_reg: c_int,
    div_mask_sft: c_int,
    div_mask: c_int,
    div_sft: c_int,
    div_apll_sel_reg: c_int,
    div_apll_sel_mask_sft: c_int,
    div_apll_sel_sft: c_int,
}

unsafe fn mck_div_at(mck_id: c_int) -> mt8192_mck_div {
    match mck_id {
        x if x == MT8192_I2S0_MCK => mt8192_mck_div { m_sel_id: CLK_TOP_I2S0_M_SEL, div_clk_id: CLK_TOP_APLL12_DIV0, div_pdn_reg: CLK_AUDDIV_0, div_pdn_mask_sft: APLL12_DIV0_PDN_MASK_SFT, div_reg: CLK_AUDDIV_2, div_mask_sft: APLL12_CK_DIV0_MASK_SFT, div_mask: APLL12_CK_DIV0_MASK, div_sft: APLL12_CK_DIV0_SFT, div_apll_sel_reg: CLK_AUDDIV_0, div_apll_sel_mask_sft: APLL_I2S0_MCK_SEL_MASK_SFT, div_apll_sel_sft: APLL_I2S0_MCK_SEL_SFT },
        x if x == MT8192_I2S1_MCK => mt8192_mck_div { m_sel_id: CLK_TOP_I2S1_M_SEL, div_clk_id: CLK_TOP_APLL12_DIV1, div_pdn_reg: CLK_AUDDIV_0, div_pdn_mask_sft: APLL12_DIV1_PDN_MASK_SFT, div_reg: CLK_AUDDIV_2, div_mask_sft: APLL12_CK_DIV1_MASK_SFT, div_mask: APLL12_CK_DIV1_MASK, div_sft: APLL12_CK_DIV1_SFT, div_apll_sel_reg: CLK_AUDDIV_0, div_apll_sel_mask_sft: APLL_I2S1_MCK_SEL_MASK_SFT, div_apll_sel_sft: APLL_I2S1_MCK_SEL_SFT },
        x if x == MT8192_I2S2_MCK => mt8192_mck_div { m_sel_id: CLK_TOP_I2S2_M_SEL, div_clk_id: CLK_TOP_APLL12_DIV2, div_pdn_reg: CLK_AUDDIV_0, div_pdn_mask_sft: APLL12_DIV2_PDN_MASK_SFT, div_reg: CLK_AUDDIV_2, div_mask_sft: APLL12_CK_DIV2_MASK_SFT, div_mask: APLL12_CK_DIV2_MASK, div_sft: APLL12_CK_DIV2_SFT, div_apll_sel_reg: CLK_AUDDIV_0, div_apll_sel_mask_sft: APLL_I2S2_MCK_SEL_MASK_SFT, div_apll_sel_sft: APLL_I2S2_MCK_SEL_SFT },
        x if x == MT8192_I2S3_MCK => mt8192_mck_div { m_sel_id: CLK_TOP_I2S3_M_SEL, div_clk_id: CLK_TOP_APLL12_DIV3, div_pdn_reg: CLK_AUDDIV_0, div_pdn_mask_sft: APLL12_DIV3_PDN_MASK_SFT, div_reg: CLK_AUDDIV_2, div_mask_sft: APLL12_CK_DIV3_MASK_SFT, div_mask: APLL12_CK_DIV3_MASK, div_sft: APLL12_CK_DIV3_SFT, div_apll_sel_reg: CLK_AUDDIV_0, div_apll_sel_mask_sft: APLL_I2S3_MCK_SEL_MASK_SFT, div_apll_sel_sft: APLL_I2S3_MCK_SEL_SFT },
        x if x == MT8192_I2S4_MCK => mt8192_mck_div { m_sel_id: CLK_TOP_I2S4_M_SEL, div_clk_id: CLK_TOP_APLL12_DIV4, div_pdn_reg: CLK_AUDDIV_0, div_pdn_mask_sft: APLL12_DIV4_PDN_MASK_SFT, div_reg: CLK_AUDDIV_3, div_mask_sft: APLL12_CK_DIV4_MASK_SFT, div_mask: APLL12_CK_DIV4_MASK, div_sft: APLL12_CK_DIV4_SFT, div_apll_sel_reg: CLK_AUDDIV_0, div_apll_sel_mask_sft: APLL_I2S4_MCK_SEL_MASK_SFT, div_apll_sel_sft: APLL_I2S4_MCK_SEL_SFT },
        x if x == MT8192_I2S4_BCK => mt8192_mck_div { m_sel_id: -1, div_clk_id: CLK_TOP_APLL12_DIVB, div_pdn_reg: CLK_AUDDIV_0, div_pdn_mask_sft: APLL12_DIVB_PDN_MASK_SFT, div_reg: CLK_AUDDIV_2, div_mask_sft: APLL12_CK_DIVB_MASK_SFT, div_mask: APLL12_CK_DIVB_MASK, div_sft: APLL12_CK_DIVB_SFT, div_apll_sel_reg: 0, div_apll_sel_mask_sft: 0, div_apll_sel_sft: 0 },
        x if x == MT8192_I2S5_MCK => mt8192_mck_div { m_sel_id: CLK_TOP_I2S5_M_SEL, div_clk_id: CLK_TOP_APLL12_DIV5, div_pdn_reg: CLK_AUDDIV_0, div_pdn_mask_sft: APLL12_DIV5_PDN_MASK_SFT, div_reg: CLK_AUDDIV_3, div_mask_sft: APLL12_CK_DIV5_MASK_SFT, div_mask: APLL12_CK_DIV5_MASK, div_sft: APLL12_CK_DIV5_SFT, div_apll_sel_reg: CLK_AUDDIV_0, div_apll_sel_mask_sft: APLL_I2S5_MCK_SEL_MASK_SFT, div_apll_sel_sft: APLL_I2S5_MCK_SEL_SFT },
        x if x == MT8192_I2S6_MCK => mt8192_mck_div { m_sel_id: CLK_TOP_I2S6_M_SEL, div_clk_id: CLK_TOP_APLL12_DIV6, div_pdn_reg: CLK_AUDDIV_0, div_pdn_mask_sft: APLL12_DIV6_PDN_MASK_SFT, div_reg: CLK_AUDDIV_3, div_mask_sft: APLL12_CK_DIV6_MASK_SFT, div_mask: APLL12_CK_DIV6_MASK, div_sft: APLL12_CK_DIV6_SFT, div_apll_sel_reg: CLK_AUDDIV_0, div_apll_sel_mask_sft: APLL_I2S6_MCK_SEL_MASK_SFT, div_apll_sel_sft: APLL_I2S6_MCK_SEL_SFT },
        x if x == MT8192_I2S7_MCK => mt8192_mck_div { m_sel_id: CLK_TOP_I2S7_M_SEL, div_clk_id: CLK_TOP_APLL12_DIV7, div_pdn_reg: CLK_AUDDIV_0, div_pdn_mask_sft: APLL12_DIV7_PDN_MASK_SFT, div_reg: CLK_AUDDIV_4, div_mask_sft: APLL12_CK_DIV7_MASK_SFT, div_mask: APLL12_CK_DIV7_MASK, div_sft: APLL12_CK_DIV7_SFT, div_apll_sel_reg: CLK_AUDDIV_0, div_apll_sel_mask_sft: APLL_I2S7_MCK_SEL_MASK_SFT, div_apll_sel_sft: APLL_I2S7_MCK_SEL_SFT },
        x if x == MT8192_I2S8_MCK => mt8192_mck_div { m_sel_id: CLK_TOP_I2S8_M_SEL, div_clk_id: CLK_TOP_APLL12_DIV8, div_pdn_reg: CLK_AUDDIV_0, div_pdn_mask_sft: APLL12_DIV8_PDN_MASK_SFT, div_reg: CLK_AUDDIV_4, div_mask_sft: APLL12_CK_DIV8_MASK_SFT, div_mask: APLL12_CK_DIV8_MASK, div_sft: APLL12_CK_DIV8_SFT, div_apll_sel_reg: CLK_AUDDIV_0, div_apll_sel_mask_sft: APLL_I2S8_MCK_SEL_MASK_SFT, div_apll_sel_sft: APLL_I2S8_MCK_SEL_SFT },
        x if x == MT8192_I2S9_MCK => mt8192_mck_div { m_sel_id: CLK_TOP_I2S9_M_SEL, div_clk_id: CLK_TOP_APLL12_DIV9, div_pdn_reg: CLK_AUDDIV_0, div_pdn_mask_sft: APLL12_DIV9_PDN_MASK_SFT, div_reg: CLK_AUDDIV_4, div_mask_sft: APLL12_CK_DIV9_MASK_SFT, div_mask: APLL12_CK_DIV9_MASK, div_sft: APLL12_CK_DIV9_SFT, div_apll_sel_reg: CLK_AUDDIV_0, div_apll_sel_mask_sft: APLL_I2S9_MCK_SEL_MASK_SFT, div_apll_sel_sft: APLL_I2S9_MCK_SEL_SFT },
        _ => mt8192_mck_div { m_sel_id: 0, div_clk_id: 0, div_pdn_reg: 0, div_pdn_mask_sft: 0, div_reg: 0, div_mask_sft: 0, div_mask: 0, div_sft: 0, div_apll_sel_reg: 0, div_apll_sel_mask_sft: 0, div_apll_sel_sft: 0 },
    }
}

#[no_mangle]
pub unsafe extern "C" fn mt8192_mck_enable(
    afe: *mut mtk_base_afe,
    mck_id: c_int,
    rate: c_int,
) -> c_int {
    let afe_priv = afe_priv(afe);
    let apll = mt8192_get_apll_by_rate(afe, rate);
    let apll_clk_id = if apll == MT8192_APLL1 { CLK_TOP_MUX_AUD_1 } else { CLK_TOP_MUX_AUD_2 };
    let div = mck_div_at(mck_id);
    let m_sel_id = div.m_sel_id;
    let div_clk_id = div.div_clk_id;
    let mut ret: c_int;

    /* select apll */
    if m_sel_id >= 0 {
        ret = clk_prepare_enable(clk_at(afe_priv, m_sel_id));
        if ret != 0 {
            dev_err((*afe).dev, b"%s(), clk_prepare_enable %s fail %d\n\0".as_ptr() as *const c_char, b"mt8192_mck_enable\0".as_ptr() as *const c_char, aud_clk(m_sel_id), ret);
            return ret;
        }
        ret = clk_set_parent(clk_at(afe_priv, m_sel_id), clk_at(afe_priv, apll_clk_id));
        if ret != 0 {
            dev_err((*afe).dev, b"%s(), clk_set_parent %s-%s fail %d\n\0".as_ptr() as *const c_char, b"mt8192_mck_enable\0".as_ptr() as *const c_char, aud_clk(m_sel_id), aud_clk(apll_clk_id), ret);
            return ret;
        }
    }

    /* enable div, set rate */
    ret = clk_prepare_enable(clk_at(afe_priv, div_clk_id));
    if ret != 0 {
        dev_err((*afe).dev, b"%s(), clk_prepare_enable %s fail %d\n\0".as_ptr() as *const c_char, b"mt8192_mck_enable\0".as_ptr() as *const c_char, aud_clk(div_clk_id), ret);
        return ret;
    }
    ret = clk_set_rate(clk_at(afe_priv, div_clk_id), rate);
    if ret != 0 {
        dev_err((*afe).dev, b"%s(), clk_set_rate %s, rate %d, fail %d\n\0".as_ptr() as *const c_char, b"mt8192_mck_enable\0".as_ptr() as *const c_char, aud_clk(div_clk_id), rate, ret);
        return ret;
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn mt8192_mck_disable(afe: *mut mtk_base_afe, mck_id: c_int) {
    let afe_priv = afe_priv(afe);
    let div = mck_div_at(mck_id);
    let m_sel_id = div.m_sel_id;
    let div_clk_id = div.div_clk_id;

    clk_disable_unprepare(clk_at(afe_priv, div_clk_id));
    if m_sel_id >= 0 {
        clk_disable_unprepare(clk_at(afe_priv, m_sel_id));
    }
}

#[no_mangle]
pub unsafe extern "C" fn mt8192_init_clock(afe: *mut mtk_base_afe) -> c_int {
    let afe_priv = afe_priv(afe);
    let of_node = (*(*afe).dev).of_node;
    let mut i: c_int = 0;

    (*afe_priv).clk = devm_kcalloc(
        (*afe).dev,
        CLK_NUM as usize,
        core::mem::size_of::<*mut clk>(),
        GFP_KERNEL,
    ) as *mut *mut clk;
    if (*afe_priv).clk.is_null() {
        return -ENOMEM;
    }

    while i < CLK_NUM {
        *(*afe_priv).clk.add(i as usize) = devm_clk_get((*afe).dev, aud_clk(i));
        if IS_ERR(*(*afe_priv).clk.add(i as usize) as *const c_void) {
            dev_warn(
                (*afe).dev,
                b"%s devm_clk_get %s fail, ret %ld\n\0".as_ptr() as *const c_char,
                b"mt8192_init_clock\0".as_ptr() as *const c_char,
                aud_clk(i),
                PTR_ERR(*(*afe_priv).clk.add(i as usize) as *const c_void),
            );
            *(*afe_priv).clk.add(i as usize) = core::ptr::null_mut();
        }
        i += 1;
    }

    (*afe_priv).apmixedsys = syscon_regmap_lookup_by_phandle(
        of_node,
        b"mediatek,apmixedsys\0".as_ptr() as *const c_char,
    );
    if IS_ERR((*afe_priv).apmixedsys as *const c_void) {
        dev_err(
            (*afe).dev,
            b"%s() Cannot find apmixedsys controller: %ld\n\0".as_ptr() as *const c_char,
            b"mt8192_init_clock\0".as_ptr() as *const c_char,
            PTR_ERR((*afe_priv).apmixedsys as *const c_void),
        );
        return PTR_ERR((*afe_priv).apmixedsys as *const c_void) as c_int;
    }

    (*afe_priv).topckgen = syscon_regmap_lookup_by_phandle(
        of_node,
        b"mediatek,topckgen\0".as_ptr() as *const c_char,
    );
    if IS_ERR((*afe_priv).topckgen as *const c_void) {
        dev_err(
            (*afe).dev,
            b"%s() Cannot find topckgen controller: %ld\n\0".as_ptr() as *const c_char,
            b"mt8192_init_clock\0".as_ptr() as *const c_char,
            PTR_ERR((*afe_priv).topckgen as *const c_void),
        );
        return PTR_ERR((*afe_priv).topckgen as *const c_void) as c_int;
    }

    (*afe_priv).infracfg = syscon_regmap_lookup_by_phandle(
        of_node,
        b"mediatek,infracfg\0".as_ptr() as *const c_char,
    );
    if IS_ERR((*afe_priv).infracfg as *const c_void) {
        dev_err(
            (*afe).dev,
            b"%s() Cannot find infracfg: %ld\n\0".as_ptr() as *const c_char,
            b"mt8192_init_clock\0".as_ptr() as *const c_char,
            PTR_ERR((*afe_priv).infracfg as *const c_void),
        );
        return PTR_ERR((*afe_priv).infracfg as *const c_void) as c_int;
    }

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
