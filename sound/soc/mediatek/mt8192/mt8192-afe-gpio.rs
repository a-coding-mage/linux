// SPDX-License-Identifier: GPL-2.0
//
// mt8192-afe-gpio.c  --  Mediatek 8192 afe gpio ctrl
//
// Copyright (c) 2020 MediaTek Inc.
// Author: Shane Chien <shane.chien@mediatek.com>
//

// C dependencies:
// #include <linux/pinctrl/consumer.h>
// #include "mt8192-afe-common.h"
// #include "mt8192-afe-gpio.h"

use core::ffi::{c_char, c_int};

#[repr(C)]
pub struct device {
	_private: [u8; 0],
}

#[repr(C)]
pub struct pinctrl {
	_private: [u8; 0],
}

#[repr(C)]
pub struct pinctrl_state {
	_private: [u8; 0],
}

#[repr(C)]
pub struct mutex {
	_private: [u8; 0],
}

extern "C" {
	static mut gpio_request_mutex: mutex;

	static MT8192_DAI_ADDA: c_int;
	static MT8192_DAI_ADDA_CH34: c_int;
	static MT8192_DAI_I2S_0: c_int;
	static MT8192_DAI_I2S_1: c_int;
	static MT8192_DAI_I2S_2: c_int;
	static MT8192_DAI_I2S_3: c_int;
	static MT8192_DAI_I2S_5: c_int;
	static MT8192_DAI_I2S_6: c_int;
	static MT8192_DAI_I2S_7: c_int;
	static MT8192_DAI_I2S_8: c_int;
	static MT8192_DAI_I2S_9: c_int;
	static MT8192_DAI_TDM: c_int;
	static MT8192_DAI_VOW: c_int;

	fn devm_pinctrl_get(dev: *mut device) -> *mut pinctrl;
	fn pinctrl_lookup_state(
		p: *mut pinctrl,
		name: *const c_char,
	) -> *mut pinctrl_state;
	fn pinctrl_select_state(
		p: *mut pinctrl,
		state: *mut pinctrl_state,
	) -> c_int;
	fn IS_ERR(ptr: *const core::ffi::c_void) -> bool;
	fn PTR_ERR(ptr: *const core::ffi::c_void) -> c_int;
	fn dev_err(dev: *mut device, fmt: *const c_char, ...);
	fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
	fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
	fn mutex_lock(lock: *mut mutex);
	fn mutex_unlock(lock: *mut mutex);
}

const EINVAL: c_int = 22;
const EIO: c_int = 5;

static mut AUD_PINCTRL: *mut pinctrl = core::ptr::null_mut();

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum mt8192_afe_gpio {
	MT8192_AFE_GPIO_DAT_MISO_OFF,
	MT8192_AFE_GPIO_DAT_MISO_ON,
	MT8192_AFE_GPIO_DAT_MOSI_OFF,
	MT8192_AFE_GPIO_DAT_MOSI_ON,
	MT8192_AFE_GPIO_DAT_MISO_CH34_OFF,
	MT8192_AFE_GPIO_DAT_MISO_CH34_ON,
	MT8192_AFE_GPIO_DAT_MOSI_CH34_OFF,
	MT8192_AFE_GPIO_DAT_MOSI_CH34_ON,
	MT8192_AFE_GPIO_I2S0_OFF,
	MT8192_AFE_GPIO_I2S0_ON,
	MT8192_AFE_GPIO_I2S1_OFF,
	MT8192_AFE_GPIO_I2S1_ON,
	MT8192_AFE_GPIO_I2S2_OFF,
	MT8192_AFE_GPIO_I2S2_ON,
	MT8192_AFE_GPIO_I2S3_OFF,
	MT8192_AFE_GPIO_I2S3_ON,
	MT8192_AFE_GPIO_I2S5_OFF,
	MT8192_AFE_GPIO_I2S5_ON,
	MT8192_AFE_GPIO_I2S6_OFF,
	MT8192_AFE_GPIO_I2S6_ON,
	MT8192_AFE_GPIO_I2S7_OFF,
	MT8192_AFE_GPIO_I2S7_ON,
	MT8192_AFE_GPIO_I2S8_OFF,
	MT8192_AFE_GPIO_I2S8_ON,
	MT8192_AFE_GPIO_I2S9_OFF,
	MT8192_AFE_GPIO_I2S9_ON,
	MT8192_AFE_GPIO_VOW_DAT_OFF,
	MT8192_AFE_GPIO_VOW_DAT_ON,
	MT8192_AFE_GPIO_VOW_CLK_OFF,
	MT8192_AFE_GPIO_VOW_CLK_ON,
	MT8192_AFE_GPIO_CLK_MOSI_OFF,
	MT8192_AFE_GPIO_CLK_MOSI_ON,
	MT8192_AFE_GPIO_TDM_OFF,
	MT8192_AFE_GPIO_TDM_ON,
	MT8192_AFE_GPIO_GPIO_NUM,
}

