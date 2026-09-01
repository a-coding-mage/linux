// SPDX-License-Identifier: GPL-2.0
//
// mt8186-afe-clk.c  --  Mediatek 8186 afe clock ctrl
//
// Copyright (c) 2022 MediaTek Inc.
// Author: Jiaxin Yu <jiaxin.yu@mediatek.com>

// Dependencies from:
// linux/clk.h, linux/regmap.h, linux/mfd/syscon.h
// mt8186-afe-common.h, mt8186-afe-clk.h, mt8186-audsys-clk.h

use core::ffi::{c_char, c_int, c_long, c_void};

static aud_clks: [*const c_char; CLK_NUM as usize] = {
    let mut clks = [core::ptr::null(); CLK_NUM as usize];
    clks[CLK_AFE as usize] = b"aud_afe_clk\0".as_ptr() as *const c_char;
    clks[CLK_DAC as usize] = b"aud_dac_clk\0".as_ptr() as *const c_char;
    clks[CLK_DAC_PREDIS as usize] = b"aud_dac_predis_clk\0".as_ptr() as *const c_char;
    clks[CLK_ADC as usize] = b"aud_adc_clk\0".as_ptr() as *const c_char;
    clks[CLK_TML as usize] = b"aud_tml_clk\0".as_ptr() as *const c_char;
    clks[CLK_APLL22M as usize] = b"aud_apll22m_clk\0".as_ptr() as *const c_char;
    clks[CLK_APLL24M as usize] = b"aud_apll24m_clk\0".as_ptr() as *const c_char;
    clks[CLK_APLL1_TUNER as usize] = b"aud_apll_tuner_clk\0".as_ptr() as *const c_char;
    clks[CLK_APLL2_TUNER as usize] = b"aud_apll2_tuner_clk\0".as_ptr() as *const c_char;
    clks[CLK_TDM as usize] = b"aud_tdm_clk\0".as_ptr() as *const c_char;
    clks[CLK_NLE as usize] = b"aud_nle_clk\0".as_ptr() as *const c_char;
    clks[CLK_DAC_HIRES as usize] = b"aud_dac_hires_clk\0".as_ptr() as *const c_char;
    clks[CLK_ADC_HIRES as usize] = b"aud_adc_hires_clk\0".as_ptr() as *const c_char;
    clks[CLK_I2S1_BCLK as usize] = b"aud_i2s1_bclk\0".as_ptr() as *const c_char;
    clks[CLK_I2S2_BCLK as usize] = b"aud_i2s2_bclk\0".as_ptr() as *const c_char;
    clks[CLK_I2S3_BCLK as usize] = b"aud_i2s3_bclk\0".as_ptr() as *const c_char;
    clks[CLK_I2S4_BCLK as usize] = b"aud_i2s4_bclk\0".as_ptr() as *const c_char;
    clks[CLK_CONNSYS_I2S_ASRC as usize] = b"aud_connsys_i2s_asrc\0".as_ptr() as *const c_char;
    clks[CLK_GENERAL1_ASRC as usize] = b"aud_general1_asrc\0".as_ptr() as *const c_char;
    clks[CLK_GENERAL2_ASRC as usize] = b"aud_general2_asrc\0".as_ptr() as *const c_char;
    clks[CLK_ADC_HIRES_TML as usize] = b"aud_adc_hires_tml\0".as_ptr() as *const c_char;
    clks[CLK_ADDA6_ADC as usize] = b"aud_adda6_adc\0".as_ptr() as *const c_char;
    clks[CLK_ADDA6_ADC_HIRES as usize] = b"aud_adda6_adc_hires\0".as_ptr() as *const c_char;
    clks[CLK_3RD_DAC as usize] = b"aud_3rd_dac\0".as_ptr() as *const c_char;
    clks[CLK_3RD_DAC_PREDIS as usize] = b"aud_3rd_dac_predis\0".as_ptr() as *const c_char;
    clks[CLK_3RD_DAC_TML as usize] = b"aud_3rd_dac_tml\0".as_ptr() as *const c_char;
    clks[CLK_3RD_DAC_HIRES as usize] = b"aud_3rd_dac_hires\0".as_ptr() as *const c_char;
    clks[CLK_ETDM_IN1_BCLK as usize] = b"aud_etdm_in1_bclk\0".as_ptr() as *const c_char;
    clks[CLK_ETDM_OUT1_BCLK as usize] = b"aud_etdm_out1_bclk\0".as_ptr() as *const c_char;
    clks[CLK_INFRA_SYS_AUDIO as usize] = b"aud_infra_clk\0".as_ptr() as *const c_char;
    clks[CLK_INFRA_AUDIO_26M as usize] = b"mtkaif_26m_clk\0".as_ptr() as *const c_char;
    clks[CLK_MUX_AUDIO as usize] = b"top_mux_audio\0".as_ptr() as *const c_char;
    clks[CLK_MUX_AUDIOINTBUS as usize] = b"top_mux_audio_int\0".as_ptr() as *const c_char;
    clks[CLK_TOP_MAINPLL_D2_D4 as usize] = b"top_mainpll_d2_d4\0".as_ptr() as *const c_char;
    clks[CLK_TOP_MUX_AUD_1 as usize] = b"top_mux_aud_1\0".as_ptr() as *const c_char;
    clks[CLK_TOP_APLL1_CK as usize] = b"top_apll1_ck\0".as_ptr() as *const c_char;
    clks[CLK_TOP_MUX_AUD_2 as usize] = b"top_mux_aud_2\0".as_ptr() as *const c_char;
    clks[CLK_TOP_APLL2_CK as usize] = b"top_apll2_ck\0".as_ptr() as *const c_char;
    clks[CLK_TOP_MUX_AUD_ENG1 as usize] = b"top_mux_aud_eng1\0".as_ptr() as *const c_char;
    clks[CLK_TOP_APLL1_D8 as usize] = b"top_apll1_d8\0".as_ptr() as *const c_char;
    clks[CLK_TOP_MUX_AUD_ENG2 as usize] = b"top_mux_aud_eng2\0".as_ptr() as *const c_char;
    clks[CLK_TOP_APLL2_D8 as usize] = b"top_apll2_d8\0".as_ptr() as *const c_char;
    clks[CLK_TOP_MUX_AUDIO_H as usize] = b"top_mux_audio_h\0".as_ptr() as *const c_char;
    clks[CLK_TOP_I2S0_M_SEL as usize] = b"top_i2s0_m_sel\0".as_ptr() as *const c_char;
    clks[CLK_TOP_I2S1_M_SEL as usize] = b"top_i2s1_m_sel\0".as_ptr() as *const c_char;
    clks[CLK_TOP_I2S2_M_SEL as usize] = b"top_i2s2_m_sel\0".as_ptr() as *const c_char;
    clks[CLK_TOP_I2S4_M_SEL as usize] = b"top_i2s4_m_sel\0".as_ptr() as *const c_char;
    clks[CLK_TOP_TDM_M_SEL as usize] = b"top_tdm_m_sel\0".as_ptr() as *const c_char;
    clks[CLK_TOP_APLL12_DIV0 as usize] = b"top_apll12_div0\0".as_ptr() as *const c_char;
    clks[CLK_TOP_APLL12_DIV1 as usize] = b"top_apll12_div1\0".as_ptr() as *const c_char;
    clks[CLK_TOP_APLL12_DIV2 as usize] = b"top_apll12_div2\0".as_ptr() as *const c_char;
    clks[CLK_TOP_APLL12_DIV4 as usize] = b"top_apll12_div4\0".as_ptr() as *const c_char;
    clks[CLK_TOP_APLL12_DIV_TDM as usize] = b"top_apll12_div_tdm\0".as_ptr() as *const c_char;
    clks[CLK_CLK26M as usize] = b"top_clk26m_clk\0".as_ptr() as *const c_char;
    clks
};

