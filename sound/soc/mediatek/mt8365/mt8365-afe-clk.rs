// SPDX-License-Identifier: GPL-2.0
/*
 * MediaTek 8365 AFE clock control
 *
 * Copyright (c) 2024 MediaTek Inc.
 * Authors: Jia Zeng <jia.zeng@mediatek.com>
 *          Alexandre Mergnat <amergnat@baylibre.com>
 */

/* Dependencies from:
 * "mt8365-afe-clk.h"
 * "mt8365-afe-common.h"
 * "mt8365-reg.h"
 * "../common/mtk-base-afe.h"
 * <linux/device.h>
 * <linux/mfd/syscon.h>
 */

use core::ffi::{c_char, c_int, c_uint, c_void};

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
pub struct mutex {
	_private: [u8; 0],
}

#[repr(C)]
pub struct mtk_base_afe {
	pub platform_priv: *mut mt8365_afe_private,
	pub dev: *mut device,
	pub regmap: *mut regmap,
}

#[repr(C)]
pub struct mt8365_afe_private {
	pub clocks: [*mut clk; MT8365_CLK_NUM],
	pub afe_ctrl_lock: spinlock_t,
	pub afe_clk_mutex: mutex,
	pub top_cg_ref_cnt: [c_int; MT8365_TOP_CG_NUM],
	pub afe_on_ref_cnt: c_int,
	pub apll_tuner_ref_cnt: [c_int; MT8365_AFE_APLL_NUM],
}