#[repr(C)]
struct audio_gpio_attr {
	name: *const c_char,
	gpio_prepare: bool,
	gpioctrl: *mut pinctrl_state,
}

const MT8192_AFE_GPIO_GPIO_NUM_USIZE: usize =
	mt8192_afe_gpio::MT8192_AFE_GPIO_GPIO_NUM as usize;

static mut AUD_GPIOS: [audio_gpio_attr; MT8192_AFE_GPIO_GPIO_NUM_USIZE] = [
	audio_gpio_attr { name: b"aud_dat_miso_off\0".as_ptr() as *const c_char, gpio_prepare: false, gpioctrl: core::ptr::null_mut() },
	audio_gpio_attr { name: b"aud_dat_miso_on\0".as_ptr() as *const c_char, gpio_prepare: false, gpioctrl: core::ptr::null_mut() },
	audio_gpio_attr { name: b"aud_dat_mosi_off\0".as_ptr() as *const c_char, gpio_prepare: false, gpioctrl: core::ptr::null_mut() },
	audio_gpio_attr { name: b"aud_dat_mosi_on\0".as_ptr() as *const c_char, gpio_prepare: false, gpioctrl: core::ptr::null_mut() },
	audio_gpio_attr { name: b"aud_dat_miso_ch34_off\0".as_ptr() as *const c_char, gpio_prepare: false, gpioctrl: core::ptr::null_mut() },
	audio_gpio_attr { name: b"aud_dat_miso_ch34_on\0".as_ptr() as *const c_char, gpio_prepare: false, gpioctrl: core::ptr::null_mut() },
	audio_gpio_attr { name: b"aud_dat_mosi_ch34_off\0".as_ptr() as *const c_char, gpio_prepare: false, gpioctrl: core::ptr::null_mut() },
	audio_gpio_attr { name: b"aud_dat_mosi_ch34_on\0".as_ptr() as *const c_char, gpio_prepare: false, gpioctrl: core::ptr::null_mut() },
	audio_gpio_attr { name: b"aud_gpio_i2s0_off\0".as_ptr() as *const c_char, gpio_prepare: false, gpioctrl: core::ptr::null_mut() },
	audio_gpio_attr { name: b"aud_gpio_i2s0_on\0".as_ptr() as *const c_char, gpio_prepare: false, gpioctrl: core::ptr::null_mut() },
	audio_gpio_attr { name: b"aud_gpio_i2s1_off\0".as_ptr() as *const c_char, gpio_prepare: false, gpioctrl: core::ptr::null_mut() },
	audio_gpio_attr { name: b"aud_gpio_i2s1_on\0".as_ptr() as *const c_char, gpio_prepare: false, gpioctrl: core::ptr::null_mut() },
	audio_gpio_attr { name: b"aud_gpio_i2s2_off\0".as_ptr() as *const c_char, gpio_prepare: false, gpioctrl: core::ptr::null_mut() },
	audio_gpio_attr { name: b"aud_gpio_i2s2_on\0".as_ptr() as *const c_char, gpio_prepare: false, gpioctrl: core::ptr::null_mut() },
	audio_gpio_attr { name: b"aud_gpio_i2s3_off\0".as_ptr() as *const c_char, gpio_prepare: false, gpioctrl: core::ptr::null_mut() },
	audio_gpio_attr { name: b"aud_gpio_i2s3_on\0".as_ptr() as *const c_char, gpio_prepare: false, gpioctrl: core::ptr::null_mut() },
	audio_gpio_attr { name: b"aud_gpio_i2s5_off\0".as_ptr() as *const c_char, gpio_prepare: false, gpioctrl: core::ptr::null_mut() },
	audio_gpio_attr { name: b"aud_gpio_i2s5_on\0".as_ptr() as *const c_char, gpio_prepare: false, gpioctrl: core::ptr::null_mut() },
	audio_gpio_attr { name: b"aud_gpio_i2s6_off\0".as_ptr() as *const c_char, gpio_prepare: false, gpioctrl: core::ptr::null_mut() },
	audio_gpio_attr { name: b"aud_gpio_i2s6_on\0".as_ptr() as *const c_char, gpio_prepare: false, gpioctrl: core::ptr::null_mut() },
	audio_gpio_attr { name: b"aud_gpio_i2s7_off\0".as_ptr() as *const c_char, gpio_prepare: false, gpioctrl: core::ptr::null_mut() },
	audio_gpio_attr { name: b"aud_gpio_i2s7_on\0".as_ptr() as *const c_char, gpio_prepare: false, gpioctrl: core::ptr::null_mut() },
	audio_gpio_attr { name: b"aud_gpio_i2s8_off\0".as_ptr() as *const c_char, gpio_prepare: false, gpioctrl: core::ptr::null_mut() },
	audio_gpio_attr { name: b"aud_gpio_i2s8_on\0".as_ptr() as *const c_char, gpio_prepare: false, gpioctrl: core::ptr::null_mut() },
	audio_gpio_attr { name: b"aud_gpio_i2s9_off\0".as_ptr() as *const c_char, gpio_prepare: false, gpioctrl: core::ptr::null_mut() },
	audio_gpio_attr { name: b"aud_gpio_i2s9_on\0".as_ptr() as *const c_char, gpio_prepare: false, gpioctrl: core::ptr::null_mut() },
	audio_gpio_attr { name: b"vow_dat_miso_off\0".as_ptr() as *const c_char, gpio_prepare: false, gpioctrl: core::ptr::null_mut() },
	audio_gpio_attr { name: b"vow_dat_miso_on\0".as_ptr() as *const c_char, gpio_prepare: false, gpioctrl: core::ptr::null_mut() },
	audio_gpio_attr { name: b"vow_clk_miso_off\0".as_ptr() as *const c_char, gpio_prepare: false, gpioctrl: core::ptr::null_mut() },
	audio_gpio_attr { name: b"vow_clk_miso_on\0".as_ptr() as *const c_char, gpio_prepare: false, gpioctrl: core::ptr::null_mut() },
	audio_gpio_attr { name: b"aud_clk_mosi_off\0".as_ptr() as *const c_char, gpio_prepare: false, gpioctrl: core::ptr::null_mut() },
	audio_gpio_attr { name: b"aud_clk_mosi_on\0".as_ptr() as *const c_char, gpio_prepare: false, gpioctrl: core::ptr::null_mut() },
	audio_gpio_attr { name: b"aud_gpio_tdm_off\0".as_ptr() as *const c_char, gpio_prepare: false, gpioctrl: core::ptr::null_mut() },
	audio_gpio_attr { name: b"aud_gpio_tdm_on\0".as_ptr() as *const c_char, gpio_prepare: false, gpioctrl: core::ptr::null_mut() },
];

