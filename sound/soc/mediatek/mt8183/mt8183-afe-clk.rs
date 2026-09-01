// SPDX-License-Identifier: GPL-2.0
//
// mt8183-afe-clk.c  --  Mediatek 8183 afe clock ctrl
//
// Copyright (c) 2018 MediaTek Inc.
// Author: KaiChieh Chuang <kaichieh.chuang@mediatek.com>

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};

// Dependencies from:
// <linux/clk.h>
// "mt8183-afe-common.h"
// "mt8183-afe-clk.h"
// "mt8183-reg.h"

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
pub struct mtk_base_afe {
	pub platform_priv: *mut c_void,
	pub dev: *mut device,
	pub regmap: *mut regmap,
}

#[repr(C)]
pub struct mt8183_afe_private {
	pub clk: *mut *mut clk,
}

unsafe extern "C" {
	fn devm_kcalloc(dev: *mut device, n: usize, size: usize, flags: c_uint) -> *mut c_void;
	fn devm_clk_get(dev: *mut device, id: *const c_char) -> *mut clk;
	fn IS_ERR(ptr: *const c_void) -> bool;
	fn PTR_ERR(ptr: *const c_void) -> c_long;
	fn dev_err(dev: *mut device, fmt: *const c_char, ...);
	fn clk_prepare_enable(clk: *mut clk) -> c_int;
	fn clk_disable_unprepare(clk: *mut clk);
	fn clk_set_parent(clk: *mut clk, parent: *mut clk) -> c_int;
	fn clk_set_rate(clk: *mut clk, rate: c_int) -> c_int;
	fn regmap_update_bits(map: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
	fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
}

const CLK_AFE: usize = 0;
const CLK_TML: usize = 1;
const CLK_APLL22M: usize = 2;
const CLK_APLL24M: usize = 3;
const CLK_APLL1_TUNER: usize = 4;
const CLK_APLL2_TUNER: usize = 5;
const CLK_I2S1_BCLK_SW: usize = 6;
const CLK_I2S2_BCLK_SW: usize = 7;
const CLK_I2S3_BCLK_SW: usize = 8;
const CLK_I2S4_BCLK_SW: usize = 9;
const CLK_INFRA_SYS_AUDIO: usize = 10;
const CLK_MUX_AUDIO: usize = 11;
const CLK_MUX_AUDIOINTBUS: usize = 12;
const CLK_TOP_SYSPLL_D2_D4: usize = 13;
/* apll related mux */
const CLK_TOP_MUX_AUD_1: usize = 14;
const CLK_TOP_APLL1_CK: usize = 15;
const CLK_TOP_MUX_AUD_2: usize = 16;
const CLK_TOP_APLL2_CK: usize = 17;
const CLK_TOP_MUX_AUD_ENG1: usize = 18;
const CLK_TOP_APLL1_D8: usize = 19;
const CLK_TOP_MUX_AUD_ENG2: usize = 20;
const CLK_TOP_APLL2_D8: usize = 21;
const CLK_TOP_I2S0_M_SEL: usize = 22;
const CLK_TOP_I2S1_M_SEL: usize = 23;
const CLK_TOP_I2S2_M_SEL: usize = 24;
const CLK_TOP_I2S3_M_SEL: usize = 25;
const CLK_TOP_I2S4_M_SEL: usize = 26;
const CLK_TOP_I2S5_M_SEL: usize = 27;
const CLK_TOP_APLL12_DIV0: usize = 28;
const CLK_TOP_APLL12_DIV1: usize = 29;
const CLK_TOP_APLL12_DIV2: usize = 30;
const CLK_TOP_APLL12_DIV3: usize = 31;
const CLK_TOP_APLL12_DIV4: usize = 32;
const CLK_TOP_APLL12_DIVB: usize = 33;
const CLK_CLK26M: usize = 34;
const CLK_NUM: usize = 35;

static AUD_CLKS: [&[u8]; CLK_NUM] = [
	b"aud_afe_clk\0",
	b"aud_tml_clk\0",
	b"aud_apll22m_clk\0",
	b"aud_apll24m_clk\0",
	b"aud_apll1_tuner_clk\0",
	b"aud_apll2_tuner_clk\0",
	b"aud_i2s1_bclk_sw\0",
	b"aud_i2s2_bclk_sw\0",
	b"aud_i2s3_bclk_sw\0",
	b"aud_i2s4_bclk_sw\0",
	b"aud_infra_clk\0",
	b"top_mux_audio\0",
	b"top_mux_aud_intbus\0",
	b"top_syspll_d2_d4\0",
	b"top_mux_aud_1\0",
	b"top_apll1_ck\0",
	b"top_mux_aud_2\0",
	b"top_apll2_ck\0",
	b"top_mux_aud_eng1\0",
	b"top_apll1_d8\0",
	b"top_mux_aud_eng2\0",
	b"top_apll2_d8\0",
	b"top_i2s0_m_sel\0",
	b"top_i2s1_m_sel\0",
	b"top_i2s2_m_sel\0",
	b"top_i2s3_m_sel\0",
	b"top_i2s4_m_sel\0",
	b"top_i2s5_m_sel\0",
	b"top_apll12_div0\0",
	b"top_apll12_div1\0",
	b"top_apll12_div2\0",
	b"top_apll12_div3\0",
	b"top_apll12_div4\0",
	b"top_apll12_divb\0",
	b"top_clk26m_clk\0",
];

unsafe fn aud_clk(id: usize) -> *const c_char {
	AUD_CLKS[id].as_ptr() as *const c_char
}

unsafe fn afe_priv<'a>(afe: *mut mtk_base_afe) -> &'a mut mt8183_afe_private {
	&mut *((*afe).platform_priv as *mut mt8183_afe_private)
}

unsafe fn priv_clk(afe_priv: *mut mt8183_afe_private, id: usize) -> *mut clk {
	*(*afe_priv).clk.add(id)
}

#[no_mangle]
pub unsafe extern "C" fn mt8183_init_clock(afe: *mut mtk_base_afe) -> c_int {
	let afe_priv = afe_priv(afe) as *mut mt8183_afe_private;

	(*afe_priv).clk = devm_kcalloc(
		(*afe).dev,
		CLK_NUM,
		core::mem::size_of::<*mut clk>(),
		GFP_KERNEL,
	) as *mut *mut clk;
	if (*afe_priv).clk.is_null() {
		return -ENOMEM;
	}

	for i in 0..CLK_NUM {
		*(*afe_priv).clk.add(i) = devm_clk_get((*afe).dev, aud_clk(i));
		if IS_ERR(*(*afe_priv).clk.add(i) as *const c_void) {
			dev_err(
				(*afe).dev,
				b"%s(), devm_clk_get %s fail, ret %ld\n\0".as_ptr() as *const c_char,
				b"mt8183_init_clock\0".as_ptr() as *const c_char,
				aud_clk(i),
				PTR_ERR(*(*afe_priv).clk.add(i) as *const c_void),
			);
			return PTR_ERR(*(*afe_priv).clk.add(i) as *const c_void) as c_int;
		}
	}

	0
}

#[no_mangle]
pub unsafe extern "C" fn mt8183_afe_enable_clock(afe: *mut mtk_base_afe) -> c_int {
	let afe_priv = afe_priv(afe) as *mut mt8183_afe_private;
	let mut ret: c_int;

	ret = clk_prepare_enable(priv_clk(afe_priv, CLK_INFRA_SYS_AUDIO));
	if ret != 0 {
		dev_err((*afe).dev, b"%s(), clk_prepare_enable %s fail %d\n\0".as_ptr() as *const c_char, b"mt8183_afe_enable_clock\0".as_ptr() as *const c_char, aud_clk(CLK_INFRA_SYS_AUDIO), ret);
		return ret;
	}

	ret = clk_prepare_enable(priv_clk(afe_priv, CLK_MUX_AUDIO));
	if ret != 0 {
		dev_err((*afe).dev, b"%s(), clk_prepare_enable %s fail %d\n\0".as_ptr() as *const c_char, b"mt8183_afe_enable_clock\0".as_ptr() as *const c_char, aud_clk(CLK_MUX_AUDIO), ret);
		clk_disable_unprepare(priv_clk(afe_priv, CLK_INFRA_SYS_AUDIO));
		return ret;
	}

	ret = clk_set_parent(priv_clk(afe_priv, CLK_MUX_AUDIO), priv_clk(afe_priv, CLK_CLK26M));
	if ret != 0 {
		dev_err((*afe).dev, b"%s(), clk_set_parent %s-%s fail %d\n\0".as_ptr() as *const c_char, b"mt8183_afe_enable_clock\0".as_ptr() as *const c_char, aud_clk(CLK_MUX_AUDIO), aud_clk(CLK_CLK26M), ret);
		clk_disable_unprepare(priv_clk(afe_priv, CLK_INFRA_SYS_AUDIO));
		return ret;
	}

	ret = clk_prepare_enable(priv_clk(afe_priv, CLK_MUX_AUDIOINTBUS));
	if ret != 0 {
		dev_err((*afe).dev, b"%s(), clk_prepare_enable %s fail %d\n\0".as_ptr() as *const c_char, b"mt8183_afe_enable_clock\0".as_ptr() as *const c_char, aud_clk(CLK_MUX_AUDIOINTBUS), ret);
		clk_disable_unprepare(priv_clk(afe_priv, CLK_MUX_AUDIO));
		clk_disable_unprepare(priv_clk(afe_priv, CLK_INFRA_SYS_AUDIO));
		return ret;
	}

	ret = clk_set_parent(priv_clk(afe_priv, CLK_MUX_AUDIOINTBUS), priv_clk(afe_priv, CLK_TOP_SYSPLL_D2_D4));
	if ret != 0 {
		dev_err((*afe).dev, b"%s(), clk_set_parent %s-%s fail %d\n\0".as_ptr() as *const c_char, b"mt8183_afe_enable_clock\0".as_ptr() as *const c_char, aud_clk(CLK_MUX_AUDIOINTBUS), aud_clk(CLK_TOP_SYSPLL_D2_D4), ret);
		clk_disable_unprepare(priv_clk(afe_priv, CLK_MUX_AUDIO));
		clk_disable_unprepare(priv_clk(afe_priv, CLK_INFRA_SYS_AUDIO));
		return ret;
	}

	ret = clk_prepare_enable(priv_clk(afe_priv, CLK_AFE));
	if ret != 0 {
		dev_err((*afe).dev, b"%s clk_prepare_enable %s fail %d\n\0".as_ptr() as *const c_char, b"mt8183_afe_enable_clock\0".as_ptr() as *const c_char, aud_clk(CLK_AFE), ret);
		clk_disable_unprepare(priv_clk(afe_priv, CLK_MUX_AUDIOINTBUS));
		clk_disable_unprepare(priv_clk(afe_priv, CLK_MUX_AUDIO));
		clk_disable_unprepare(priv_clk(afe_priv, CLK_INFRA_SYS_AUDIO));
		return ret;
	}

	ret = clk_prepare_enable(priv_clk(afe_priv, CLK_I2S1_BCLK_SW));
	if ret != 0 {
		dev_err((*afe).dev, b"%s clk_prepare_enable %s fail %d\n\0".as_ptr() as *const c_char, b"mt8183_afe_enable_clock\0".as_ptr() as *const c_char, aud_clk(CLK_I2S1_BCLK_SW), ret);
		clk_disable_unprepare(priv_clk(afe_priv, CLK_AFE));
		clk_disable_unprepare(priv_clk(afe_priv, CLK_MUX_AUDIOINTBUS));
		clk_disable_unprepare(priv_clk(afe_priv, CLK_MUX_AUDIO));
		clk_disable_unprepare(priv_clk(afe_priv, CLK_INFRA_SYS_AUDIO));
		return ret;
	}

	ret = clk_prepare_enable(priv_clk(afe_priv, CLK_I2S2_BCLK_SW));
	if ret != 0 {
		dev_err((*afe).dev, b"%s clk_prepare_enable %s fail %d\n\0".as_ptr() as *const c_char, b"mt8183_afe_enable_clock\0".as_ptr() as *const c_char, aud_clk(CLK_I2S2_BCLK_SW), ret);
		clk_disable_unprepare(priv_clk(afe_priv, CLK_I2S1_BCLK_SW));
		clk_disable_unprepare(priv_clk(afe_priv, CLK_AFE));
		clk_disable_unprepare(priv_clk(afe_priv, CLK_MUX_AUDIOINTBUS));
		clk_disable_unprepare(priv_clk(afe_priv, CLK_MUX_AUDIO));
		clk_disable_unprepare(priv_clk(afe_priv, CLK_INFRA_SYS_AUDIO));
		return ret;
	}

	ret = clk_prepare_enable(priv_clk(afe_priv, CLK_I2S3_BCLK_SW));
	if ret != 0 {
		dev_err((*afe).dev, b"%s clk_prepare_enable %s fail %d\n\0".as_ptr() as *const c_char, b"mt8183_afe_enable_clock\0".as_ptr() as *const c_char, aud_clk(CLK_I2S3_BCLK_SW), ret);
		clk_disable_unprepare(priv_clk(afe_priv, CLK_I2S2_BCLK_SW));
		clk_disable_unprepare(priv_clk(afe_priv, CLK_I2S1_BCLK_SW));
		clk_disable_unprepare(priv_clk(afe_priv, CLK_AFE));
		clk_disable_unprepare(priv_clk(afe_priv, CLK_MUX_AUDIOINTBUS));
		clk_disable_unprepare(priv_clk(afe_priv, CLK_MUX_AUDIO));
		clk_disable_unprepare(priv_clk(afe_priv, CLK_INFRA_SYS_AUDIO));
		return ret;
	}

	ret = clk_prepare_enable(priv_clk(afe_priv, CLK_I2S4_BCLK_SW));
	if ret != 0 {
		dev_err((*afe).dev, b"%s clk_prepare_enable %s fail %d\n\0".as_ptr() as *const c_char, b"mt8183_afe_enable_clock\0".as_ptr() as *const c_char, aud_clk(CLK_I2S4_BCLK_SW), ret);
		clk_disable_unprepare(priv_clk(afe_priv, CLK_I2S3_BCLK_SW));
		clk_disable_unprepare(priv_clk(afe_priv, CLK_I2S2_BCLK_SW));
		clk_disable_unprepare(priv_clk(afe_priv, CLK_I2S1_BCLK_SW));
		clk_disable_unprepare(priv_clk(afe_priv, CLK_AFE));
		clk_disable_unprepare(priv_clk(afe_priv, CLK_MUX_AUDIOINTBUS));
		clk_disable_unprepare(priv_clk(afe_priv, CLK_MUX_AUDIO));
		clk_disable_unprepare(priv_clk(afe_priv, CLK_INFRA_SYS_AUDIO));
		return ret;
	}

	0
}

#[no_mangle]
pub unsafe extern "C" fn mt8183_afe_disable_clock(afe: *mut mtk_base_afe) -> c_int {
	let afe_priv = afe_priv(afe) as *mut mt8183_afe_private;

	clk_disable_unprepare(priv_clk(afe_priv, CLK_I2S4_BCLK_SW));
	clk_disable_unprepare(priv_clk(afe_priv, CLK_I2S3_BCLK_SW));
	clk_disable_unprepare(priv_clk(afe_priv, CLK_I2S2_BCLK_SW));
	clk_disable_unprepare(priv_clk(afe_priv, CLK_I2S1_BCLK_SW));
	clk_disable_unprepare(priv_clk(afe_priv, CLK_AFE));
	clk_disable_unprepare(priv_clk(afe_priv, CLK_MUX_AUDIOINTBUS));
	clk_disable_unprepare(priv_clk(afe_priv, CLK_MUX_AUDIO));
	clk_disable_unprepare(priv_clk(afe_priv, CLK_INFRA_SYS_AUDIO));

	0
}

/* apll */
unsafe fn apll1_mux_setting(afe: *mut mtk_base_afe, enable: bool) -> c_int {
	let afe_priv = afe_priv(afe) as *mut mt8183_afe_private;
	let mut ret: c_int;

	if enable {
		ret = clk_prepare_enable(priv_clk(afe_priv, CLK_TOP_MUX_AUD_1));
		if ret != 0 {
			dev_err((*afe).dev, b"%s clk_prepare_enable %s fail %d\n\0".as_ptr() as *const c_char, b"apll1_mux_setting\0".as_ptr() as *const c_char, aud_clk(CLK_TOP_MUX_AUD_1), ret);
			return ret;
		}
		ret = clk_set_parent(priv_clk(afe_priv, CLK_TOP_MUX_AUD_1), priv_clk(afe_priv, CLK_TOP_APLL1_CK));
		if ret != 0 {
			dev_err((*afe).dev, b"%s clk_set_parent %s-%s fail %d\n\0".as_ptr() as *const c_char, b"apll1_mux_setting\0".as_ptr() as *const c_char, aud_clk(CLK_TOP_MUX_AUD_1), aud_clk(CLK_TOP_APLL1_CK), ret);
			clk_set_parent(priv_clk(afe_priv, CLK_TOP_MUX_AUD_1), priv_clk(afe_priv, CLK_CLK26M));
			clk_disable_unprepare(priv_clk(afe_priv, CLK_TOP_MUX_AUD_1));
			return ret;
		}

		/* 180.6336 / 8 = 22.5792MHz */
		ret = clk_prepare_enable(priv_clk(afe_priv, CLK_TOP_MUX_AUD_ENG1));
		if ret != 0 {
			dev_err((*afe).dev, b"%s clk_prepare_enable %s fail %d\n\0".as_ptr() as *const c_char, b"apll1_mux_setting\0".as_ptr() as *const c_char, aud_clk(CLK_TOP_MUX_AUD_ENG1), ret);
			clk_set_parent(priv_clk(afe_priv, CLK_TOP_MUX_AUD_1), priv_clk(afe_priv, CLK_CLK26M));
			clk_disable_unprepare(priv_clk(afe_priv, CLK_TOP_MUX_AUD_1));
			return ret;
		}
		ret = clk_set_parent(priv_clk(afe_priv, CLK_TOP_MUX_AUD_ENG1), priv_clk(afe_priv, CLK_TOP_APLL1_D8));
		if ret != 0 {
			dev_err((*afe).dev, b"%s clk_set_parent %s-%s fail %d\n\0".as_ptr() as *const c_char, b"apll1_mux_setting\0".as_ptr() as *const c_char, aud_clk(CLK_TOP_MUX_AUD_ENG1), aud_clk(CLK_TOP_APLL1_D8), ret);
			clk_set_parent(priv_clk(afe_priv, CLK_TOP_MUX_AUD_ENG1), priv_clk(afe_priv, CLK_CLK26M));
			clk_disable_unprepare(priv_clk(afe_priv, CLK_TOP_MUX_AUD_ENG1));
			clk_set_parent(priv_clk(afe_priv, CLK_TOP_MUX_AUD_1), priv_clk(afe_priv, CLK_CLK26M));
			clk_disable_unprepare(priv_clk(afe_priv, CLK_TOP_MUX_AUD_1));
			return ret;
		}
	} else {
		ret = clk_set_parent(priv_clk(afe_priv, CLK_TOP_MUX_AUD_ENG1), priv_clk(afe_priv, CLK_CLK26M));
		if ret != 0 {
			dev_err((*afe).dev, b"%s clk_set_parent %s-%s fail %d\n\0".as_ptr() as *const c_char, b"apll1_mux_setting\0".as_ptr() as *const c_char, aud_clk(CLK_TOP_MUX_AUD_ENG1), aud_clk(CLK_CLK26M), ret);
			return ret;
		}
		clk_disable_unprepare(priv_clk(afe_priv, CLK_TOP_MUX_AUD_ENG1));

		ret = clk_set_parent(priv_clk(afe_priv, CLK_TOP_MUX_AUD_1), priv_clk(afe_priv, CLK_CLK26M));
		if ret != 0 {
			dev_err((*afe).dev, b"%s clk_set_parent %s-%s fail %d\n\0".as_ptr() as *const c_char, b"apll1_mux_setting\0".as_ptr() as *const c_char, aud_clk(CLK_TOP_MUX_AUD_1), aud_clk(CLK_CLK26M), ret);
			return ret;
		}
		clk_disable_unprepare(priv_clk(afe_priv, CLK_TOP_MUX_AUD_1));
	}

	0
}

unsafe fn apll2_mux_setting(afe: *mut mtk_base_afe, enable: bool) -> c_int {
	let afe_priv = afe_priv(afe) as *mut mt8183_afe_private;
	let mut ret: c_int;

	if enable {
		ret = clk_prepare_enable(priv_clk(afe_priv, CLK_TOP_MUX_AUD_2));
		if ret != 0 {
			dev_err((*afe).dev, b"%s clk_prepare_enable %s fail %d\n\0".as_ptr() as *const c_char, b"apll2_mux_setting\0".as_ptr() as *const c_char, aud_clk(CLK_TOP_MUX_AUD_2), ret);
			return ret;
		}
		ret = clk_set_parent(priv_clk(afe_priv, CLK_TOP_MUX_AUD_2), priv_clk(afe_priv, CLK_TOP_APLL2_CK));
		if ret != 0 {
			dev_err((*afe).dev, b"%s clk_set_parent %s-%s fail %d\n\0".as_ptr() as *const c_char, b"apll2_mux_setting\0".as_ptr() as *const c_char, aud_clk(CLK_TOP_MUX_AUD_2), aud_clk(CLK_TOP_APLL2_CK), ret);
			clk_set_parent(priv_clk(afe_priv, CLK_TOP_MUX_AUD_2), priv_clk(afe_priv, CLK_CLK26M));
			clk_disable_unprepare(priv_clk(afe_priv, CLK_TOP_MUX_AUD_2));
			return ret;
		}

		/* 196.608 / 8 = 24.576MHz */
		ret = clk_prepare_enable(priv_clk(afe_priv, CLK_TOP_MUX_AUD_ENG2));
		if ret != 0 {
			dev_err((*afe).dev, b"%s clk_prepare_enable %s fail %d\n\0".as_ptr() as *const c_char, b"apll2_mux_setting\0".as_ptr() as *const c_char, aud_clk(CLK_TOP_MUX_AUD_ENG2), ret);
			clk_set_parent(priv_clk(afe_priv, CLK_TOP_MUX_AUD_2), priv_clk(afe_priv, CLK_CLK26M));
			clk_disable_unprepare(priv_clk(afe_priv, CLK_TOP_MUX_AUD_2));
			return ret;
		}
		ret = clk_set_parent(priv_clk(afe_priv, CLK_TOP_MUX_AUD_ENG2), priv_clk(afe_priv, CLK_TOP_APLL2_D8));
		if ret != 0 {
			dev_err((*afe).dev, b"%s clk_set_parent %s-%s fail %d\n\0".as_ptr() as *const c_char, b"apll2_mux_setting\0".as_ptr() as *const c_char, aud_clk(CLK_TOP_MUX_AUD_ENG2), aud_clk(CLK_TOP_APLL2_D8), ret);
			clk_set_parent(priv_clk(afe_priv, CLK_TOP_MUX_AUD_ENG2), priv_clk(afe_priv, CLK_CLK26M));
			clk_disable_unprepare(priv_clk(afe_priv, CLK_TOP_MUX_AUD_ENG2));
			clk_set_parent(priv_clk(afe_priv, CLK_TOP_MUX_AUD_2), priv_clk(afe_priv, CLK_CLK26M));
			clk_disable_unprepare(priv_clk(afe_priv, CLK_TOP_MUX_AUD_2));
			return ret;
		}
	} else {
		ret = clk_set_parent(priv_clk(afe_priv, CLK_TOP_MUX_AUD_ENG2), priv_clk(afe_priv, CLK_CLK26M));
		if ret != 0 {
			dev_err((*afe).dev, b"%s clk_set_parent %s-%s fail %d\n\0".as_ptr() as *const c_char, b"apll2_mux_setting\0".as_ptr() as *const c_char, aud_clk(CLK_TOP_MUX_AUD_ENG2), aud_clk(CLK_CLK26M), ret);
			return ret;
		}
		clk_disable_unprepare(priv_clk(afe_priv, CLK_TOP_MUX_AUD_ENG2));

		ret = clk_set_parent(priv_clk(afe_priv, CLK_TOP_MUX_AUD_2), priv_clk(afe_priv, CLK_CLK26M));
		if ret != 0 {
			dev_err((*afe).dev, b"%s clk_set_parent %s-%s fail %d\n\0".as_ptr() as *const c_char, b"apll2_mux_setting\0".as_ptr() as *const c_char, aud_clk(CLK_TOP_MUX_AUD_2), aud_clk(CLK_CLK26M), ret);
			return ret;
		}
		clk_disable_unprepare(priv_clk(afe_priv, CLK_TOP_MUX_AUD_2));
	}

	0
}

#[no_mangle]
pub unsafe extern "C" fn mt8183_apll1_enable(afe: *mut mtk_base_afe) -> c_int {
	let afe_priv = afe_priv(afe) as *mut mt8183_afe_private;
	let mut ret: c_int;

	/* setting for APLL */
	apll1_mux_setting(afe, true);

	ret = clk_prepare_enable(priv_clk(afe_priv, CLK_APLL22M));
	if ret != 0 {
		dev_err((*afe).dev, b"%s clk_prepare_enable %s fail %d\n\0".as_ptr() as *const c_char, b"mt8183_apll1_enable\0".as_ptr() as *const c_char, aud_clk(CLK_APLL22M), ret);
		return ret;
	}

	ret = clk_prepare_enable(priv_clk(afe_priv, CLK_APLL1_TUNER));
	if ret != 0 {
		dev_err((*afe).dev, b"%s clk_prepare_enable %s fail %d\n\0".as_ptr() as *const c_char, b"mt8183_apll1_enable\0".as_ptr() as *const c_char, aud_clk(CLK_APLL1_TUNER), ret);
		clk_disable_unprepare(priv_clk(afe_priv, CLK_APLL22M));
		return ret;
	}

	regmap_update_bits((*afe).regmap, AFE_APLL1_TUNER_CFG, 0x0000FFF7, 0x00000832);
	regmap_update_bits((*afe).regmap, AFE_APLL1_TUNER_CFG, 0x1, 0x1);

	regmap_update_bits(
		(*afe).regmap,
		AFE_HD_ENGEN_ENABLE,
		AFE_22M_ON_MASK_SFT,
		0x1 << AFE_22M_ON_SFT,
	);

	0
}

#[no_mangle]
pub unsafe extern "C" fn mt8183_apll1_disable(afe: *mut mtk_base_afe) {
	let afe_priv = afe_priv(afe) as *mut mt8183_afe_private;

	regmap_update_bits(
		(*afe).regmap,
		AFE_HD_ENGEN_ENABLE,
		AFE_22M_ON_MASK_SFT,
		0x0 << AFE_22M_ON_SFT,
	);

	regmap_update_bits((*afe).regmap, AFE_APLL1_TUNER_CFG, 0x1, 0x0);

	clk_disable_unprepare(priv_clk(afe_priv, CLK_APLL1_TUNER));
	clk_disable_unprepare(priv_clk(afe_priv, CLK_APLL22M));

	apll1_mux_setting(afe, false);
}

#[no_mangle]
pub unsafe extern "C" fn mt8183_apll2_enable(afe: *mut mtk_base_afe) -> c_int {
	let afe_priv = afe_priv(afe) as *mut mt8183_afe_private;
	let mut ret: c_int;

	/* setting for APLL */
	apll2_mux_setting(afe, true);

	ret = clk_prepare_enable(priv_clk(afe_priv, CLK_APLL24M));
	if ret != 0 {
		dev_err((*afe).dev, b"%s clk_prepare_enable %s fail %d\n\0".as_ptr() as *const c_char, b"mt8183_apll2_enable\0".as_ptr() as *const c_char, aud_clk(CLK_APLL24M), ret);
		return ret;
	}

	ret = clk_prepare_enable(priv_clk(afe_priv, CLK_APLL2_TUNER));
	if ret != 0 {
		dev_err((*afe).dev, b"%s clk_prepare_enable %s fail %d\n\0".as_ptr() as *const c_char, b"mt8183_apll2_enable\0".as_ptr() as *const c_char, aud_clk(CLK_APLL2_TUNER), ret);
		clk_disable_unprepare(priv_clk(afe_priv, CLK_APLL24M));
		return ret;
	}

	regmap_update_bits((*afe).regmap, AFE_APLL2_TUNER_CFG, 0x0000FFF7, 0x00000634);
	regmap_update_bits((*afe).regmap, AFE_APLL2_TUNER_CFG, 0x1, 0x1);

	regmap_update_bits(
		(*afe).regmap,
		AFE_HD_ENGEN_ENABLE,
		AFE_24M_ON_MASK_SFT,
		0x1 << AFE_24M_ON_SFT,
	);

	0
}

#[no_mangle]
pub unsafe extern "C" fn mt8183_apll2_disable(afe: *mut mtk_base_afe) {
	let afe_priv = afe_priv(afe) as *mut mt8183_afe_private;

	regmap_update_bits(
		(*afe).regmap,
		AFE_HD_ENGEN_ENABLE,
		AFE_24M_ON_MASK_SFT,
		0x0 << AFE_24M_ON_SFT,
	);

	regmap_update_bits((*afe).regmap, AFE_APLL2_TUNER_CFG, 0x1, 0x0);

	clk_disable_unprepare(priv_clk(afe_priv, CLK_APLL2_TUNER));
	clk_disable_unprepare(priv_clk(afe_priv, CLK_APLL24M));

	apll2_mux_setting(afe, false);
}

#[no_mangle]
pub unsafe extern "C" fn mt8183_get_apll_rate(_afe: *mut mtk_base_afe, apll: c_int) -> c_int {
	if apll == MT8183_APLL1 {
		180633600
	} else {
		196608000
	}
}

#[no_mangle]
pub unsafe extern "C" fn mt8183_get_apll_by_rate(_afe: *mut mtk_base_afe, rate: c_int) -> c_int {
	if (rate % 8000) == 0 {
		MT8183_APLL2
	} else {
		MT8183_APLL1
	}
}

#[no_mangle]
pub unsafe extern "C" fn mt8183_get_apll_by_name(_afe: *mut mtk_base_afe, name: *const c_char) -> c_int {
	if strcmp(name, APLL1_W_NAME) == 0 {
		MT8183_APLL1
	} else {
		MT8183_APLL2
	}
}

/* mck */
#[repr(C)]
struct mt8183_mck_div {
	m_sel_id: c_int,
	div_clk_id: c_int,
}

static MCK_DIV: [mt8183_mck_div; MT8183_MCK_NUM] = [
	mt8183_mck_div {
		m_sel_id: CLK_TOP_I2S0_M_SEL as c_int,
		div_clk_id: CLK_TOP_APLL12_DIV0 as c_int,
	},
	mt8183_mck_div {
		m_sel_id: CLK_TOP_I2S1_M_SEL as c_int,
		div_clk_id: CLK_TOP_APLL12_DIV1 as c_int,
	},
	mt8183_mck_div {
		m_sel_id: CLK_TOP_I2S2_M_SEL as c_int,
		div_clk_id: CLK_TOP_APLL12_DIV2 as c_int,
	},
	mt8183_mck_div {
		m_sel_id: CLK_TOP_I2S3_M_SEL as c_int,
		div_clk_id: CLK_TOP_APLL12_DIV3 as c_int,
	},
	mt8183_mck_div {
		m_sel_id: CLK_TOP_I2S4_M_SEL as c_int,
		div_clk_id: CLK_TOP_APLL12_DIV4 as c_int,
	},
	mt8183_mck_div {
		m_sel_id: -1,
		div_clk_id: CLK_TOP_APLL12_DIVB as c_int,
	},
	mt8183_mck_div {
		m_sel_id: -1,
		div_clk_id: -1,
	},
];

#[no_mangle]
pub unsafe extern "C" fn mt8183_mck_enable(
	afe: *mut mtk_base_afe,
	mck_id: c_int,
	rate: c_int,
) -> c_int {
	let afe_priv = afe_priv(afe) as *mut mt8183_afe_private;
	let apll = mt8183_get_apll_by_rate(afe, rate);
	let apll_clk_id = if apll == MT8183_APLL1 {
		CLK_TOP_MUX_AUD_1
	} else {
		CLK_TOP_MUX_AUD_2
	};
	let m_sel_id = MCK_DIV[mck_id as usize].m_sel_id;
	let div_clk_id = MCK_DIV[mck_id as usize].div_clk_id;
	let mut ret: c_int;

	/* i2s5 mck not support */
	if mck_id == MT8183_I2S5_MCK {
		return 0;
	}

	/* select apll */
	if m_sel_id >= 0 {
		ret = clk_prepare_enable(priv_clk(afe_priv, m_sel_id as usize));
		if ret != 0 {
			dev_err((*afe).dev, b"%s(), clk_prepare_enable %s fail %d\n\0".as_ptr() as *const c_char, b"mt8183_mck_enable\0".as_ptr() as *const c_char, aud_clk(m_sel_id as usize), ret);
			return ret;
		}
		ret = clk_set_parent(priv_clk(afe_priv, m_sel_id as usize), priv_clk(afe_priv, apll_clk_id));
		if ret != 0 {
			dev_err((*afe).dev, b"%s(), clk_set_parent %s-%s fail %d\n\0".as_ptr() as *const c_char, b"mt8183_mck_enable\0".as_ptr() as *const c_char, aud_clk(m_sel_id as usize), aud_clk(apll_clk_id), ret);
			clk_disable_unprepare(priv_clk(afe_priv, m_sel_id as usize));
			return ret;
		}
	}

	/* enable div, set rate */
	ret = clk_prepare_enable(priv_clk(afe_priv, div_clk_id as usize));
	if ret != 0 {
		dev_err((*afe).dev, b"%s(), clk_prepare_enable %s fail %d\n\0".as_ptr() as *const c_char, b"mt8183_mck_enable\0".as_ptr() as *const c_char, aud_clk(div_clk_id as usize), ret);
		if m_sel_id >= 0 {
			clk_disable_unprepare(priv_clk(afe_priv, m_sel_id as usize));
		}
		return ret;
	}
	ret = clk_set_rate(priv_clk(afe_priv, div_clk_id as usize), rate);
	if ret != 0 {
		dev_err((*afe).dev, b"%s(), clk_set_rate %s, rate %d, fail %d\n\0".as_ptr() as *const c_char, b"mt8183_mck_enable\0".as_ptr() as *const c_char, aud_clk(div_clk_id as usize), rate, ret);
		clk_disable_unprepare(priv_clk(afe_priv, div_clk_id as usize));
		if m_sel_id >= 0 {
			clk_disable_unprepare(priv_clk(afe_priv, m_sel_id as usize));
		}
		return ret;
	}

	0
}

#[no_mangle]
pub unsafe extern "C" fn mt8183_mck_disable(afe: *mut mtk_base_afe, mck_id: c_int) {
	let afe_priv = afe_priv(afe) as *mut mt8183_afe_private;
	let m_sel_id = MCK_DIV[mck_id as usize].m_sel_id;
	let div_clk_id = MCK_DIV[mck_id as usize].div_clk_id;

	/* i2s5 mck not support */
	if mck_id == MT8183_I2S5_MCK {
		return;
	}

	clk_disable_unprepare(priv_clk(afe_priv, div_clk_id as usize));
	if m_sel_id >= 0 {
		clk_disable_unprepare(priv_clk(afe_priv, m_sel_id as usize));
	}
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
