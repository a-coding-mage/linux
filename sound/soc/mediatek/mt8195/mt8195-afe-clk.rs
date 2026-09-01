// SPDX-License-Identifier: GPL-2.0
/*
 * mt8195-afe-clk.c  --  Mediatek 8195 afe clock ctrl
 *
 * Copyright (c) 2021 MediaTek Inc.
 * Author: Bicycle Tsai <bicycle.tsai@mediatek.com>
 *         Trevor Wu <trevor.wu@mediatek.com>
 */

// C dependencies:
// #include <linux/clk.h>
// #include "mt8195-afe-common.h"
// #include "mt8195-afe-clk.h"
// #include "mt8195-reg.h"
// #include "mt8195-audsys-clk.h"

use core::ffi::{c_char, c_int, c_uint, c_ulong};
use core::ptr;

use crate::*;

static aud_clks: [*const c_char; MT8195_CLK_NUM as usize] = {
    let mut clks = [ptr::null(); MT8195_CLK_NUM as usize];

    /* xtal */
    clks[MT8195_CLK_XTAL_26M as usize] = c"clk26m".as_ptr();
    /* divider */
    clks[MT8195_CLK_TOP_APLL1 as usize] = c"apll1_ck".as_ptr();
    clks[MT8195_CLK_TOP_APLL2 as usize] = c"apll2_ck".as_ptr();
    clks[MT8195_CLK_TOP_APLL12_DIV0 as usize] = c"apll12_div0".as_ptr();
    clks[MT8195_CLK_TOP_APLL12_DIV1 as usize] = c"apll12_div1".as_ptr();
    clks[MT8195_CLK_TOP_APLL12_DIV2 as usize] = c"apll12_div2".as_ptr();
    clks[MT8195_CLK_TOP_APLL12_DIV3 as usize] = c"apll12_div3".as_ptr();
    clks[MT8195_CLK_TOP_APLL12_DIV9 as usize] = c"apll12_div9".as_ptr();
    /* mux */
    clks[MT8195_CLK_TOP_A1SYS_HP_SEL as usize] = c"a1sys_hp_sel".as_ptr();
    clks[MT8195_CLK_TOP_AUD_INTBUS_SEL as usize] = c"aud_intbus_sel".as_ptr();
    clks[MT8195_CLK_TOP_AUDIO_H_SEL as usize] = c"audio_h_sel".as_ptr();
    clks[MT8195_CLK_TOP_AUDIO_LOCAL_BUS_SEL as usize] = c"audio_local_bus_sel".as_ptr();
    clks[MT8195_CLK_TOP_DPTX_M_SEL as usize] = c"dptx_m_sel".as_ptr();
    clks[MT8195_CLK_TOP_I2SO1_M_SEL as usize] = c"i2so1_m_sel".as_ptr();
    clks[MT8195_CLK_TOP_I2SO2_M_SEL as usize] = c"i2so2_m_sel".as_ptr();
    clks[MT8195_CLK_TOP_I2SI1_M_SEL as usize] = c"i2si1_m_sel".as_ptr();
    clks[MT8195_CLK_TOP_I2SI2_M_SEL as usize] = c"i2si2_m_sel".as_ptr();
    /* clock gate */
    clks[MT8195_CLK_INFRA_AO_AUDIO_26M_B as usize] = c"infra_ao_audio_26m_b".as_ptr();
    clks[MT8195_CLK_SCP_ADSP_AUDIODSP as usize] = c"scp_adsp_audiodsp".as_ptr();
    /* afe clock gate */
    clks[MT8195_CLK_AUD_AFE as usize] = c"aud_afe".as_ptr();
    clks[MT8195_CLK_AUD_APLL1_TUNER as usize] = c"aud_apll1_tuner".as_ptr();
    clks[MT8195_CLK_AUD_APLL2_TUNER as usize] = c"aud_apll2_tuner".as_ptr();
    clks[MT8195_CLK_AUD_APLL as usize] = c"aud_apll".as_ptr();
    clks[MT8195_CLK_AUD_APLL2 as usize] = c"aud_apll2".as_ptr();
    clks[MT8195_CLK_AUD_DAC as usize] = c"aud_dac".as_ptr();
    clks[MT8195_CLK_AUD_ADC as usize] = c"aud_adc".as_ptr();
    clks[MT8195_CLK_AUD_DAC_HIRES as usize] = c"aud_dac_hires".as_ptr();
    clks[MT8195_CLK_AUD_A1SYS_HP as usize] = c"aud_a1sys_hp".as_ptr();
    clks[MT8195_CLK_AUD_ADC_HIRES as usize] = c"aud_adc_hires".as_ptr();
    clks[MT8195_CLK_AUD_ADDA6_ADC as usize] = c"aud_adda6_adc".as_ptr();
    clks[MT8195_CLK_AUD_ADDA6_ADC_HIRES as usize] = c"aud_adda6_adc_hires".as_ptr();
    clks[MT8195_CLK_AUD_I2SIN as usize] = c"aud_i2sin".as_ptr();
    clks[MT8195_CLK_AUD_TDM_IN as usize] = c"aud_tdm_in".as_ptr();
    clks[MT8195_CLK_AUD_I2S_OUT as usize] = c"aud_i2s_out".as_ptr();
    clks[MT8195_CLK_AUD_TDM_OUT as usize] = c"aud_tdm_out".as_ptr();
    clks[MT8195_CLK_AUD_HDMI_OUT as usize] = c"aud_hdmi_out".as_ptr();
    clks[MT8195_CLK_AUD_ASRC11 as usize] = c"aud_asrc11".as_ptr();
    clks[MT8195_CLK_AUD_ASRC12 as usize] = c"aud_asrc12".as_ptr();
    clks[MT8195_CLK_AUD_A1SYS as usize] = c"aud_a1sys".as_ptr();
    clks[MT8195_CLK_AUD_A2SYS as usize] = c"aud_a2sys".as_ptr();
    clks[MT8195_CLK_AUD_PCMIF as usize] = c"aud_pcmif".as_ptr();
    clks[MT8195_CLK_AUD_MEMIF_UL1 as usize] = c"aud_memif_ul1".as_ptr();
    clks[MT8195_CLK_AUD_MEMIF_UL2 as usize] = c"aud_memif_ul2".as_ptr();
    clks[MT8195_CLK_AUD_MEMIF_UL3 as usize] = c"aud_memif_ul3".as_ptr();
    clks[MT8195_CLK_AUD_MEMIF_UL4 as usize] = c"aud_memif_ul4".as_ptr();
    clks[MT8195_CLK_AUD_MEMIF_UL5 as usize] = c"aud_memif_ul5".as_ptr();
    clks[MT8195_CLK_AUD_MEMIF_UL6 as usize] = c"aud_memif_ul6".as_ptr();
    clks[MT8195_CLK_AUD_MEMIF_UL8 as usize] = c"aud_memif_ul8".as_ptr();
    clks[MT8195_CLK_AUD_MEMIF_UL9 as usize] = c"aud_memif_ul9".as_ptr();
    clks[MT8195_CLK_AUD_MEMIF_UL10 as usize] = c"aud_memif_ul10".as_ptr();
    clks[MT8195_CLK_AUD_MEMIF_DL2 as usize] = c"aud_memif_dl2".as_ptr();
    clks[MT8195_CLK_AUD_MEMIF_DL3 as usize] = c"aud_memif_dl3".as_ptr();
    clks[MT8195_CLK_AUD_MEMIF_DL6 as usize] = c"aud_memif_dl6".as_ptr();
    clks[MT8195_CLK_AUD_MEMIF_DL7 as usize] = c"aud_memif_dl7".as_ptr();
    clks[MT8195_CLK_AUD_MEMIF_DL8 as usize] = c"aud_memif_dl8".as_ptr();
    clks[MT8195_CLK_AUD_MEMIF_DL10 as usize] = c"aud_memif_dl10".as_ptr();
    clks[MT8195_CLK_AUD_MEMIF_DL11 as usize] = c"aud_memif_dl11".as_ptr();

    clks
};