unsafe fn mt8192_afe_gpio_select(
	dev: *mut device,
	gpio_type: mt8192_afe_gpio,
) -> c_int {
	let ret: c_int;
	let gpio_index = gpio_type as isize;

	if gpio_index < 0
		|| gpio_index >= mt8192_afe_gpio::MT8192_AFE_GPIO_GPIO_NUM as isize
	{
		dev_err(
			dev,
			b"%s(), error, invalid gpio type %d\n\0".as_ptr() as *const c_char,
			b"mt8192_afe_gpio_select\0".as_ptr() as *const c_char,
			gpio_index as c_int,
		);
		return -EINVAL;
	}

	let gpio_index = gpio_index as usize;
	if !AUD_GPIOS[gpio_index].gpio_prepare {
		dev_warn(
			dev,
			b"%s(), error, gpio type %d not prepared\n\0".as_ptr() as *const c_char,
			b"mt8192_afe_gpio_select\0".as_ptr() as *const c_char,
			gpio_index as c_int,
		);
		return -EIO;
	}

	ret = pinctrl_select_state(AUD_PINCTRL, AUD_GPIOS[gpio_index].gpioctrl);
	if ret != 0 {
		dev_dbg(
			dev,
			b"%s(), error, can not set gpio type %d\n\0".as_ptr() as *const c_char,
			b"mt8192_afe_gpio_select\0".as_ptr() as *const c_char,
			gpio_index as c_int,
		);
	}

	ret
}

