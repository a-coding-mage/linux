// SPDX-License-Identifier: GPL-2.0
/*
 * mt2701-afe-clock-ctrl.c  --  Mediatek 2701 afe clock ctrl
 *
 * Copyright (c) 2016 MediaTek Inc.
 * Author: Garlic Tseng <garlic.tseng@mediatek.com>
 *	   Ryder Lee <ryder.lee@mediatek.com>
 */

// Depends on declarations from:
// "mt2701-afe-common.h"
// "mt2701-afe-clock-ctrl.h"

use core::ffi::{c_char, c_int};

static BASE_CLKS: [*const c_char; MT2701_BASE_CLK_NUM as usize] = {
    let mut base_clks = [core::ptr::null(); MT2701_BASE_CLK_NUM as usize];
    base_clks[MT2701_INFRA_SYS_AUDIO as usize] = b"infra_sys_audio_clk\0".as_ptr() as *const c_char;
    base_clks[MT2701_TOP_AUD_MCLK_SRC0 as usize] = b"top_audio_mux1_sel\0".as_ptr() as *const c_char;
    base_clks[MT2701_TOP_AUD_MCLK_SRC1 as usize] = b"top_audio_mux2_sel\0".as_ptr() as *const c_char;
    base_clks[MT2701_TOP_AUD_A1SYS as usize] = b"top_audio_a1sys_hp\0".as_ptr() as *const c_char;
    base_clks[MT2701_TOP_AUD_A2SYS as usize] = b"top_audio_a2sys_hp\0".as_ptr() as *const c_char;
    base_clks[MT2701_AUDSYS_AFE as usize] = b"audio_afe_pd\0".as_ptr() as *const c_char;
    base_clks[MT2701_AUDSYS_AFE_CONN as usize] = b"audio_afe_conn_pd\0".as_ptr() as *const c_char;
    base_clks[MT2701_AUDSYS_A1SYS as usize] = b"audio_a1sys_pd\0".as_ptr() as *const c_char;
    base_clks[MT2701_AUDSYS_A2SYS as usize] = b"audio_a2sys_pd\0".as_ptr() as *const c_char;
    base_clks
};