#[repr(C)]
struct mt8195_afe_tuner_cfg {
    id: c_uint,
    apll_div_reg: c_int,
    apll_div_shift: c_uint,
    apll_div_maskbit: c_uint,
    apll_div_default: c_uint,
    ref_ck_sel_reg: c_int,
    ref_ck_sel_shift: c_uint,
    ref_ck_sel_maskbit: c_uint,
    ref_ck_sel_default: c_uint,
    tuner_en_reg: c_int,
    tuner_en_shift: c_uint,
    tuner_en_maskbit: c_uint,
    upper_bound_reg: c_int,
    upper_bound_shift: c_uint,
    upper_bound_maskbit: c_uint,
    upper_bound_default: c_uint,
    ctrl_lock: spinlock_t, /* lock for apll tuner ctrl*/
    ref_cnt: c_int,
}

static mut mt8195_afe_tuner_cfgs: [mt8195_afe_tuner_cfg; MT8195_AUD_PLL_NUM as usize] = [
    mt8195_afe_tuner_cfg {
        id: MT8195_AUD_PLL1,
        apll_div_reg: AFE_APLL_TUNER_CFG,
        apll_div_shift: 4,
        apll_div_maskbit: 0xf,
        apll_div_default: 0x7,
        ref_ck_sel_reg: AFE_APLL_TUNER_CFG,
        ref_ck_sel_shift: 1,
        ref_ck_sel_maskbit: 0x3,
        ref_ck_sel_default: 0x2,
        tuner_en_reg: AFE_APLL_TUNER_CFG,
        tuner_en_shift: 0,
        tuner_en_maskbit: 0x1,
        upper_bound_reg: AFE_APLL_TUNER_CFG,
        upper_bound_shift: 8,
        upper_bound_maskbit: 0xff,
        upper_bound_default: 0x3,
        ctrl_lock: unsafe { core::mem::zeroed() },
        ref_cnt: 0,
    },
    mt8195_afe_tuner_cfg {
        id: MT8195_AUD_PLL2,
        apll_div_reg: AFE_APLL_TUNER_CFG1,
        apll_div_shift: 4,
        apll_div_maskbit: 0xf,
        apll_div_default: 0x7,
        ref_ck_sel_reg: AFE_APLL_TUNER_CFG1,
        ref_ck_sel_shift: 1,
        ref_ck_sel_maskbit: 0x3,
        ref_ck_sel_default: 0x1,
        tuner_en_reg: AFE_APLL_TUNER_CFG1,
        tuner_en_shift: 0,
        tuner_en_maskbit: 0x1,
        upper_bound_reg: AFE_APLL_TUNER_CFG1,
        upper_bound_shift: 8,
        upper_bound_maskbit: 0xff,
        upper_bound_default: 0x3,
        ctrl_lock: unsafe { core::mem::zeroed() },
        ref_cnt: 0,
    },
    mt8195_afe_tuner_cfg {
        id: MT8195_AUD_PLL3,
        apll_div_reg: AFE_EARC_APLL_TUNER_CFG,
        apll_div_shift: 4,
        apll_div_maskbit: 0x3f,
        apll_div_default: 0x3,
        ref_ck_sel_reg: AFE_EARC_APLL_TUNER_CFG,
        ref_ck_sel_shift: 24,
        ref_ck_sel_maskbit: 0x3,
        ref_ck_sel_default: 0x0,
        tuner_en_reg: AFE_EARC_APLL_TUNER_CFG,
        tuner_en_shift: 0,
        tuner_en_maskbit: 0x1,
        upper_bound_reg: AFE_EARC_APLL_TUNER_CFG,
        upper_bound_shift: 12,
        upper_bound_maskbit: 0xff,
        upper_bound_default: 0x4,
        ctrl_lock: unsafe { core::mem::zeroed() },
        ref_cnt: 0,
    },
    mt8195_afe_tuner_cfg {
        id: MT8195_AUD_PLL4,
        apll_div_reg: AFE_SPDIFIN_APLL_TUNER_CFG,
        apll_div_shift: 4,
        apll_div_maskbit: 0x3f,
        apll_div_default: 0x7,
        ref_ck_sel_reg: AFE_SPDIFIN_APLL_TUNER_CFG1,
        ref_ck_sel_shift: 8,
        ref_ck_sel_maskbit: 0x1,
        ref_ck_sel_default: 0,
        tuner_en_reg: AFE_SPDIFIN_APLL_TUNER_CFG,
        tuner_en_shift: 0,
        tuner_en_maskbit: 0x1,
        upper_bound_reg: AFE_SPDIFIN_APLL_TUNER_CFG,
        upper_bound_shift: 12,
        upper_bound_maskbit: 0xff,
        upper_bound_default: 0x4,
        ctrl_lock: unsafe { core::mem::zeroed() },
        ref_cnt: 0,
    },
    mt8195_afe_tuner_cfg {
        id: MT8195_AUD_PLL5,
        apll_div_reg: AFE_LINEIN_APLL_TUNER_CFG,
        apll_div_shift: 4,
        apll_div_maskbit: 0x3f,
        apll_div_default: 0x3,
        ref_ck_sel_reg: AFE_LINEIN_APLL_TUNER_CFG,
        ref_ck_sel_shift: 24,
        ref_ck_sel_maskbit: 0x1,
        ref_ck_sel_default: 0,
        tuner_en_reg: AFE_LINEIN_APLL_TUNER_CFG,
        tuner_en_shift: 0,
        tuner_en_maskbit: 0x1,
        upper_bound_reg: AFE_LINEIN_APLL_TUNER_CFG,
        upper_bound_shift: 12,
        upper_bound_maskbit: 0xff,
        upper_bound_default: 0x4,
        ctrl_lock: unsafe { core::mem::zeroed() },
        ref_cnt: 0,
    },
];