#[no_mangle]
pub unsafe extern "C" fn mt8192_afe_gpio_init(dev: *mut device) -> c_int {
	let mut ret: c_int;

	AUD_PINCTRL = devm_pinctrl_get(dev);
	if IS_ERR(AUD_PINCTRL as *const core::ffi::c_void) {
		ret = PTR_ERR(AUD_PINCTRL as *const core::ffi::c_void);
		dev_err(
			dev,
			b"%s(), ret %d, cannot get aud_pinctrl!\n\0".as_ptr() as *const c_char,
			b"mt8192_afe_gpio_init\0".as_ptr() as *const c_char,
			ret,
		);
		return ret;
	}

	let mut i = 0usize;
	while i < AUD_GPIOS.len() {
		AUD_GPIOS[i].gpioctrl = pinctrl_lookup_state(AUD_PINCTRL, AUD_GPIOS[i].name);
		if IS_ERR(AUD_GPIOS[i].gpioctrl as *const core::ffi::c_void) {
			ret = PTR_ERR(AUD_GPIOS[i].gpioctrl as *const core::ffi::c_void);
			dev_dbg(
				dev,
				b"%s(), pinctrl_lookup_state %s fail, ret %d\n\0".as_ptr()
					as *const c_char,
				b"mt8192_afe_gpio_init\0".as_ptr() as *const c_char,
				AUD_GPIOS[i].name,
				ret,
			);
		} else {
			AUD_GPIOS[i].gpio_prepare = true;
		}
		i += 1;
	}

	mt8192_afe_gpio_select(dev, mt8192_afe_gpio::MT8192_AFE_GPIO_CLK_MOSI_ON);

	/* gpio status init */
	mt8192_afe_gpio_request(dev, false, MT8192_DAI_ADDA, 0);
	mt8192_afe_gpio_request(dev, false, MT8192_DAI_ADDA, 1);

	0
}

// EXPORT_SYMBOL(mt8192_afe_gpio_init);

unsafe fn mt8192_afe_gpio_adda_dl(dev: *mut device, enable: bool) -> c_int {
	if enable {
		return mt8192_afe_gpio_select(dev, mt8192_afe_gpio::MT8192_AFE_GPIO_DAT_MOSI_ON);
	} else {
		return mt8192_afe_gpio_select(dev, mt8192_afe_gpio::MT8192_AFE_GPIO_DAT_MOSI_OFF);
	}
}

unsafe fn mt8192_afe_gpio_adda_ul(dev: *mut device, enable: bool) -> c_int {
	if enable {
		return mt8192_afe_gpio_select(dev, mt8192_afe_gpio::MT8192_AFE_GPIO_DAT_MISO_ON);
	} else {
		return mt8192_afe_gpio_select(dev, mt8192_afe_gpio::MT8192_AFE_GPIO_DAT_MISO_OFF);
	}
}

unsafe fn mt8192_afe_gpio_adda_ch34_dl(dev: *mut device, enable: bool) -> c_int {
	if enable {
		return mt8192_afe_gpio_select(
			dev,
			mt8192_afe_gpio::MT8192_AFE_GPIO_DAT_MOSI_CH34_ON,
		);
	} else {
		return mt8192_afe_gpio_select(
			dev,
			mt8192_afe_gpio::MT8192_AFE_GPIO_DAT_MOSI_CH34_OFF,
		);
	}
}

unsafe fn mt8192_afe_gpio_adda_ch34_ul(dev: *mut device, enable: bool) -> c_int {
	if enable {
		return mt8192_afe_gpio_select(
			dev,
			mt8192_afe_gpio::MT8192_AFE_GPIO_DAT_MISO_CH34_ON,
		);
	} else {
		return mt8192_afe_gpio_select(
			dev,
			mt8192_afe_gpio::MT8192_AFE_GPIO_DAT_MISO_CH34_OFF,
		);
	}
}