pub unsafe extern "C" fn mt8186_set_audio_int_bus_parent(
    afe: *mut mtk_base_afe,
    clk_id: c_int,
) -> c_int {
    let afe_priv = (*afe).platform_priv as *mut mt8186_afe_private;
    let mut ret: c_int;

    ret = clk_set_parent(
        *(*afe_priv).clk.add(CLK_MUX_AUDIOINTBUS as usize),
        *(*afe_priv).clk.add(clk_id as usize),
    );
    if ret != 0 {
        dev_err!(
            (*afe).dev,
            "%s clk_set_parent %s-%s fail %d\n",
            c"mt8186_set_audio_int_bus_parent".as_ptr(),
            aud_clks[CLK_MUX_AUDIOINTBUS as usize],
            aud_clks[clk_id as usize],
            ret
        );
        return ret;
    }

    0
}

unsafe fn apll1_mux_setting(afe: *mut mtk_base_afe, enable: bool) -> c_int {
    let afe_priv = (*afe).platform_priv as *mut mt8186_afe_private;
    let mut ret: c_int;

    if enable {
        ret = clk_prepare_enable(*(*afe_priv).clk.add(CLK_TOP_MUX_AUD_1 as usize));
        if ret != 0 {
            dev_err!((*afe).dev, "%s clk_prepare_enable %s fail %d\n",
                c"apll1_mux_setting".as_ptr(), aud_clks[CLK_TOP_MUX_AUD_1 as usize], ret);
            return ret;
        }
        ret = clk_set_parent(
            *(*afe_priv).clk.add(CLK_TOP_MUX_AUD_1 as usize),
            *(*afe_priv).clk.add(CLK_TOP_APLL1_CK as usize),
        );
        if ret != 0 {
            dev_err!((*afe).dev, "%s clk_set_parent %s-%s fail %d\n",
                c"apll1_mux_setting".as_ptr(), aud_clks[CLK_TOP_MUX_AUD_1 as usize],
                aud_clks[CLK_TOP_APLL1_CK as usize], ret);
            return ret;
        }

        /* 180.6336 / 8 = 22.5792MHz */
        ret = clk_prepare_enable(*(*afe_priv).clk.add(CLK_TOP_MUX_AUD_ENG1 as usize));
        if ret != 0 {
            dev_err!((*afe).dev, "%s clk_prepare_enable %s fail %d\n",
                c"apll1_mux_setting".as_ptr(), aud_clks[CLK_TOP_MUX_AUD_ENG1 as usize], ret);
            return ret;
        }
        ret = clk_set_parent(
            *(*afe_priv).clk.add(CLK_TOP_MUX_AUD_ENG1 as usize),
            *(*afe_priv).clk.add(CLK_TOP_APLL1_D8 as usize),
        );
        if ret != 0 {
            dev_err!((*afe).dev, "%s clk_set_parent %s-%s fail %d\n",
                c"apll1_mux_setting".as_ptr(), aud_clks[CLK_TOP_MUX_AUD_ENG1 as usize],
                aud_clks[CLK_TOP_APLL1_D8 as usize], ret);
            return ret;
        }
    } else {
        ret = clk_set_parent(
            *(*afe_priv).clk.add(CLK_TOP_MUX_AUD_ENG1 as usize),
            *(*afe_priv).clk.add(CLK_CLK26M as usize),
        );
        if ret != 0 {
            dev_err!((*afe).dev, "%s clk_set_parent %s-%s fail %d\n",
                c"apll1_mux_setting".as_ptr(), aud_clks[CLK_TOP_MUX_AUD_ENG1 as usize],
                aud_clks[CLK_CLK26M as usize], ret);
            return ret;
        }
        clk_disable_unprepare(*(*afe_priv).clk.add(CLK_TOP_MUX_AUD_ENG1 as usize));

        ret = clk_set_parent(
            *(*afe_priv).clk.add(CLK_TOP_MUX_AUD_1 as usize),
            *(*afe_priv).clk.add(CLK_CLK26M as usize),
        );
        if ret != 0 {
            dev_err!((*afe).dev, "%s clk_set_parent %s-%s fail %d\n",
                c"apll1_mux_setting".as_ptr(), aud_clks[CLK_TOP_MUX_AUD_1 as usize],
                aud_clks[CLK_CLK26M as usize], ret);
            return ret;
        }
        clk_disable_unprepare(*(*afe_priv).clk.add(CLK_TOP_MUX_AUD_1 as usize));
    }

    0
}