extern "C" {
	static MT8365_CLK_NUM: usize;
	static MT8365_TOP_CG_NUM: usize;
	static MT8365_AFE_APLL_NUM: usize;

	static MT8365_CLK_TOP_AUD_SEL: usize;
	static MT8365_CLK_AUD_I2S0_M: usize;
	static MT8365_CLK_AUD_I2S1_M: usize;
	static MT8365_CLK_AUD_I2S2_M: usize;
	static MT8365_CLK_AUD_I2S3_M: usize;
	static MT8365_CLK_ENGEN1: usize;
	static MT8365_CLK_ENGEN2: usize;
	static MT8365_CLK_AUD1: usize;
	static MT8365_CLK_AUD2: usize;
	static MT8365_CLK_I2S0_M_SEL: usize;
	static MT8365_CLK_I2S1_M_SEL: usize;
	static MT8365_CLK_I2S2_M_SEL: usize;
	static MT8365_CLK_I2S3_M_SEL: usize;
	static MT8365_CLK_CLK26M: usize;

	static MT8365_TOP_CG_AFE: c_uint;
	static MT8365_TOP_CG_I2S_IN: c_uint;
	static MT8365_TOP_CG_22M: c_uint;
	static MT8365_TOP_CG_24M: c_uint;
	static MT8365_TOP_CG_INTDIR_CK: c_uint;
	static MT8365_TOP_CG_APLL2_TUNER: c_uint;
	static MT8365_TOP_CG_APLL_TUNER: c_uint;
	static MT8365_TOP_CG_SPDIF: c_uint;
	static MT8365_TOP_CG_TDM_OUT: c_uint;
	static MT8365_TOP_CG_TDM_IN: c_uint;
	static MT8365_TOP_CG_ADC: c_uint;
	static MT8365_TOP_CG_DAC: c_uint;
	static MT8365_TOP_CG_DAC_PREDIS: c_uint;
	static MT8365_TOP_CG_TML: c_uint;
	static MT8365_TOP_CG_I2S1_BCLK: c_uint;
	static MT8365_TOP_CG_I2S2_BCLK: c_uint;
	static MT8365_TOP_CG_I2S3_BCLK: c_uint;
	static MT8365_TOP_CG_I2S4_BCLK: c_uint;
	static MT8365_TOP_CG_DMIC0_ADC: c_uint;
	static MT8365_TOP_CG_DMIC1_ADC: c_uint;
	static MT8365_TOP_CG_DMIC2_ADC: c_uint;
	static MT8365_TOP_CG_DMIC3_ADC: c_uint;
	static MT8365_TOP_CG_CONNSYS_I2S_ASRC: c_uint;
	static MT8365_TOP_CG_GENERAL1_ASRC: c_uint;
	static MT8365_TOP_CG_GENERAL2_ASRC: c_uint;
	static MT8365_TOP_CG_TDM_ASRC: c_uint;

	static MT8365_AFE_APLL1: c_uint;
	static MT8365_AFE_APLL2: c_uint;

	static AUDIO_TOP_CON0: c_uint;
	static AUDIO_TOP_CON1: c_uint;
	static AUD_TCON0_PDN_AFE: c_uint;
	static AUD_TCON0_PDN_I2S_IN: c_uint;
	static AUD_TCON0_PDN_22M: c_uint;
	static AUD_TCON0_PDN_24M: c_uint;
	static AUD_TCON0_PDN_INTDIR: c_uint;
	static AUD_TCON0_PDN_APLL2_TUNER: c_uint;
	static AUD_TCON0_PDN_APLL_TUNER: c_uint;
	static AUD_TCON0_PDN_SPDIF: c_uint;
	static AUD_TCON0_PDN_TDM_OUT: c_uint;
	static AUD_TCON0_PDN_TDM_IN: c_uint;
	static AUD_TCON0_PDN_ADC: c_uint;
	static AUD_TCON0_PDN_DAC: c_uint;
	static AUD_TCON0_PDN_DAC_PREDIS: c_uint;
	static AUD_TCON0_PDN_TML: c_uint;
	static AUD_TCON1_PDN_I2S1_BCLK: c_uint;
	static AUD_TCON1_PDN_I2S2_BCLK: c_uint;
	static AUD_TCON1_PDN_I2S3_BCLK: c_uint;
	static AUD_TCON1_PDN_I2S4_BCLK: c_uint;
	static AUD_TCON1_PDN_DMIC0_ADC: c_uint;
	static AUD_TCON1_PDN_DMIC1_ADC: c_uint;
	static AUD_TCON1_PDN_DMIC2_ADC: c_uint;
	static AUD_TCON1_PDN_DMIC3_ADC: c_uint;
	static AUD_TCON1_PDN_CONNSYS_I2S_ASRC: c_uint;
	static AUD_TCON1_PDN_GENERAL1_ASRC: c_uint;
	static AUD_TCON1_PDN_GENERAL2_ASRC: c_uint;
	static AUD_TCON1_PDN_TDM_ASRC: c_uint;
	static AFE_DAC_CON0: c_uint;
	static AFE_HD_ENGEN_ENABLE: c_uint;
	static AFE_22M_PLL_EN: c_uint;
	static AFE_24M_PLL_EN: c_uint;
	static AFE_APLL_TUNER_CFG: c_uint;
	static AFE_APLL_TUNER_CFG_MASK: c_uint;
	static AFE_APLL_TUNER_CFG_EN_MASK: c_uint;
	static AFE_APLL_TUNER_CFG1: c_uint;
	static AFE_APLL_TUNER_CFG1_MASK: c_uint;
	static AFE_APLL_TUNER_CFG1_EN_MASK: c_uint;

	fn devm_clk_get(dev: *mut device, id: *const c_char) -> *mut clk;
	fn IS_ERR(ptr: *const c_void) -> bool;
	fn PTR_ERR(ptr: *const c_void) -> c_int;
	fn dev_err(dev: *mut device, fmt: *const c_char, ...) -> c_int;
	fn dev_info(dev: *mut device, fmt: *const c_char, ...) -> c_int;
	fn clk_disable_unprepare(clk: *mut clk);
	fn clk_set_rate(clk: *mut clk, rate: c_uint) -> c_int;
	fn clk_set_parent(clk: *mut clk, parent: *mut clk) -> c_int;
	fn regmap_update_bits(map: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
	fn clk_prepare_enable(clk: *mut clk) -> c_int;
}

static mut aud_clks: [*const c_char; MT8365_CLK_NUM] = {
	let mut clks = [core::ptr::null(); MT8365_CLK_NUM];
	clks[MT8365_CLK_TOP_AUD_SEL] = b"top_audio_sel\0".as_ptr() as *const c_char;
	clks[MT8365_CLK_AUD_I2S0_M] = b"audio_i2s0_m\0".as_ptr() as *const c_char;
	clks[MT8365_CLK_AUD_I2S1_M] = b"audio_i2s1_m\0".as_ptr() as *const c_char;
	clks[MT8365_CLK_AUD_I2S2_M] = b"audio_i2s2_m\0".as_ptr() as *const c_char;
	clks[MT8365_CLK_AUD_I2S3_M] = b"audio_i2s3_m\0".as_ptr() as *const c_char;
	clks[MT8365_CLK_ENGEN1] = b"engen1\0".as_ptr() as *const c_char;
	clks[MT8365_CLK_ENGEN2] = b"engen2\0".as_ptr() as *const c_char;
	clks[MT8365_CLK_AUD1] = b"aud1\0".as_ptr() as *const c_char;
	clks[MT8365_CLK_AUD2] = b"aud2\0".as_ptr() as *const c_char;
	clks[MT8365_CLK_I2S0_M_SEL] = b"i2s0_m_sel\0".as_ptr() as *const c_char;
	clks[MT8365_CLK_I2S1_M_SEL] = b"i2s1_m_sel\0".as_ptr() as *const c_char;
	clks[MT8365_CLK_I2S2_M_SEL] = b"i2s2_m_sel\0".as_ptr() as *const c_char;
	clks[MT8365_CLK_I2S3_M_SEL] = b"i2s3_m_sel\0".as_ptr() as *const c_char;
	clks[MT8365_CLK_CLK26M] = b"top_clk26m_clk\0".as_ptr() as *const c_char;
	clks
};

#[no_mangle]
pub unsafe extern "C" fn mt8365_afe_init_audio_clk(afe: *mut mtk_base_afe) -> c_int {
	let mut i: usize;
	let afe_priv = (*afe).platform_priv;

	i = 0;
	while i < aud_clks.len() {
		(*afe_priv).clocks[i] = devm_clk_get((*afe).dev, aud_clks[i]);
		if IS_ERR((*afe_priv).clocks[i] as *const c_void) {
			dev_err(
				(*afe).dev,
				b"%s devm_clk_get %s fail\n\0".as_ptr() as *const c_char,
				b"mt8365_afe_init_audio_clk\0".as_ptr() as *const c_char,
				aud_clks[i],
			);
			return PTR_ERR((*afe_priv).clocks[i] as *const c_void);
		}
		i += 1;
	}
	0
}

#[no_mangle]
pub unsafe extern "C" fn mt8365_afe_disable_clk(_afe: *mut mtk_base_afe, clk: *mut clk) {
	clk_disable_unprepare(clk);
}

#[no_mangle]
pub unsafe extern "C" fn mt8365_afe_set_clk_rate(
	afe: *mut mtk_base_afe,
	clk: *mut clk,
	rate: c_uint,
) -> c_int {
	let ret: c_int;

	if !clk.is_null() {
		ret = clk_set_rate(clk, rate);
		if ret != 0 {
			dev_err((*afe).dev, b"Failed to set rate\n\0".as_ptr() as *const c_char);
			return ret;
		}
	}
	0
}

#[no_mangle]
pub unsafe extern "C" fn mt8365_afe_set_clk_parent(
	afe: *mut mtk_base_afe,
	clk: *mut clk,
	parent: *mut clk,
) -> c_int {
	let ret: c_int;

	if !clk.is_null() && !parent.is_null() {
		ret = clk_set_parent(clk, parent);
		if ret != 0 {
			dev_err((*afe).dev, b"Failed to set parent\n\0".as_ptr() as *const c_char);
			return ret;
		}
	}
	0
}

unsafe fn get_top_cg_reg(cg_type: c_uint) -> c_uint {
	if cg_type == MT8365_TOP_CG_AFE
		|| cg_type == MT8365_TOP_CG_I2S_IN
		|| cg_type == MT8365_TOP_CG_22M
		|| cg_type == MT8365_TOP_CG_24M
		|| cg_type == MT8365_TOP_CG_INTDIR_CK
		|| cg_type == MT8365_TOP_CG_APLL2_TUNER
		|| cg_type == MT8365_TOP_CG_APLL_TUNER
		|| cg_type == MT8365_TOP_CG_SPDIF
		|| cg_type == MT8365_TOP_CG_TDM_OUT
		|| cg_type == MT8365_TOP_CG_TDM_IN
		|| cg_type == MT8365_TOP_CG_ADC
		|| cg_type == MT8365_TOP_CG_DAC
		|| cg_type == MT8365_TOP_CG_DAC_PREDIS
		|| cg_type == MT8365_TOP_CG_TML
	{
		AUDIO_TOP_CON0
	} else if cg_type == MT8365_TOP_CG_I2S1_BCLK
		|| cg_type == MT8365_TOP_CG_I2S2_BCLK
		|| cg_type == MT8365_TOP_CG_I2S3_BCLK
		|| cg_type == MT8365_TOP_CG_I2S4_BCLK
		|| cg_type == MT8365_TOP_CG_DMIC0_ADC
		|| cg_type == MT8365_TOP_CG_DMIC1_ADC
		|| cg_type == MT8365_TOP_CG_DMIC2_ADC
		|| cg_type == MT8365_TOP_CG_DMIC3_ADC
		|| cg_type == MT8365_TOP_CG_CONNSYS_I2S_ASRC
		|| cg_type == MT8365_TOP_CG_GENERAL1_ASRC
		|| cg_type == MT8365_TOP_CG_GENERAL2_ASRC
		|| cg_type == MT8365_TOP_CG_TDM_ASRC
	{
		AUDIO_TOP_CON1
	} else {
		0
	}
}

unsafe fn get_top_cg_mask(cg_type: c_uint) -> c_uint {
	if cg_type == MT8365_TOP_CG_AFE {
		AUD_TCON0_PDN_AFE
	} else if cg_type == MT8365_TOP_CG_I2S_IN {
		AUD_TCON0_PDN_I2S_IN
	} else if cg_type == MT8365_TOP_CG_22M {
		AUD_TCON0_PDN_22M
	} else if cg_type == MT8365_TOP_CG_24M {
		AUD_TCON0_PDN_24M
	} else if cg_type == MT8365_TOP_CG_INTDIR_CK {
		AUD_TCON0_PDN_INTDIR
	} else if cg_type == MT8365_TOP_CG_APLL2_TUNER {
		AUD_TCON0_PDN_APLL2_TUNER
	} else if cg_type == MT8365_TOP_CG_APLL_TUNER {
		AUD_TCON0_PDN_APLL_TUNER
	} else if cg_type == MT8365_TOP_CG_SPDIF {
		AUD_TCON0_PDN_SPDIF
	} else if cg_type == MT8365_TOP_CG_TDM_OUT {
		AUD_TCON0_PDN_TDM_OUT
	} else if cg_type == MT8365_TOP_CG_TDM_IN {
		AUD_TCON0_PDN_TDM_IN
	} else if cg_type == MT8365_TOP_CG_ADC {
		AUD_TCON0_PDN_ADC
	} else if cg_type == MT8365_TOP_CG_DAC {
		AUD_TCON0_PDN_DAC
	} else if cg_type == MT8365_TOP_CG_DAC_PREDIS {
		AUD_TCON0_PDN_DAC_PREDIS
	} else if cg_type == MT8365_TOP_CG_TML {
		AUD_TCON0_PDN_TML
	} else if cg_type == MT8365_TOP_CG_I2S1_BCLK {
		AUD_TCON1_PDN_I2S1_BCLK
	} else if cg_type == MT8365_TOP_CG_I2S2_BCLK {
		AUD_TCON1_PDN_I2S2_BCLK
	} else if cg_type == MT8365_TOP_CG_I2S3_BCLK {
		AUD_TCON1_PDN_I2S3_BCLK
	} else if cg_type == MT8365_TOP_CG_I2S4_BCLK {
		AUD_TCON1_PDN_I2S4_BCLK
	} else if cg_type == MT8365_TOP_CG_DMIC0_ADC {
		AUD_TCON1_PDN_DMIC0_ADC
	} else if cg_type == MT8365_TOP_CG_DMIC1_ADC {
		AUD_TCON1_PDN_DMIC1_ADC
	} else if cg_type == MT8365_TOP_CG_DMIC2_ADC {
		AUD_TCON1_PDN_DMIC2_ADC
	} else if cg_type == MT8365_TOP_CG_DMIC3_ADC {
		AUD_TCON1_PDN_DMIC3_ADC
	} else if cg_type == MT8365_TOP_CG_CONNSYS_I2S_ASRC {
		AUD_TCON1_PDN_CONNSYS_I2S_ASRC
	} else if cg_type == MT8365_TOP_CG_GENERAL1_ASRC {
		AUD_TCON1_PDN_GENERAL1_ASRC
	} else if cg_type == MT8365_TOP_CG_GENERAL2_ASRC {
		AUD_TCON1_PDN_GENERAL2_ASRC
	} else if cg_type == MT8365_TOP_CG_TDM_ASRC {
		AUD_TCON1_PDN_TDM_ASRC
	} else {
		0
	}
}

unsafe fn get_top_cg_on_val(_cg_type: c_uint) -> c_uint {
	0
}

unsafe fn get_top_cg_off_val(cg_type: c_uint) -> c_uint {
	get_top_cg_mask(cg_type)
}

#[no_mangle]
pub unsafe extern "C" fn mt8365_afe_enable_top_cg(
	afe: *mut mtk_base_afe,
	cg_type: c_uint,
) -> c_int {
	let afe_priv = (*afe).platform_priv;
	let reg = get_top_cg_reg(cg_type);
	let mask = get_top_cg_mask(cg_type);
	let val = get_top_cg_on_val(cg_type);

	/* C source uses guard(spinlock_irqsave)(&afe_priv->afe_ctrl_lock). */
	(*afe_priv).top_cg_ref_cnt[cg_type as usize] += 1;
	if (*afe_priv).top_cg_ref_cnt[cg_type as usize] == 1 {
		regmap_update_bits((*afe).regmap, reg, mask, val);
	}

	0
}

#[no_mangle]
pub unsafe extern "C" fn mt8365_afe_disable_top_cg(
	afe: *mut mtk_base_afe,
	cg_type: c_uint,
) -> c_int {
	let afe_priv = (*afe).platform_priv;
	let reg = get_top_cg_reg(cg_type);
	let mask = get_top_cg_mask(cg_type);
	let val = get_top_cg_off_val(cg_type);

	/* C source uses guard(spinlock_irqsave)(&afe_priv->afe_ctrl_lock). */
	(*afe_priv).top_cg_ref_cnt[cg_type as usize] -= 1;
	if (*afe_priv).top_cg_ref_cnt[cg_type as usize] == 0 {
		regmap_update_bits((*afe).regmap, reg, mask, val);
	} else if (*afe_priv).top_cg_ref_cnt[cg_type as usize] < 0 {
		(*afe_priv).top_cg_ref_cnt[cg_type as usize] = 0;
	}

	0
}

#[no_mangle]
pub unsafe extern "C" fn mt8365_afe_enable_main_clk(afe: *mut mtk_base_afe) -> c_int {
	let afe_priv = (*afe).platform_priv;

	clk_prepare_enable((*afe_priv).clocks[MT8365_CLK_TOP_AUD_SEL]);
	mt8365_afe_enable_top_cg(afe, MT8365_TOP_CG_AFE);
	mt8365_afe_enable_afe_on(afe);

	0
}

#[no_mangle]
pub unsafe extern "C" fn mt8365_afe_disable_main_clk(afe: *mut mtk_base_afe) -> c_int {
	let afe_priv = (*afe).platform_priv;

	mt8365_afe_disable_afe_on(afe);
	mt8365_afe_disable_top_cg(afe, MT8365_TOP_CG_AFE);
	mt8365_afe_disable_clk(afe, (*afe_priv).clocks[MT8365_CLK_TOP_AUD_SEL]);

	0
}

#[no_mangle]
pub unsafe extern "C" fn mt8365_afe_emi_clk_on(_afe: *mut mtk_base_afe) -> c_int {
	0
}

#[no_mangle]
pub unsafe extern "C" fn mt8365_afe_emi_clk_off(_afe: *mut mtk_base_afe) -> c_int {
	0
}

#[no_mangle]
pub unsafe extern "C" fn mt8365_afe_enable_afe_on(afe: *mut mtk_base_afe) -> c_int {
	let afe_priv = (*afe).platform_priv;

	/* C source uses guard(spinlock_irqsave)(&afe_priv->afe_ctrl_lock). */
	(*afe_priv).afe_on_ref_cnt += 1;
	if (*afe_priv).afe_on_ref_cnt == 1 {
		regmap_update_bits((*afe).regmap, AFE_DAC_CON0, 0x1, 0x1);
	}

	0
}

#[no_mangle]
pub unsafe extern "C" fn mt8365_afe_disable_afe_on(afe: *mut mtk_base_afe) -> c_int {
	let afe_priv = (*afe).platform_priv;

	/* C source uses guard(spinlock_irqsave)(&afe_priv->afe_ctrl_lock). */
	(*afe_priv).afe_on_ref_cnt -= 1;
	if (*afe_priv).afe_on_ref_cnt == 0 {
		regmap_update_bits((*afe).regmap, AFE_DAC_CON0, 0x1, 0x0);
	} else if (*afe_priv).afe_on_ref_cnt < 0 {
		(*afe_priv).afe_on_ref_cnt = 0;
	}

	0
}

unsafe fn mt8365_afe_hd_engen_enable(afe: *mut mtk_base_afe, apll1: bool) -> c_int {
	if apll1 {
		regmap_update_bits(
			(*afe).regmap,
			AFE_HD_ENGEN_ENABLE,
			AFE_22M_PLL_EN,
			AFE_22M_PLL_EN,
		);
	} else {
		regmap_update_bits(
			(*afe).regmap,
			AFE_HD_ENGEN_ENABLE,
			AFE_24M_PLL_EN,
			AFE_24M_PLL_EN,
		);
	}

	0
}

unsafe fn mt8365_afe_hd_engen_disable(afe: *mut mtk_base_afe, apll1: bool) -> c_int {
	if apll1 {
		regmap_update_bits(
			(*afe).regmap,
			AFE_HD_ENGEN_ENABLE,
			AFE_22M_PLL_EN,
			!AFE_22M_PLL_EN,
		);
	} else {
		regmap_update_bits(
			(*afe).regmap,
			AFE_HD_ENGEN_ENABLE,
			AFE_24M_PLL_EN,
			!AFE_24M_PLL_EN,
		);
	}

	0
}

#[no_mangle]
pub unsafe extern "C" fn mt8365_afe_enable_apll_tuner_cfg(
	afe: *mut mtk_base_afe,
	apll: c_uint,
) -> c_int {
	let afe_priv = (*afe).platform_priv;

	/* C source uses guard(mutex)(&afe_priv->afe_clk_mutex). */
	(*afe_priv).apll_tuner_ref_cnt[apll as usize] += 1;
	if (*afe_priv).apll_tuner_ref_cnt[apll as usize] != 1 {
		return 0;
	}

	if apll == MT8365_AFE_APLL1 {
		regmap_update_bits(
			(*afe).regmap,
			AFE_APLL_TUNER_CFG,
			AFE_APLL_TUNER_CFG_MASK,
			0x432,
		);
		regmap_update_bits(
			(*afe).regmap,
			AFE_APLL_TUNER_CFG,
			AFE_APLL_TUNER_CFG_EN_MASK,
			0x1,
		);
	} else {
		regmap_update_bits(
			(*afe).regmap,
			AFE_APLL_TUNER_CFG1,
			AFE_APLL_TUNER_CFG1_MASK,
			0x434,
		);
		regmap_update_bits(
			(*afe).regmap,
			AFE_APLL_TUNER_CFG1,
			AFE_APLL_TUNER_CFG1_EN_MASK,
			0x1,
		);
	}

	0
}

#[no_mangle]
pub unsafe extern "C" fn mt8365_afe_disable_apll_tuner_cfg(
	afe: *mut mtk_base_afe,
	apll: c_uint,
) -> c_int {
	let afe_priv = (*afe).platform_priv;

	/* C source uses guard(mutex)(&afe_priv->afe_clk_mutex). */
	(*afe_priv).apll_tuner_ref_cnt[apll as usize] -= 1;
	if (*afe_priv).apll_tuner_ref_cnt[apll as usize] == 0 {
		if apll == MT8365_AFE_APLL1 {
			regmap_update_bits(
				(*afe).regmap,
				AFE_APLL_TUNER_CFG,
				AFE_APLL_TUNER_CFG_EN_MASK,
				0x0,
			);
		} else {
			regmap_update_bits(
				(*afe).regmap,
				AFE_APLL_TUNER_CFG1,
				AFE_APLL_TUNER_CFG1_EN_MASK,
				0x0,
			);
		}
	} else if (*afe_priv).apll_tuner_ref_cnt[apll as usize] < 0 {
		(*afe_priv).apll_tuner_ref_cnt[apll as usize] = 0;
	}

	0
}

#[no_mangle]
pub unsafe extern "C" fn mt8365_afe_enable_apll_associated_cfg(
	afe: *mut mtk_base_afe,
	apll: c_uint,
) -> c_int {
	let afe_priv = (*afe).platform_priv;

	if apll == MT8365_AFE_APLL1 {
		if clk_prepare_enable((*afe_priv).clocks[MT8365_CLK_ENGEN1]) != 0 {
			dev_info(
				(*afe).dev,
				b"%s Failed to enable ENGEN1 clk\n\0".as_ptr() as *const c_char,
				b"mt8365_afe_enable_apll_associated_cfg\0".as_ptr() as *const c_char,
			);
			return 0;
		}
		mt8365_afe_enable_top_cg(afe, MT8365_TOP_CG_22M);
		mt8365_afe_hd_engen_enable(afe, true);
		mt8365_afe_enable_top_cg(afe, MT8365_TOP_CG_APLL_TUNER);
		mt8365_afe_enable_apll_tuner_cfg(afe, MT8365_AFE_APLL1);
	} else {
		if clk_prepare_enable((*afe_priv).clocks[MT8365_CLK_ENGEN2]) != 0 {
			dev_info(
				(*afe).dev,
				b"%s Failed to enable ENGEN2 clk\n\0".as_ptr() as *const c_char,
				b"mt8365_afe_enable_apll_associated_cfg\0".as_ptr() as *const c_char,
			);
			return 0;
		}
		mt8365_afe_enable_top_cg(afe, MT8365_TOP_CG_24M);
		mt8365_afe_hd_engen_enable(afe, false);
		mt8365_afe_enable_top_cg(afe, MT8365_TOP_CG_APLL2_TUNER);
		mt8365_afe_enable_apll_tuner_cfg(afe, MT8365_AFE_APLL2);
	}

	0
}

#[no_mangle]
pub unsafe extern "C" fn mt8365_afe_disable_apll_associated_cfg(
	afe: *mut mtk_base_afe,
	apll: c_uint,
) -> c_int {
	let afe_priv = (*afe).platform_priv;

	if apll == MT8365_AFE_APLL1 {
		mt8365_afe_disable_apll_tuner_cfg(afe, MT8365_AFE_APLL1);
		mt8365_afe_disable_top_cg(afe, MT8365_TOP_CG_APLL_TUNER);
		mt8365_afe_hd_engen_disable(afe, true);
		mt8365_afe_disable_top_cg(afe, MT8365_TOP_CG_22M);
		clk_disable_unprepare((*afe_priv).clocks[MT8365_CLK_ENGEN1]);
	} else {
		mt8365_afe_disable_apll_tuner_cfg(afe, MT8365_AFE_APLL2);
		mt8365_afe_disable_top_cg(afe, MT8365_TOP_CG_APLL2_TUNER);
		mt8365_afe_hd_engen_disable(afe, false);
		mt8365_afe_disable_top_cg(afe, MT8365_TOP_CG_24M);
		clk_disable_unprepare((*afe_priv).clocks[MT8365_CLK_ENGEN2]);
	}

	0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
