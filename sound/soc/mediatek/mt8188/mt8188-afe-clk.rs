// SPDX-License-Identifier: GPL-2.0
/*
 * mt8188-afe-clk.c  --  MediaTek 8188 afe clock ctrl
 *
 * Copyright (c) 2022 MediaTek Inc.
 * Author: Bicycle Tsai <bicycle.tsai@mediatek.com>
 *         Trevor Wu <trevor.wu@mediatek.com>
 *         Chun-Chia Chiu <chun-chia.chiu@mediatek.com>
 */

// C dependencies: <linux/clk.h>, "mt8188-afe-common.h",
// "mt8188-afe-clk.h", "mt8188-audsys-clk.h", "mt8188-reg.h".

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::ptr;

extern "C" {
    static MT8188_CLK_NUM: usize;
    static MT8188_CLK_XTAL_26M: usize;
    static MT8188_CLK_APMIXED_APLL1: usize;
    static MT8188_CLK_APMIXED_APLL2: usize;
    static MT8188_CLK_TOP_APLL1_D4: usize;
    static MT8188_CLK_TOP_APLL2_D4: usize;
    static MT8188_CLK_TOP_APLL12_DIV0: usize;
    static MT8188_CLK_TOP_APLL12_DIV1: usize;
    static MT8188_CLK_TOP_APLL12_DIV2: usize;
    static MT8188_CLK_TOP_APLL12_DIV3: usize;
    static MT8188_CLK_TOP_APLL12_DIV4: usize;
    static MT8188_CLK_TOP_APLL12_DIV9: usize;
    static MT8188_CLK_TOP_A1SYS_HP_SEL: usize;
    static MT8188_CLK_TOP_A2SYS_SEL: usize;
    static MT8188_CLK_TOP_AUD_IEC_SEL: usize;
    static MT8188_CLK_TOP_AUD_INTBUS_SEL: usize;
    static MT8188_CLK_TOP_AUDIO_H_SEL: usize;
    static MT8188_CLK_TOP_AUDIO_LOCAL_BUS_SEL: usize;
    static MT8188_CLK_TOP_DPTX_M_SEL: usize;
    static MT8188_CLK_TOP_I2SO1_M_SEL: usize;
    static MT8188_CLK_TOP_I2SO2_M_SEL: usize;
    static MT8188_CLK_TOP_I2SI1_M_SEL: usize;
    static MT8188_CLK_TOP_I2SI2_M_SEL: usize;
    static MT8188_CLK_ADSP_AUDIO_26M: usize;
    static MT8188_CLK_AUD_AFE: usize;
    static MT8188_CLK_AUD_APLL1_TUNER: usize;
    static MT8188_CLK_AUD_APLL2_TUNER: usize;
    static MT8188_CLK_AUD_APLL: usize;
    static MT8188_CLK_AUD_APLL2: usize;
    static MT8188_CLK_AUD_DAC: usize;
    static MT8188_CLK_AUD_ADC: usize;
    static MT8188_CLK_AUD_DAC_HIRES: usize;
    static MT8188_CLK_AUD_A1SYS_HP: usize;
    static MT8188_CLK_AUD_AFE_DMIC1: usize;
    static MT8188_CLK_AUD_AFE_DMIC2: usize;
    static MT8188_CLK_AUD_AFE_DMIC3: usize;
    static MT8188_CLK_AUD_AFE_DMIC4: usize;
    static MT8188_CLK_AUD_ADC_HIRES: usize;
    static MT8188_CLK_AUD_DMIC_HIRES1: usize;
    static MT8188_CLK_AUD_DMIC_HIRES2: usize;
    static MT8188_CLK_AUD_DMIC_HIRES3: usize;
    static MT8188_CLK_AUD_DMIC_HIRES4: usize;
    static MT8188_CLK_AUD_I2SIN: usize;
    static MT8188_CLK_AUD_TDM_IN: usize;
    static MT8188_CLK_AUD_I2S_OUT: usize;
    static MT8188_CLK_AUD_TDM_OUT: usize;
    static MT8188_CLK_AUD_HDMI_OUT: usize;
    static MT8188_CLK_AUD_ASRC11: usize;
    static MT8188_CLK_AUD_ASRC12: usize;
    static MT8188_CLK_AUD_A1SYS: usize;
    static MT8188_CLK_AUD_A2SYS: usize;
    static MT8188_CLK_AUD_PCMIF: usize;
    static MT8188_CLK_AUD_MEMIF_UL1: usize;
    static MT8188_CLK_AUD_MEMIF_UL2: usize;
    static MT8188_CLK_AUD_MEMIF_UL3: usize;
    static MT8188_CLK_AUD_MEMIF_UL4: usize;
    static MT8188_CLK_AUD_MEMIF_UL5: usize;
    static MT8188_CLK_AUD_MEMIF_UL6: usize;
    static MT8188_CLK_AUD_MEMIF_UL8: usize;
    static MT8188_CLK_AUD_MEMIF_UL9: usize;
    static MT8188_CLK_AUD_MEMIF_UL10: usize;
    static MT8188_CLK_AUD_MEMIF_DL2: usize;
    static MT8188_CLK_AUD_MEMIF_DL3: usize;
    static MT8188_CLK_AUD_MEMIF_DL6: usize;
    static MT8188_CLK_AUD_MEMIF_DL7: usize;
    static MT8188_CLK_AUD_MEMIF_DL8: usize;
    static MT8188_CLK_AUD_MEMIF_DL10: usize;
    static MT8188_CLK_AUD_MEMIF_DL11: usize;
    static MT8188_AUD_PLL_NUM: usize;
    static MT8188_AUD_PLL1: c_uint;
    static MT8188_AUD_PLL2: c_uint;
    static MT8188_AUD_PLL3: c_uint;
    static MT8188_AUD_PLL4: c_uint;
    static MT8188_AUD_PLL5: c_uint;
    static MT8188_MCK_SEL_26M: c_int;
    static MT8188_MCK_SEL_APLL1: c_int;
    static MT8188_MCK_SEL_APLL2: c_int;
    static MT8188_TOP_CG_A1SYS_TIMING: c_uint;
    static MT8188_TOP_CG_A2SYS_TIMING: c_uint;
    static MT8188_TOP_CG_26M_TIMING: c_uint;
    static ASYS_TOP_CON: c_uint;
    static ASYS_TOP_CON_A1SYS_TIMING_ON: c_uint;
    static ASYS_TOP_CON_A2SYS_TIMING_ON: c_uint;
    static ASYS_TOP_CON_26M_TIMING_ON: c_uint;
    static AFE_APLL_TUNER_CFG: c_int;
    static AFE_APLL_TUNER_CFG1: c_int;
    static AFE_EARC_APLL_TUNER_CFG: c_int;
    static AFE_SPDIFIN_APLL_TUNER_CFG: c_int;
    static AFE_SPDIFIN_APLL_TUNER_CFG1: c_int;
    static AFE_LINEIN_APLL_TUNER_CFG: c_int;
    static AFE_DAC_CON0: c_int;
    static GFP_KERNEL: c_uint;
    static EINVAL: c_int;
    static ENOMEM: c_int;
    static APLL1_W_NAME: *const c_char;

    fn mt8188_audsys_clk_register(afe: *mut mtk_base_afe) -> c_int;
    fn regmap_update_bits(regmap: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn devm_kcalloc(dev: *mut device, n: usize, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_clk_get(dev: *mut device, id: *const c_char) -> *mut clk;
    fn clk_prepare_enable(clk: *mut clk) -> c_int;
    fn clk_disable_unprepare(clk: *mut clk);
    fn clk_set_rate(clk: *mut clk, rate: c_uint) -> c_int;
    fn clk_set_parent(clk: *mut clk, parent: *mut clk) -> c_int;
    fn clk_get_rate(clk: *mut clk) -> c_ulong;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn spin_lock_init(lock: *mut spinlock_t);
    fn spin_lock_irqsave(lock: *mut spinlock_t) -> c_ulong;
    fn spin_unlock_irqrestore(lock: *mut spinlock_t, flags: c_ulong);
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_long;
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_info(dev: *mut device, fmt: *const c_char, ...);
}

#[repr(C)]
pub struct clk {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct spinlock_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mtk_base_afe {
    pub regmap: *mut regmap,
    pub dev: *mut device,
    pub platform_priv: *mut mt8188_afe_private,
}

#[repr(C)]
pub struct mt8188_afe_private {
    pub clk: *mut *mut clk,
}

#[repr(C)]
struct mt8188_afe_tuner_cfg {
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

fn bit(nr: c_uint) -> c_uint {
    1u32 << nr
}

static mut AUD_CLKS: [*const c_char; 57] = [ptr::null(); 57];
static mut MT8188_AFE_TUNER_CFGS: [mt8188_afe_tuner_cfg; 5] = unsafe {
    [
        mt8188_afe_tuner_cfg {
            id: MT8188_AUD_PLL1,
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
            ctrl_lock: spinlock_t { _private: [] },
            ref_cnt: 0,
        },
        mt8188_afe_tuner_cfg {
            id: MT8188_AUD_PLL2,
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
            ctrl_lock: spinlock_t { _private: [] },
            ref_cnt: 0,
        },
        mt8188_afe_tuner_cfg {
            id: MT8188_AUD_PLL3,
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
            ctrl_lock: spinlock_t { _private: [] },
            ref_cnt: 0,
        },
        mt8188_afe_tuner_cfg {
            id: MT8188_AUD_PLL4,
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
            ctrl_lock: spinlock_t { _private: [] },
            ref_cnt: 0,
        },
        mt8188_afe_tuner_cfg {
            id: MT8188_AUD_PLL5,
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
            ctrl_lock: spinlock_t { _private: [] },
            ref_cnt: 0,
        },
    ]
};

unsafe fn clk_at(afe_priv: *mut mt8188_afe_private, id: usize) -> *mut clk {
    *(*afe_priv).clk.add(id)
}

unsafe fn init_aud_clks() {
    /* xtal */
    AUD_CLKS[MT8188_CLK_XTAL_26M] = c"clk26m".as_ptr();

    /* pll */
    AUD_CLKS[MT8188_CLK_APMIXED_APLL1] = c"apll1".as_ptr();
    AUD_CLKS[MT8188_CLK_APMIXED_APLL2] = c"apll2".as_ptr();

    /* divider */
    AUD_CLKS[MT8188_CLK_TOP_APLL1_D4] = c"apll1_d4".as_ptr();
    AUD_CLKS[MT8188_CLK_TOP_APLL2_D4] = c"apll2_d4".as_ptr();
    AUD_CLKS[MT8188_CLK_TOP_APLL12_DIV0] = c"apll12_div0".as_ptr();
    AUD_CLKS[MT8188_CLK_TOP_APLL12_DIV1] = c"apll12_div1".as_ptr();
    AUD_CLKS[MT8188_CLK_TOP_APLL12_DIV2] = c"apll12_div2".as_ptr();
    AUD_CLKS[MT8188_CLK_TOP_APLL12_DIV3] = c"apll12_div3".as_ptr();
    AUD_CLKS[MT8188_CLK_TOP_APLL12_DIV4] = c"apll12_div4".as_ptr();
    AUD_CLKS[MT8188_CLK_TOP_APLL12_DIV9] = c"apll12_div9".as_ptr();

    /* mux */
    AUD_CLKS[MT8188_CLK_TOP_A1SYS_HP_SEL] = c"top_a1sys_hp".as_ptr();
    AUD_CLKS[MT8188_CLK_TOP_A2SYS_SEL] = c"top_a2sys".as_ptr();
    AUD_CLKS[MT8188_CLK_TOP_AUD_IEC_SEL] = c"top_aud_iec".as_ptr();
    AUD_CLKS[MT8188_CLK_TOP_AUD_INTBUS_SEL] = c"top_aud_intbus".as_ptr();
    AUD_CLKS[MT8188_CLK_TOP_AUDIO_H_SEL] = c"top_audio_h".as_ptr();
    AUD_CLKS[MT8188_CLK_TOP_AUDIO_LOCAL_BUS_SEL] = c"top_audio_local_bus".as_ptr();
    AUD_CLKS[MT8188_CLK_TOP_DPTX_M_SEL] = c"top_dptx".as_ptr();
    AUD_CLKS[MT8188_CLK_TOP_I2SO1_M_SEL] = c"top_i2so1".as_ptr();
    AUD_CLKS[MT8188_CLK_TOP_I2SO2_M_SEL] = c"top_i2so2".as_ptr();
    AUD_CLKS[MT8188_CLK_TOP_I2SI1_M_SEL] = c"top_i2si1".as_ptr();
    AUD_CLKS[MT8188_CLK_TOP_I2SI2_M_SEL] = c"top_i2si2".as_ptr();

    /* clock gate */
    AUD_CLKS[MT8188_CLK_ADSP_AUDIO_26M] = c"adsp_audio_26m".as_ptr();
    /* afe clock gate */
    AUD_CLKS[MT8188_CLK_AUD_AFE] = c"aud_afe".as_ptr();
    AUD_CLKS[MT8188_CLK_AUD_APLL1_TUNER] = c"aud_apll1_tuner".as_ptr();
    AUD_CLKS[MT8188_CLK_AUD_APLL2_TUNER] = c"aud_apll2_tuner".as_ptr();
    AUD_CLKS[MT8188_CLK_AUD_APLL] = c"aud_apll".as_ptr();
    AUD_CLKS[MT8188_CLK_AUD_APLL2] = c"aud_apll2".as_ptr();
    AUD_CLKS[MT8188_CLK_AUD_DAC] = c"aud_dac".as_ptr();
    AUD_CLKS[MT8188_CLK_AUD_ADC] = c"aud_adc".as_ptr();
    AUD_CLKS[MT8188_CLK_AUD_DAC_HIRES] = c"aud_dac_hires".as_ptr();
    AUD_CLKS[MT8188_CLK_AUD_A1SYS_HP] = c"aud_a1sys_hp".as_ptr();
    AUD_CLKS[MT8188_CLK_AUD_AFE_DMIC1] = c"aud_afe_dmic1".as_ptr();
    AUD_CLKS[MT8188_CLK_AUD_AFE_DMIC2] = c"aud_afe_dmic2".as_ptr();
    AUD_CLKS[MT8188_CLK_AUD_AFE_DMIC3] = c"aud_afe_dmic3".as_ptr();
    AUD_CLKS[MT8188_CLK_AUD_AFE_DMIC4] = c"aud_afe_dmic4".as_ptr();
    AUD_CLKS[MT8188_CLK_AUD_ADC_HIRES] = c"aud_adc_hires".as_ptr();
    AUD_CLKS[MT8188_CLK_AUD_DMIC_HIRES1] = c"aud_dmic_hires1".as_ptr();
    AUD_CLKS[MT8188_CLK_AUD_DMIC_HIRES2] = c"aud_dmic_hires2".as_ptr();
    AUD_CLKS[MT8188_CLK_AUD_DMIC_HIRES3] = c"aud_dmic_hires3".as_ptr();
    AUD_CLKS[MT8188_CLK_AUD_DMIC_HIRES4] = c"aud_dmic_hires4".as_ptr();
    AUD_CLKS[MT8188_CLK_AUD_I2SIN] = c"aud_i2sin".as_ptr();
    AUD_CLKS[MT8188_CLK_AUD_TDM_IN] = c"aud_tdm_in".as_ptr();
    AUD_CLKS[MT8188_CLK_AUD_I2S_OUT] = c"aud_i2s_out".as_ptr();
    AUD_CLKS[MT8188_CLK_AUD_TDM_OUT] = c"aud_tdm_out".as_ptr();
    AUD_CLKS[MT8188_CLK_AUD_HDMI_OUT] = c"aud_hdmi_out".as_ptr();
    AUD_CLKS[MT8188_CLK_AUD_ASRC11] = c"aud_asrc11".as_ptr();
    AUD_CLKS[MT8188_CLK_AUD_ASRC12] = c"aud_asrc12".as_ptr();
    AUD_CLKS[MT8188_CLK_AUD_A1SYS] = c"aud_a1sys".as_ptr();
    AUD_CLKS[MT8188_CLK_AUD_A2SYS] = c"aud_a2sys".as_ptr();
    AUD_CLKS[MT8188_CLK_AUD_PCMIF] = c"aud_pcmif".as_ptr();
    AUD_CLKS[MT8188_CLK_AUD_MEMIF_UL1] = c"aud_memif_ul1".as_ptr();
    AUD_CLKS[MT8188_CLK_AUD_MEMIF_UL2] = c"aud_memif_ul2".as_ptr();
    AUD_CLKS[MT8188_CLK_AUD_MEMIF_UL3] = c"aud_memif_ul3".as_ptr();
    AUD_CLKS[MT8188_CLK_AUD_MEMIF_UL4] = c"aud_memif_ul4".as_ptr();
    AUD_CLKS[MT8188_CLK_AUD_MEMIF_UL5] = c"aud_memif_ul5".as_ptr();
    AUD_CLKS[MT8188_CLK_AUD_MEMIF_UL6] = c"aud_memif_ul6".as_ptr();
    AUD_CLKS[MT8188_CLK_AUD_MEMIF_UL8] = c"aud_memif_ul8".as_ptr();
    AUD_CLKS[MT8188_CLK_AUD_MEMIF_UL9] = c"aud_memif_ul9".as_ptr();
    AUD_CLKS[MT8188_CLK_AUD_MEMIF_UL10] = c"aud_memif_ul10".as_ptr();
    AUD_CLKS[MT8188_CLK_AUD_MEMIF_DL2] = c"aud_memif_dl2".as_ptr();
    AUD_CLKS[MT8188_CLK_AUD_MEMIF_DL3] = c"aud_memif_dl3".as_ptr();
    AUD_CLKS[MT8188_CLK_AUD_MEMIF_DL6] = c"aud_memif_dl6".as_ptr();
    AUD_CLKS[MT8188_CLK_AUD_MEMIF_DL7] = c"aud_memif_dl7".as_ptr();
    AUD_CLKS[MT8188_CLK_AUD_MEMIF_DL8] = c"aud_memif_dl8".as_ptr();
    AUD_CLKS[MT8188_CLK_AUD_MEMIF_DL10] = c"aud_memif_dl10".as_ptr();
    AUD_CLKS[MT8188_CLK_AUD_MEMIF_DL11] = c"aud_memif_dl11".as_ptr();
}

unsafe fn mt8188_afe_found_apll_tuner(id: c_uint) -> *mut mt8188_afe_tuner_cfg {
    if id >= MT8188_AUD_PLL_NUM as c_uint {
        return ptr::null_mut();
    }

    &mut MT8188_AFE_TUNER_CFGS[id as usize]
}

unsafe fn mt8188_afe_init_apll_tuner(id: c_uint) -> c_int {
    let cfg = mt8188_afe_found_apll_tuner(id);

    if cfg.is_null() {
        return -EINVAL;
    }

    (*cfg).ref_cnt = 0;
    spin_lock_init(&mut (*cfg).ctrl_lock);

    0
}

unsafe fn mt8188_afe_setup_apll_tuner(afe: *mut mtk_base_afe, id: c_uint) -> c_int {
    let cfg = mt8188_afe_found_apll_tuner(id);

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

unsafe fn mt8188_afe_enable_tuner_clk(afe: *mut mtk_base_afe, id: c_uint) -> c_int {
    let afe_priv = (*afe).platform_priv;

    if id == MT8188_AUD_PLL1 {
        mt8188_afe_enable_clk(afe, clk_at(afe_priv, MT8188_CLK_AUD_APLL));
        mt8188_afe_enable_clk(afe, clk_at(afe_priv, MT8188_CLK_AUD_APLL1_TUNER));
    } else if id == MT8188_AUD_PLL2 {
        mt8188_afe_enable_clk(afe, clk_at(afe_priv, MT8188_CLK_AUD_APLL2));
        mt8188_afe_enable_clk(afe, clk_at(afe_priv, MT8188_CLK_AUD_APLL2_TUNER));
    } else {
        return -EINVAL;
    }

    0
}

unsafe fn mt8188_afe_disable_tuner_clk(afe: *mut mtk_base_afe, id: c_uint) -> c_int {
    let afe_priv = (*afe).platform_priv;

    if id == MT8188_AUD_PLL1 {
        mt8188_afe_disable_clk(afe, clk_at(afe_priv, MT8188_CLK_AUD_APLL1_TUNER));
        mt8188_afe_disable_clk(afe, clk_at(afe_priv, MT8188_CLK_AUD_APLL));
    } else if id == MT8188_AUD_PLL2 {
        mt8188_afe_disable_clk(afe, clk_at(afe_priv, MT8188_CLK_AUD_APLL2_TUNER));
        mt8188_afe_disable_clk(afe, clk_at(afe_priv, MT8188_CLK_AUD_APLL2));
    } else {
        return -EINVAL;
    }

    0
}

unsafe fn mt8188_afe_enable_apll_tuner(afe: *mut mtk_base_afe, id: c_uint) -> c_int {
    let cfg = mt8188_afe_found_apll_tuner(id);
    let mut ret: c_int;

    if cfg.is_null() {
        return -EINVAL;
    }

    ret = mt8188_afe_setup_apll_tuner(afe, id);
    if ret != 0 {
        return ret;
    }

    ret = mt8188_afe_enable_tuner_clk(afe, id);
    if ret != 0 {
        return ret;
    }

    let flags = spin_lock_irqsave(&mut (*cfg).ctrl_lock);
    (*cfg).ref_cnt += 1;
    if (*cfg).ref_cnt == 1 {
        regmap_update_bits(
            (*afe).regmap,
            (*cfg).tuner_en_reg as c_uint,
            (*cfg).tuner_en_maskbit << (*cfg).tuner_en_shift,
            bit((*cfg).tuner_en_shift),
        );
    }
    spin_unlock_irqrestore(&mut (*cfg).ctrl_lock, flags);

    0
}

unsafe fn mt8188_afe_disable_apll_tuner(afe: *mut mtk_base_afe, id: c_uint) -> c_int {
    let cfg = mt8188_afe_found_apll_tuner(id);
    let ret: c_int;

    if cfg.is_null() {
        return -EINVAL;
    }

    let flags = spin_lock_irqsave(&mut (*cfg).ctrl_lock);
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

    ret = mt8188_afe_disable_tuner_clk(afe, id);
    if ret != 0 {
        return ret;
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn mt8188_afe_get_mclk_source_clk_id(sel: c_int) -> c_int {
    if sel == MT8188_MCK_SEL_26M {
        MT8188_CLK_XTAL_26M as c_int
    } else if sel == MT8188_MCK_SEL_APLL1 {
        MT8188_CLK_APMIXED_APLL1 as c_int
    } else if sel == MT8188_MCK_SEL_APLL2 {
        MT8188_CLK_APMIXED_APLL2 as c_int
    } else {
        -EINVAL
    }
}

#[no_mangle]
pub unsafe extern "C" fn mt8188_afe_get_mclk_source_rate(
    afe: *mut mtk_base_afe,
    apll: c_int,
) -> c_int {
    let afe_priv = (*afe).platform_priv;
    let clk_id = mt8188_afe_get_mclk_source_clk_id(apll);

    if clk_id < 0 {
        dev_dbg((*afe).dev, c"invalid clk id\n".as_ptr());
        return 0;
    }

    clk_get_rate(clk_at(afe_priv, clk_id as usize)) as c_int
}

#[no_mangle]
pub unsafe extern "C" fn mt8188_afe_get_default_mclk_source_by_rate(rate: c_int) -> c_int {
    if (rate % 8000) == 0 {
        MT8188_MCK_SEL_APLL1
    } else {
        MT8188_MCK_SEL_APLL2
    }
}

#[no_mangle]
pub unsafe extern "C" fn mt8188_get_apll_by_rate(_afe: *mut mtk_base_afe, rate: c_int) -> c_int {
    if (rate % 8000) == 0 {
        MT8188_AUD_PLL1 as c_int
    } else {
        MT8188_AUD_PLL2 as c_int
    }
}

#[no_mangle]
pub unsafe extern "C" fn mt8188_get_apll_by_name(
    _afe: *mut mtk_base_afe,
    name: *const c_char,
) -> c_int {
    if strcmp(name, APLL1_W_NAME) == 0 {
        return MT8188_AUD_PLL1 as c_int;
    }

    MT8188_AUD_PLL2 as c_int
}

#[no_mangle]
pub unsafe extern "C" fn mt8188_afe_init_clock(afe: *mut mtk_base_afe) -> c_int {
    let afe_priv = (*afe).platform_priv;
    let mut i: c_int;
    let mut ret: c_int;

    init_aud_clks();

    ret = mt8188_audsys_clk_register(afe);
    if ret != 0 {
        dev_err((*afe).dev, c"register audsys clk fail %d\n".as_ptr(), ret);
        return ret;
    }

    (*afe_priv).clk = devm_kcalloc(
        (*afe).dev,
        MT8188_CLK_NUM,
        core::mem::size_of::<*mut clk>(),
        GFP_KERNEL,
    ) as *mut *mut clk;
    if (*afe_priv).clk.is_null() {
        return -ENOMEM;
    }

    i = 0;
    while i < MT8188_CLK_NUM as c_int {
        *(*afe_priv).clk.add(i as usize) = devm_clk_get((*afe).dev, AUD_CLKS[i as usize]);
        if IS_ERR(*(*afe_priv).clk.add(i as usize) as *const c_void) {
            dev_err(
                (*afe).dev,
                c"%s(), devm_clk_get %s fail, ret %ld\n".as_ptr(),
                c"mt8188_afe_init_clock".as_ptr(),
                AUD_CLKS[i as usize],
                PTR_ERR(*(*afe_priv).clk.add(i as usize) as *const c_void),
            );
            return PTR_ERR(*(*afe_priv).clk.add(i as usize) as *const c_void) as c_int;
        }
        i += 1;
    }

    /* initial tuner */
    i = 0;
    while i < MT8188_AUD_PLL_NUM as c_int {
        ret = mt8188_afe_init_apll_tuner(i as c_uint);
        if ret != 0 {
            dev_info(
                (*afe).dev,
                c"%s(), init apll_tuner%d failed".as_ptr(),
                c"mt8188_afe_init_clock".as_ptr(),
                i + 1,
            );
            return -EINVAL;
        }
        i += 1;
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn mt8188_afe_enable_clk(afe: *mut mtk_base_afe, clk: *mut clk) -> c_int {
    let ret: c_int;

    if !clk.is_null() {
        ret = clk_prepare_enable(clk);
        if ret != 0 {
            dev_dbg(
                (*afe).dev,
                c"%s(), failed to enable clk\n".as_ptr(),
                c"mt8188_afe_enable_clk".as_ptr(),
            );
            return ret;
        }
    } else {
        dev_dbg((*afe).dev, c"NULL clk\n".as_ptr());
    }
    0
}
// EXPORT_SYMBOL_GPL(mt8188_afe_enable_clk);

#[no_mangle]
pub unsafe extern "C" fn mt8188_afe_disable_clk(afe: *mut mtk_base_afe, clk: *mut clk) {
    if !clk.is_null() {
        clk_disable_unprepare(clk);
    } else {
        dev_dbg((*afe).dev, c"NULL clk\n".as_ptr());
    }
}
// EXPORT_SYMBOL_GPL(mt8188_afe_disable_clk);

#[no_mangle]
pub unsafe extern "C" fn mt8188_afe_set_clk_rate(
    afe: *mut mtk_base_afe,
    clk: *mut clk,
    rate: c_uint,
) -> c_int {
    let ret: c_int;

    if !clk.is_null() {
        ret = clk_set_rate(clk, rate);
        if ret != 0 {
            dev_dbg(
                (*afe).dev,
                c"%s(), failed to set clk rate\n".as_ptr(),
                c"mt8188_afe_set_clk_rate".as_ptr(),
            );
            return ret;
        }
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn mt8188_afe_set_clk_parent(
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
                c"%s(), failed to set clk parent %d\n".as_ptr(),
                c"mt8188_afe_set_clk_parent".as_ptr(),
                ret,
            );
            return ret;
        }
    }

    0
}

unsafe fn get_top_cg_reg(cg_type: c_uint) -> c_uint {
    if cg_type == MT8188_TOP_CG_A1SYS_TIMING
        || cg_type == MT8188_TOP_CG_A2SYS_TIMING
        || cg_type == MT8188_TOP_CG_26M_TIMING
    {
        ASYS_TOP_CON
    } else {
        0
    }
}

unsafe fn get_top_cg_mask(cg_type: c_uint) -> c_uint {
    if cg_type == MT8188_TOP_CG_A1SYS_TIMING {
        ASYS_TOP_CON_A1SYS_TIMING_ON
    } else if cg_type == MT8188_TOP_CG_A2SYS_TIMING {
        ASYS_TOP_CON_A2SYS_TIMING_ON
    } else if cg_type == MT8188_TOP_CG_26M_TIMING {
        ASYS_TOP_CON_26M_TIMING_ON
    } else {
        0
    }
}

unsafe fn get_top_cg_on_val(cg_type: c_uint) -> c_uint {
    if cg_type == MT8188_TOP_CG_A1SYS_TIMING
        || cg_type == MT8188_TOP_CG_A2SYS_TIMING
        || cg_type == MT8188_TOP_CG_26M_TIMING
    {
        get_top_cg_mask(cg_type)
    } else {
        0
    }
}

unsafe fn get_top_cg_off_val(cg_type: c_uint) -> c_uint {
    if cg_type == MT8188_TOP_CG_A1SYS_TIMING
        || cg_type == MT8188_TOP_CG_A2SYS_TIMING
        || cg_type == MT8188_TOP_CG_26M_TIMING
    {
        0
    } else {
        get_top_cg_mask(cg_type)
    }
}

unsafe fn mt8188_afe_enable_top_cg(afe: *mut mtk_base_afe, cg_type: c_uint) -> c_int {
    let reg = get_top_cg_reg(cg_type);
    let mask = get_top_cg_mask(cg_type);
    let val = get_top_cg_on_val(cg_type);

    regmap_update_bits((*afe).regmap, reg, mask, val);

    0
}

unsafe fn mt8188_afe_disable_top_cg(afe: *mut mtk_base_afe, cg_type: c_uint) -> c_int {
    let reg = get_top_cg_reg(cg_type);
    let mask = get_top_cg_mask(cg_type);
    let val = get_top_cg_off_val(cg_type);

    regmap_update_bits((*afe).regmap, reg, mask, val);

    0
}

#[no_mangle]
pub unsafe extern "C" fn mt8188_afe_enable_reg_rw_clk(afe: *mut mtk_base_afe) -> c_int {
    let afe_priv = (*afe).platform_priv;

    /* bus clock for AFE external access, like DRAM */
    mt8188_afe_enable_clk(afe, clk_at(afe_priv, MT8188_CLK_TOP_AUDIO_LOCAL_BUS_SEL));

    /* bus clock for AFE internal access, like AFE SRAM */
    mt8188_afe_enable_clk(afe, clk_at(afe_priv, MT8188_CLK_TOP_AUD_INTBUS_SEL));

    /* audio 26m clock source */
    mt8188_afe_enable_clk(afe, clk_at(afe_priv, MT8188_CLK_ADSP_AUDIO_26M));

    /* AFE hw clock */
    mt8188_afe_enable_clk(afe, clk_at(afe_priv, MT8188_CLK_AUD_AFE));
    mt8188_afe_enable_clk(afe, clk_at(afe_priv, MT8188_CLK_AUD_A1SYS_HP));
    mt8188_afe_enable_clk(afe, clk_at(afe_priv, MT8188_CLK_AUD_A1SYS));

    0
}

#[no_mangle]
pub unsafe extern "C" fn mt8188_afe_disable_reg_rw_clk(afe: *mut mtk_base_afe) -> c_int {
    let afe_priv = (*afe).platform_priv;

    mt8188_afe_disable_clk(afe, clk_at(afe_priv, MT8188_CLK_AUD_A1SYS));
    mt8188_afe_disable_clk(afe, clk_at(afe_priv, MT8188_CLK_AUD_A1SYS_HP));
    mt8188_afe_disable_clk(afe, clk_at(afe_priv, MT8188_CLK_AUD_AFE));
    mt8188_afe_disable_clk(afe, clk_at(afe_priv, MT8188_CLK_ADSP_AUDIO_26M));
    mt8188_afe_disable_clk(afe, clk_at(afe_priv, MT8188_CLK_TOP_AUD_INTBUS_SEL));
    mt8188_afe_disable_clk(afe, clk_at(afe_priv, MT8188_CLK_TOP_AUDIO_LOCAL_BUS_SEL));

    0
}

unsafe fn mt8188_afe_enable_afe_on(afe: *mut mtk_base_afe) -> c_int {
    regmap_update_bits((*afe).regmap, AFE_DAC_CON0 as c_uint, 0x1, 0x1);
    0
}

unsafe fn mt8188_afe_disable_afe_on(afe: *mut mtk_base_afe) -> c_int {
    regmap_update_bits((*afe).regmap, AFE_DAC_CON0 as c_uint, 0x1, 0x0);
    0
}

unsafe fn mt8188_afe_enable_a1sys(afe: *mut mtk_base_afe) -> c_int {
    let afe_priv = (*afe).platform_priv;
    let ret: c_int;

    ret = mt8188_afe_enable_clk(afe, clk_at(afe_priv, MT8188_CLK_AUD_A1SYS));
    if ret != 0 {
        return ret;
    }

    mt8188_afe_enable_top_cg(afe, MT8188_TOP_CG_A1SYS_TIMING)
}

unsafe fn mt8188_afe_disable_a1sys(afe: *mut mtk_base_afe) -> c_int {
    let afe_priv = (*afe).platform_priv;

    mt8188_afe_disable_top_cg(afe, MT8188_TOP_CG_A1SYS_TIMING);
    mt8188_afe_disable_clk(afe, clk_at(afe_priv, MT8188_CLK_AUD_A1SYS));
    0
}

unsafe fn mt8188_afe_enable_a2sys(afe: *mut mtk_base_afe) -> c_int {
    let afe_priv = (*afe).platform_priv;
    let ret: c_int;

    ret = mt8188_afe_enable_clk(afe, clk_at(afe_priv, MT8188_CLK_AUD_A2SYS));
    if ret != 0 {
        return ret;
    }

    mt8188_afe_enable_top_cg(afe, MT8188_TOP_CG_A2SYS_TIMING)
}

unsafe fn mt8188_afe_disable_a2sys(afe: *mut mtk_base_afe) -> c_int {
    let afe_priv = (*afe).platform_priv;

    mt8188_afe_disable_top_cg(afe, MT8188_TOP_CG_A2SYS_TIMING);
    mt8188_afe_disable_clk(afe, clk_at(afe_priv, MT8188_CLK_AUD_A2SYS));
    0
}

#[no_mangle]
pub unsafe extern "C" fn mt8188_apll1_enable(afe: *mut mtk_base_afe) -> c_int {
    let afe_priv = (*afe).platform_priv;
    let mut ret: c_int;

    ret = mt8188_afe_enable_clk(afe, clk_at(afe_priv, MT8188_CLK_TOP_APLL1_D4));
    if ret != 0 {
        return ret;
    }

    ret = mt8188_afe_set_clk_parent(
        afe,
        clk_at(afe_priv, MT8188_CLK_TOP_A1SYS_HP_SEL),
        clk_at(afe_priv, MT8188_CLK_TOP_APLL1_D4),
    );
    if ret != 0 {
        mt8188_afe_disable_clk(afe, clk_at(afe_priv, MT8188_CLK_TOP_APLL1_D4));
        return ret;
    }

    ret = mt8188_afe_enable_apll_tuner(afe, MT8188_AUD_PLL1);
    if ret != 0 {
        mt8188_afe_set_clk_parent(
            afe,
            clk_at(afe_priv, MT8188_CLK_TOP_A1SYS_HP_SEL),
            clk_at(afe_priv, MT8188_CLK_XTAL_26M),
        );
        mt8188_afe_disable_clk(afe, clk_at(afe_priv, MT8188_CLK_TOP_APLL1_D4));
        return ret;
    }

    ret = mt8188_afe_enable_a1sys(afe);
    if ret != 0 {
        mt8188_afe_disable_apll_tuner(afe, MT8188_AUD_PLL1);
        mt8188_afe_set_clk_parent(
            afe,
            clk_at(afe_priv, MT8188_CLK_TOP_A1SYS_HP_SEL),
            clk_at(afe_priv, MT8188_CLK_XTAL_26M),
        );
        mt8188_afe_disable_clk(afe, clk_at(afe_priv, MT8188_CLK_TOP_APLL1_D4));
        return ret;
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn mt8188_apll1_disable(afe: *mut mtk_base_afe) -> c_int {
    let afe_priv = (*afe).platform_priv;

    mt8188_afe_disable_a1sys(afe);
    mt8188_afe_disable_apll_tuner(afe, MT8188_AUD_PLL1);
    mt8188_afe_set_clk_parent(
        afe,
        clk_at(afe_priv, MT8188_CLK_TOP_A1SYS_HP_SEL),
        clk_at(afe_priv, MT8188_CLK_XTAL_26M),
    );
    mt8188_afe_disable_clk(afe, clk_at(afe_priv, MT8188_CLK_TOP_APLL1_D4));

    0
}

#[no_mangle]
pub unsafe extern "C" fn mt8188_apll2_enable(afe: *mut mtk_base_afe) -> c_int {
    let mut ret: c_int;

    ret = mt8188_afe_enable_apll_tuner(afe, MT8188_AUD_PLL2);
    if ret != 0 {
        return ret;
    }

    ret = mt8188_afe_enable_a2sys(afe);
    if ret != 0 {
        mt8188_afe_disable_apll_tuner(afe, MT8188_AUD_PLL2);
        return ret;
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn mt8188_apll2_disable(afe: *mut mtk_base_afe) -> c_int {
    mt8188_afe_disable_a2sys(afe);
    mt8188_afe_disable_apll_tuner(afe, MT8188_AUD_PLL2);
    0
}

#[no_mangle]
pub unsafe extern "C" fn mt8188_afe_enable_main_clock(afe: *mut mtk_base_afe) -> c_int {
    mt8188_afe_enable_top_cg(afe, MT8188_TOP_CG_26M_TIMING);
    mt8188_afe_enable_afe_on(afe);
    0
}

#[no_mangle]
pub unsafe extern "C" fn mt8188_afe_disable_main_clock(afe: *mut mtk_base_afe) -> c_int {
    mt8188_afe_disable_afe_on(afe);
    mt8188_afe_disable_top_cg(afe, MT8188_TOP_CG_26M_TIMING);
    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