unsafe fn apll2_mux_setting(afe: *mut mtk_base_afe, enable: bool) -> c_int {
    let afe_priv = (*afe).platform_priv as *mut mt8186_afe_private;
    let mut ret: c_int;

    if enable {
        ret = clk_prepare_enable(*(*afe_priv).clk.add(CLK_TOP_MUX_AUD_2 as usize));
        if ret != 0 {
            dev_err!((*afe).dev, "%s clk_prepare_enable %s fail %d\n",
                c"apll2_mux_setting".as_ptr(), aud_clks[CLK_TOP_MUX_AUD_2 as usize], ret);
            return ret;
        }
        ret = clk_set_parent(
            *(*afe_priv).clk.add(CLK_TOP_MUX_AUD_2 as usize),
            *(*afe_priv).clk.add(CLK_TOP_APLL2_CK as usize),
        );
        if ret != 0 {
            dev_err!((*afe).dev, "%s clk_set_parent %s-%s fail %d\n",
                c"apll2_mux_setting".as_ptr(), aud_clks[CLK_TOP_MUX_AUD_2 as usize],
                aud_clks[CLK_TOP_APLL2_CK as usize], ret);
            return ret;
        }

        /* 196.608 / 8 = 24.576MHz */
        ret = clk_prepare_enable(*(*afe_priv).clk.add(CLK_TOP_MUX_AUD_ENG2 as usize));
        if ret != 0 {
            dev_err!((*afe).dev, "%s clk_prepare_enable %s fail %d\n",
                c"apll2_mux_setting".as_ptr(), aud_clks[CLK_TOP_MUX_AUD_ENG2 as usize], ret);
            return ret;
        }
        ret = clk_set_parent(
            *(*afe_priv).clk.add(CLK_TOP_MUX_AUD_ENG2 as usize),
            *(*afe_priv).clk.add(CLK_TOP_APLL2_D8 as usize),
        );
        if ret != 0 {
            dev_err!((*afe).dev, "%s clk_set_parent %s-%s fail %d\n",
                c"apll2_mux_setting".as_ptr(), aud_clks[CLK_TOP_MUX_AUD_ENG2 as usize],
                aud_clks[CLK_TOP_APLL2_D8 as usize], ret);
            return ret;
        }
    } else {
        ret = clk_set_parent(
            *(*afe_priv).clk.add(CLK_TOP_MUX_AUD_ENG2 as usize),
            *(*afe_priv).clk.add(CLK_CLK26M as usize),
        );
        if ret != 0 {
            dev_err!((*afe).dev, "%s clk_set_parent %s-%s fail %d\n",
                c"apll2_mux_setting".as_ptr(), aud_clks[CLK_TOP_MUX_AUD_ENG2 as usize],
                aud_clks[CLK_CLK26M as usize], ret);
            return ret;
        }
        clk_disable_unprepare(*(*afe_priv).clk.add(CLK_TOP_MUX_AUD_ENG2 as usize));

        ret = clk_set_parent(
            *(*afe_priv).clk.add(CLK_TOP_MUX_AUD_2 as usize),
            *(*afe_priv).clk.add(CLK_CLK26M as usize),
        );
        if ret != 0 {
            dev_err!((*afe).dev, "%s clk_set_parent %s-%s fail %d\n",
                c"apll2_mux_setting".as_ptr(), aud_clks[CLK_TOP_MUX_AUD_2 as usize],
                aud_clks[CLK_CLK26M as usize], ret);
            return ret;
        }
        clk_disable_unprepare(*(*afe_priv).clk.add(CLK_TOP_MUX_AUD_2 as usize));
    }

    0
}