pub unsafe extern "C" fn mt2701_init_clock(afe: *mut mtk_base_afe) -> c_int {
    let afe_priv = (*afe).platform_priv as *mut mt2701_afe_private;
    let mut i: c_int;

    i = 0;
    while i < MT2701_BASE_CLK_NUM {
        (*afe_priv).base_ck[i as usize] = devm_clk_get((*afe).dev, BASE_CLKS[i as usize]);
        if IS_ERR((*afe_priv).base_ck[i as usize] as *const _) != 0 {
            dev_err((*afe).dev, b"failed to get %s\n\0".as_ptr() as *const c_char, BASE_CLKS[i as usize]);
            return PTR_ERR((*afe_priv).base_ck[i as usize] as *const _) as c_int;
        }
        i += 1;
    }

    let i2s_num = min((*(*afe_priv).soc).i2s_num, MT2701_BASE_CLK_NUM);
    /* Get I2S related clocks */
    i = 0;
    while i < i2s_num {
        let i2s_path = &mut (*afe_priv).i2s_path[i as usize] as *mut mt2701_i2s_path;
        let mut i2s_ck: *mut clk;
        let mut name = [0 as c_char; 13];

        snprintf(
            name.as_mut_ptr(),
            core::mem::size_of_val(&name),
            b"i2s%d_src_sel\0".as_ptr() as *const c_char,
            i,
        );
        (*i2s_path).sel_ck = devm_clk_get((*afe).dev, name.as_ptr());
        if IS_ERR((*i2s_path).sel_ck as *const _) != 0 {
            dev_err((*afe).dev, b"failed to get %s\n\0".as_ptr() as *const c_char, name.as_ptr());
            return PTR_ERR((*i2s_path).sel_ck as *const _) as c_int;
        }

        snprintf(
            name.as_mut_ptr(),
            core::mem::size_of_val(&name),
            b"i2s%d_src_div\0".as_ptr() as *const c_char,
            i,
        );
        (*i2s_path).div_ck = devm_clk_get((*afe).dev, name.as_ptr());
        if IS_ERR((*i2s_path).div_ck as *const _) != 0 {
            dev_err((*afe).dev, b"failed to get %s\n\0".as_ptr() as *const c_char, name.as_ptr());
            return PTR_ERR((*i2s_path).div_ck as *const _) as c_int;
        }

        snprintf(
            name.as_mut_ptr(),
            core::mem::size_of_val(&name),
            b"i2s%d_mclk_en\0".as_ptr() as *const c_char,
            i,
        );
        (*i2s_path).mclk_ck = devm_clk_get((*afe).dev, name.as_ptr());
        if IS_ERR((*i2s_path).mclk_ck as *const _) != 0 {
            dev_err((*afe).dev, b"failed to get %s\n\0".as_ptr() as *const c_char, name.as_ptr());
            return PTR_ERR((*i2s_path).mclk_ck as *const _) as c_int;
        }

        snprintf(
            name.as_mut_ptr(),
            core::mem::size_of_val(&name),
            b"i2so%d_hop_ck\0".as_ptr() as *const c_char,
            i,
        );
        i2s_ck = devm_clk_get((*afe).dev, name.as_ptr());
        if IS_ERR(i2s_ck as *const _) != 0 {
            dev_err((*afe).dev, b"failed to get %s\n\0".as_ptr() as *const c_char, name.as_ptr());
            return PTR_ERR(i2s_ck as *const _) as c_int;
        }
        (*i2s_path).hop_ck[SNDRV_PCM_STREAM_PLAYBACK as usize] = i2s_ck;

        snprintf(
            name.as_mut_ptr(),
            core::mem::size_of_val(&name),
            b"i2si%d_hop_ck\0".as_ptr() as *const c_char,
            i,
        );
        i2s_ck = devm_clk_get((*afe).dev, name.as_ptr());
        if IS_ERR(i2s_ck as *const _) != 0 {
            dev_err((*afe).dev, b"failed to get %s\n\0".as_ptr() as *const c_char, name.as_ptr());
            return PTR_ERR(i2s_ck as *const _) as c_int;
        }
        (*i2s_path).hop_ck[SNDRV_PCM_STREAM_CAPTURE as usize] = i2s_ck;

        snprintf(
            name.as_mut_ptr(),
            core::mem::size_of_val(&name),
            b"asrc%d_out_ck\0".as_ptr() as *const c_char,
            i,
        );
        (*i2s_path).asrco_ck = devm_clk_get((*afe).dev, name.as_ptr());
        if IS_ERR((*i2s_path).asrco_ck as *const _) != 0 {
            dev_err((*afe).dev, b"failed to get %s\n\0".as_ptr() as *const c_char, name.as_ptr());
            return PTR_ERR((*i2s_path).asrco_ck as *const _) as c_int;
        }

        i += 1;
    }

    /* Some platforms may support BT path */
    (*afe_priv).mrgif_ck = devm_clk_get((*afe).dev, b"audio_mrgif_pd\0".as_ptr() as *const c_char);
    if IS_ERR((*afe_priv).mrgif_ck as *const _) != 0 {
        if PTR_ERR((*afe_priv).mrgif_ck as *const _) == -EPROBE_DEFER as isize {
            return -EPROBE_DEFER;
        }

        (*afe_priv).mrgif_ck = core::ptr::null_mut();
    }

    /*
     * Optional HDMI audio clocks. Platforms that do not wire up the
     * HDMI output (e.g. MT2701 devkits using only the I2S BE DAIs)
     * may omit these; in that case the HDMI BE DAI simply cannot be
     * enabled, but the rest of the AFE still probes.
     */
    (*afe_priv).hadds2pll_ck = devm_clk_get_optional((*afe).dev, b"hadds2pll_294m\0".as_ptr() as *const c_char);
    if IS_ERR((*afe_priv).hadds2pll_ck as *const _) != 0 {
        return PTR_ERR((*afe_priv).hadds2pll_ck as *const _) as c_int;
    }

    (*afe_priv).audio_hdmi_ck = devm_clk_get_optional((*afe).dev, b"audio_hdmi_pd\0".as_ptr() as *const c_char);
    if IS_ERR((*afe_priv).audio_hdmi_ck as *const _) != 0 {
        return PTR_ERR((*afe_priv).audio_hdmi_ck as *const _) as c_int;
    }

    (*afe_priv).audio_spdf_ck = devm_clk_get_optional((*afe).dev, b"audio_spdf_pd\0".as_ptr() as *const c_char);
    if IS_ERR((*afe_priv).audio_spdf_ck as *const _) != 0 {
        return PTR_ERR((*afe_priv).audio_spdf_ck as *const _) as c_int;
    }

    (*afe_priv).audio_apll_ck = devm_clk_get_optional((*afe).dev, b"audio_apll_pd\0".as_ptr() as *const c_char);
    if IS_ERR((*afe_priv).audio_apll_ck as *const _) != 0 {
        return PTR_ERR((*afe_priv).audio_apll_ck as *const _) as c_int;
    }

    0
}