unsafe fn mt8195_afe_found_apll_tuner(id: c_uint) -> *mut mt8195_afe_tuner_cfg {
    if id >= MT8195_AUD_PLL_NUM {
        return ptr::null_mut();
    }

    &mut mt8195_afe_tuner_cfgs[id as usize]
}

unsafe fn mt8195_afe_init_apll_tuner(id: c_uint) -> c_int {
    let cfg = mt8195_afe_found_apll_tuner(id);

    if cfg.is_null() {
        return -EINVAL;
    }

    (*cfg).ref_cnt = 0;
    spin_lock_init(&mut (*cfg).ctrl_lock);

    0
}

unsafe fn mt8195_afe_setup_apll_tuner(afe: *mut mtk_base_afe, id: c_uint) -> c_int {
    let cfg = mt8195_afe_found_apll_tuner(id);

    if cfg.is_null() {
        return -EINVAL;
    }

    regmap_update_bits(
        (*afe).regmap,
        (*cfg).apll_div_reg as c_uint,
        (*cfg).apll_div_maskbit << (*cfg).apll_div_shift,
        (*cfg).apll_div_default << (*cfg).apll_div_shift,
    );

    regmap_update_bits(
        (*afe).regmap,
        (*cfg).ref_ck_sel_reg as c_uint,
        (*cfg).ref_ck_sel_maskbit << (*cfg).ref_ck_sel_shift,
        (*cfg).ref_ck_sel_default << (*cfg).ref_ck_sel_shift,
    );

    regmap_update_bits(
        (*afe).regmap,
        (*cfg).upper_bound_reg as c_uint,
        (*cfg).upper_bound_maskbit << (*cfg).upper_bound_shift,
        (*cfg).upper_bound_default << (*cfg).upper_bound_shift,
    );

    0
}