pub unsafe extern "C" fn mt8186_afe_enable_cgs(afe: *mut mtk_base_afe) -> c_int {
    let afe_priv = (*afe).platform_priv as *mut mt8186_afe_private;
    let mut ret: c_int = 0;
    let mut i: c_int;

    i = CLK_I2S1_BCLK;
    while i <= CLK_ETDM_OUT1_BCLK {
        ret = clk_prepare_enable(*(*afe_priv).clk.add(i as usize));
        if ret != 0 {
            dev_err!((*afe).dev, "%s clk_prepare_enable %s fail %d\n",
                c"mt8186_afe_enable_cgs".as_ptr(), aud_clks[i as usize], ret);
            return ret;
        }
        i += 1;
    }

    0
}

pub unsafe extern "C" fn mt8186_afe_disable_cgs(afe: *mut mtk_base_afe) {
    let afe_priv = (*afe).platform_priv as *mut mt8186_afe_private;
    let mut i: c_int;

    i = CLK_I2S1_BCLK;
    while i <= CLK_ETDM_OUT1_BCLK {
        clk_disable_unprepare(*(*afe_priv).clk.add(i as usize));
        i += 1;
    }
}

pub unsafe extern "C" fn mt8186_afe_enable_clock(afe: *mut mtk_base_afe) -> c_int {
    let afe_priv = (*afe).platform_priv as *mut mt8186_afe_private;
    let mut ret: c_int = 0;

    ret = clk_prepare_enable(*(*afe_priv).clk.add(CLK_INFRA_SYS_AUDIO as usize));
    if ret != 0 {
        dev_err!((*afe).dev, "%s clk_prepare_enable %s fail %d\n",
            c"mt8186_afe_enable_clock".as_ptr(), aud_clks[CLK_INFRA_SYS_AUDIO as usize], ret);
        return ret;
    }

    ret = clk_prepare_enable(*(*afe_priv).clk.add(CLK_INFRA_AUDIO_26M as usize));
    if ret != 0 {
        dev_err!((*afe).dev, "%s clk_prepare_enable %s fail %d\n",
            c"mt8186_afe_enable_clock".as_ptr(), aud_clks[CLK_INFRA_AUDIO_26M as usize], ret);
        clk_disable_unprepare(*(*afe_priv).clk.add(CLK_INFRA_AUDIO_26M as usize));
        return ret;
    }

    ret = clk_prepare_enable(*(*afe_priv).clk.add(CLK_MUX_AUDIO as usize));
    if ret != 0 {
        dev_err!((*afe).dev, "%s clk_prepare_enable %s fail %d\n",
            c"mt8186_afe_enable_clock".as_ptr(), aud_clks[CLK_MUX_AUDIO as usize], ret);
        clk_disable_unprepare(*(*afe_priv).clk.add(CLK_MUX_AUDIO as usize));
        clk_disable_unprepare(*(*afe_priv).clk.add(CLK_INFRA_SYS_AUDIO as usize));
        clk_disable_unprepare(*(*afe_priv).clk.add(CLK_INFRA_AUDIO_26M as usize));
        return ret;
    }
    ret = clk_set_parent(
        *(*afe_priv).clk.add(CLK_MUX_AUDIO as usize),
        *(*afe_priv).clk.add(CLK_CLK26M as usize),
    );
    if ret != 0 {
        dev_err!((*afe).dev, "%s clk_set_parent %s-%s fail %d\n",
            c"mt8186_afe_enable_clock".as_ptr(), aud_clks[CLK_MUX_AUDIO as usize],
            aud_clks[CLK_CLK26M as usize], ret);
        clk_disable_unprepare(*(*afe_priv).clk.add(CLK_MUX_AUDIO as usize));
        clk_disable_unprepare(*(*afe_priv).clk.add(CLK_INFRA_SYS_AUDIO as usize));
        clk_disable_unprepare(*(*afe_priv).clk.add(CLK_INFRA_AUDIO_26M as usize));
        return ret;
    }

    ret = clk_prepare_enable(*(*afe_priv).clk.add(CLK_MUX_AUDIOINTBUS as usize));
    if ret != 0 {
        dev_err!((*afe).dev, "%s clk_prepare_enable %s fail %d\n",
            c"mt8186_afe_enable_clock".as_ptr(), aud_clks[CLK_MUX_AUDIOINTBUS as usize], ret);
        clk_disable_unprepare(*(*afe_priv).clk.add(CLK_MUX_AUDIOINTBUS as usize));
        clk_disable_unprepare(*(*afe_priv).clk.add(CLK_MUX_AUDIO as usize));
        clk_disable_unprepare(*(*afe_priv).clk.add(CLK_INFRA_SYS_AUDIO as usize));
        clk_disable_unprepare(*(*afe_priv).clk.add(CLK_INFRA_AUDIO_26M as usize));
        return ret;
    }
    ret = mt8186_set_audio_int_bus_parent(afe, CLK_TOP_MAINPLL_D2_D4);
    if ret != 0 {
        mt8186_set_audio_int_bus_parent(afe, CLK_CLK26M);
        clk_disable_unprepare(*(*afe_priv).clk.add(CLK_MUX_AUDIOINTBUS as usize));
        clk_disable_unprepare(*(*afe_priv).clk.add(CLK_MUX_AUDIO as usize));
        clk_disable_unprepare(*(*afe_priv).clk.add(CLK_INFRA_SYS_AUDIO as usize));
        clk_disable_unprepare(*(*afe_priv).clk.add(CLK_INFRA_AUDIO_26M as usize));
        return ret;
    }

    ret = clk_set_parent(
        *(*afe_priv).clk.add(CLK_TOP_MUX_AUDIO_H as usize),
        *(*afe_priv).clk.add(CLK_TOP_APLL2_CK as usize),
    );
    if ret != 0 {
        dev_err!((*afe).dev, "%s clk_set_parent %s-%s fail %d\n",
            c"mt8186_afe_enable_clock".as_ptr(), aud_clks[CLK_TOP_MUX_AUDIO_H as usize],
            aud_clks[CLK_TOP_APLL2_CK as usize], ret);
        mt8186_set_audio_int_bus_parent(afe, CLK_CLK26M);
        clk_disable_unprepare(*(*afe_priv).clk.add(CLK_MUX_AUDIOINTBUS as usize));
        clk_disable_unprepare(*(*afe_priv).clk.add(CLK_MUX_AUDIO as usize));
        clk_disable_unprepare(*(*afe_priv).clk.add(CLK_INFRA_SYS_AUDIO as usize));
        clk_disable_unprepare(*(*afe_priv).clk.add(CLK_INFRA_AUDIO_26M as usize));
        return ret;
    }

    ret = clk_prepare_enable(*(*afe_priv).clk.add(CLK_AFE as usize));
    if ret != 0 {
        dev_err!((*afe).dev, "%s clk_prepare_enable %s fail %d\n",
            c"mt8186_afe_enable_clock".as_ptr(), aud_clks[CLK_AFE as usize], ret);
        clk_disable_unprepare(*(*afe_priv).clk.add(CLK_AFE as usize));
        mt8186_set_audio_int_bus_parent(afe, CLK_CLK26M);
        clk_disable_unprepare(*(*afe_priv).clk.add(CLK_MUX_AUDIOINTBUS as usize));
        clk_disable_unprepare(*(*afe_priv).clk.add(CLK_MUX_AUDIO as usize));
        clk_disable_unprepare(*(*afe_priv).clk.add(CLK_INFRA_SYS_AUDIO as usize));
        clk_disable_unprepare(*(*afe_priv).clk.add(CLK_INFRA_AUDIO_26M as usize));
        return ret;
    }

    0
}

