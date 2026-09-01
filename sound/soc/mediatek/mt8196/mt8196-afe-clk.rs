// SPDX-License-Identifier: GPL-2.0
/*
 *  mt8196-afe-clk.c  --  Mediatek 8196 afe clock ctrl
 *
 *  Copyright (c) 2025 MediaTek Inc.
 *  Author: Darren Ye <darren.ye@mediatek.com>
 */

use core::ffi::{c_char, c_int, c_uint, c_void};

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
pub struct mt8196_afe_private {
	pub clk: *mut *mut clk,
}

unsafe extern "C" {
	static MT8196_CLK_NUM: usize;
	static MT8196_CLK_VLP_MUX_AUDIOINTBUS: usize;
	static MT8196_CLK_VLP_MUX_AUD_ENG1: usize;
	static MT8196_CLK_VLP_MUX_AUD_ENG2: usize;
	static MT8196_CLK_VLP_MUX_AUDIO_H: usize;
	static MT8196_CLK_TOP_APLL1_CK: usize;
	static MT8196_CLK_TOP_APLL2_CK: usize;
	static MT8196_CLK_TOP_APLL12_DIV_I2SIN0: usize;
	static MT8196_CLK_TOP_APLL12_DIV_I2SIN1: usize;
	static MT8196_CLK_TOP_APLL12_DIV_FMI2S: usize;
	static MT8196_CLK_TOP_APLL12_DIV_TDMOUT_M: usize;
	static MT8196_CLK_TOP_APLL12_DIV_TDMOUT_B: usize;
	static MT8196_CLK_TOP_ADSP_SEL: usize;

	static MT8196_AUDIO_26M_EN_ON: c_uint;
	static MT8196_AUDIO_F3P25M_EN_ON: c_uint;
	static MT8196_AUDIO_APLL1_EN_ON: c_uint;
	static MT8196_AUDIO_APLL2_EN_ON: c_uint;
	static MT8196_CG_AUDIO_HOPPING_CK: c_uint;
	static MT8196_CG_AUDIO_F26M_CK: c_uint;
	static MT8196_CG_APLL1_CK: c_uint;
	static MT8196_CG_APLL2_CK: c_uint;
	static MT8196_PDN_APLL_TUNER2: c_uint;
	static MT8196_PDN_APLL_TUNER1: c_uint;

	static AUDIO_ENGEN_CON0: c_uint;
	static AUDIO_TOP_CON4: c_uint;
	static AUDIO_26M_EN_ON_MASK_SFT: c_uint;
	static AUDIO_F3P25M_EN_ON_MASK_SFT: c_uint;
	static AUDIO_APLL1_EN_ON_MASK_SFT: c_uint;
	static AUDIO_APLL2_EN_ON_MASK_SFT: c_uint;
	static CG_AUDIO_HOPPING_CK_MASK_SFT: c_uint;
	static CG_AUDIO_F26M_CK_MASK_SFT: c_uint;
	static CG_APLL1_CK_MASK_SFT: c_uint;
	static CG_APLL2_CK_MASK_SFT: c_uint;
	static PDN_APLL_TUNER2_MASK_SFT: c_uint;
	static PDN_APLL_TUNER1_MASK_SFT: c_uint;

	static MT8196_APLL1: c_int;
	static MT8196_APLL2: c_int;
	static MT8196_AUD_ENG1_CLK: c_uint;
	static MT8196_AUD_ENG2_CLK: c_uint;
	static MT8196_AFE_26M: c_uint;
	static AFE_APLL1_TUNER_CFG: c_uint;
	static AFE_APLL2_TUNER_CFG: c_uint;
	static XTAL_EN_128FS_SEL_MASK_SFT: c_uint;
	static APLL_DIV_MASK_SFT: c_uint;
	static UPPER_BOUND_MASK_SFT: c_uint;
	static XTAL_EN_128FS_SEL_SFT: c_uint;
	static APLL_DIV_SFT: c_uint;
	static UPPER_BOUND_SFT: c_uint;
	static FREQ_TUNER_EN_MASK_SFT: c_uint;
	static FREQ_TUNER_EN_SFT: c_uint;

	static APLL1_W_NAME: *const c_char;
	static MT8196_MCK_NUM: usize;
	static MT8196_I2SIN0_MCK: usize;
	static MT8196_I2SIN1_MCK: usize;
	static MT8196_FMI2S_MCK: usize;
	static MT8196_TDMOUT_MCK: usize;
	static MT8196_TDMOUT_BCK: usize;

	static EINVAL: c_int;
	static ENOMEM: c_int;
	static GFP_KERNEL: c_uint;

	fn clk_prepare_enable(clk: *mut clk) -> c_int;
	fn clk_disable_unprepare(clk: *mut clk);
	fn clk_set_rate(clk: *mut clk, rate: c_uint) -> c_int;
	fn clk_get_rate(clk: *mut clk) -> c_int;
	fn regmap_update_bits(map: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
	fn devm_kcalloc(dev: *mut device, n: usize, size: usize, flags: c_uint) -> *mut c_void;
	fn devm_clk_get(dev: *mut device, id: *const c_char) -> *mut clk;
	fn IS_ERR(ptr: *const c_void) -> bool;
	fn PTR_ERR(ptr: *const c_void) -> c_int;
	fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;

	fn dev_err(dev: *mut device, fmt: *const c_char, ...);
	fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
	fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
}

static AUD_CLK_TOP_AUD_INTBUS: &[u8] = b"top_aud_intbus\0";
static AUD_CLK_TOP_AUD_ENG1: &[u8] = b"top_aud_eng1\0";
static AUD_CLK_TOP_AUD_ENG2: &[u8] = b"top_aud_eng2\0";
static AUD_CLK_TOP_AUD_H: &[u8] = b"top_aud_h\0";
static AUD_CLK_APLL1: &[u8] = b"apll1\0";
static AUD_CLK_APLL2: &[u8] = b"apll2\0";
static AUD_CLK_APLL12_DIV_I2SIN0: &[u8] = b"apll12_div_i2sin0\0";
static AUD_CLK_APLL12_DIV_I2SIN1: &[u8] = b"apll12_div_i2sin1\0";
static AUD_CLK_APLL12_DIV_FMI2S: &[u8] = b"apll12_div_fmi2s\0";
static AUD_CLK_APLL12_DIV_TDMOUT_M: &[u8] = b"apll12_div_tdmout_m\0";
static AUD_CLK_APLL12_DIV_TDMOUT_B: &[u8] = b"apll12_div_tdmout_b\0";
static AUD_CLK_TOP_ADSP: &[u8] = b"top_adsp\0";

static mut aud_clks: [*const c_char; 12] = [
	AUD_CLK_TOP_AUD_INTBUS.as_ptr() as *const c_char,
	AUD_CLK_TOP_AUD_ENG1.as_ptr() as *const c_char,
	AUD_CLK_TOP_AUD_ENG2.as_ptr() as *const c_char,
	AUD_CLK_TOP_AUD_H.as_ptr() as *const c_char,
	AUD_CLK_APLL1.as_ptr() as *const c_char,
	AUD_CLK_APLL2.as_ptr() as *const c_char,
	AUD_CLK_APLL12_DIV_I2SIN0.as_ptr() as *const c_char,
	AUD_CLK_APLL12_DIV_I2SIN1.as_ptr() as *const c_char,
	AUD_CLK_APLL12_DIV_FMI2S.as_ptr() as *const c_char,
	AUD_CLK_APLL12_DIV_TDMOUT_M.as_ptr() as *const c_char,
	AUD_CLK_APLL12_DIV_TDMOUT_B.as_ptr() as *const c_char,
	AUD_CLK_TOP_ADSP.as_ptr() as *const c_char,
];

#[no_mangle]
pub unsafe extern "C" fn mt8196_afe_enable_clk(afe: *mut mtk_base_afe, clk: *mut clk) -> c_int {
	let ret: c_int;

	ret = clk_prepare_enable(clk);
	if ret != 0 {
		dev_err((*afe).dev, c"failed to enable clk\n".as_ptr());
		return ret;
	}

	0
}

#[no_mangle]
pub unsafe extern "C" fn mt8196_afe_disable_clk(afe: *mut mtk_base_afe, clk: *mut clk) {
	if !clk.is_null() {
		clk_disable_unprepare(clk);
	} else {
		dev_err((*afe).dev, c"NULL clk\n".as_ptr());
	}
}

unsafe fn mt8196_afe_set_clk_rate(
	afe: *mut mtk_base_afe,
	clk: *mut clk,
	rate: c_uint,
) -> c_int {
	let ret: c_int;

	if !clk.is_null() {
		ret = clk_set_rate(clk, rate);
		if ret != 0 {
			dev_err((*afe).dev, c"failed to set clk rate\n".as_ptr());
			return ret;
		}
	}

	0
}

unsafe fn get_top_cg_reg(cg_type: c_uint) -> c_uint {
	if cg_type == MT8196_AUDIO_26M_EN_ON
		|| cg_type == MT8196_AUDIO_F3P25M_EN_ON
		|| cg_type == MT8196_AUDIO_APLL1_EN_ON
		|| cg_type == MT8196_AUDIO_APLL2_EN_ON
	{
		AUDIO_ENGEN_CON0
	} else if cg_type == MT8196_CG_AUDIO_HOPPING_CK
		|| cg_type == MT8196_CG_AUDIO_F26M_CK
		|| cg_type == MT8196_CG_APLL1_CK
		|| cg_type == MT8196_CG_APLL2_CK
		|| cg_type == MT8196_PDN_APLL_TUNER2
		|| cg_type == MT8196_PDN_APLL_TUNER1
	{
		AUDIO_TOP_CON4
	} else {
		0
	}
}

unsafe fn get_top_cg_mask(cg_type: c_uint) -> c_uint {
	if cg_type == MT8196_AUDIO_26M_EN_ON {
		AUDIO_26M_EN_ON_MASK_SFT
	} else if cg_type == MT8196_AUDIO_F3P25M_EN_ON {
		AUDIO_F3P25M_EN_ON_MASK_SFT
	} else if cg_type == MT8196_AUDIO_APLL1_EN_ON {
		AUDIO_APLL1_EN_ON_MASK_SFT
	} else if cg_type == MT8196_AUDIO_APLL2_EN_ON {
		AUDIO_APLL2_EN_ON_MASK_SFT
	} else if cg_type == MT8196_CG_AUDIO_HOPPING_CK {
		CG_AUDIO_HOPPING_CK_MASK_SFT
	} else if cg_type == MT8196_CG_AUDIO_F26M_CK {
		CG_AUDIO_F26M_CK_MASK_SFT
	} else if cg_type == MT8196_CG_APLL1_CK {
		CG_APLL1_CK_MASK_SFT
	} else if cg_type == MT8196_CG_APLL2_CK {
		CG_APLL2_CK_MASK_SFT
	} else if cg_type == MT8196_PDN_APLL_TUNER2 {
		PDN_APLL_TUNER2_MASK_SFT
	} else if cg_type == MT8196_PDN_APLL_TUNER1 {
		PDN_APLL_TUNER1_MASK_SFT
	} else {
		0
	}
}

unsafe fn get_top_cg_on_val(cg_type: c_uint) -> c_uint {
	if cg_type == MT8196_AUDIO_26M_EN_ON
		|| cg_type == MT8196_AUDIO_F3P25M_EN_ON
		|| cg_type == MT8196_AUDIO_APLL1_EN_ON
		|| cg_type == MT8196_AUDIO_APLL2_EN_ON
	{
		get_top_cg_mask(cg_type)
	} else if cg_type == MT8196_CG_AUDIO_HOPPING_CK
		|| cg_type == MT8196_CG_AUDIO_F26M_CK
		|| cg_type == MT8196_CG_APLL1_CK
		|| cg_type == MT8196_CG_APLL2_CK
		|| cg_type == MT8196_PDN_APLL_TUNER2
		|| cg_type == MT8196_PDN_APLL_TUNER1
	{
		0
	} else {
		0
	}
}

unsafe fn get_top_cg_off_val(cg_type: c_uint) -> c_uint {
	if cg_type == MT8196_AUDIO_26M_EN_ON
		|| cg_type == MT8196_AUDIO_F3P25M_EN_ON
		|| cg_type == MT8196_AUDIO_APLL1_EN_ON
		|| cg_type == MT8196_AUDIO_APLL2_EN_ON
	{
		0
	} else if cg_type == MT8196_CG_AUDIO_HOPPING_CK
		|| cg_type == MT8196_CG_AUDIO_F26M_CK
		|| cg_type == MT8196_CG_APLL1_CK
		|| cg_type == MT8196_CG_APLL2_CK
		|| cg_type == MT8196_PDN_APLL_TUNER2
		|| cg_type == MT8196_PDN_APLL_TUNER1
	{
		get_top_cg_mask(cg_type)
	} else {
		get_top_cg_mask(cg_type)
	}
}

unsafe fn mt8196_afe_enable_top_cg(afe: *mut mtk_base_afe, cg_type: c_uint) -> c_int {
	let ret: c_int;
	let reg: c_uint = get_top_cg_reg(cg_type);
	let mask: c_uint = get_top_cg_mask(cg_type);
	let val: c_uint = get_top_cg_on_val(cg_type);

	if (*afe).regmap.is_null() {
		dev_err((*afe).dev, c"afe regmap is null !!!\n".as_ptr());
		return 0;
	}

	dev_dbg(
		(*afe).dev,
		c"reg: 0x%x, mask: 0x%x, val: 0x%x\n".as_ptr(),
		reg,
		mask,
		val,
	);

	ret = regmap_update_bits((*afe).regmap, reg, mask, val);
	if ret != 0 {
		dev_err((*afe).dev, c"regmap_update_bits failed: %d\n".as_ptr(), ret);
	}

	ret
}

unsafe fn mt8196_afe_disable_top_cg(afe: *mut mtk_base_afe, cg_type: c_uint) -> c_int {
	let ret: c_int;
	let reg: c_uint = get_top_cg_reg(cg_type);
	let mask: c_uint = get_top_cg_mask(cg_type);
	let val: c_uint = get_top_cg_off_val(cg_type);

	if (*afe).regmap.is_null() {
		dev_err((*afe).dev, c"afe regmap is null !!!\n".as_ptr());
		return 0;
	}

	dev_dbg(
		(*afe).dev,
		c"reg: 0x%x, mask: 0x%x, val: 0x%x\n".as_ptr(),
		reg,
		mask,
		val,
	);

	ret = regmap_update_bits((*afe).regmap, reg, mask, val);
	if ret != 0 {
		dev_err((*afe).dev, c"regmap_update_bits failed: %d\n".as_ptr(), ret);
	}

	ret
}

unsafe fn clk_at(afe_priv: *mut mt8196_afe_private, id: usize) -> *mut clk {
	*(*afe_priv).clk.add(id)
}

unsafe fn apll1_mux_setting(afe: *mut mtk_base_afe, enable: bool) -> c_int {
	let afe_priv: *mut mt8196_afe_private = (*afe).platform_priv as *mut mt8196_afe_private;
	let apll_rate: c_int;
	let mut ret: c_int;

	dev_dbg((*afe).dev, c"enable: %d\n".as_ptr(), enable as c_int);

	if enable {
		apll_rate = mt8196_get_apll_rate(afe, MT8196_APLL1);

		/* 180.6336 / 4 = 45.1584MHz */
		ret = mt8196_afe_enable_clk(afe, clk_at(afe_priv, MT8196_CLK_VLP_MUX_AUD_ENG1));
		if ret != 0 {
			return ret;
		}

		ret = mt8196_afe_set_clk_rate(
			afe,
			clk_at(afe_priv, MT8196_CLK_VLP_MUX_AUD_ENG1),
			MT8196_AUD_ENG1_CLK,
		);
		if ret != 0 {
			return ret;
		}

		ret = mt8196_afe_enable_clk(afe, clk_at(afe_priv, MT8196_CLK_VLP_MUX_AUDIO_H));
		if ret != 0 {
			return ret;
		}

		ret = mt8196_afe_set_clk_rate(
			afe,
			clk_at(afe_priv, MT8196_CLK_VLP_MUX_AUDIO_H),
			apll_rate as c_uint,
		);
		if ret != 0 {
			return ret;
		}
	} else {
		ret = mt8196_afe_set_clk_rate(
			afe,
			clk_at(afe_priv, MT8196_CLK_VLP_MUX_AUD_ENG1),
			MT8196_AFE_26M,
		);
		if ret != 0 {
			return ret;
		}

		mt8196_afe_disable_clk(afe, clk_at(afe_priv, MT8196_CLK_VLP_MUX_AUD_ENG1));

		ret = mt8196_afe_set_clk_rate(
			afe,
			clk_at(afe_priv, MT8196_CLK_VLP_MUX_AUDIO_H),
			MT8196_AFE_26M,
		);
		if ret != 0 {
			return ret;
		}

		mt8196_afe_disable_clk(afe, clk_at(afe_priv, MT8196_CLK_VLP_MUX_AUDIO_H));
	}

	0
}

unsafe fn apll2_mux_setting(afe: *mut mtk_base_afe, enable: bool) -> c_int {
	let afe_priv: *mut mt8196_afe_private = (*afe).platform_priv as *mut mt8196_afe_private;
	let apll_rate: c_int;
	let mut ret: c_int;

	dev_dbg((*afe).dev, c"enable: %d\n".as_ptr(), enable as c_int);

	if enable {
		apll_rate = mt8196_get_apll_rate(afe, MT8196_APLL2);

		/* 196.608 / 4 = 49.152MHz */
		ret = mt8196_afe_enable_clk(afe, clk_at(afe_priv, MT8196_CLK_VLP_MUX_AUD_ENG2));
		if ret != 0 {
			return ret;
		}

		ret = mt8196_afe_set_clk_rate(
			afe,
			clk_at(afe_priv, MT8196_CLK_VLP_MUX_AUD_ENG2),
			MT8196_AUD_ENG2_CLK,
		);
		if ret != 0 {
			return ret;
		}

		ret = mt8196_afe_enable_clk(afe, clk_at(afe_priv, MT8196_CLK_VLP_MUX_AUDIO_H));
		if ret != 0 {
			return ret;
		}

		ret = mt8196_afe_set_clk_rate(
			afe,
			clk_at(afe_priv, MT8196_CLK_VLP_MUX_AUDIO_H),
			apll_rate as c_uint,
		);
		if ret != 0 {
			return ret;
		}
	} else {
		ret = mt8196_afe_set_clk_rate(
			afe,
			clk_at(afe_priv, MT8196_CLK_VLP_MUX_AUD_ENG2),
			MT8196_AFE_26M,
		);
		if ret != 0 {
			return ret;
		}

		mt8196_afe_disable_clk(afe, clk_at(afe_priv, MT8196_CLK_VLP_MUX_AUD_ENG2));

		ret = mt8196_afe_set_clk_rate(
			afe,
			clk_at(afe_priv, MT8196_CLK_VLP_MUX_AUDIO_H),
			MT8196_AFE_26M,
		);
		if ret != 0 {
			return ret;
		}

		mt8196_afe_disable_clk(afe, clk_at(afe_priv, MT8196_CLK_VLP_MUX_AUDIO_H));
	}

	0
}

#[no_mangle]
pub unsafe extern "C" fn mt8196_apll1_enable(afe: *mut mtk_base_afe) -> c_int {
	let mut ret: c_int;

	/* setting for APLL */
	apll1_mux_setting(afe, true);

	ret = mt8196_afe_enable_top_cg(afe, MT8196_CG_APLL1_CK);
	if ret != 0 {
		mt8196_afe_disable_top_cg(afe, MT8196_CG_APLL1_CK);
		return ret;
	}

	ret = mt8196_afe_enable_top_cg(afe, MT8196_PDN_APLL_TUNER1);
	if ret != 0 {
		mt8196_afe_disable_top_cg(afe, MT8196_PDN_APLL_TUNER1);
		mt8196_afe_disable_top_cg(afe, MT8196_CG_APLL1_CK);
		return ret;
	}

	/* sel 44.1kHz:1, apll_div:7, upper bound:3 */
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

	/* audio apll1 on */
	mt8196_afe_enable_top_cg(afe, MT8196_AUDIO_APLL1_EN_ON);

	0
}

#[no_mangle]
pub unsafe extern "C" fn mt8196_apll1_disable(afe: *mut mtk_base_afe) {
	/* audio apll1 off */
	mt8196_afe_disable_top_cg(afe, MT8196_AUDIO_APLL1_EN_ON);

	/* apll1 freq tuner disable */
	regmap_update_bits((*afe).regmap, AFE_APLL1_TUNER_CFG, FREQ_TUNER_EN_MASK_SFT, 0x0);

	mt8196_afe_disable_top_cg(afe, MT8196_PDN_APLL_TUNER1);
	mt8196_afe_disable_top_cg(afe, MT8196_CG_APLL1_CK);
	apll1_mux_setting(afe, false);
}

#[no_mangle]
pub unsafe extern "C" fn mt8196_apll2_enable(afe: *mut mtk_base_afe) -> c_int {
	let mut ret: c_int;

	/* setting for APLL */
	apll2_mux_setting(afe, true);

	ret = mt8196_afe_enable_top_cg(afe, MT8196_CG_APLL2_CK);
	if ret != 0 {
		mt8196_afe_disable_top_cg(afe, MT8196_CG_APLL2_CK);
		return 0;
	}

	ret = mt8196_afe_enable_top_cg(afe, MT8196_PDN_APLL_TUNER2);
	if ret != 0 {
		mt8196_afe_disable_top_cg(afe, MT8196_PDN_APLL_TUNER2);
		mt8196_afe_disable_top_cg(afe, MT8196_CG_APLL2_CK);
		return 0;
	}

	/* sel 48kHz: 2, apll_div: 7, upper bound: 3*/
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

	/* audio apll2 on */
	mt8196_afe_enable_top_cg(afe, MT8196_AUDIO_APLL2_EN_ON);
	0
}

#[no_mangle]
pub unsafe extern "C" fn mt8196_apll2_disable(afe: *mut mtk_base_afe) {
	/* audio apll2 off */
	mt8196_afe_disable_top_cg(afe, MT8196_AUDIO_APLL2_EN_ON);

	/* apll2 freq tuner disable */
	regmap_update_bits((*afe).regmap, AFE_APLL2_TUNER_CFG, FREQ_TUNER_EN_MASK_SFT, 0x0);

	mt8196_afe_disable_top_cg(afe, MT8196_PDN_APLL_TUNER2);
	mt8196_afe_disable_top_cg(afe, MT8196_CG_APLL2_CK);
	apll2_mux_setting(afe, false);
}

#[no_mangle]
pub unsafe extern "C" fn mt8196_get_apll_rate(afe: *mut mtk_base_afe, apll: c_int) -> c_int {
	let afe_priv: *mut mt8196_afe_private = (*afe).platform_priv as *mut mt8196_afe_private;
	let clk_id: c_int;

	if apll < MT8196_APLL1 || apll > MT8196_APLL2 {
		dev_warn((*afe).dev, c"invalid clk id %d\n".as_ptr(), apll);
		return 0;
	}

	if apll == MT8196_APLL1 {
		clk_id = MT8196_CLK_TOP_APLL1_CK as c_int;
	} else {
		clk_id = MT8196_CLK_TOP_APLL2_CK as c_int;
	}

	clk_get_rate(clk_at(afe_priv, clk_id as usize))
}

/* 48K: select APLL2; 44.1k: select APLL1 */
#[no_mangle]
pub unsafe extern "C" fn mt8196_get_apll_by_rate(_afe: *mut mtk_base_afe, rate: c_int) -> c_int {
	if rate % 8000 != 0 {
		MT8196_APLL1
	} else {
		MT8196_APLL2
	}
}

#[no_mangle]
pub unsafe extern "C" fn mt8196_get_apll_by_name(
	_afe: *mut mtk_base_afe,
	name: *const c_char,
) -> c_int {
	if strcmp(name, APLL1_W_NAME) == 0 {
		return MT8196_APLL1;
	}

	MT8196_APLL2
}

static mut mck_div: [c_int; 5] = [
	0,
	0,
	0,
	0,
	0,
];

unsafe fn init_mck_div() {
	mck_div[MT8196_I2SIN0_MCK] = MT8196_CLK_TOP_APLL12_DIV_I2SIN0 as c_int;
	mck_div[MT8196_I2SIN1_MCK] = MT8196_CLK_TOP_APLL12_DIV_I2SIN1 as c_int;
	mck_div[MT8196_FMI2S_MCK] = MT8196_CLK_TOP_APLL12_DIV_FMI2S as c_int;
	mck_div[MT8196_TDMOUT_MCK] = MT8196_CLK_TOP_APLL12_DIV_TDMOUT_M as c_int;
	mck_div[MT8196_TDMOUT_BCK] = MT8196_CLK_TOP_APLL12_DIV_TDMOUT_B as c_int;
}

#[no_mangle]
pub unsafe extern "C" fn mt8196_mck_enable(
	afe: *mut mtk_base_afe,
	mck_id: c_int,
	mut rate: c_int,
) -> c_int {
	let afe_priv: *mut mt8196_afe_private = (*afe).platform_priv as *mut mt8196_afe_private;
	let div_clk_id: c_int;
	let mut ret: c_int;

	dev_dbg((*afe).dev, c"mck_id: %d, rate: %d\n".as_ptr(), mck_id, rate);

	if mck_id >= MT8196_MCK_NUM as c_int || mck_id < 0 {
		return -EINVAL;
	}

	init_mck_div();
	div_clk_id = mck_div[mck_id as usize];

	/* enable div, set rate */
	if div_clk_id < 0 {
		dev_err((*afe).dev, c"invalid div_clk_id %d\n".as_ptr(), div_clk_id);
		return -EINVAL;
	}

	if div_clk_id == MT8196_CLK_TOP_APLL12_DIV_TDMOUT_B as c_int {
		rate = rate.wrapping_mul(16);
	}

	ret = mt8196_afe_enable_clk(afe, clk_at(afe_priv, div_clk_id as usize));
	if ret != 0 {
		return ret;
	}

	ret = mt8196_afe_set_clk_rate(afe, clk_at(afe_priv, div_clk_id as usize), rate as c_uint);
	if ret != 0 {
		return ret;
	}

	0
}

#[no_mangle]
pub unsafe extern "C" fn mt8196_mck_disable(afe: *mut mtk_base_afe, mck_id: c_int) -> c_int {
	let afe_priv: *mut mt8196_afe_private = (*afe).platform_priv as *mut mt8196_afe_private;
	let div_clk_id: c_int;
	let ret: c_int;

	dev_dbg((*afe).dev, c"mck_id: %d.\n".as_ptr(), mck_id);

	if mck_id < 0 {
		dev_err((*afe).dev, c"mck_id = %d < 0\n".as_ptr(), mck_id);
		return -EINVAL;
	}

	init_mck_div();
	div_clk_id = mck_div[mck_id as usize];

	if div_clk_id < 0 {
		dev_err((*afe).dev, c"div_clk_id = %d < 0\n".as_ptr(), div_clk_id);
		return -EINVAL;
	}

	ret = mt8196_afe_set_clk_rate(
		afe,
		clk_at(afe_priv, div_clk_id as usize),
		MT8196_AFE_26M,
	);
	if ret != 0 {
		return ret;
	}

	mt8196_afe_disable_clk(afe, clk_at(afe_priv, div_clk_id as usize));

	0
}

#[no_mangle]
pub unsafe extern "C" fn mt8196_afe_enable_reg_rw_clk(afe: *mut mtk_base_afe) -> c_int {
	let afe_priv: *mut mt8196_afe_private = (*afe).platform_priv as *mut mt8196_afe_private;
	let mut ret: c_int;

	/* bus clock for AFE external access, like DRAM */
	mt8196_afe_enable_clk(afe, clk_at(afe_priv, MT8196_CLK_TOP_ADSP_SEL));

	/* bus clock for AFE internal access, like AFE SRAM */
	mt8196_afe_enable_clk(afe, clk_at(afe_priv, MT8196_CLK_VLP_MUX_AUDIOINTBUS));
	ret = mt8196_afe_set_clk_rate(
		afe,
		clk_at(afe_priv, MT8196_CLK_VLP_MUX_AUDIOINTBUS),
		MT8196_AFE_26M,
	);
	if ret != 0 {
		return ret;
	}

	/* enable audio h clock */
	mt8196_afe_enable_clk(afe, clk_at(afe_priv, MT8196_CLK_VLP_MUX_AUDIO_H));
	ret = mt8196_afe_set_clk_rate(
		afe,
		clk_at(afe_priv, MT8196_CLK_VLP_MUX_AUDIO_H),
		MT8196_AFE_26M,
	);
	if ret != 0 {
		return ret;
	}

	/* AFE hw clock */
	/* IPM2.0: USE HOPPING & 26M */
	/* set in the regmap_register_patch */
	0
}

#[no_mangle]
pub unsafe extern "C" fn mt8196_afe_disable_reg_rw_clk(afe: *mut mtk_base_afe) -> c_int {
	let afe_priv: *mut mt8196_afe_private = (*afe).platform_priv as *mut mt8196_afe_private;

	/* IPM2.0: Use HOPPING & 26M */
	/* set in the regmap_register_patch */

	mt8196_afe_disable_clk(afe, clk_at(afe_priv, MT8196_CLK_VLP_MUX_AUDIO_H));
	mt8196_afe_disable_clk(afe, clk_at(afe_priv, MT8196_CLK_VLP_MUX_AUDIOINTBUS));
	mt8196_afe_disable_clk(afe, clk_at(afe_priv, MT8196_CLK_TOP_ADSP_SEL));
	0
}

#[no_mangle]
pub unsafe extern "C" fn mt8196_afe_enable_main_clock(afe: *mut mtk_base_afe) -> c_int {
	mt8196_afe_enable_top_cg(afe, MT8196_AUDIO_26M_EN_ON);
	0
}

#[no_mangle]
pub unsafe extern "C" fn mt8196_afe_disable_main_clock(afe: *mut mtk_base_afe) -> c_int {
	mt8196_afe_disable_top_cg(afe, MT8196_AUDIO_26M_EN_ON);
	0
}

#[no_mangle]
pub unsafe extern "C" fn mt8196_init_clock(afe: *mut mtk_base_afe) -> c_int {
	let afe_priv: *mut mt8196_afe_private = (*afe).platform_priv as *mut mt8196_afe_private;
	let mut i: c_int;

	(*afe_priv).clk = devm_kcalloc(
		(*afe).dev,
		MT8196_CLK_NUM,
		core::mem::size_of::<*mut clk>(),
		GFP_KERNEL,
	) as *mut *mut clk;
	if (*afe_priv).clk.is_null() {
		return -ENOMEM;
	}

	i = 0;
	while i < MT8196_CLK_NUM as c_int {
		*(*afe_priv).clk.add(i as usize) =
			devm_clk_get((*afe).dev, aud_clks[i as usize]);
		if IS_ERR(*(*afe_priv).clk.add(i as usize) as *const c_void) {
			dev_err(
				(*afe).dev,
				c"devm_clk_get %s fail\n".as_ptr(),
				aud_clks[i as usize],
			);
			return PTR_ERR(*(*afe_priv).clk.add(i as usize) as *const c_void);
		}
		i += 1;
	}

	0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