unsafe fn mt8195_afe_enable_tuner_clk(afe: *mut mtk_base_afe, id: c_uint) -> c_int {
    let afe_priv = (*afe).platform_priv as *mut mt8195_afe_private;

    match id {
        MT8195_AUD_PLL1 => {
            mt8195_afe_enable_clk(afe, *(*afe_priv).clk.add(MT8195_CLK_AUD_APLL as usize));
            mt8195_afe_enable_clk(
                afe,
                *(*afe_priv).clk.add(MT8195_CLK_AUD_APLL1_TUNER as usize),
            );
        }
        MT8195_AUD_PLL2 => {
            mt8195_afe_enable_clk(afe, *(*afe_priv).clk.add(MT8195_CLK_AUD_APLL2 as usize));
            mt8195_afe_enable_clk(
                afe,
                *(*afe_priv).clk.add(MT8195_CLK_AUD_APLL2_TUNER as usize),
            );
        }
        _ => {}
    }

    0
}

unsafe fn mt8195_afe_disable_tuner_clk(afe: *mut mtk_base_afe, id: c_uint) -> c_int {
    let afe_priv = (*afe).platform_priv as *mut mt8195_afe_private;

    match id {
        MT8195_AUD_PLL1 => {
            mt8195_afe_disable_clk(
                afe,
                *(*afe_priv).clk.add(MT8195_CLK_AUD_APLL1_TUNER as usize),
            );
            mt8195_afe_disable_clk(afe, *(*afe_priv).clk.add(MT8195_CLK_AUD_APLL as usize));
        }
        MT8195_AUD_PLL2 => {
            mt8195_afe_disable_clk(
                afe,
                *(*afe_priv).clk.add(MT8195_CLK_AUD_APLL2_TUNER as usize),
            );
            mt8195_afe_disable_clk(afe, *(*afe_priv).clk.add(MT8195_CLK_AUD_APLL2 as usize));
        }
        _ => {}
    }

    0
}