pub unsafe extern "C" fn mt8186_afe_disable_clock(afe: *mut mtk_base_afe) {
    let afe_priv = (*afe).platform_priv as *mut mt8186_afe_private;

    clk_disable_unprepare(*(*afe_priv).clk.add(CLK_AFE as usize));
    mt8186_set_audio_int_bus_parent(afe, CLK_CLK26M);
    clk_disable_unprepare(*(*afe_priv).clk.add(CLK_MUX_AUDIOINTBUS as usize));
    clk_disable_unprepare(*(*afe_priv).clk.add(CLK_MUX_AUDIO as usize));
    clk_disable_unprepare(*(*afe_priv).clk.add(CLK_INFRA_AUDIO_26M as usize));
    clk_disable_unprepare(*(*afe_priv).clk.add(CLK_INFRA_SYS_AUDIO as usize));
}

pub unsafe extern "C" fn mt8186_apll1_enable(afe: *mut mtk_base_afe) -> c_int {
    let afe_priv = (*afe).platform_priv as *mut mt8186_afe_private;
    let mut ret: c_int;

    /* setting for APLL */
    apll1_mux_setting(afe, true);

    ret = clk_prepare_enable(*(*afe_priv).clk.add(CLK_APLL22M as usize));
    if ret != 0 {
        dev_err!((*afe).dev, "%s clk_prepare_enable %s fail %d\n",
            c"mt8186_apll1_enable".as_ptr(), aud_clks[CLK_APLL22M as usize], ret);
        clk_disable_unprepare(*(*afe_priv).clk.add(CLK_APLL22M as usize));
        return ret;
    }

    ret = clk_prepare_enable(*(*afe_priv).clk.add(CLK_APLL1_TUNER as usize));
    if ret != 0 {
        dev_err!((*afe).dev, "%s clk_prepare_enable %s fail %d\n",
            c"mt8186_apll1_enable".as_ptr(), aud_clks[CLK_APLL1_TUNER as usize], ret);
        clk_disable_unprepare(*(*afe_priv).clk.add(CLK_APLL1_TUNER as usize));
        clk_disable_unprepare(*(*afe_priv).clk.add(CLK_APLL22M as usize));
        return ret;
    }

    regmap_update_bits((*afe).regmap, AFE_APLL1_TUNER_CFG, 0xfff7, 0x832);
    regmap_update_bits((*afe).regmap, AFE_APLL1_TUNER_CFG, 0x1, 0x1);

    regmap_update_bits(
        (*afe).regmap,
        AFE_HD_ENGEN_ENABLE,
        AFE_22M_ON_MASK_SFT,
        BIT(AFE_22M_ON_SFT),
    );

    0
}