#[no_mangle]
pub unsafe extern "C" fn mt8192_afe_gpio_request(
	dev: *mut device,
	enable: bool,
	dai: c_int,
	uplink: c_int,
) -> c_int {
	mutex_lock(&mut gpio_request_mutex);

	if dai == MT8192_DAI_ADDA {
		if uplink != 0 {
			mt8192_afe_gpio_adda_ul(dev, enable);
		} else {
			mt8192_afe_gpio_adda_dl(dev, enable);
		}
	} else if dai == MT8192_DAI_ADDA_CH34 {
		if uplink != 0 {
			mt8192_afe_gpio_adda_ch34_ul(dev, enable);
		} else {
			mt8192_afe_gpio_adda_ch34_dl(dev, enable);
		}
	} else if dai == MT8192_DAI_I2S_0 {
		if enable {
			mt8192_afe_gpio_select(dev, mt8192_afe_gpio::MT8192_AFE_GPIO_I2S0_ON);
		} else {
			mt8192_afe_gpio_select(dev, mt8192_afe_gpio::MT8192_AFE_GPIO_I2S0_OFF);
		}
	} else if dai == MT8192_DAI_I2S_1 {
		if enable {
			mt8192_afe_gpio_select(dev, mt8192_afe_gpio::MT8192_AFE_GPIO_I2S1_ON);
		} else {
			mt8192_afe_gpio_select(dev, mt8192_afe_gpio::MT8192_AFE_GPIO_I2S1_OFF);
		}
	} else if dai == MT8192_DAI_I2S_2 {
		if enable {
			mt8192_afe_gpio_select(dev, mt8192_afe_gpio::MT8192_AFE_GPIO_I2S2_ON);
		} else {
			mt8192_afe_gpio_select(dev, mt8192_afe_gpio::MT8192_AFE_GPIO_I2S2_OFF);
		}
	} else if dai == MT8192_DAI_I2S_3 {
		if enable {
			mt8192_afe_gpio_select(dev, mt8192_afe_gpio::MT8192_AFE_GPIO_I2S3_ON);
		} else {
			mt8192_afe_gpio_select(dev, mt8192_afe_gpio::MT8192_AFE_GPIO_I2S3_OFF);
		}
	} else if dai == MT8192_DAI_I2S_5 {
		if enable {
			mt8192_afe_gpio_select(dev, mt8192_afe_gpio::MT8192_AFE_GPIO_I2S5_ON);
		} else {
			mt8192_afe_gpio_select(dev, mt8192_afe_gpio::MT8192_AFE_GPIO_I2S5_OFF);
		}
	} else if dai == MT8192_DAI_I2S_6 {
		if enable {
			mt8192_afe_gpio_select(dev, mt8192_afe_gpio::MT8192_AFE_GPIO_I2S6_ON);
		} else {
			mt8192_afe_gpio_select(dev, mt8192_afe_gpio::MT8192_AFE_GPIO_I2S6_OFF);
		}
	} else if dai == MT8192_DAI_I2S_7 {
		if enable {
			mt8192_afe_gpio_select(dev, mt8192_afe_gpio::MT8192_AFE_GPIO_I2S7_ON);
		} else {
			mt8192_afe_gpio_select(dev, mt8192_afe_gpio::MT8192_AFE_GPIO_I2S7_OFF);
		}
	} else if dai == MT8192_DAI_I2S_8 {
		if enable {
			mt8192_afe_gpio_select(dev, mt8192_afe_gpio::MT8192_AFE_GPIO_I2S8_ON);
		} else {
			mt8192_afe_gpio_select(dev, mt8192_afe_gpio::MT8192_AFE_GPIO_I2S8_OFF);
		}
	} else if dai == MT8192_DAI_I2S_9 {
		if enable {
			mt8192_afe_gpio_select(dev, mt8192_afe_gpio::MT8192_AFE_GPIO_I2S9_ON);
		} else {
			mt8192_afe_gpio_select(dev, mt8192_afe_gpio::MT8192_AFE_GPIO_I2S9_OFF);
		}
	} else if dai == MT8192_DAI_TDM {
		if enable {
			mt8192_afe_gpio_select(dev, mt8192_afe_gpio::MT8192_AFE_GPIO_TDM_ON);
		} else {
			mt8192_afe_gpio_select(dev, mt8192_afe_gpio::MT8192_AFE_GPIO_TDM_OFF);
		}
	} else if dai == MT8192_DAI_VOW {
		if enable {
			mt8192_afe_gpio_select(dev, mt8192_afe_gpio::MT8192_AFE_GPIO_VOW_CLK_ON);
			mt8192_afe_gpio_select(dev, mt8192_afe_gpio::MT8192_AFE_GPIO_VOW_DAT_ON);
		} else {
			mt8192_afe_gpio_select(dev, mt8192_afe_gpio::MT8192_AFE_GPIO_VOW_CLK_OFF);
			mt8192_afe_gpio_select(dev, mt8192_afe_gpio::MT8192_AFE_GPIO_VOW_DAT_OFF);
		}
	} else {
		dev_warn(
			dev,
			b"%s(), invalid dai %d\n\0".as_ptr() as *const c_char,
			b"mt8192_afe_gpio_request\0".as_ptr() as *const c_char,
			dai,
		);
		mutex_unlock(&mut gpio_request_mutex);
		return -EINVAL;
	}

	mutex_unlock(&mut gpio_request_mutex);
	0
}

// EXPORT_SYMBOL(mt8192_afe_gpio_request);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