unsafe fn mt8195_afe_enable_apll_tuner(afe: *mut mtk_base_afe, id: c_uint) -> c_int {
    let cfg = mt8195_afe_found_apll_tuner(id);
    let mut ret: c_int;

    if cfg.is_null() {
        return -EINVAL;
    }

    ret = mt8195_afe_setup_apll_tuner(afe, id);
    if ret != 0 {
        return ret;
    }

    ret = mt8195_afe_enable_tuner_clk(afe, id);
    if ret != 0 {
        return ret;
    }

    /* scoped_guard(spinlock_irqsave, &cfg->ctrl_lock) */
    let mut flags: c_ulong = 0;
    spin_lock_irqsave(&mut (*cfg).ctrl_lock, &mut flags);
    (*cfg).ref_cnt += 1;
    if (*cfg).ref_cnt == 1 {
        regmap_update_bits(
            (*afe).regmap,
            (*cfg).tuner_en_reg as c_uint,
            (*cfg).tuner_en_maskbit << (*cfg).tuner_en_shift,
            1 << (*cfg).tuner_en_shift,
        );
    }
    spin_unlock_irqrestore(&mut (*cfg).ctrl_lock, flags);

    0
}

unsafe fn mt8195_afe_disable_apll_tuner(afe: *mut mtk_base_afe, id: c_uint) -> c_int {
    let cfg = mt8195_afe_found_apll_tuner(id);
    let ret: c_int;

    if cfg.is_null() {
        return -EINVAL;
    }

    /* scoped_guard(spinlock_irqsave, &cfg->ctrl_lock) */
    let mut flags: c_ulong = 0;
    spin_lock_irqsave(&mut (*cfg).ctrl_lock, &mut flags);
    (*cfg).ref_cnt -= 1;
    if (*cfg).ref_cnt == 0 {
        regmap_update_bits(
            (*afe).regmap,
            (*cfg).tuner_en_reg as c_uint,
            (*cfg).tuner_en_maskbit << (*cfg).tuner_en_shift,
            0 << (*cfg).tuner_en_shift,
        );
    } else if (*cfg).ref_cnt < 0 {
        (*cfg).ref_cnt = 0;
    }
    spin_unlock_irqrestore(&mut (*cfg).ctrl_lock, flags);

    ret = mt8195_afe_disable_tuner_clk(afe, id);
    if ret != 0 {
        return ret;
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn mt8195_afe_get_mclk_source_clk_id(sel: c_int) -> c_int {
    match sel {
        MT8195_MCK_SEL_26M => MT8195_CLK_XTAL_26M as c_int,
        MT8195_MCK_SEL_APLL1 => MT8195_CLK_TOP_APLL1 as c_int,
        MT8195_MCK_SEL_APLL2 => MT8195_CLK_TOP_APLL2 as c_int,
        _ => -EINVAL,
    }
}

#[no_mangle]
pub unsafe extern "C" fn mt8195_afe_get_mclk_source_rate(
    afe: *mut mtk_base_afe,
    apll: c_int,
) -> c_int {
    let afe_priv = (*afe).platform_priv as *mut mt8195_afe_private;
    let clk_id = mt8195_afe_get_mclk_source_clk_id(apll);

    if clk_id < 0 {
        dev_dbg((*afe).dev, c"invalid clk id\n".as_ptr());
        return 0;
    }

    clk_get_rate(*(*afe_priv).clk.add(clk_id as usize)) as c_int
}

#[no_mangle]
pub unsafe extern "C" fn mt8195_afe_get_default_mclk_source_by_rate(rate: c_int) -> c_int {
    if (rate % 8000) == 0 {
        MT8195_MCK_SEL_APLL1
    } else {
        MT8195_MCK_SEL_APLL2
    }
}

#[no_mangle]
pub unsafe extern "C" fn mt8195_afe_init_clock(afe: *mut mtk_base_afe) -> c_int {
    let afe_priv = (*afe).platform_priv as *mut mt8195_afe_private;
    let mut i: c_int;
    let mut ret: c_int;

    mt8195_audsys_clk_register(afe);

    (*afe_priv).clk = devm_kcalloc(
        (*afe).dev,
        MT8195_CLK_NUM as usize,
        core::mem::size_of::<*mut clk>(),
        GFP_KERNEL,
    ) as *mut *mut clk;
    if (*afe_priv).clk.is_null() {
        return -ENOMEM;
    }

    i = 0;
    while i < MT8195_CLK_NUM as c_int {
        *(*afe_priv).clk.add(i as usize) = devm_clk_get((*afe).dev, aud_clks[i as usize]);
        if IS_ERR(*(*afe_priv).clk.add(i as usize)) {
            dev_dbg(
                (*afe).dev,
                c"%s(), devm_clk_get %s fail, ret %ld\n".as_ptr(),
                c"mt8195_afe_init_clock".as_ptr(),
                aud_clks[i as usize],
                PTR_ERR(*(*afe_priv).clk.add(i as usize)),
            );
            return PTR_ERR(*(*afe_priv).clk.add(i as usize)) as c_int;
        }
        i += 1;
    }

    /* initial tuner */
    i = 0;
    while i < MT8195_AUD_PLL_NUM as c_int {
        ret = mt8195_afe_init_apll_tuner(i as c_uint);
        if ret != 0 {
            dev_dbg(
                (*afe).dev,
                c"%s(), init apll_tuner%d failed".as_ptr(),
                c"mt8195_afe_init_clock".as_ptr(),
                i + 1,
            );
            return -EINVAL;
        }
        i += 1;
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn mt8195_afe_enable_clk(afe: *mut mtk_base_afe, clk: *mut clk) -> c_int {
    let ret: c_int;

    if !clk.is_null() {
        ret = clk_prepare_enable(clk);
        if ret != 0 {
            dev_dbg(
                (*afe).dev,
                c"%s(), failed to enable clk\n".as_ptr(),
                c"mt8195_afe_enable_clk".as_ptr(),
            );
            return ret;
        }
    } else {
        dev_dbg((*afe).dev, c"NULL clk\n".as_ptr());
    }
    0
}
// EXPORT_SYMBOL_GPL(mt8195_afe_enable_clk);

#[no_mangle]
pub unsafe extern "C" fn mt8195_afe_disable_clk(afe: *mut mtk_base_afe, clk: *mut clk) {
    if !clk.is_null() {
        clk_disable_unprepare(clk);
    } else {
        dev_dbg((*afe).dev, c"NULL clk\n".as_ptr());
    }
}
// EXPORT_SYMBOL_GPL(mt8195_afe_disable_clk);

#[no_mangle]
pub unsafe extern "C" fn mt8195_afe_prepare_clk(afe: *mut mtk_base_afe, clk: *mut clk) -> c_int {
    let ret: c_int;

    if !clk.is_null() {
        ret = clk_prepare(clk);
        if ret != 0 {
            dev_dbg(
                (*afe).dev,
                c"%s(), failed to prepare clk\n".as_ptr(),
                c"mt8195_afe_prepare_clk".as_ptr(),
            );
            return ret;
        }
    } else {
        dev_dbg((*afe).dev, c"NULL clk\n".as_ptr());
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn mt8195_afe_unprepare_clk(afe: *mut mtk_base_afe, clk: *mut clk) {
    if !clk.is_null() {
        clk_unprepare(clk);
    } else {
        dev_dbg((*afe).dev, c"NULL clk\n".as_ptr());
    }
}

#[no_mangle]
pub unsafe extern "C" fn mt8195_afe_enable_clk_atomic(
    afe: *mut mtk_base_afe,
    clk: *mut clk,
) -> c_int {
    let ret: c_int;

    if !clk.is_null() {
        ret = clk_enable(clk);
        if ret != 0 {
            dev_dbg(
                (*afe).dev,
                c"%s(), failed to clk enable\n".as_ptr(),
                c"mt8195_afe_enable_clk_atomic".as_ptr(),
            );
            return ret;
        }
    } else {
        dev_dbg((*afe).dev, c"NULL clk\n".as_ptr());
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn mt8195_afe_disable_clk_atomic(afe: *mut mtk_base_afe, clk: *mut clk) {
    if !clk.is_null() {
        clk_disable(clk);
    } else {
        dev_dbg((*afe).dev, c"NULL clk\n".as_ptr());
    }
}

#[no_mangle]
pub unsafe extern "C" fn mt8195_afe_set_clk_rate(
    afe: *mut mtk_base_afe,
    clk: *mut clk,
    rate: c_uint,
) -> c_int {
    let ret: c_int;

    if !clk.is_null() {
        ret = clk_set_rate(clk, rate as c_ulong);
        if ret != 0 {
            dev_dbg(
                (*afe).dev,
                c"%s(), failed to set clk rate\n".as_ptr(),
                c"mt8195_afe_set_clk_rate".as_ptr(),
            );
            return ret;
        }
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn mt8195_afe_set_clk_parent(
    afe: *mut mtk_base_afe,
    clk: *mut clk,
    parent: *mut clk,
) -> c_int {
    let ret: c_int;

    if !clk.is_null() && !parent.is_null() {
        ret = clk_set_parent(clk, parent);
        if ret != 0 {
            dev_dbg(
                (*afe).dev,
                c"%s(), failed to set clk parent\n".as_ptr(),
                c"mt8195_afe_set_clk_parent".as_ptr(),
            );
            return ret;
        }
    }

    0
}

unsafe fn get_top_cg_reg(cg_type: c_uint) -> c_uint {
    match cg_type {
        MT8195_TOP_CG_A1SYS_TIMING | MT8195_TOP_CG_A2SYS_TIMING | MT8195_TOP_CG_26M_TIMING => {
            ASYS_TOP_CON
        }
        _ => 0,
    }
}

unsafe fn get_top_cg_mask(cg_type: c_uint) -> c_uint {
    match cg_type {
        MT8195_TOP_CG_A1SYS_TIMING => ASYS_TOP_CON_A1SYS_TIMING_ON,
        MT8195_TOP_CG_A2SYS_TIMING => ASYS_TOP_CON_A2SYS_TIMING_ON,
        MT8195_TOP_CG_26M_TIMING => ASYS_TOP_CON_26M_TIMING_ON,
        _ => 0,
    }
}

unsafe fn get_top_cg_on_val(cg_type: c_uint) -> c_uint {
    match cg_type {
        MT8195_TOP_CG_A1SYS_TIMING | MT8195_TOP_CG_A2SYS_TIMING | MT8195_TOP_CG_26M_TIMING => {
            get_top_cg_mask(cg_type)
        }
        _ => 0,
    }
}

unsafe fn get_top_cg_off_val(cg_type: c_uint) -> c_uint {
    match cg_type {
        MT8195_TOP_CG_A1SYS_TIMING | MT8195_TOP_CG_A2SYS_TIMING | MT8195_TOP_CG_26M_TIMING => 0,
        _ => get_top_cg_mask(cg_type),
    }
}

unsafe fn mt8195_afe_enable_top_cg(afe: *mut mtk_base_afe, cg_type: c_uint) -> c_int {
    let reg = get_top_cg_reg(cg_type);
    let mask = get_top_cg_mask(cg_type);
    let val = get_top_cg_on_val(cg_type);

    regmap_update_bits((*afe).regmap, reg, mask, val);
    0
}

unsafe fn mt8195_afe_disable_top_cg(afe: *mut mtk_base_afe, cg_type: c_uint) -> c_int {
    let reg = get_top_cg_reg(cg_type);
    let mask = get_top_cg_mask(cg_type);
    let val = get_top_cg_off_val(cg_type);

    regmap_update_bits((*afe).regmap, reg, mask, val);
    0
}

#[no_mangle]
pub unsafe extern "C" fn mt8195_afe_enable_reg_rw_clk(afe: *mut mtk_base_afe) -> c_int {
    let afe_priv = (*afe).platform_priv as *mut mt8195_afe_private;
    let mut i: c_int;
    static clk_array: [c_uint; 8] = [
        MT8195_CLK_SCP_ADSP_AUDIODSP, /* bus clock for infra */
        MT8195_CLK_TOP_AUDIO_H_SEL, /* clock for ADSP bus */
        MT8195_CLK_TOP_AUDIO_LOCAL_BUS_SEL, /* bus clock for DRAM access */
        MT8195_CLK_TOP_AUD_INTBUS_SEL, /* bus clock for AFE SRAM access */
        MT8195_CLK_INFRA_AO_AUDIO_26M_B, /* audio 26M clock */
        MT8195_CLK_AUD_AFE, /* AFE HW master switch */
        MT8195_CLK_AUD_A1SYS_HP, /* AFE HW clock*/
        MT8195_CLK_AUD_A1SYS, /* AFE HW clock */
    ];

    i = 0;
    while (i as usize) < clk_array.len() {
        mt8195_afe_enable_clk(afe, *(*afe_priv).clk.add(clk_array[i as usize] as usize));
        i += 1;
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn mt8195_afe_disable_reg_rw_clk(afe: *mut mtk_base_afe) -> c_int {
    let afe_priv = (*afe).platform_priv as *mut mt8195_afe_private;
    let mut i: c_int;
    static clk_array: [c_uint; 8] = [
        MT8195_CLK_AUD_A1SYS,
        MT8195_CLK_AUD_A1SYS_HP,
        MT8195_CLK_AUD_AFE,
        MT8195_CLK_INFRA_AO_AUDIO_26M_B,
        MT8195_CLK_TOP_AUD_INTBUS_SEL,
        MT8195_CLK_TOP_AUDIO_LOCAL_BUS_SEL,
        MT8195_CLK_TOP_AUDIO_H_SEL,
        MT8195_CLK_SCP_ADSP_AUDIODSP,
    ];

    i = 0;
    while (i as usize) < clk_array.len() {
        mt8195_afe_disable_clk(afe, *(*afe_priv).clk.add(clk_array[i as usize] as usize));
        i += 1;
    }

    0
}

unsafe fn mt8195_afe_enable_afe_on(afe: *mut mtk_base_afe) -> c_int {
    regmap_update_bits((*afe).regmap, AFE_DAC_CON0, 0x1, 0x1);
    0
}

unsafe fn mt8195_afe_disable_afe_on(afe: *mut mtk_base_afe) -> c_int {
    regmap_update_bits((*afe).regmap, AFE_DAC_CON0, 0x1, 0x0);
    0
}

unsafe fn mt8195_afe_enable_timing_sys(afe: *mut mtk_base_afe) -> c_int {
    let afe_priv = (*afe).platform_priv as *mut mt8195_afe_private;
    let mut i: c_int;
    static clk_array: [c_uint; 2] = [MT8195_CLK_AUD_A1SYS, MT8195_CLK_AUD_A2SYS];
    static cg_array: [c_uint; 3] = [
        MT8195_TOP_CG_A1SYS_TIMING,
        MT8195_TOP_CG_A2SYS_TIMING,
        MT8195_TOP_CG_26M_TIMING,
    ];

    i = 0;
    while (i as usize) < clk_array.len() {
        mt8195_afe_enable_clk(afe, *(*afe_priv).clk.add(clk_array[i as usize] as usize));
        i += 1;
    }

    i = 0;
    while (i as usize) < cg_array.len() {
        mt8195_afe_enable_top_cg(afe, cg_array[i as usize]);
        i += 1;
    }

    0
}

unsafe fn mt8195_afe_disable_timing_sys(afe: *mut mtk_base_afe) -> c_int {
    let afe_priv = (*afe).platform_priv as *mut mt8195_afe_private;
    let mut i: c_int;
    static clk_array: [c_uint; 2] = [MT8195_CLK_AUD_A2SYS, MT8195_CLK_AUD_A1SYS];
    static cg_array: [c_uint; 3] = [
        MT8195_TOP_CG_26M_TIMING,
        MT8195_TOP_CG_A2SYS_TIMING,
        MT8195_TOP_CG_A1SYS_TIMING,
    ];

    i = 0;
    while (i as usize) < cg_array.len() {
        mt8195_afe_disable_top_cg(afe, cg_array[i as usize]);
        i += 1;
    }

    i = 0;
    while (i as usize) < clk_array.len() {
        mt8195_afe_disable_clk(afe, *(*afe_priv).clk.add(clk_array[i as usize] as usize));
        i += 1;
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn mt8195_afe_enable_main_clock(afe: *mut mtk_base_afe) -> c_int {
    mt8195_afe_enable_timing_sys(afe);

    mt8195_afe_enable_afe_on(afe);

    mt8195_afe_enable_apll_tuner(afe, MT8195_AUD_PLL1);
    mt8195_afe_enable_apll_tuner(afe, MT8195_AUD_PLL2);

    0
}

#[no_mangle]
pub unsafe extern "C" fn mt8195_afe_disable_main_clock(afe: *mut mtk_base_afe) -> c_int {
    mt8195_afe_disable_apll_tuner(afe, MT8195_AUD_PLL2);
    mt8195_afe_disable_apll_tuner(afe, MT8195_AUD_PLL1);

    mt8195_afe_disable_afe_on(afe);

    mt8195_afe_disable_timing_sys(afe);

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