pub unsafe extern "C" fn mt8186_apll1_disable(afe: *mut mtk_base_afe) {
    let afe_priv = (*afe).platform_priv as *mut mt8186_afe_private;

    regmap_update_bits((*afe).regmap, AFE_HD_ENGEN_ENABLE, AFE_22M_ON_MASK_SFT, 0);

    regmap_update_bits((*afe).regmap, AFE_APLL1_TUNER_CFG, 0x1, 0);

    clk_disable_unprepare(*(*afe_priv).clk.add(CLK_APLL1_TUNER as usize));
    clk_disable_unprepare(*(*afe_priv).clk.add(CLK_APLL22M as usize));

    apll1_mux_setting(afe, false);
}

pub unsafe extern "C" fn mt8186_apll2_enable(afe: *mut mtk_base_afe) -> c_int {
    let afe_priv = (*afe).platform_priv as *mut mt8186_afe_private;
    let mut ret: c_int;

    /* setting for APLL */
    apll2_mux_setting(afe, true);

    ret = clk_prepare_enable(*(*afe_priv).clk.add(CLK_APLL24M as usize));
    if ret != 0 {
        dev_err!((*afe).dev, "%s clk_prepare_enable %s fail %d\n",
            c"mt8186_apll2_enable".as_ptr(), aud_clks[CLK_APLL24M as usize], ret);
        clk_disable_unprepare(*(*afe_priv).clk.add(CLK_APLL24M as usize));
        return ret;
    }

    ret = clk_prepare_enable(*(*afe_priv).clk.add(CLK_APLL2_TUNER as usize));
    if ret != 0 {
        dev_err!((*afe).dev, "%s clk_prepare_enable %s fail %d\n",
            c"mt8186_apll2_enable".as_ptr(), aud_clks[CLK_APLL2_TUNER as usize], ret);
        clk_disable_unprepare(*(*afe_priv).clk.add(CLK_APLL2_TUNER as usize));
        clk_disable_unprepare(*(*afe_priv).clk.add(CLK_APLL24M as usize));
        return ret;
    }

    regmap_update_bits((*afe).regmap, AFE_APLL2_TUNER_CFG, 0xfff7, 0x634);
    regmap_update_bits((*afe).regmap, AFE_APLL2_TUNER_CFG, 0x1, 0x1);

    regmap_update_bits(
        (*afe).regmap,
        AFE_HD_ENGEN_ENABLE,
        AFE_24M_ON_MASK_SFT,
        BIT(AFE_24M_ON_SFT),
    );

    0
}

