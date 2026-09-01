// SPDX-License-Identifier: GPL-2.0
//
// TI SRC4xxx Audio Codec driver
//
// Copyright 2021-2022 Deqx Pty Ltd
// Author: Matt Flax <flatmax@flatmax.com>

// C dependencies: linux/module.h, sound/soc.h, sound/tlv.h, "src4xxx.h".

use core::ffi::{c_char, c_int, c_uint, c_void};

#[repr(C)]
pub struct regmap {
	_private: [u8; 0],
}

#[repr(C)]
pub struct device {
	_private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_component {
	pub dev: *mut device,
}

#[repr(C)]
pub struct snd_soc_dai {
	pub id: c_int,
	pub component: *mut snd_soc_component,
	pub dev: *mut device,
}

#[repr(C)]
pub struct snd_pcm_substream {
	_private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_hw_params {
	_private: [u8; 0],
}

#[repr(C)]
pub struct snd_kcontrol_new {
	_private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dapm_widget {
	_private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dapm_route {
	pub sink: *const c_char,
	pub control: *const c_char,
	pub source: *const c_char,
}

#[repr(C)]
pub struct snd_soc_component_driver {
	pub controls: *const snd_kcontrol_new,
	pub num_controls: c_uint,
	pub dapm_widgets: *const snd_soc_dapm_widget,
	pub num_dapm_widgets: c_uint,
	pub dapm_routes: *const snd_soc_dapm_route,
	pub num_dapm_routes: c_uint,
}

#[repr(C)]
pub struct snd_soc_dai_ops {
	pub hw_params: Option<
		unsafe extern "C" fn(
			*mut snd_pcm_substream,
			*mut snd_pcm_hw_params,
			*mut snd_soc_dai,
		) -> c_int,
	>,
	pub set_sysclk:
		Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_uint, c_int) -> c_int>,
	pub set_fmt: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint) -> c_int>,
}

#[repr(C)]
pub struct snd_soc_pcm_stream {
	pub stream_name: *const c_char,
	pub channels_min: c_uint,
	pub channels_max: c_uint,
	pub rates: c_uint,
	pub formats: c_uint,
}

#[repr(C)]
pub struct snd_soc_dai_driver {
	pub id: c_int,
	pub name: *const c_char,
	pub playback: snd_soc_pcm_stream,
	pub capture: snd_soc_pcm_stream,
	pub ops: *const snd_soc_dai_ops,
}

#[repr(C)]
pub struct reg_default {
	pub reg: c_uint,
	pub def: c_uint,
}

#[repr(C)]
pub struct regmap_config {
	pub val_bits: c_uint,
	pub reg_bits: c_uint,
	pub max_register: c_uint,
	pub reg_defaults: *const reg_default,
	pub num_reg_defaults: c_uint,
	pub volatile_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool>,
	pub cache_type: c_uint,
}

#[repr(C)]
struct src4xxx {
	regmap: *mut regmap,
	master: [bool; 2],
	mclk_hz: c_int,
	dev: *mut device,
}

const SRC4XXX_PORTA: c_int = 0;
const SRC4XXX_PORTB: c_int = 1;

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const GFP_KERNEL: c_uint = 0;

unsafe extern "C" {
	fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
	fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
	fn regmap_update_bits(
		map: *mut regmap,
		reg: c_uint,
		mask: c_uint,
		val: c_uint,
	) -> c_int;
	fn regmap_write(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
	fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
	fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
	fn devm_snd_soc_register_component(
		dev: *mut device,
		cmpnt_drv: *const snd_soc_component_driver,
		dai_drv: *mut snd_soc_dai_driver,
		num_dai: c_int,
	) -> c_int;
	fn usleep_range(min: c_uint, max: c_uint);
	fn IS_ERR(ptr: *const c_void) -> bool;
	fn PTR_ERR(ptr: *const c_void) -> c_int;
	fn dev_info(dev: *mut device, fmt: *const c_char, ...);
	fn dev_err(dev: *mut device, fmt: *const c_char, ...);
}

unsafe extern "C" {
	static src_tlv: c_uint;
	static src4xxx_controls: [snd_kcontrol_new; 1];
	static porta_out_control: snd_kcontrol_new;
	static portb_out_control: snd_kcontrol_new;
	static dit_mux_control: snd_kcontrol_new;
	static src_in_control: snd_kcontrol_new;
	static dir_in_control: snd_kcontrol_new;
}

// SRC attenuation
// static const DECLARE_TLV_DB_SCALE(src_tlv, -12750, 50, 0);
// static const struct snd_kcontrol_new src4xxx_controls[] =
//     SOC_DOUBLE_R_TLV("SRC Volume", SRC4XXX_SCR_CTL_30, SRC4XXX_SCR_CTL_31, 0, 255, 1, src_tlv);

// I2S port control
static port_out_src_text: [*const c_char; 4] = [
	b"loopback\0".as_ptr() as *const c_char,
	b"other_port\0".as_ptr() as *const c_char,
	b"DIR\0".as_ptr() as *const c_char,
	b"SRC\0".as_ptr() as *const c_char,
];
// static SOC_ENUM_SINGLE_DECL(porta_out_src_enum, SRC4XXX_PORTA_CTL_03, 4, port_out_src_text);
// static SOC_ENUM_SINGLE_DECL(portb_out_src_enum, SRC4XXX_PORTB_CTL_05, 4, port_out_src_text);
// static const struct snd_kcontrol_new porta_out_control =
//     SOC_DAPM_ENUM("Port A source select", porta_out_src_enum);
// static const struct snd_kcontrol_new portb_out_control =
//     SOC_DAPM_ENUM("Port B source select", portb_out_src_enum);

// Digital audio transmitter control
static dit_mux_text: [*const c_char; 4] = [
	b"Port A\0".as_ptr() as *const c_char,
	b"Port B\0".as_ptr() as *const c_char,
	b"DIR\0".as_ptr() as *const c_char,
	b"SRC\0".as_ptr() as *const c_char,
];
// static SOC_ENUM_SINGLE_DECL(dit_mux_enum, SRC4XXX_TX_CTL_07, 3, dit_mux_text);
// static const struct snd_kcontrol_new dit_mux_control =
//     SOC_DAPM_ENUM("DIT source", dit_mux_enum);

// SRC control
static src_in_text: [*const c_char; 3] = [
	b"Port A\0".as_ptr() as *const c_char,
	b"Port B\0".as_ptr() as *const c_char,
	b"DIR\0".as_ptr() as *const c_char,
];
// static SOC_ENUM_SINGLE_DECL(src_in_enum, SRC4XXX_SCR_CTL_2D, 0, src_in_text);
// static const struct snd_kcontrol_new src_in_control =
//     SOC_DAPM_ENUM("SRC source select", src_in_enum);

// DIR control
static dir_in_text: [*const c_char; 4] = [
	b"Ch 1\0".as_ptr() as *const c_char,
	b"Ch 2\0".as_ptr() as *const c_char,
	b"Ch 3\0".as_ptr() as *const c_char,
	b"Ch 4\0".as_ptr() as *const c_char,
];
// static SOC_ENUM_SINGLE_DECL(dir_in_enum, SRC4XXX_RCV_CTL_0D, 0, dir_in_text);
// static const struct snd_kcontrol_new dir_in_control =
//     SOC_DAPM_ENUM("Digital Input", dir_in_enum);

// static const struct snd_soc_dapm_widget src4xxx_dapm_widgets[] = {
//     SND_SOC_DAPM_INPUT("loopback_A"), ...
//     SND_SOC_DAPM_MUX("Digital Input", SRC4XXX_PWR_RST_01,
//         SRC4XXX_ENABLE_DIR_SHIFT, 1, &dir_in_control),
// };
unsafe extern "C" {
	static src4xxx_dapm_widgets: [snd_soc_dapm_widget; 27];
}

static src4xxx_audio_routes: [snd_soc_dapm_route; 24] = [
	snd_soc_dapm_route { sink: b"Port A source\0".as_ptr() as *const c_char, control: b"loopback\0".as_ptr() as *const c_char, source: b"loopback_A\0".as_ptr() as *const c_char },
	snd_soc_dapm_route { sink: b"Port A source\0".as_ptr() as *const c_char, control: b"other_port\0".as_ptr() as *const c_char, source: b"other_port_A\0".as_ptr() as *const c_char },
	snd_soc_dapm_route { sink: b"Port A source\0".as_ptr() as *const c_char, control: b"DIR\0".as_ptr() as *const c_char, source: b"DIR_A\0".as_ptr() as *const c_char },
	snd_soc_dapm_route { sink: b"Port A source\0".as_ptr() as *const c_char, control: b"SRC\0".as_ptr() as *const c_char, source: b"SRC_A\0".as_ptr() as *const c_char },
	snd_soc_dapm_route { sink: b"Port B source\0".as_ptr() as *const c_char, control: b"loopback\0".as_ptr() as *const c_char, source: b"loopback_B\0".as_ptr() as *const c_char },
	snd_soc_dapm_route { sink: b"Port B source\0".as_ptr() as *const c_char, control: b"other_port\0".as_ptr() as *const c_char, source: b"other_port_B\0".as_ptr() as *const c_char },
	snd_soc_dapm_route { sink: b"Port B source\0".as_ptr() as *const c_char, control: b"DIR\0".as_ptr() as *const c_char, source: b"DIR_B\0".as_ptr() as *const c_char },
	snd_soc_dapm_route { sink: b"Port B source\0".as_ptr() as *const c_char, control: b"SRC\0".as_ptr() as *const c_char, source: b"SRC_B\0".as_ptr() as *const c_char },
	snd_soc_dapm_route { sink: b"DIT Out Src\0".as_ptr() as *const c_char, control: b"Port A\0".as_ptr() as *const c_char, source: b"Capture A\0".as_ptr() as *const c_char },
	snd_soc_dapm_route { sink: b"DIT Out Src\0".as_ptr() as *const c_char, control: b"Port B\0".as_ptr() as *const c_char, source: b"Capture B\0".as_ptr() as *const c_char },
	snd_soc_dapm_route { sink: b"DIT Out Src\0".as_ptr() as *const c_char, control: b"DIR\0".as_ptr() as *const c_char, source: b"DIR_OUT\0".as_ptr() as *const c_char },
	snd_soc_dapm_route { sink: b"DIT Out Src\0".as_ptr() as *const c_char, control: b"SRC\0".as_ptr() as *const c_char, source: b"SRC_OUT\0".as_ptr() as *const c_char },
	snd_soc_dapm_route { sink: b"SRC source\0".as_ptr() as *const c_char, control: b"Port A\0".as_ptr() as *const c_char, source: b"Port_A\0".as_ptr() as *const c_char },
	snd_soc_dapm_route { sink: b"SRC source\0".as_ptr() as *const c_char, control: b"Port B\0".as_ptr() as *const c_char, source: b"Port_B\0".as_ptr() as *const c_char },
	snd_soc_dapm_route { sink: b"SRC source\0".as_ptr() as *const c_char, control: b"DIR\0".as_ptr() as *const c_char, source: b"DIR_\0".as_ptr() as *const c_char },
	snd_soc_dapm_route { sink: b"SRC mclk source\0".as_ptr() as *const c_char, control: b"Master (MCLK)\0".as_ptr() as *const c_char, source: b"MCLK\0".as_ptr() as *const c_char },
	snd_soc_dapm_route { sink: b"SRC mclk source\0".as_ptr() as *const c_char, control: b"Master (RXCLKI)\0".as_ptr() as *const c_char, source: b"RXMCLKI\0".as_ptr() as *const c_char },
	snd_soc_dapm_route { sink: b"SRC mclk source\0".as_ptr() as *const c_char, control: b"Recovered receiver clk\0".as_ptr() as *const c_char, source: b"RXMCLKO\0".as_ptr() as *const c_char },
	snd_soc_dapm_route { sink: b"Digital Input\0".as_ptr() as *const c_char, control: b"Ch 1\0".as_ptr() as *const c_char, source: b"RX1\0".as_ptr() as *const c_char },
	snd_soc_dapm_route { sink: b"Digital Input\0".as_ptr() as *const c_char, control: b"Ch 2\0".as_ptr() as *const c_char, source: b"RX2\0".as_ptr() as *const c_char },
	snd_soc_dapm_route { sink: b"Digital Input\0".as_ptr() as *const c_char, control: b"Ch 3\0".as_ptr() as *const c_char, source: b"RX3\0".as_ptr() as *const c_char },
	snd_soc_dapm_route { sink: b"Digital Input\0".as_ptr() as *const c_char, control: b"Ch 4\0".as_ptr() as *const c_char, source: b"RX4\0".as_ptr() as *const c_char },
];

static src4xxx_driver: snd_soc_component_driver = snd_soc_component_driver {
	controls: unsafe { src4xxx_controls.as_ptr() },
	num_controls: 1,
	dapm_widgets: unsafe { src4xxx_dapm_widgets.as_ptr() },
	num_dapm_widgets: 27,
	dapm_routes: src4xxx_audio_routes.as_ptr(),
	num_dapm_routes: 24,
};

unsafe extern "C" fn src4xxx_set_dai_fmt(dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
	let component = (*dai).component;
	let src4xxx = snd_soc_component_get_drvdata(component) as *mut src4xxx;
	let mut ctrl: c_uint;

	match fmt & SND_SOC_DAIFMT_MASTER_MASK {
		SND_SOC_DAIFMT_CBP_CFP => {
			ctrl = SRC4XXX_BUS_MASTER;
			(*src4xxx).master[(*dai).id as usize] = true;
		}
		SND_SOC_DAIFMT_CBC_CFC => {
			ctrl = 0;
			(*src4xxx).master[(*dai).id as usize] = false;
		}
		_ => return -EINVAL,
	}

	match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
		SND_SOC_DAIFMT_I2S => ctrl |= SRC4XXX_BUS_I2S,
		SND_SOC_DAIFMT_LEFT_J => ctrl |= SRC4XXX_BUS_LEFT_J,
		SND_SOC_DAIFMT_RIGHT_J => ctrl |= SRC4XXX_BUS_RIGHT_J_24,
		_ => return -EINVAL,
	}

	match fmt & SND_SOC_DAIFMT_INV_MASK {
		SND_SOC_DAIFMT_NB_NF => {}
		_ => return -EINVAL,
	}

	regmap_update_bits(
		(*src4xxx).regmap,
		SRC4XXX_BUS_FMT((*dai).id),
		SRC4XXX_BUS_FMT_MS_MASK,
		ctrl,
	);

	0
}

unsafe extern "C" fn src4xxx_set_mclk_hz(
	codec_dai: *mut snd_soc_dai,
	_clk_id: c_int,
	freq: c_uint,
	_dir: c_int,
) -> c_int {
	let component = (*codec_dai).component;
	let src4xxx = snd_soc_component_get_drvdata(component) as *mut src4xxx;

	dev_info(
		(*component).dev,
		b"changing mclk rate from %d to %d Hz\n\0".as_ptr() as *const c_char,
		(*src4xxx).mclk_hz,
		freq,
	);
	(*src4xxx).mclk_hz = freq as c_int;

	0
}

unsafe extern "C" fn src4xxx_hw_params(
	_substream: *mut snd_pcm_substream,
	params: *mut snd_pcm_hw_params,
	dai: *mut snd_soc_dai,
) -> c_int {
	let component = (*dai).component;
	let src4xxx = snd_soc_component_get_drvdata(component) as *mut src4xxx;
	let mclk_div: c_uint;
	let mut val: c_int;
	let mut pj: c_int;
	let mut jd: c_int;
	let mut d: c_int;
	let reg: c_int;
	let mut ret: c_int;

	match (*dai).id {
		SRC4XXX_PORTB => reg = SRC4XXX_PORTB_CTL_06 as c_int,
		_ => reg = SRC4XXX_PORTA_CTL_04 as c_int,
	}

	if (*src4xxx).master[(*dai).id as usize] {
		mclk_div = ((*src4xxx).mclk_hz as c_uint) / params_rate(params);
		if (*src4xxx).mclk_hz as c_uint != mclk_div.wrapping_mul(params_rate(params)) {
			dev_err(
				(*component).dev,
				b"mclk %d / rate %d has a remainder.\n\0".as_ptr() as *const c_char,
				(*src4xxx).mclk_hz,
				params_rate(params),
			);
			return -EINVAL;
		}

		val = (mclk_div as c_int - 128) / 128;
		if (val < 0) | (val > 3) {
			dev_err(
				(*component).dev,
				b"div register setting %d is out of range\n\0".as_ptr() as *const c_char,
				val,
			);
			dev_err(
				(*component).dev,
				b"unsupported sample rate %d Hz for the master clock of %d Hz\n\0".as_ptr()
					as *const c_char,
				params_rate(params),
				(*src4xxx).mclk_hz,
			);
			return -EINVAL;
		}

		// set the TX DIV
		ret = regmap_update_bits(
			(*src4xxx).regmap,
			SRC4XXX_TX_CTL_07,
			SRC4XXX_TX_MCLK_DIV_MASK,
			(val as c_uint) << SRC4XXX_TX_MCLK_DIV_SHIFT,
		);
		if ret != 0 {
			dev_err(
				(*component).dev,
				b"Couldn't set the TX's div register to %d << %d = 0x%x\n\0".as_ptr()
					as *const c_char,
				val,
				SRC4XXX_TX_MCLK_DIV_SHIFT,
				(val as c_uint) << SRC4XXX_TX_MCLK_DIV_SHIFT,
			);
			return ret;
		}

		// set the PLL for the digital receiver
		match (*src4xxx).mclk_hz {
			24576000 => {
				pj = 0x22;
				jd = 0x00;
				d = 0x00;
			}
			22579200 => {
				pj = 0x22;
				jd = 0x1b;
				d = 0xa3;
			}
			_ => {
				// don't error out here,
				// other parts of the chip are still functional
				// Dummy initialize variables to avoid
				// -Wsometimes-uninitialized from clang.
				dev_info(
					(*component).dev,
					b"Couldn't set the RCV PLL as this master clock rate is unknown. Chosen regmap values may not match real world values.\n\0".as_ptr()
						as *const c_char,
				);
				pj = 0x0;
				jd = 0xff;
				d = 0xff;
			}
		}
		ret = regmap_write((*src4xxx).regmap, SRC4XXX_RCV_PLL_0F, pj as c_uint);
		if ret < 0 {
			dev_err(
				(*component).dev,
				b"Failed to update PLL register 0x%x\n\0".as_ptr() as *const c_char,
				SRC4XXX_RCV_PLL_0F,
			);
		}
		ret = regmap_write((*src4xxx).regmap, SRC4XXX_RCV_PLL_10, jd as c_uint);
		if ret < 0 {
			dev_err(
				(*component).dev,
				b"Failed to update PLL register 0x%x\n\0".as_ptr() as *const c_char,
				SRC4XXX_RCV_PLL_10,
			);
		}
		ret = regmap_write((*src4xxx).regmap, SRC4XXX_RCV_PLL_11, d as c_uint);
		if ret < 0 {
			dev_err(
				(*component).dev,
				b"Failed to update PLL register 0x%x\n\0".as_ptr() as *const c_char,
				SRC4XXX_RCV_PLL_11,
			);
		}

		ret = regmap_update_bits(
			(*src4xxx).regmap,
			SRC4XXX_TX_CTL_07,
			SRC4XXX_TX_MCLK_DIV_MASK,
			(val as c_uint) << SRC4XXX_TX_MCLK_DIV_SHIFT,
		);
		if ret < 0 {
			dev_err(
				(*component).dev,
				b"Couldn't set the TX's div register to %d << %d = 0x%x\n\0".as_ptr()
					as *const c_char,
				val,
				SRC4XXX_TX_MCLK_DIV_SHIFT,
				(val as c_uint) << SRC4XXX_TX_MCLK_DIV_SHIFT,
			);
			return ret;
		}

		return regmap_update_bits(
			(*src4xxx).regmap,
			reg as c_uint,
			SRC4XXX_MCLK_DIV_MASK,
			val as c_uint,
		);
	} else {
		dev_info(
			(*dai).dev,
			b"not setting up MCLK as not master\n\0".as_ptr() as *const c_char,
		);
	}

	0
}

static src4xxx_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
	hw_params: Some(src4xxx_hw_params),
	set_sysclk: Some(src4xxx_set_mclk_hz),
	set_fmt: Some(src4xxx_set_dai_fmt),
};

const SRC4XXX_FORMATS: c_uint = SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S32_LE;
const SRC4XXX_RATES: c_uint = SNDRV_PCM_RATE_44100
	| SNDRV_PCM_RATE_48000
	| SNDRV_PCM_RATE_88200
	| SNDRV_PCM_RATE_96000
	| SNDRV_PCM_RATE_176400
	| SNDRV_PCM_RATE_192000;

static mut src4xxx_dai_driver: [snd_soc_dai_driver; 2] = [
	snd_soc_dai_driver {
		id: SRC4XXX_PORTA,
		name: b"src4xxx-portA\0".as_ptr() as *const c_char,
		playback: snd_soc_pcm_stream {
			stream_name: b"Playback A\0".as_ptr() as *const c_char,
			channels_min: 2,
			channels_max: 2,
			rates: SRC4XXX_RATES,
			formats: SRC4XXX_FORMATS,
		},
		capture: snd_soc_pcm_stream {
			stream_name: b"Capture A\0".as_ptr() as *const c_char,
			channels_min: 2,
			channels_max: 2,
			rates: SRC4XXX_RATES,
			formats: SRC4XXX_FORMATS,
		},
		ops: &src4xxx_dai_ops,
	},
	snd_soc_dai_driver {
		id: SRC4XXX_PORTB,
		name: b"src4xxx-portB\0".as_ptr() as *const c_char,
		playback: snd_soc_pcm_stream {
			stream_name: b"Playback B\0".as_ptr() as *const c_char,
			channels_min: 2,
			channels_max: 2,
			rates: SRC4XXX_RATES,
			formats: SRC4XXX_FORMATS,
		},
		capture: snd_soc_pcm_stream {
			stream_name: b"Capture B\0".as_ptr() as *const c_char,
			channels_min: 2,
			channels_max: 2,
			rates: SRC4XXX_RATES,
			formats: SRC4XXX_FORMATS,
		},
		ops: &src4xxx_dai_ops,
	},
];

static src4xxx_reg_defaults: [reg_default; 29] = [
	reg_default { reg: SRC4XXX_PWR_RST_01, def: 0x00 }, // all powered down intially
	reg_default { reg: SRC4XXX_PORTA_CTL_03, def: 0x00 },
	reg_default { reg: SRC4XXX_PORTA_CTL_04, def: 0x00 },
	reg_default { reg: SRC4XXX_PORTB_CTL_05, def: 0x00 },
	reg_default { reg: SRC4XXX_PORTB_CTL_06, def: 0x00 },
	reg_default { reg: SRC4XXX_TX_CTL_07, def: 0x00 },
	reg_default { reg: SRC4XXX_TX_CTL_08, def: 0x00 },
	reg_default { reg: SRC4XXX_TX_CTL_09, def: 0x00 },
	reg_default { reg: SRC4XXX_SRC_DIT_IRQ_MSK_0B, def: 0x00 },
	reg_default { reg: SRC4XXX_SRC_DIT_IRQ_MODE_0C, def: 0x00 },
	reg_default { reg: SRC4XXX_RCV_CTL_0D, def: 0x00 },
	reg_default { reg: SRC4XXX_RCV_CTL_0E, def: 0x00 },
	reg_default { reg: SRC4XXX_RCV_PLL_0F, def: 0x00 }, // not spec. in the datasheet
	reg_default { reg: SRC4XXX_RCV_PLL_10, def: 0xff }, // not spec. in the datasheet
	reg_default { reg: SRC4XXX_RCV_PLL_11, def: 0xff }, // not spec. in the datasheet
	reg_default { reg: SRC4XXX_RVC_IRQ_MSK_16, def: 0x00 },
	reg_default { reg: SRC4XXX_RVC_IRQ_MSK_17, def: 0x00 },
	reg_default { reg: SRC4XXX_RVC_IRQ_MODE_18, def: 0x00 },
	reg_default { reg: SRC4XXX_RVC_IRQ_MODE_19, def: 0x00 },
	reg_default { reg: SRC4XXX_RVC_IRQ_MODE_1A, def: 0x00 },
	reg_default { reg: SRC4XXX_GPIO_1_1B, def: 0x00 },
	reg_default { reg: SRC4XXX_GPIO_2_1C, def: 0x00 },
	reg_default { reg: SRC4XXX_GPIO_3_1D, def: 0x00 },
	reg_default { reg: SRC4XXX_GPIO_4_1E, def: 0x00 },
	reg_default { reg: SRC4XXX_SCR_CTL_2D, def: 0x00 },
	reg_default { reg: SRC4XXX_SCR_CTL_2E, def: 0x00 },
	reg_default { reg: SRC4XXX_SCR_CTL_2F, def: 0x00 },
	reg_default { reg: SRC4XXX_SCR_CTL_30, def: 0x00 },
	reg_default { reg: SRC4XXX_SCR_CTL_31, def: 0x00 },
];

#[no_mangle]
pub unsafe extern "C" fn src4xxx_probe(
	dev: *mut device,
	regmap: *mut regmap,
	_switch_mode: Option<unsafe extern "C" fn(*mut device)>,
) -> c_int {
	let src4xxx: *mut src4xxx;
	let mut ret: c_int;

	if IS_ERR(regmap as *const c_void) {
		return PTR_ERR(regmap as *const c_void);
	}

	src4xxx = devm_kzalloc(dev, core::mem::size_of::<src4xxx>(), GFP_KERNEL) as *mut src4xxx;
	if src4xxx.is_null() {
		return -ENOMEM;
	}

	(*src4xxx).regmap = regmap;
	(*src4xxx).dev = dev;
	(*src4xxx).mclk_hz = 0; // mclk has not been configured yet
	dev_set_drvdata(dev, src4xxx as *mut c_void);

	ret = regmap_write(regmap, SRC4XXX_PWR_RST_01, SRC4XXX_RESET);
	if ret < 0 {
		dev_err(dev, b"Failed to issue reset: %d\n\0".as_ptr() as *const c_char, ret);
	}
	usleep_range(1, 500); // sleep for more then 500 ns
	ret = regmap_write(regmap, SRC4XXX_PWR_RST_01, SRC4XXX_POWER_DOWN);
	if ret < 0 {
		dev_err(
			dev,
			b"Failed to decommission reset: %d\n\0".as_ptr() as *const c_char,
			ret,
		);
	}
	usleep_range(500, 1000); // sleep for 500 us or more

	ret = regmap_update_bits(
		(*src4xxx).regmap,
		SRC4XXX_PWR_RST_01,
		SRC4XXX_POWER_ENABLE,
		SRC4XXX_POWER_ENABLE,
	);
	if ret < 0 {
		dev_err(dev, b"Failed to port A and B : %d\n\0".as_ptr() as *const c_char, ret);
	}

	// set receiver to use master clock (rcv mclk is most likely jittery)
	ret = regmap_update_bits(
		(*src4xxx).regmap,
		SRC4XXX_RCV_CTL_0D,
		SRC4XXX_RXCLK_MCLK,
		SRC4XXX_RXCLK_MCLK,
	);
	if ret < 0 {
		dev_err(
			dev,
			b"Failed to enable mclk as the PLL1 DIR reference : %d\n\0".as_ptr()
				as *const c_char,
			ret,
		);
	}

	// default to leaving the PLL2 running on loss of lock, divide by 8
	ret = regmap_update_bits(
		(*src4xxx).regmap,
		SRC4XXX_RCV_CTL_0E,
		SRC4XXX_PLL2_DIV_8 | SRC4XXX_REC_MCLK_EN | SRC4XXX_PLL2_LOL,
		SRC4XXX_PLL2_DIV_8 | SRC4XXX_REC_MCLK_EN | SRC4XXX_PLL2_LOL,
	);
	if ret < 0 {
		dev_err(
			dev,
			b"Failed to enable mclk rec and div : %d\n\0".as_ptr() as *const c_char,
			ret,
		);
	}

	ret = devm_snd_soc_register_component(
		dev,
		&src4xxx_driver,
		src4xxx_dai_driver.as_mut_ptr(),
		src4xxx_dai_driver.len() as c_int,
	);
	if ret == 0 {
		dev_info(dev, b"src4392 probe ok %d\n\0".as_ptr() as *const c_char, ret);
	}
	ret
}

unsafe extern "C" fn src4xxx_volatile_register(_dev: *mut device, reg: c_uint) -> bool {
	match reg {
		SRC4XXX_RES_00
		| SRC4XXX_GLOBAL_ITR_STS_02
		| SRC4XXX_SRC_DIT_STS_0A
		| SRC4XXX_NON_AUDIO_D_12
		| SRC4XXX_RVC_STS_13
		| SRC4XXX_RVC_STS_14
		| SRC4XXX_RVC_STS_15
		| SRC4XXX_SUB_CODE_1F
		| SRC4XXX_SUB_CODE_20
		| SRC4XXX_SUB_CODE_21
		| SRC4XXX_SUB_CODE_22
		| SRC4XXX_SUB_CODE_23
		| SRC4XXX_SUB_CODE_24
		| SRC4XXX_SUB_CODE_25
		| SRC4XXX_SUB_CODE_26
		| SRC4XXX_SUB_CODE_27
		| SRC4XXX_SUB_CODE_28
		| SRC4XXX_PC_PREAMBLE_HI_29
		| SRC4XXX_PC_PREAMBLE_LO_2A
		| SRC4XXX_PD_PREAMBLE_HI_2B
		| SRC4XXX_PC_PREAMBLE_LO_2C
		| SRC4XXX_IO_RATIO_32
		| SRC4XXX_IO_RATIO_33 => return true,
		_ => {}
	}

	if reg > SRC4XXX_IO_RATIO_33 && reg < SRC4XXX_PAGE_SEL_7F {
		return true;
	}

	false
}

#[no_mangle]
pub static src4xxx_regmap_config: regmap_config = regmap_config {
	val_bits: 8,
	reg_bits: 8,
	max_register: SRC4XXX_IO_RATIO_33,
	reg_defaults: src4xxx_reg_defaults.as_ptr(),
	num_reg_defaults: 29,
	volatile_reg: Some(src4xxx_volatile_register),
	cache_type: REGCACHE_RBTREE,
};

// EXPORT_SYMBOL_GPL(src4xxx_probe);
// EXPORT_SYMBOL_GPL(src4xxx_regmap_config);
// MODULE_DESCRIPTION("ASoC SRC4XXX CODEC driver");
// MODULE_AUTHOR("Matt Flax <flatmax@flatmax.com>");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
