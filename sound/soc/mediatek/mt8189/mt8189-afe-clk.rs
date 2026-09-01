// SPDX-License-Identifier: GPL-2.0
/*
 *  mt8189-afe-clk.c  --  Mediatek 8189 afe clock ctrl
 *
 *  Copyright (c) 2025 MediaTek Inc.
 *  Author: Darren Ye <darren.ye@mediatek.com>
 */

use core::ffi::{c_char, c_int, c_uint, c_void};

// Dependencies from linux/clk.h, linux/regmap.h, linux/mfd/syscon.h,
// mt8189-afe-common.h, and mt8189-afe-clk.h are expected to be supplied by
// surrounding translated units.
unsafe extern "C" {
    fn clk_prepare_enable(clk: *mut clk) -> c_int;
    fn clk_disable_unprepare(clk: *mut clk);
    fn clk_set_rate(clk: *mut clk, rate: c_uint) -> c_int;
    fn clk_set_parent(clk: *mut clk, parent: *mut clk) -> c_int;
    fn clk_get_rate(clk: *mut clk) -> c_int;
    fn regmap_update_bits(map: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn devm_kcalloc(dev: *mut device, n: usize, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_clk_get(dev: *mut device, id: *const c_char) -> *mut clk;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
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
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mtk_base_afe {
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub platform_priv: *mut c_void,
}

#[repr(C)]
pub struct mt8189_afe_private {
    pub clk: *mut *mut clk,
}

/* mck */
#[repr(C)]
#[derive(Copy, Clone)]
struct mt8189_mck_div {
    m_sel_id: c_int,
    div_clk_id: c_int,
}

static mck_div: [mt8189_mck_div; MT8189_MCK_NUM as usize] = {
    let mut a = [mt8189_mck_div {
        m_sel_id: 0,
        div_clk_id: 0,
    }; MT8189_MCK_NUM as usize];
    a[MT8189_I2SIN0_MCK as usize] = mt8189_mck_div {
        m_sel_id: MT8189_CLK_TOP_I2SIN0_M_SEL,
        div_clk_id: MT8189_CLK_TOP_APLL12_DIV_I2SIN0,
    };
    a[MT8189_I2SIN1_MCK as usize] = mt8189_mck_div {
        m_sel_id: MT8189_CLK_TOP_I2SIN1_M_SEL,
        div_clk_id: MT8189_CLK_TOP_APLL12_DIV_I2SIN1,
    };
    a[MT8189_I2SOUT0_MCK as usize] = mt8189_mck_div {
        m_sel_id: MT8189_CLK_TOP_I2SOUT0_M_SEL,
        div_clk_id: MT8189_CLK_TOP_APLL12_DIV_I2SOUT0,
    };
    a[MT8189_I2SOUT1_MCK as usize] = mt8189_mck_div {
        m_sel_id: MT8189_CLK_TOP_I2SOUT1_M_SEL,
        div_clk_id: MT8189_CLK_TOP_APLL12_DIV_I2SOUT1,
    };
    a[MT8189_FMI2S_MCK as usize] = mt8189_mck_div {
        m_sel_id: MT8189_CLK_TOP_FMI2S_M_SEL,
        div_clk_id: MT8189_CLK_TOP_APLL12_DIV_FMI2S,
    };
    a[MT8189_TDMOUT_MCK as usize] = mt8189_mck_div {
        m_sel_id: MT8189_CLK_TOP_TDMOUT_M_SEL,
        div_clk_id: MT8189_CLK_TOP_APLL12_DIV_TDMOUT_M,
    };
    a[MT8189_TDMOUT_BCK as usize] = mt8189_mck_div {
        m_sel_id: -1,
        div_clk_id: MT8189_CLK_TOP_APLL12_DIV_TDMOUT_B,
    };
    a
};

static aud_clks: [*const c_char; MT8189_CLK_NUM as usize] = {
    let mut a = [core::ptr::null(); MT8189_CLK_NUM as usize];
    a[MT8189_CLK_TOP_MUX_AUDIOINTBUS as usize] = c"top_aud_intbus".as_ptr();
    a[MT8189_CLK_TOP_MUX_AUD_ENG1 as usize] = c"top_aud_eng1".as_ptr();
    a[MT8189_CLK_TOP_MUX_AUD_ENG2 as usize] = c"top_aud_eng2".as_ptr();
    a[MT8189_CLK_TOP_MUX_AUDIO_H as usize] = c"top_aud_h".as_ptr();
    /* pll */
    a[MT8189_CLK_TOP_APLL1_CK as usize] = c"apll1".as_ptr();
    a[MT8189_CLK_TOP_APLL2_CK as usize] = c"apll2".as_ptr();
    /* divider */
    a[MT8189_CLK_TOP_APLL1_D4 as usize] = c"apll1_d4".as_ptr();
    a[MT8189_CLK_TOP_APLL2_D4 as usize] = c"apll2_d4".as_ptr();
    a[MT8189_CLK_TOP_APLL12_DIV_I2SIN0 as usize] = c"apll12_div_i2sin0".as_ptr();
    a[MT8189_CLK_TOP_APLL12_DIV_I2SIN1 as usize] = c"apll12_div_i2sin1".as_ptr();
    a[MT8189_CLK_TOP_APLL12_DIV_I2SOUT0 as usize] = c"apll12_div_i2sout0".as_ptr();
    a[MT8189_CLK_TOP_APLL12_DIV_I2SOUT1 as usize] = c"apll12_div_i2sout1".as_ptr();
    a[MT8189_CLK_TOP_APLL12_DIV_FMI2S as usize] = c"apll12_div_fmi2s".as_ptr();
    a[MT8189_CLK_TOP_APLL12_DIV_TDMOUT_M as usize] = c"apll12_div_tdmout_m".as_ptr();
    a[MT8189_CLK_TOP_APLL12_DIV_TDMOUT_B as usize] = c"apll12_div_tdmout_b".as_ptr();
    /* mux */
    a[MT8189_CLK_TOP_MUX_AUD_1 as usize] = c"top_apll1".as_ptr();
    a[MT8189_CLK_TOP_MUX_AUD_2 as usize] = c"top_apll2".as_ptr();
    a[MT8189_CLK_TOP_I2SIN0_M_SEL as usize] = c"top_i2sin0".as_ptr();
    a[MT8189_CLK_TOP_I2SIN1_M_SEL as usize] = c"top_i2sin1".as_ptr();
    a[MT8189_CLK_TOP_I2SOUT0_M_SEL as usize] = c"top_i2sout0".as_ptr();
    a[MT8189_CLK_TOP_I2SOUT1_M_SEL as usize] = c"top_i2sout1".as_ptr();
    a[MT8189_CLK_TOP_FMI2S_M_SEL as usize] = c"top_fmi2s".as_ptr();
    a[MT8189_CLK_TOP_TDMOUT_M_SEL as usize] = c"top_dptx".as_ptr();
    /* top 26m*/
    a[MT8189_CLK_TOP_CLK26M as usize] = c"clk26m".as_ptr();
    /* peri */
    a[MT8189_CLK_PERAO_AUDIO_SLV_CK_PERI as usize] = c"aud_slv_ck_peri".as_ptr();
    a[MT8189_CLK_PERAO_AUDIO_MST_CK_PERI as usize] = c"aud_mst_ck_peri".as_ptr();
    a[MT8189_CLK_PERAO_INTBUS_CK_PERI as usize] = c"aud_intbus_ck_peri".as_ptr();
    a
};

#[unsafe(no_mangle)]
pub unsafe extern "C" fn mt8189_afe_enable_clk(afe: *mut mtk_base_afe, clk: *mut clk) -> c_int {
    let ret: c_int = unsafe { clk_prepare_enable(clk) };

    if ret != 0 {
        unsafe { dev_err((*afe).dev, c"failed to enable clk\n".as_ptr()) };
        return ret;
    }

    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn mt8189_afe_disable_clk(afe: *mut mtk_base_afe, clk: *mut clk) {
    if !clk.is_null() {
        unsafe { clk_disable_unprepare(clk) };
    } else {
        unsafe { dev_dbg((*afe).dev, c"NULL clk\n".as_ptr()) };
    }
}

unsafe fn mt8189_afe_set_clk_rate(
    afe: *mut mtk_base_afe,
    clk: *mut clk,
    rate: c_uint,
) -> c_int {
    let ret: c_int;

    if !clk.is_null() {
        ret = unsafe { clk_set_rate(clk, rate) };
        if ret != 0 {
            unsafe { dev_err((*afe).dev, c"failed to set clk rate\n".as_ptr()) };
            return ret;
        }
    }

    0
}

unsafe fn mt8189_afe_set_clk_parent(
    afe: *mut mtk_base_afe,
    clk: *mut clk,
    parent: *mut clk,
) -> c_int {
    let ret: c_int;

    if !clk.is_null() && !parent.is_null() {
        ret = unsafe { clk_set_parent(clk, parent) };
        if ret != 0 {
            unsafe { dev_dbg((*afe).dev, c"failed to set clk parent %d\n".as_ptr(), ret) };
            return ret;
        }
    }

    0
}

fn get_top_cg_reg(cg_type: c_uint) -> c_uint {
    match cg_type {
        MT8189_AUDIO_26M_EN_ON
        | MT8189_AUDIO_F3P25M_EN_ON
        | MT8189_AUDIO_APLL1_EN_ON
        | MT8189_AUDIO_APLL2_EN_ON => AUDIO_ENGEN_CON0,
        MT8189_CG_AUDIO_HOPPING_CK
        | MT8189_CG_AUDIO_F26M_CK
        | MT8189_CG_APLL1_CK
        | MT8189_CG_APLL2_CK
        | MT8189_PDN_APLL_TUNER2
        | MT8189_PDN_APLL_TUNER1 => AUDIO_TOP_CON4,
        _ => 0,
    }
}

fn get_top_cg_mask(cg_type: c_uint) -> c_uint {
    match cg_type {
        MT8189_AUDIO_26M_EN_ON => AUDIO_26M_EN_ON_MASK_SFT,
        MT8189_AUDIO_F3P25M_EN_ON => AUDIO_F3P25M_EN_ON_MASK_SFT,
        MT8189_AUDIO_APLL1_EN_ON => AUDIO_APLL1_EN_ON_MASK_SFT,
        MT8189_AUDIO_APLL2_EN_ON => AUDIO_APLL2_EN_ON_MASK_SFT,
        MT8189_CG_AUDIO_HOPPING_CK => CG_AUDIO_HOPPING_CK_MASK_SFT,
        MT8189_CG_AUDIO_F26M_CK => CG_AUDIO_F26M_CK_MASK_SFT,
        MT8189_CG_APLL1_CK => CG_APLL1_CK_MASK_SFT,
        MT8189_CG_APLL2_CK => CG_APLL2_CK_MASK_SFT,
        MT8189_PDN_APLL_TUNER2 => PDN_APLL_TUNER2_MASK_SFT,
        MT8189_PDN_APLL_TUNER1 => PDN_APLL_TUNER1_MASK_SFT,
        _ => 0,
    }
}

fn get_top_cg_on_val(cg_type: c_uint) -> c_uint {
    match cg_type {
        MT8189_AUDIO_26M_EN_ON
        | MT8189_AUDIO_F3P25M_EN_ON
        | MT8189_AUDIO_APLL1_EN_ON
        | MT8189_AUDIO_APLL2_EN_ON => get_top_cg_mask(cg_type),
        MT8189_CG_AUDIO_HOPPING_CK
        | MT8189_CG_AUDIO_F26M_CK
        | MT8189_CG_APLL1_CK
        | MT8189_CG_APLL2_CK
        | MT8189_PDN_APLL_TUNER2
        | MT8189_PDN_APLL_TUNER1 => 0,
        _ => 0,
    }
}

fn get_top_cg_off_val(cg_type: c_uint) -> c_uint {
    match cg_type {
        MT8189_AUDIO_26M_EN_ON
        | MT8189_AUDIO_F3P25M_EN_ON
        | MT8189_AUDIO_APLL1_EN_ON
        | MT8189_AUDIO_APLL2_EN_ON => 0,
        MT8189_CG_AUDIO_HOPPING_CK
        | MT8189_CG_AUDIO_F26M_CK
        | MT8189_CG_APLL1_CK
        | MT8189_CG_APLL2_CK
        | MT8189_PDN_APLL_TUNER2
        | MT8189_PDN_APLL_TUNER1 => get_top_cg_mask(cg_type),
        _ => get_top_cg_mask(cg_type),
    }
}

unsafe fn mt8189_afe_enable_top_cg(afe: *mut mtk_base_afe, cg_type: c_uint) -> c_int {
    let reg: c_uint = get_top_cg_reg(cg_type);
    let mask: c_uint = get_top_cg_mask(cg_type);
    let val: c_uint = get_top_cg_on_val(cg_type);

    if unsafe { (*afe).regmap.is_null() } {
        unsafe { dev_err((*afe).dev, c"afe regmap is null !!!\n".as_ptr()) };
        return 0;
    }

    unsafe { dev_dbg((*afe).dev, c"reg: 0x%x, mask: 0x%x, val: 0x%x\n".as_ptr(), reg, mask, val) };

    unsafe { regmap_update_bits((*afe).regmap, reg, mask, val) }
}

unsafe fn mt8189_afe_disable_top_cg(afe: *mut mtk_base_afe, cg_type: c_uint) {
    let reg: c_uint = get_top_cg_reg(cg_type);
    let mask: c_uint = get_top_cg_mask(cg_type);
    let val: c_uint = get_top_cg_off_val(cg_type);

    if unsafe { (*afe).regmap.is_null() } {
        unsafe { dev_warn((*afe).dev, c"skip regmap\n".as_ptr()) };
        return;
    }

    unsafe { dev_dbg((*afe).dev, c"reg: 0x%x, mask: 0x%x, val: 0x%x\n".as_ptr(), reg, mask, val) };
    unsafe { regmap_update_bits((*afe).regmap, reg, mask, val) };
}

unsafe fn clk_at(afe_priv: *mut mt8189_afe_private, id: c_int) -> *mut clk {
    unsafe { *(*afe_priv).clk.add(id as usize) }
}

unsafe fn apll1_mux_setting(afe: *mut mtk_base_afe, enable: bool) -> c_int {
    let afe_priv = unsafe { (*afe).platform_priv as *mut mt8189_afe_private };
    let mut ret: c_int;

    unsafe { dev_dbg((*afe).dev, c"enable: %d\n".as_ptr(), enable as c_int) };

    if enable {
        ret = unsafe { mt8189_afe_enable_clk(afe, clk_at(afe_priv, MT8189_CLK_TOP_MUX_AUD_1)) };
        if ret != 0 {
            return ret;
        }

        ret = unsafe {
            mt8189_afe_set_clk_parent(
                afe,
                clk_at(afe_priv, MT8189_CLK_TOP_MUX_AUD_1),
                clk_at(afe_priv, MT8189_CLK_TOP_APLL1_CK),
            )
        };
        if ret != 0 {
            unsafe { mt8189_afe_disable_clk(afe, clk_at(afe_priv, MT8189_CLK_TOP_MUX_AUD_1)) };
            return ret;
        }

        /* 180.6336 / 4 = 45.1584MHz */
        ret = unsafe { mt8189_afe_enable_clk(afe, clk_at(afe_priv, MT8189_CLK_TOP_MUX_AUD_ENG1)) };
        if ret != 0 {
            unsafe {
                mt8189_afe_set_clk_parent(
                    afe,
                    clk_at(afe_priv, MT8189_CLK_TOP_MUX_AUD_1),
                    clk_at(afe_priv, MT8189_CLK_TOP_CLK26M),
                );
                mt8189_afe_disable_clk(afe, clk_at(afe_priv, MT8189_CLK_TOP_MUX_AUD_1));
            }
            return ret;
        }

        ret = unsafe {
            mt8189_afe_set_clk_parent(
                afe,
                clk_at(afe_priv, MT8189_CLK_TOP_MUX_AUD_ENG1),
                clk_at(afe_priv, MT8189_CLK_TOP_APLL1_D4),
            )
        };
        if ret != 0 {
            unsafe {
                mt8189_afe_disable_clk(afe, clk_at(afe_priv, MT8189_CLK_TOP_MUX_AUD_ENG1));
                mt8189_afe_set_clk_parent(
                    afe,
                    clk_at(afe_priv, MT8189_CLK_TOP_MUX_AUD_1),
                    clk_at(afe_priv, MT8189_CLK_TOP_CLK26M),
                );
                mt8189_afe_disable_clk(afe, clk_at(afe_priv, MT8189_CLK_TOP_MUX_AUD_1));
            }
            return ret;
        }

        ret = unsafe { mt8189_afe_enable_clk(afe, clk_at(afe_priv, MT8189_CLK_TOP_MUX_AUDIO_H)) };
        if ret != 0 {
            unsafe {
                mt8189_afe_set_clk_parent(
                    afe,
                    clk_at(afe_priv, MT8189_CLK_TOP_MUX_AUD_ENG1),
                    clk_at(afe_priv, MT8189_CLK_TOP_CLK26M),
                );
                mt8189_afe_disable_clk(afe, clk_at(afe_priv, MT8189_CLK_TOP_MUX_AUD_ENG1));
                mt8189_afe_set_clk_parent(
                    afe,
                    clk_at(afe_priv, MT8189_CLK_TOP_MUX_AUD_1),
                    clk_at(afe_priv, MT8189_CLK_TOP_CLK26M),
                );
                mt8189_afe_disable_clk(afe, clk_at(afe_priv, MT8189_CLK_TOP_MUX_AUD_1));
            }
            return ret;
        }

        ret = unsafe {
            mt8189_afe_set_clk_parent(
                afe,
                clk_at(afe_priv, MT8189_CLK_TOP_MUX_AUDIO_H),
                clk_at(afe_priv, MT8189_CLK_TOP_APLL1_CK),
            )
        };
        if ret != 0 {
            unsafe {
                mt8189_afe_disable_clk(afe, clk_at(afe_priv, MT8189_CLK_TOP_MUX_AUDIO_H));
                mt8189_afe_set_clk_parent(
                    afe,
                    clk_at(afe_priv, MT8189_CLK_TOP_MUX_AUD_ENG1),
                    clk_at(afe_priv, MT8189_CLK_TOP_CLK26M),
                );
                mt8189_afe_disable_clk(afe, clk_at(afe_priv, MT8189_CLK_TOP_MUX_AUD_ENG1));
                mt8189_afe_set_clk_parent(
                    afe,
                    clk_at(afe_priv, MT8189_CLK_TOP_MUX_AUD_1),
                    clk_at(afe_priv, MT8189_CLK_TOP_CLK26M),
                );
                mt8189_afe_disable_clk(afe, clk_at(afe_priv, MT8189_CLK_TOP_MUX_AUD_1));
            }
            return ret;
        }
    } else {
        unsafe {
            mt8189_afe_set_clk_parent(
                afe,
                clk_at(afe_priv, MT8189_CLK_TOP_MUX_AUD_ENG1),
                clk_at(afe_priv, MT8189_CLK_TOP_CLK26M),
            );
            mt8189_afe_disable_clk(afe, clk_at(afe_priv, MT8189_CLK_TOP_MUX_AUD_ENG1));
            mt8189_afe_set_clk_parent(
                afe,
                clk_at(afe_priv, MT8189_CLK_TOP_MUX_AUD_1),
                clk_at(afe_priv, MT8189_CLK_TOP_CLK26M),
            );
            mt8189_afe_disable_clk(afe, clk_at(afe_priv, MT8189_CLK_TOP_MUX_AUD_1));
            mt8189_afe_set_clk_parent(
                afe,
                clk_at(afe_priv, MT8189_CLK_TOP_MUX_AUDIO_H),
                clk_at(afe_priv, MT8189_CLK_TOP_CLK26M),
            );
            mt8189_afe_disable_clk(afe, clk_at(afe_priv, MT8189_CLK_TOP_MUX_AUDIO_H));
        }
    }

    0
}

unsafe fn apll2_mux_setting(afe: *mut mtk_base_afe, enable: bool) -> c_int {
    let afe_priv = unsafe { (*afe).platform_priv as *mut mt8189_afe_private };
    let mut ret: c_int;

    unsafe { dev_dbg((*afe).dev, c"enable: %d\n".as_ptr(), enable as c_int) };

    if enable {
        ret = unsafe { mt8189_afe_enable_clk(afe, clk_at(afe_priv, MT8189_CLK_TOP_MUX_AUD_2)) };
        if ret != 0 {
            return ret;
        }

        ret = unsafe {
            mt8189_afe_set_clk_parent(
                afe,
                clk_at(afe_priv, MT8189_CLK_TOP_MUX_AUD_2),
                clk_at(afe_priv, MT8189_CLK_TOP_APLL2_CK),
            )
        };
        if ret != 0 {
            unsafe { mt8189_afe_disable_clk(afe, clk_at(afe_priv, MT8189_CLK_TOP_MUX_AUD_2)) };
            return ret;
        }

        /* 196.608 / 4 = 49.152MHz */
        ret = unsafe { mt8189_afe_enable_clk(afe, clk_at(afe_priv, MT8189_CLK_TOP_MUX_AUD_ENG2)) };
        if ret != 0 {
            unsafe {
                mt8189_afe_set_clk_parent(
                    afe,
                    clk_at(afe_priv, MT8189_CLK_TOP_MUX_AUD_2),
                    clk_at(afe_priv, MT8189_CLK_TOP_CLK26M),
                );
                mt8189_afe_disable_clk(afe, clk_at(afe_priv, MT8189_CLK_TOP_MUX_AUD_2));
            }
            return ret;
        }

        ret = unsafe {
            mt8189_afe_set_clk_parent(
                afe,
                clk_at(afe_priv, MT8189_CLK_TOP_MUX_AUD_ENG2),
                clk_at(afe_priv, MT8189_CLK_TOP_APLL2_D4),
            )
        };
        if ret != 0 {
            unsafe {
                mt8189_afe_disable_clk(afe, clk_at(afe_priv, MT8189_CLK_TOP_MUX_AUD_ENG2));
                mt8189_afe_set_clk_parent(
                    afe,
                    clk_at(afe_priv, MT8189_CLK_TOP_MUX_AUD_2),
                    clk_at(afe_priv, MT8189_CLK_TOP_CLK26M),
                );
                mt8189_afe_disable_clk(afe, clk_at(afe_priv, MT8189_CLK_TOP_MUX_AUD_2));
            }
            return ret;
        }

        ret = unsafe { mt8189_afe_enable_clk(afe, clk_at(afe_priv, MT8189_CLK_TOP_MUX_AUDIO_H)) };
        if ret != 0 {
            unsafe {
                mt8189_afe_set_clk_parent(
                    afe,
                    clk_at(afe_priv, MT8189_CLK_TOP_MUX_AUD_ENG2),
                    clk_at(afe_priv, MT8189_CLK_TOP_CLK26M),
                );
                mt8189_afe_disable_clk(afe, clk_at(afe_priv, MT8189_CLK_TOP_MUX_AUD_ENG2));
                mt8189_afe_set_clk_parent(
                    afe,
                    clk_at(afe_priv, MT8189_CLK_TOP_MUX_AUD_2),
                    clk_at(afe_priv, MT8189_CLK_TOP_CLK26M),
                );
                mt8189_afe_disable_clk(afe, clk_at(afe_priv, MT8189_CLK_TOP_MUX_AUD_2));
            }
            return ret;
        }

        ret = unsafe {
            mt8189_afe_set_clk_parent(
                afe,
                clk_at(afe_priv, MT8189_CLK_TOP_MUX_AUDIO_H),
                clk_at(afe_priv, MT8189_CLK_TOP_APLL2_CK),
            )
        };
        if ret != 0 {
            unsafe {
                mt8189_afe_disable_clk(afe, clk_at(afe_priv, MT8189_CLK_TOP_MUX_AUDIO_H));
                mt8189_afe_set_clk_parent(
                    afe,
                    clk_at(afe_priv, MT8189_CLK_TOP_MUX_AUD_ENG2),
                    clk_at(afe_priv, MT8189_CLK_TOP_CLK26M),
                );
                mt8189_afe_disable_clk(afe, clk_at(afe_priv, MT8189_CLK_TOP_MUX_AUD_ENG2));
                mt8189_afe_set_clk_parent(
                    afe,
                    clk_at(afe_priv, MT8189_CLK_TOP_MUX_AUD_2),
                    clk_at(afe_priv, MT8189_CLK_TOP_CLK26M),
                );
                mt8189_afe_disable_clk(afe, clk_at(afe_priv, MT8189_CLK_TOP_MUX_AUD_2));
            }
            return ret;
        }
    } else {
        unsafe {
            mt8189_afe_set_clk_parent(
                afe,
                clk_at(afe_priv, MT8189_CLK_TOP_MUX_AUD_ENG2),
                clk_at(afe_priv, MT8189_CLK_TOP_CLK26M),
            );
            mt8189_afe_disable_clk(afe, clk_at(afe_priv, MT8189_CLK_TOP_MUX_AUD_ENG2));
            mt8189_afe_set_clk_parent(
                afe,
                clk_at(afe_priv, MT8189_CLK_TOP_MUX_AUD_2),
                clk_at(afe_priv, MT8189_CLK_TOP_CLK26M),
            );
            mt8189_afe_disable_clk(afe, clk_at(afe_priv, MT8189_CLK_TOP_MUX_AUD_2));
            mt8189_afe_set_clk_parent(
                afe,
                clk_at(afe_priv, MT8189_CLK_TOP_MUX_AUDIO_H),
                clk_at(afe_priv, MT8189_CLK_TOP_CLK26M),
            );
            mt8189_afe_disable_clk(afe, clk_at(afe_priv, MT8189_CLK_TOP_MUX_AUDIO_H));
        }
    }

    0
}

unsafe fn mt8189_afe_disable_apll(afe: *mut mtk_base_afe) -> c_int {
    let afe_priv = unsafe { (*afe).platform_priv as *mut mt8189_afe_private };
    let mut ret: c_int;

    ret = unsafe { mt8189_afe_enable_clk(afe, clk_at(afe_priv, MT8189_CLK_TOP_MUX_AUDIO_H)) };
    if ret != 0 {
        return ret;
    }

    ret = unsafe { mt8189_afe_enable_clk(afe, clk_at(afe_priv, MT8189_CLK_TOP_MUX_AUD_1)) };
    if ret != 0 {
        unsafe { mt8189_afe_disable_clk(afe, clk_at(afe_priv, MT8189_CLK_TOP_MUX_AUDIO_H)) };
        return ret;
    }

    ret = unsafe {
        mt8189_afe_set_clk_parent(
            afe,
            clk_at(afe_priv, MT8189_CLK_TOP_MUX_AUD_1),
            clk_at(afe_priv, MT8189_CLK_TOP_CLK26M),
        )
    };
    if ret != 0 {
        unsafe {
            mt8189_afe_disable_clk(afe, clk_at(afe_priv, MT8189_CLK_TOP_MUX_AUD_1));
            mt8189_afe_disable_clk(afe, clk_at(afe_priv, MT8189_CLK_TOP_MUX_AUDIO_H));
        }
        return ret;
    }

    ret = unsafe { mt8189_afe_enable_clk(afe, clk_at(afe_priv, MT8189_CLK_TOP_MUX_AUD_2)) };
    if ret != 0 {
        unsafe {
            mt8189_afe_set_clk_parent(
                afe,
                clk_at(afe_priv, MT8189_CLK_TOP_MUX_AUD_1),
                clk_at(afe_priv, MT8189_CLK_TOP_APLL1_CK),
            );
            mt8189_afe_disable_clk(afe, clk_at(afe_priv, MT8189_CLK_TOP_MUX_AUD_1));
            mt8189_afe_disable_clk(afe, clk_at(afe_priv, MT8189_CLK_TOP_MUX_AUDIO_H));
        }
        return ret;
    }

    ret = unsafe {
        mt8189_afe_set_clk_parent(
            afe,
            clk_at(afe_priv, MT8189_CLK_TOP_MUX_AUD_2),
            clk_at(afe_priv, MT8189_CLK_TOP_CLK26M),
        )
    };
    if ret != 0 {
        unsafe {
            mt8189_afe_disable_clk(afe, clk_at(afe_priv, MT8189_CLK_TOP_MUX_AUD_2));
            mt8189_afe_set_clk_parent(
                afe,
                clk_at(afe_priv, MT8189_CLK_TOP_MUX_AUD_1),
                clk_at(afe_priv, MT8189_CLK_TOP_APLL1_CK),
            );
            mt8189_afe_disable_clk(afe, clk_at(afe_priv, MT8189_CLK_TOP_MUX_AUD_1));
            mt8189_afe_disable_clk(afe, clk_at(afe_priv, MT8189_CLK_TOP_MUX_AUDIO_H));
        }
        return ret;
    }

    unsafe {
        mt8189_afe_disable_clk(afe, clk_at(afe_priv, MT8189_CLK_TOP_MUX_AUD_1));
        mt8189_afe_disable_clk(afe, clk_at(afe_priv, MT8189_CLK_TOP_MUX_AUD_2));
        mt8189_afe_set_clk_parent(
            afe,
            clk_at(afe_priv, MT8189_CLK_TOP_MUX_AUDIO_H),
            clk_at(afe_priv, MT8189_CLK_TOP_CLK26M),
        );
        mt8189_afe_disable_clk(afe, clk_at(afe_priv, MT8189_CLK_TOP_MUX_AUDIO_H));
    }

    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn mt8189_apll1_enable(afe: *mut mtk_base_afe) -> c_int {
    let mut ret: c_int;

    /* setting for APLL */
    ret = unsafe { apll1_mux_setting(afe, true) };
    if ret != 0 {
        return ret;
    }

    ret = unsafe { mt8189_afe_enable_top_cg(afe, MT8189_CG_APLL1_CK) };
    if ret != 0 {
        return ret;
    }

    ret = unsafe { mt8189_afe_enable_top_cg(afe, MT8189_PDN_APLL_TUNER1) };
    if ret != 0 {
        return ret;
    }

    /* sel 44.1kHz:1, apll_div:7, upper bound:3 */
    unsafe {
        regmap_update_bits(
            (*afe).regmap,
            AFE_APLL1_TUNER_CFG,
            XTAL_EN_128FS_SEL_MASK_SFT | APLL_DIV_MASK_SFT | UPPER_BOUND_MASK_SFT,
            (0x1 << XTAL_EN_128FS_SEL_SFT) | (7 << APLL_DIV_SFT) | (3 << UPPER_BOUND_SFT),
        );

        /* apll1 freq tuner enable */
        regmap_update_bits(
            (*afe).regmap,
            AFE_APLL1_TUNER_CFG,
            FREQ_TUNER_EN_MASK_SFT,
            0x1 << FREQ_TUNER_EN_SFT,
        );
    }

    /* audio apll1 on */
    ret = unsafe { mt8189_afe_enable_top_cg(afe, MT8189_AUDIO_APLL1_EN_ON) };
    if ret != 0 {
        return ret;
    }

    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn mt8189_apll1_disable(afe: *mut mtk_base_afe) {
    /* audio apll1 off */
    unsafe { mt8189_afe_disable_top_cg(afe, MT8189_AUDIO_APLL1_EN_ON) };

    /* apll1 freq tuner disable */
    unsafe {
        regmap_update_bits((*afe).regmap, AFE_APLL1_TUNER_CFG, FREQ_TUNER_EN_MASK_SFT, 0x0);
        mt8189_afe_disable_top_cg(afe, MT8189_PDN_APLL_TUNER1);
        mt8189_afe_disable_top_cg(afe, MT8189_CG_APLL1_CK);
        apll1_mux_setting(afe, false);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn mt8189_apll2_enable(afe: *mut mtk_base_afe) -> c_int {
    let mut ret: c_int;

    /* setting for APLL */
    ret = unsafe { apll2_mux_setting(afe, true) };
    if ret != 0 {
        return ret;
    }

    ret = unsafe { mt8189_afe_enable_top_cg(afe, MT8189_CG_APLL2_CK) };
    if ret != 0 {
        return ret;
    }

    ret = unsafe { mt8189_afe_enable_top_cg(afe, MT8189_PDN_APLL_TUNER2) };
    if ret != 0 {
        return ret;
    }

    /* sel 48kHz: 2, apll_div: 7, upper bound: 3*/
    unsafe {
        regmap_update_bits(
            (*afe).regmap,
            AFE_APLL2_TUNER_CFG,
            XTAL_EN_128FS_SEL_MASK_SFT | APLL_DIV_MASK_SFT | UPPER_BOUND_MASK_SFT,
            (0x2 << XTAL_EN_128FS_SEL_SFT) | (7 << APLL_DIV_SFT) | (3 << UPPER_BOUND_SFT),
        );

        /* apll2 freq tuner enable */
        regmap_update_bits(
            (*afe).regmap,
            AFE_APLL2_TUNER_CFG,
            FREQ_TUNER_EN_MASK_SFT,
            0x1 << FREQ_TUNER_EN_SFT,
        );
    }

    /* audio apll2 on */
    ret = unsafe { mt8189_afe_enable_top_cg(afe, MT8189_AUDIO_APLL2_EN_ON) };
    if ret != 0 {
        return ret;
    }

    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn mt8189_apll2_disable(afe: *mut mtk_base_afe) {
    /* audio apll2 off */
    unsafe { mt8189_afe_disable_top_cg(afe, MT8189_AUDIO_APLL2_EN_ON) };

    /* apll2 freq tuner disable */
    unsafe {
        regmap_update_bits((*afe).regmap, AFE_APLL2_TUNER_CFG, FREQ_TUNER_EN_MASK_SFT, 0x0);
        mt8189_afe_disable_top_cg(afe, MT8189_PDN_APLL_TUNER2);
        mt8189_afe_disable_top_cg(afe, MT8189_CG_APLL2_CK);
        apll2_mux_setting(afe, false);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn mt8189_get_apll_rate(afe: *mut mtk_base_afe, apll: c_int) -> c_int {
    let afe_priv = unsafe { (*afe).platform_priv as *mut mt8189_afe_private };
    let clk_id: c_int;

    if apll < MT8189_APLL1 || apll > MT8189_APLL2 {
        unsafe { dev_warn((*afe).dev, c"invalid clk id %d\n".as_ptr(), apll) };
        return 0;
    }

    if apll == MT8189_APLL1 {
        clk_id = MT8189_CLK_TOP_APLL1_CK;
    } else {
        clk_id = MT8189_CLK_TOP_APLL2_CK;
    }

    unsafe { clk_get_rate(clk_at(afe_priv, clk_id)) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn mt8189_get_apll_by_rate(_afe: *mut mtk_base_afe, rate: c_int) -> c_int {
    if rate % 8000 != 0 {
        MT8189_APLL1
    } else {
        MT8189_APLL2
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn mt8189_get_apll_by_name(
    _afe: *mut mtk_base_afe,
    name: *const c_char,
) -> c_int {
    if unsafe { strcmp(name, APLL1_W_NAME) } == 0 {
        return MT8189_APLL1;
    }

    MT8189_APLL2
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn mt8189_mck_enable(
    afe: *mut mtk_base_afe,
    mck_id: c_int,
    rate: c_int,
) -> c_int {
    let afe_priv = unsafe { (*afe).platform_priv as *mut mt8189_afe_private };
    let apll: c_int = unsafe { mt8189_get_apll_by_rate(afe, rate) };
    let apll_clk_id: c_int = if apll == MT8189_APLL1 {
        MT8189_CLK_TOP_MUX_AUD_1
    } else {
        MT8189_CLK_TOP_MUX_AUD_2
    };
    let m_sel_id: c_int;
    let div_clk_id: c_int;
    let mut ret: c_int;

    unsafe { dev_dbg((*afe).dev, c"mck_id: %d, rate: %d\n".as_ptr(), mck_id, rate) };

    if mck_id >= MT8189_MCK_NUM || mck_id < 0 {
        return -EINVAL;
    }

    m_sel_id = mck_div[mck_id as usize].m_sel_id;
    div_clk_id = mck_div[mck_id as usize].div_clk_id;

    /* select apll */
    if m_sel_id >= 0 {
        ret = unsafe { mt8189_afe_enable_clk(afe, clk_at(afe_priv, m_sel_id)) };
        if ret != 0 {
            return ret;
        }

        ret = unsafe {
            mt8189_afe_set_clk_parent(
                afe,
                clk_at(afe_priv, m_sel_id),
                clk_at(afe_priv, apll_clk_id),
            )
        };
        if ret != 0 {
            return ret;
        }
    }

    /* enable div, set rate */
    if div_clk_id < 0 {
        unsafe { dev_err((*afe).dev, c"invalid div_clk_id %d\n".as_ptr(), div_clk_id) };
        return -EINVAL;
    }

    ret = unsafe { mt8189_afe_enable_clk(afe, clk_at(afe_priv, div_clk_id)) };
    if ret != 0 {
        return ret;
    }

    ret = unsafe { mt8189_afe_set_clk_rate(afe, clk_at(afe_priv, div_clk_id), rate as c_uint) };
    if ret != 0 {
        return ret;
    }

    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn mt8189_mck_disable(afe: *mut mtk_base_afe, mck_id: c_int) -> c_int {
    let afe_priv = unsafe { (*afe).platform_priv as *mut mt8189_afe_private };
    let m_sel_id: c_int;
    let div_clk_id: c_int;

    unsafe { dev_dbg((*afe).dev, c"mck_id: %d.\n".as_ptr(), mck_id) };

    if mck_id < 0 {
        unsafe { dev_err((*afe).dev, c"mck_id = %d < 0\n".as_ptr(), mck_id) };
        return -EINVAL;
    }

    m_sel_id = mck_div[mck_id as usize].m_sel_id;
    div_clk_id = mck_div[mck_id as usize].div_clk_id;

    if div_clk_id < 0 {
        unsafe { dev_err((*afe).dev, c"div_clk_id = %d < 0\n".as_ptr(), div_clk_id) };
        return -EINVAL;
    }

    unsafe { mt8189_afe_disable_clk(afe, clk_at(afe_priv, div_clk_id)) };

    if m_sel_id >= 0 {
        unsafe { mt8189_afe_disable_clk(afe, clk_at(afe_priv, m_sel_id)) };
    }

    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn mt8189_afe_enable_reg_rw_clk(afe: *mut mtk_base_afe) -> c_int {
    let afe_priv = unsafe { (*afe).platform_priv as *mut mt8189_afe_private };

    /* bus clock for AFE internal access, like AFE SRAM */
    unsafe {
        mt8189_afe_enable_clk(afe, clk_at(afe_priv, MT8189_CLK_TOP_MUX_AUDIOINTBUS));
        mt8189_afe_set_clk_parent(
            afe,
            clk_at(afe_priv, MT8189_CLK_TOP_MUX_AUDIOINTBUS),
            clk_at(afe_priv, MT8189_CLK_TOP_CLK26M),
        );
        /* enable audio clock source */
        mt8189_afe_enable_clk(afe, clk_at(afe_priv, MT8189_CLK_TOP_MUX_AUDIO_H));
        mt8189_afe_set_clk_parent(
            afe,
            clk_at(afe_priv, MT8189_CLK_TOP_MUX_AUDIO_H),
            clk_at(afe_priv, MT8189_CLK_TOP_CLK26M),
        );
    }

    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn mt8189_afe_disable_reg_rw_clk(afe: *mut mtk_base_afe) -> c_int {
    let afe_priv = unsafe { (*afe).platform_priv as *mut mt8189_afe_private };

    unsafe {
        mt8189_afe_disable_clk(afe, clk_at(afe_priv, MT8189_CLK_TOP_MUX_AUDIO_H));
        mt8189_afe_disable_clk(afe, clk_at(afe_priv, MT8189_CLK_TOP_MUX_AUDIOINTBUS));
    }

    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn mt8189_afe_enable_main_clock(afe: *mut mtk_base_afe) -> c_int {
    unsafe { mt8189_afe_enable_top_cg(afe, MT8189_AUDIO_26M_EN_ON) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn mt8189_afe_disable_main_clock(afe: *mut mtk_base_afe) {
    unsafe { mt8189_afe_disable_top_cg(afe, MT8189_AUDIO_26M_EN_ON) };
}

unsafe fn mt8189_afe_enable_ao_clock(afe: *mut mtk_base_afe) -> c_int {
    let afe_priv = unsafe { (*afe).platform_priv as *mut mt8189_afe_private };
    let mut ret: c_int;

    /* Peri clock AO enable */
    ret = unsafe { mt8189_afe_enable_clk(afe, clk_at(afe_priv, MT8189_CLK_PERAO_INTBUS_CK_PERI)) };
    if ret != 0 {
        return ret;
    }

    ret = unsafe {
        mt8189_afe_enable_clk(afe, clk_at(afe_priv, MT8189_CLK_PERAO_AUDIO_SLV_CK_PERI))
    };
    if ret != 0 {
        unsafe { mt8189_afe_disable_clk(afe, clk_at(afe_priv, MT8189_CLK_PERAO_INTBUS_CK_PERI)) };
        return ret;
    }

    ret = unsafe {
        mt8189_afe_enable_clk(afe, clk_at(afe_priv, MT8189_CLK_PERAO_AUDIO_MST_CK_PERI))
    };
    if ret != 0 {
        unsafe {
            mt8189_afe_disable_clk(afe, clk_at(afe_priv, MT8189_CLK_PERAO_AUDIO_SLV_CK_PERI));
            mt8189_afe_disable_clk(afe, clk_at(afe_priv, MT8189_CLK_PERAO_INTBUS_CK_PERI));
        }
        return ret;
    }

    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn mt8189_init_clock(afe: *mut mtk_base_afe) -> c_int {
    let afe_priv = unsafe { (*afe).platform_priv as *mut mt8189_afe_private };
    let mut ret: c_int;
    let mut i: c_int;

    unsafe {
        (*afe_priv).clk = devm_kcalloc(
            (*afe).dev,
            MT8189_CLK_NUM as usize,
            core::mem::size_of::<*mut clk>(),
            GFP_KERNEL,
        ) as *mut *mut clk;
    }
    if unsafe { (*afe_priv).clk.is_null() } {
        return -ENOMEM;
    }

    i = 0;
    while i < MT8189_CLK_NUM {
        unsafe {
            *(*afe_priv).clk.add(i as usize) = devm_clk_get((*afe).dev, aud_clks[i as usize]);
            if IS_ERR(*(*afe_priv).clk.add(i as usize) as *const c_void) {
                dev_err(
                    (*afe).dev,
                    c"devm_clk_get %s fail\n".as_ptr(),
                    aud_clks[i as usize],
                );
                return PTR_ERR(*(*afe_priv).clk.add(i as usize) as *const c_void);
            }
        }
        i += 1;
    }

    ret = unsafe { mt8189_afe_disable_apll(afe) };
    if ret != 0 {
        return ret;
    }

    ret = unsafe { mt8189_afe_enable_ao_clock(afe) };
    if ret != 0 {
        return ret;
    }

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