pub unsafe extern "C" fn mt8186_apll2_disable(afe: *mut mtk_base_afe) {
    let afe_priv = (*afe).platform_priv as *mut mt8186_afe_private;

    regmap_update_bits((*afe).regmap, AFE_HD_ENGEN_ENABLE, AFE_24M_ON_MASK_SFT, 0);

    regmap_update_bits((*afe).regmap, AFE_APLL2_TUNER_CFG, 0x1, 0);

    clk_disable_unprepare(*(*afe_priv).clk.add(CLK_APLL2_TUNER as usize));
    clk_disable_unprepare(*(*afe_priv).clk.add(CLK_APLL24M as usize));

    apll2_mux_setting(afe, false);
}

pub unsafe extern "C" fn mt8186_get_apll_rate(
    _afe: *mut mtk_base_afe,
    apll: c_int,
) -> c_int {
    if apll == MT8186_APLL1 {
        180633600
    } else {
        196608000
    }
}

pub unsafe extern "C" fn mt8186_get_apll_by_rate(
    _afe: *mut mtk_base_afe,
    rate: c_int,
) -> c_int {
    if rate % 8000 == 0 {
        MT8186_APLL2
    } else {
        MT8186_APLL1
    }
}

pub unsafe extern "C" fn mt8186_get_apll_by_name(
    _afe: *mut mtk_base_afe,
    name: *const c_char,
) -> c_int {
    if strcmp(name, APLL1_W_NAME) == 0 {
        return MT8186_APLL1;
    }

    MT8186_APLL2
}

/* mck */
#[repr(C)]
struct mt8186_mck_div {
    m_sel_id: u32,
    div_clk_id: u32,
}

static mck_div: [mt8186_mck_div; MT8186_MCK_NUM as usize] = {
    let mut div = [mt8186_mck_div { m_sel_id: 0, div_clk_id: 0 }; MT8186_MCK_NUM as usize];
    div[MT8186_I2S0_MCK as usize] = mt8186_mck_div {
        m_sel_id: CLK_TOP_I2S0_M_SEL as u32,
        div_clk_id: CLK_TOP_APLL12_DIV0 as u32,
    };
    div[MT8186_I2S1_MCK as usize] = mt8186_mck_div {
        m_sel_id: CLK_TOP_I2S1_M_SEL as u32,
        div_clk_id: CLK_TOP_APLL12_DIV1 as u32,
    };
    div[MT8186_I2S2_MCK as usize] = mt8186_mck_div {
        m_sel_id: CLK_TOP_I2S2_M_SEL as u32,
        div_clk_id: CLK_TOP_APLL12_DIV2 as u32,
    };
    div[MT8186_I2S4_MCK as usize] = mt8186_mck_div {
        m_sel_id: CLK_TOP_I2S4_M_SEL as u32,
        div_clk_id: CLK_TOP_APLL12_DIV4 as u32,
    };
    div[MT8186_TDM_MCK as usize] = mt8186_mck_div {
        m_sel_id: CLK_TOP_TDM_M_SEL as u32,
        div_clk_id: CLK_TOP_APLL12_DIV_TDM as u32,
    };
    div
};