pub unsafe extern "C" fn mt2701_afe_enable_i2s(
    afe: *mut mtk_base_afe,
    i2s_path: *mut mt2701_i2s_path,
    dir: c_int,
) -> c_int {
    let mut ret: c_int;

    ret = clk_prepare_enable((*i2s_path).asrco_ck);
    if ret != 0 {
        dev_err((*afe).dev, b"failed to enable ASRC clock %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }

    ret = clk_prepare_enable((*i2s_path).hop_ck[dir as usize]);
    if ret != 0 {
        dev_err((*afe).dev, b"failed to enable I2S clock %d\n\0".as_ptr() as *const c_char, ret);
        clk_disable_unprepare((*i2s_path).asrco_ck);
        return ret;
    }

    0
}

pub unsafe extern "C" fn mt2701_afe_disable_i2s(
    _afe: *mut mtk_base_afe,
    i2s_path: *mut mt2701_i2s_path,
    dir: c_int,
) {
    clk_disable_unprepare((*i2s_path).hop_ck[dir as usize]);
    clk_disable_unprepare((*i2s_path).asrco_ck);
}

pub unsafe extern "C" fn mt2701_afe_enable_mclk(afe: *mut mtk_base_afe, id: c_int) -> c_int {
    let afe_priv = (*afe).platform_priv as *mut mt2701_afe_private;
    let i2s_path = &mut (*afe_priv).i2s_path[id as usize] as *mut mt2701_i2s_path;

    clk_prepare_enable((*i2s_path).mclk_ck)
}

pub unsafe extern "C" fn mt2701_afe_disable_mclk(afe: *mut mtk_base_afe, id: c_int) {
    let afe_priv = (*afe).platform_priv as *mut mt2701_afe_private;
    let i2s_path = &mut (*afe_priv).i2s_path[id as usize] as *mut mt2701_i2s_path;

    clk_disable_unprepare((*i2s_path).mclk_ck);
}

pub unsafe extern "C" fn mt2701_enable_btmrg_clk(afe: *mut mtk_base_afe) -> c_int {
    let afe_priv = (*afe).platform_priv as *mut mt2701_afe_private;

    clk_prepare_enable((*afe_priv).mrgif_ck)
}

pub unsafe extern "C" fn mt2701_disable_btmrg_clk(afe: *mut mtk_base_afe) {
    let afe_priv = (*afe).platform_priv as *mut mt2701_afe_private;

    clk_disable_unprepare((*afe_priv).mrgif_ck);
}

unsafe extern "C" fn mt2701_afe_enable_audsys(afe: *mut mtk_base_afe) -> c_int {
    let afe_priv = (*afe).platform_priv as *mut mt2701_afe_private;
    let mut ret: c_int;

    /* Enable infra clock gate */
    ret = clk_prepare_enable((*afe_priv).base_ck[MT2701_INFRA_SYS_AUDIO as usize]);
    if ret != 0 {
        return ret;
    }

    /* Enable top a1sys clock gate */
    ret = clk_prepare_enable((*afe_priv).base_ck[MT2701_TOP_AUD_A1SYS as usize]);
    if ret != 0 {
        clk_disable_unprepare((*afe_priv).base_ck[MT2701_INFRA_SYS_AUDIO as usize]);
        return ret;
    }

    /* Enable top a2sys clock gate */
    ret = clk_prepare_enable((*afe_priv).base_ck[MT2701_TOP_AUD_A2SYS as usize]);
    if ret != 0 {
        clk_disable_unprepare((*afe_priv).base_ck[MT2701_TOP_AUD_A1SYS as usize]);
        clk_disable_unprepare((*afe_priv).base_ck[MT2701_INFRA_SYS_AUDIO as usize]);
        return ret;
    }

    /* Internal clock gates */
    ret = clk_prepare_enable((*afe_priv).base_ck[MT2701_AUDSYS_AFE as usize]);
    if ret != 0 {
        clk_disable_unprepare((*afe_priv).base_ck[MT2701_TOP_AUD_A2SYS as usize]);
        clk_disable_unprepare((*afe_priv).base_ck[MT2701_TOP_AUD_A1SYS as usize]);
        clk_disable_unprepare((*afe_priv).base_ck[MT2701_INFRA_SYS_AUDIO as usize]);
        return ret;
    }

    ret = clk_prepare_enable((*afe_priv).base_ck[MT2701_AUDSYS_A1SYS as usize]);
    if ret != 0 {
        clk_disable_unprepare((*afe_priv).base_ck[MT2701_AUDSYS_AFE as usize]);
        clk_disable_unprepare((*afe_priv).base_ck[MT2701_TOP_AUD_A2SYS as usize]);
        clk_disable_unprepare((*afe_priv).base_ck[MT2701_TOP_AUD_A1SYS as usize]);
        clk_disable_unprepare((*afe_priv).base_ck[MT2701_INFRA_SYS_AUDIO as usize]);
        return ret;
    }

    ret = clk_prepare_enable((*afe_priv).base_ck[MT2701_AUDSYS_A2SYS as usize]);
    if ret != 0 {
        clk_disable_unprepare((*afe_priv).base_ck[MT2701_AUDSYS_A1SYS as usize]);
        clk_disable_unprepare((*afe_priv).base_ck[MT2701_AUDSYS_AFE as usize]);
        clk_disable_unprepare((*afe_priv).base_ck[MT2701_TOP_AUD_A2SYS as usize]);
        clk_disable_unprepare((*afe_priv).base_ck[MT2701_TOP_AUD_A1SYS as usize]);
        clk_disable_unprepare((*afe_priv).base_ck[MT2701_INFRA_SYS_AUDIO as usize]);
        return ret;
    }

    ret = clk_prepare_enable((*afe_priv).base_ck[MT2701_AUDSYS_AFE_CONN as usize]);
    if ret != 0 {
        clk_disable_unprepare((*afe_priv).base_ck[MT2701_AUDSYS_A2SYS as usize]);
        clk_disable_unprepare((*afe_priv).base_ck[MT2701_AUDSYS_A1SYS as usize]);
        clk_disable_unprepare((*afe_priv).base_ck[MT2701_AUDSYS_AFE as usize]);
        clk_disable_unprepare((*afe_priv).base_ck[MT2701_TOP_AUD_A2SYS as usize]);
        clk_disable_unprepare((*afe_priv).base_ck[MT2701_TOP_AUD_A1SYS as usize]);
        clk_disable_unprepare((*afe_priv).base_ck[MT2701_INFRA_SYS_AUDIO as usize]);
        return ret;
    }

    0
}

unsafe extern "C" fn mt2701_afe_disable_audsys(afe: *mut mtk_base_afe) {
    let afe_priv = (*afe).platform_priv as *mut mt2701_afe_private;

    clk_disable_unprepare((*afe_priv).base_ck[MT2701_AUDSYS_AFE_CONN as usize]);
    clk_disable_unprepare((*afe_priv).base_ck[MT2701_AUDSYS_A2SYS as usize]);
    clk_disable_unprepare((*afe_priv).base_ck[MT2701_AUDSYS_A1SYS as usize]);
    clk_disable_unprepare((*afe_priv).base_ck[MT2701_AUDSYS_AFE as usize]);
    clk_disable_unprepare((*afe_priv).base_ck[MT2701_TOP_AUD_A1SYS as usize]);
    clk_disable_unprepare((*afe_priv).base_ck[MT2701_TOP_AUD_A2SYS as usize]);
    clk_disable_unprepare((*afe_priv).base_ck[MT2701_INFRA_SYS_AUDIO as usize]);
}

pub unsafe extern "C" fn mt2701_afe_enable_clock(afe: *mut mtk_base_afe) -> c_int {
    let ret: c_int;

    /* Enable audio system */
    ret = mt2701_afe_enable_audsys(afe);
    if ret != 0 {
        dev_err((*afe).dev, b"failed to enable audio system %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }

    regmap_update_bits(
        (*afe).regmap,
        ASYS_TOP_CON,
        ASYS_TOP_CON_ASYS_TIMING_ON,
        ASYS_TOP_CON_ASYS_TIMING_ON,
    );
    regmap_update_bits(
        (*afe).regmap,
        AFE_DAC_CON0,
        AFE_DAC_CON0_AFE_ON,
        AFE_DAC_CON0_AFE_ON,
    );

    /* Configure ASRC */
    regmap_write((*afe).regmap, PWR1_ASM_CON1, PWR1_ASM_CON1_INIT_VAL);
    regmap_write((*afe).regmap, PWR2_ASM_CON1, PWR2_ASM_CON1_INIT_VAL);

    0
}

pub unsafe extern "C" fn mt2701_afe_disable_clock(afe: *mut mtk_base_afe) -> c_int {
    regmap_update_bits(
        (*afe).regmap,
        ASYS_TOP_CON,
        ASYS_TOP_CON_ASYS_TIMING_ON,
        0,
    );
    regmap_update_bits((*afe).regmap, AFE_DAC_CON0, AFE_DAC_CON0_AFE_ON, 0);

    mt2701_afe_disable_audsys(afe);

    0
}

pub unsafe extern "C" fn mt2701_mclk_configuration(afe: *mut mtk_base_afe, id: c_int) -> c_int {
    let priv_ = (*afe).platform_priv as *mut mt2701_afe_private;
    let i2s_path = &mut (*priv_).i2s_path[id as usize] as *mut mt2701_i2s_path;
    let mut ret: c_int = -EINVAL;

    /* Set mclk source */
    if MT2701_PLL_DOMAIN_0_RATE % (*i2s_path).mclk_rate == 0 {
        ret = clk_set_parent(
            (*i2s_path).sel_ck,
            (*priv_).base_ck[MT2701_TOP_AUD_MCLK_SRC0 as usize],
        );
    } else if MT2701_PLL_DOMAIN_1_RATE % (*i2s_path).mclk_rate == 0 {
        ret = clk_set_parent(
            (*i2s_path).sel_ck,
            (*priv_).base_ck[MT2701_TOP_AUD_MCLK_SRC1 as usize],
        );
    }

    if ret != 0 {
        dev_err((*afe).dev, b"failed to set mclk source\n\0".as_ptr() as *const c_char);
        return ret;
    }

    /* Set mclk divider */
    ret = clk_set_rate((*i2s_path).div_ck, (*i2s_path).mclk_rate);
    if ret != 0 {
        dev_err((*afe).dev, b"failed to set mclk divider %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