pub unsafe extern "C" fn mt8186_mck_enable(
    afe: *mut mtk_base_afe,
    mck_id: c_int,
    rate: c_int,
) -> c_int {
    let afe_priv = (*afe).platform_priv as *mut mt8186_afe_private;
    let apll = mt8186_get_apll_by_rate(afe, rate);
    let apll_clk_id = if apll == MT8186_APLL1 {
        CLK_TOP_MUX_AUD_1
    } else {
        CLK_TOP_MUX_AUD_2
    };
    let m_sel_id = mck_div[mck_id as usize].m_sel_id as c_int;
    let div_clk_id = mck_div[mck_id as usize].div_clk_id as c_int;
    let mut ret: c_int;

    /* select apll */
    if m_sel_id >= 0 {
        ret = clk_prepare_enable(*(*afe_priv).clk.add(m_sel_id as usize));
        if ret != 0 {
            dev_err!((*afe).dev, "%s(), clk_prepare_enable %s fail %d\n",
                c"mt8186_mck_enable".as_ptr(), aud_clks[m_sel_id as usize], ret);
            return ret;
        }
        ret = clk_set_parent(
            *(*afe_priv).clk.add(m_sel_id as usize),
            *(*afe_priv).clk.add(apll_clk_id as usize),
        );
        if ret != 0 {
            dev_err!((*afe).dev, "%s(), clk_set_parent %s-%s fail %d\n",
                c"mt8186_mck_enable".as_ptr(), aud_clks[m_sel_id as usize],
                aud_clks[apll_clk_id as usize], ret);
            return ret;
        }
    }

    /* enable div, set rate */
    ret = clk_prepare_enable(*(*afe_priv).clk.add(div_clk_id as usize));
    if ret != 0 {
        dev_err!((*afe).dev, "%s(), clk_prepare_enable %s fail %d\n",
            c"mt8186_mck_enable".as_ptr(), aud_clks[div_clk_id as usize], ret);
        return ret;
    }
    ret = clk_set_rate(*(*afe_priv).clk.add(div_clk_id as usize), rate as u64);
    if ret != 0 {
        dev_err!((*afe).dev, "%s(), clk_set_rate %s, rate %d, fail %d\n",
            c"mt8186_mck_enable".as_ptr(), aud_clks[div_clk_id as usize], rate, ret);
        return ret;
    }

    0
}

pub unsafe extern "C" fn mt8186_mck_disable(afe: *mut mtk_base_afe, mck_id: c_int) {
    let afe_priv = (*afe).platform_priv as *mut mt8186_afe_private;
    let m_sel_id = mck_div[mck_id as usize].m_sel_id as c_int;
    let div_clk_id = mck_div[mck_id as usize].div_clk_id as c_int;

    clk_disable_unprepare(*(*afe_priv).clk.add(div_clk_id as usize));
    if m_sel_id >= 0 {
        clk_disable_unprepare(*(*afe_priv).clk.add(m_sel_id as usize));
    }
}

pub unsafe extern "C" fn mt8186_init_clock(afe: *mut mtk_base_afe) -> c_int {
    let afe_priv = (*afe).platform_priv as *mut mt8186_afe_private;
    let of_node = (*(*afe).dev).of_node;
    let mut i: c_int = 0;

    mt8186_audsys_clk_register(afe);

    (*afe_priv).clk = devm_kcalloc(
        (*afe).dev,
        CLK_NUM as usize,
        core::mem::size_of::<*mut clk>() as usize,
        GFP_KERNEL,
    ) as *mut *mut clk;
    if (*afe_priv).clk.is_null() {
        return -ENOMEM;
    }

    while i < CLK_NUM {
        *(*afe_priv).clk.add(i as usize) = devm_clk_get((*afe).dev, aud_clks[i as usize]);
        if IS_ERR(*(*afe_priv).clk.add(i as usize) as *const c_void) {
            dev_err!((*afe).dev, "%s devm_clk_get %s fail, ret %ld\n",
                c"mt8186_init_clock".as_ptr(), aud_clks[i as usize],
                PTR_ERR(*(*afe_priv).clk.add(i as usize) as *const c_void));
            *(*afe_priv).clk.add(i as usize) = core::ptr::null_mut();
        }
        i += 1;
    }

    (*afe_priv).apmixedsys =
        syscon_regmap_lookup_by_phandle(of_node, b"mediatek,apmixedsys\0".as_ptr() as *const c_char);
    if IS_ERR((*afe_priv).apmixedsys as *const c_void) {
        dev_err!((*afe).dev, "%s() Cannot find apmixedsys controller: %ld\n",
            c"mt8186_init_clock".as_ptr(), PTR_ERR((*afe_priv).apmixedsys as *const c_void));
        return PTR_ERR((*afe_priv).apmixedsys as *const c_void) as c_int;
    }

    (*afe_priv).topckgen =
        syscon_regmap_lookup_by_phandle(of_node, b"mediatek,topckgen\0".as_ptr() as *const c_char);
    if IS_ERR((*afe_priv).topckgen as *const c_void) {
        dev_err!((*afe).dev, "%s() Cannot find topckgen controller: %ld\n",
            c"mt8186_init_clock".as_ptr(), PTR_ERR((*afe_priv).topckgen as *const c_void));
        return PTR_ERR((*afe_priv).topckgen as *const c_void) as c_int;
    }

    (*afe_priv).infracfg =
        syscon_regmap_lookup_by_phandle(of_node, b"mediatek,infracfg\0".as_ptr() as *const c_char);
    if IS_ERR((*afe_priv).infracfg as *const c_void) {
        dev_err!((*afe).dev, "%s() Cannot find infracfg: %ld\n",
            c"mt8186_init_clock".as_ptr(), PTR_ERR((*afe_priv).infracfg as *const c_void));
        return PTR_ERR((*afe_priv).infracfg as *const c_void) as c_int;
    }

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
